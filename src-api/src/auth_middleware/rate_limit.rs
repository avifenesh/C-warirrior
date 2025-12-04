//! Adaptive rate limiting middleware
//!
//! Rate limits are based on user XP level:
//! - XP 0-99:     30 requests/min
//! - XP 100-499:  60 requests/min
//! - XP 500-1999: 120 requests/min
//! - XP 2000+:    240 requests/min
//!
//! For unauthenticated requests (auth endpoints), uses IP-based limiting.

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use dashmap::DashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::auth::AuthUser;

/// Rate limit configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Requests per minute for each XP tier
    pub xp_0_99: u32,
    pub xp_100_499: u32,
    pub xp_500_1999: u32,
    pub xp_2000_plus: u32,
    /// Default rate for unauthenticated requests (by IP)
    pub unauthenticated: u32,
    /// Rate for auth endpoints (stricter)
    pub auth_endpoints: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            xp_0_99: 30,
            xp_100_499: 60,
            xp_500_1999: 120,
            xp_2000_plus: 240,
            unauthenticated: 30,
            auth_endpoints: 10, // Stricter for login/register to prevent brute force
        }
    }
}

/// Rate limit entry for a single identifier
#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: u32,
    window_start: Instant,
}

/// In-memory rate limiter state
#[derive(Debug)]
pub struct RateLimiter {
    /// Map of identifier -> rate limit entry
    entries: DashMap<String, RateLimitEntry>,
    config: RateLimitConfig,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            entries: DashMap::new(),
            config,
            window_duration: Duration::from_secs(60), // 1 minute window
        }
    }

    /// Check if request should be rate limited
    /// Returns Ok(remaining) if allowed, Err(retry_after_secs) if limited
    pub fn check_rate_limit(&self, identifier: &str, limit: u32) -> Result<u32, u64> {
        let now = Instant::now();

        let mut entry = self.entries.entry(identifier.to_string()).or_insert(RateLimitEntry {
            count: 0,
            window_start: now,
        });

        // Check if window has expired
        if now.duration_since(entry.window_start) >= self.window_duration {
            // Reset window
            entry.count = 1;
            entry.window_start = now;
            return Ok(limit - 1);
        }

        // Check if limit exceeded
        if entry.count >= limit {
            let retry_after = self.window_duration.as_secs()
                - now.duration_since(entry.window_start).as_secs();
            return Err(retry_after);
        }

        // Increment count
        entry.count += 1;
        Ok(limit - entry.count)
    }
}

/// Shared rate limiter state
pub type SharedRateLimiter = Arc<RateLimiter>;

/// Create a new shared rate limiter
pub fn create_rate_limiter() -> SharedRateLimiter {
    Arc::new(RateLimiter::new(RateLimitConfig::default()))
}

/// Extract client IP from request
fn get_client_ip(req: &Request<Body>) -> Option<IpAddr> {
    // Try X-Forwarded-For header first (for proxied requests)
    if let Some(forwarded) = req.headers().get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            if let Some(first_ip) = forwarded_str.split(',').next() {
                if let Ok(ip) = first_ip.trim().parse() {
                    return Some(ip);
                }
            }
        }
    }

    // Try X-Real-IP header
    if let Some(real_ip) = req.headers().get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            if let Ok(ip) = ip_str.parse() {
                return Some(ip);
            }
        }
    }

    // Fallback: would need connection info which isn't available in middleware
    None
}

/// Rate limiting middleware for authenticated routes
///
/// Uses user XP to determine rate limit tier.
pub async fn rate_limit_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get rate limiter from extensions (must be added in app setup)
    let rate_limiter = req
        .extensions()
        .get::<SharedRateLimiter>()
        .cloned()
        .unwrap_or_else(create_rate_limiter);

    // Determine identifier and limit
    let (identifier, limit) = if let Some(auth_user) = req.extensions().get::<AuthUser>() {
        // Authenticated user - use user_id and XP-based limit
        let xp = auth_user.xp;
        let actual_limit = match xp {
            0..=99 => rate_limiter.config.xp_0_99,
            100..=499 => rate_limiter.config.xp_100_499,
            500..=1999 => rate_limiter.config.xp_500_1999,
            _ => rate_limiter.config.xp_2000_plus,
        };
        (format!("user:{}", auth_user.user_id), actual_limit)
    } else {
        // Unauthenticated - use IP-based limit
        let ip = get_client_ip(&req)
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        (format!("ip:{}", ip), rate_limiter.config.unauthenticated)
    };

    // Check rate limit
    match rate_limiter.check_rate_limit(&identifier, limit) {
        Ok(remaining) => {
            // Add rate limit headers to response
            let mut response = next.run(req).await;
            response.headers_mut().insert(
                "X-RateLimit-Limit",
                limit.to_string().parse().unwrap(),
            );
            response.headers_mut().insert(
                "X-RateLimit-Remaining",
                remaining.to_string().parse().unwrap(),
            );
            Ok(response)
        }
        Err(retry_after) => {
            tracing::warn!("Rate limit exceeded for {}", identifier);
            let mut response = Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .body(Body::from("Rate limit exceeded"))
                .unwrap();
            response.headers_mut().insert(
                "Retry-After",
                retry_after.to_string().parse().unwrap(),
            );
            Err(StatusCode::TOO_MANY_REQUESTS)
        }
    }
}

/// Stricter rate limiting for auth endpoints
///
/// Uses IP-based limiting with lower thresholds to prevent brute force attacks.
pub async fn auth_rate_limit_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let rate_limiter = req
        .extensions()
        .get::<SharedRateLimiter>()
        .cloned()
        .unwrap_or_else(create_rate_limiter);

    // Always use IP for auth endpoints
    let ip = get_client_ip(&req)
        .map(|ip| ip.to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let identifier = format!("auth:{}", ip);

    // Use stricter auth endpoint limit
    match rate_limiter.check_rate_limit(&identifier, rate_limiter.config.auth_endpoints) {
        Ok(_) => Ok(next.run(req).await),
        Err(_retry_after) => {
            tracing::warn!("Auth rate limit exceeded for IP {}", ip);
            Err(StatusCode::TOO_MANY_REQUESTS)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_config_default() {
        let config = RateLimitConfig::default();
        assert_eq!(config.xp_0_99, 30);
        assert_eq!(config.xp_100_499, 60);
        assert_eq!(config.xp_500_1999, 120);
        assert_eq!(config.xp_2000_plus, 240);
    }

    #[test]
    fn test_rate_limiter_allows_requests() {
        let limiter = RateLimiter::new(RateLimitConfig::default());
        
        // First request should be allowed
        let result = limiter.check_rate_limit("test", 30);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 29);
    }

    #[test]
    fn test_rate_limiter_blocks_excess() {
        let limiter = RateLimiter::new(RateLimitConfig::default());

        // Exhaust the limit
        for _ in 0..30 {
            let _ = limiter.check_rate_limit("test", 30);
        }

        // Next request should be blocked
        let result = limiter.check_rate_limit("test", 30);
        assert!(result.is_err());
    }
}


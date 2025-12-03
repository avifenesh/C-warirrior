//! Authentication middleware module
//!
//! Middleware stack order:
//! Request → JWT Validation → Ban Check → Rate Limit → Handler

pub mod auth;
pub mod ban;
pub mod rate_limit;

// Re-export commonly used types
pub use auth::{AuthUser, JwtClaims, jwt_auth_middleware, optional_jwt_auth_middleware, validate_token};
pub use ban::{BanResponse, ban_check_middleware, lightweight_ban_check_middleware};
pub use rate_limit::{create_rate_limiter, SharedRateLimiter, rate_limit_middleware, auth_rate_limit_middleware};


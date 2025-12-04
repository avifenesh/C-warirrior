//! Authentication middleware module
//!
//! Middleware stack order:
//! Request → JWT Validation → Ban Check → Rate Limit → Handler

pub mod auth;
pub mod ban;
pub mod rate_limit;
pub mod verification;

// Re-export commonly used types
pub use auth::{AuthUser, jwt_auth_middleware};
pub use ban::ban_check_middleware;
pub use rate_limit::{create_rate_limiter, SharedRateLimiter, rate_limit_middleware, auth_rate_limit_middleware};
pub use verification::verification_check_middleware;


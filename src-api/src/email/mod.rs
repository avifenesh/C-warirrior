//! Email service module for Code Warrior
//!
//! Uses Resend API for transactional emails (verification, password reset).

pub mod sender;
mod templates;

pub use sender::OptionalEmailService;


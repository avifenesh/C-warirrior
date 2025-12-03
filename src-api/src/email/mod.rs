//! Email service module for Code Warrior
//!
//! Uses Resend API for transactional emails (verification, password reset).

pub mod sender;
pub mod templates;

pub use sender::{EmailService, OptionalEmailService};
pub use templates::EmailTemplate;


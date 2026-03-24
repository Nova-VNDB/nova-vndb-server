//! # WAF Module
//!
//! The module that prevent the project from abusing. It provides CAPTCHA, anti-XSS, rate-limit and other security features for other modules.

#![forbid(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![forbid(clippy::expect_used)]
#![forbid(clippy::panic)]

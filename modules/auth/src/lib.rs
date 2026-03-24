//! # Auth Module
//!
//! The module handing user's login, register, RBAC. It provides RBAC and authentication layer for other modules.

#![forbid(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![forbid(clippy::expect_used)]
#![forbid(clippy::panic)]

pub mod api;
pub mod entities;
pub mod events;
pub mod hook;
pub mod services;
pub mod view;
pub mod utils;

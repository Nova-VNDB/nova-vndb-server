//! # Wiki Module
//!
//! The major module that handle all games' data.

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

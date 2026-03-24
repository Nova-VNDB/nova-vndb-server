//! # Community Module
//!
//! A major module that handle user's comment, discussion, rating.

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

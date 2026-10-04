//! This crate exposes two modules:
//!
//! # Crate structure
//! ## Api
//! This module contains a basic wrapper for libnotify calls. It just translates data types, etc,
//! so usage is basically the same as libnotify.
//!
//! ## Wrappers
//! This module is a higher level abstraction over api. Recommended to use over api.
//!
//! # Features
//!
//! - clap – enables [clap](https://docs.rs/clap/latest/clap/) support for `Urgency`
//!
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![warn(missing_docs)]
#![warn(clippy::missing_errors_doc)]

#[cfg(not(target_os = "linux"))]
compile_error!("Libnotify-rs only supports linux");

/// Basic libnotify rust wrapper
pub mod api;
/// Higher level abstraction over api
pub mod wrappers;

pub use wrappers::{builder, context};

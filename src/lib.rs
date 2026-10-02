//! This crate exposes two modules:
//!
//! # Api
//! This module contains a basic wrapper for libnotify calls. It just translates data types, etc,
//! so usage is basically the same as libnotify.
//!
//! # Wrappers
//! This module is a higher level abstraction over api. Recommended to use over api.
//!
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![warn(missing_docs)]

/// Basic libnotify rust wrapper
pub mod api;
/// Higher level abstraction over api
pub mod wrappers;

pub use wrappers::{builder, context};

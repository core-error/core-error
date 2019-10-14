//! Traits for working with Errors without std.
//!
//! # Usage
//! This crate simply provides an Error trait, which is identical to the one in std
//! except for not providing deprecated methods. It also contains two features:
//!
//! - `std` (default): simply reexport `std::error::Error`
//! - `alloc`: implement Error on alloc Errors (incl. Box)
//!
//! Libraries using this crate should forward their equivalent features to this library.
//!
//! # Minimum Rust Version
//!
//! This crate works all the way down to 1.0.0. It auto-detects the Rust version in
//! order to know which error structs to implement the trait on.
//!
//! With `no-default-features`, the crate only compiles from 1.6.0 onwards (version
//! at which `no_std` became stable).

// If std feature is disabled, this crate is no_std.
// This avoids making this crate fail on std build in version 1.5.0 and under
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unknown_lints)]
#![allow(bare_trait_objects)]
#![allow(where_clauses_object_safety)]

#[cfg(feature = "std")]
extern crate std as core;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
pub use std::error::*;

#[cfg(not(feature = "std"))]
mod error_trait;

/// Implementation of the not-yet-merged TypeInfo RFC, which is required for sound downcasting in
/// user crates.
#[doc(hidden)]
#[cfg(not(feature = "std"))]
pub mod typeinfo;

#[cfg(not(feature = "std"))]
pub use error_trait::*;

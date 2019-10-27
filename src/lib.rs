//! Traits for working with `Error` in `std`-less environments.
//!
//! # Warning
//!
//! This is a pre-release meant to allow experimentation and integration into
//! the various error handling crates. Please do not use it yet! 1.0.0 is right
//! around the corner.
//!
//! # Usage
//!
//! This crate provides an [`Error`](trait.Error.html) trait which is identical
//! to [the one in std][std-error], except for not providing deprecated methods.
//!
//! This library also exposes two opt-in features:
//!
//! - `std`: simply re-export [`std::error::Error`][std-error].
//! - `alloc`: implement the [`Error`](trait.Error.html) trait on `alloc` errors
//!   (including boxed types).
//!
//! Libraries using this crate should forward their equivalent features to this
//! library.
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate works all the way down to 1.0.0. It auto-detects the Rust version
//! in order to know which error structs to implement the trait on.
//!
//! With `no-default-features`, the crate only compiles from 1.6.0 onwards
//! (version at which `no_std` became stable).
//!
//! [std-error]: https://doc.rust-lang.org/std/error/trait.Error.html

// If std feature is disabled, this crate is no_std.
// On 1.5.0 and under, no_std is unstable, so we can't just unconditionally use it.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unknown_lints)]
#![allow(bare_trait_objects)]

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

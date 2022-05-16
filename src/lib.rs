#![no_std]

//!
//! A simple and flexible multidimensional coordinate point, based on an array.
//!
//! See the `PointND` struct for basic usage
//!
//! # Compatibility
//!
//! This crate is designed to be `#![no_std]` and `wasm` compatible, and has been tested in those environments.
//!
//! `PointND` uses constant generics, it is recommended for use with a Rust version of **at least 1.51**
//!
//! # Features
//!
//! - `conv_methods`
//!
//!     - **Enabled by default**
//!
//!     - Methods which access and transform the values contained by **1..=4** dimensional points.
//!
//!     - Enables the following sub-features (each of which can be enabled individually if needed):
//!
//!         - `x`: Convenience methods for `1D` points
//!
//!         - `y`: Convenience methods for `2D` points
//!
//!         - `z`: Convenience methods for `3D` points
//!
//!         - `w`: Convenience methods for `4D` points
//!
//! - `dim_macros`
//!
//!     - **Enabled by default**
//!
//!     - Macros which allow usize values to be generated from identifiers.
//!
//! - `appliers`
//!
//!     - **Enabled by default**
//!
//!     - Methods which allow function pointers to be passed to points in order to transform values.
//!
//!     - If this and the `var_dims` feature are disabled, this crate will include zero dependencies
//!
//! - `var_dims`
//!
//!     - Methods which append or remove values from points.
//!
//!     - If this and the `appliers` feature are disabled, this crate will include zero dependencies
//!

mod dimension_macros;
mod point;
mod utils;

pub use point::PointND;

#[cfg(feature = "appliers")]
pub use utils::{ApplyFn, ApplyDimsFn, ApplyValsFn, ApplyPointFn};

// For testing the README.md file
// - Commented out because it throws import errors in the README
/*
#[cfg(doctest)]
use doc_comment::doctest;

#[cfg(doctest)]
doctest!("../README.md");
 */
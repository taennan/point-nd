//!
//! A simple and flexible multidimensional coordinate point, based on an array.
//!
//! See the ```PointND``` struct for basic usage
//!
//! # Compatibility
//!
//! This crate is ```#![no_std]``` compatible.
//!
//! This crate uses constant generics, it is recommended for use with a Rust version of **at least 1.51**
//!
//! # Features
//!
//! - ```conv_methods```
//!
//!     - **Enabled by default**
//!
//!     - Methods which access and transform the values contained by **1..=4** dimensional points.
//!
//!     - Enables the following sub-features (each of which can be enabled individually if needed):
//!
//!         - ```x```: Methods for ```1D``` points
//!
//!         - ```y```: Methods for ```2D``` points
//!
//!         - ```z```: Methods for ```3D``` points
//!
//!         - ```w```: Methods for ```4D``` points
//!
//! - ```dim_macros```
//!
//!     - **Enabled by default**
//!
//!     - Macros which allow usize values to be generated from identifiers.
//!
//! - ```appliers```
//!
//!     - **Enabled by default**
//!
//!     - Methods which allow closures to be passed to points in order to transform values.
//!
//! - ```var_dims```
//!
//!     - Methods which append or remove values from points.
//!

#![no_std]

mod dimension_macros;
mod point;
mod utils;

pub use point::PointND;

#[cfg(feature = "appliers")]
pub use utils::{ApplyFn, ApplyDimsFn, ApplyValsFn, ApplyPointFn};
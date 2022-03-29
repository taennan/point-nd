#![no_std]
/*!

A simple and flexible multidimensional point struct, based on an array.

See the ```PointND``` struct for basic usage

# Compatibility

This crate is ```#![no_std]``` compatible.

This crate uses constant generics, it is recommended
for use with a Rust version of at least **1.51**

 */

mod dimension_macros;
mod point;
mod types;

pub use point::PointND;

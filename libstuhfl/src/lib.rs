//! # ST Ultra High Frequency RFID Library wrapper
//!
//! Safe rust wrappers for the libstuhfl C middleware library. See [`reader::Reader`] to get started.
//!
#![warn(missing_docs)]

#[macro_use]
extern crate enum_primitive;
extern crate num;

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate lazy_static;

extern crate ffi;

/* General Submodules */
pub mod data_types;
pub mod error;
pub mod gen2;
mod helpers; // helpers contains crate-wide helper functions
pub mod prelude;
pub mod reader;

/* Unit Testing Submodules */
#[cfg(test)]
mod tests;

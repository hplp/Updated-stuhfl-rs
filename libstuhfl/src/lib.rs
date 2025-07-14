//! # ST Ultra High Frequency RFID Library wrapper 
//!
//! Safe rust wrappers for the libstuhfl C middleware library. See [`reader::Reader`] to get started.
//!
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(rustdoc::missing_doc_code_examples)]

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
mod helpers;
pub mod prelude;
pub mod reader;

/* Unit Testing Submodules */
#[cfg(test)]
mod tests;

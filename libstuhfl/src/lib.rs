//! # ST Ultra High Frequency RFID Library wrapper
//! 
//! Safe rust wrappers for the libstuhfl C middleware library. See [`ST25RU3993`] to get started.
//! 
//! ## Example
//! 
//! ```no_run
//! use libstuhfl::*;
//! 
//! // Create a reader instance using port scanning to find it
//! # #[cfg(feature = "port_scanning")]
//! let mut reader = ST25RU3993::new().expect("Couldn't connect to reader");
//! 
//! // Create a reader instance with a hard-coded path
//! # #[cfg(not(feature = "port_scanning"))]
//! let mut reader = ST25RU3993::from_port("/dev/ttyUSB0").expect("Couldn't connect to reader");
//! 
//! // Check the reader's version
//! let version = reader.get_board_version().expect("Failed to get board version");
//! println!("Reader version: {}", &version);
//! 
//! // Configure the reader for Gen2 communication
//! let gen2_cfg = Gen2Cfg::builder()
//!     .build()
//!     .expect("Failed to build configuration instance");
//! 
//! reader.configure_gen2(&gen2_cfg);
//! ```

#![warn(missing_docs)]

#[macro_use]
extern crate enum_primitive;

extern crate num;
use num::FromPrimitive;

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate lazy_static;

use std::{fmt,mem};

extern crate ffi;

mod errors;
pub use errors::*;

mod data_types;
pub use data_types::*;

mod profile;
use profile::*;

mod st25ru3993;
pub use st25ru3993::*;

#[cfg(test)]
mod tests;
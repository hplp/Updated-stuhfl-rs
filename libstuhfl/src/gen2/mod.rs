// All good
//! Module pertaining to the Gen2 RFID protocol.

/* Note: All of these submodules are
 * so closely related, there's no point in
 * separating their namespaces... */

/// Reader submodule
mod gen2_reader;
pub use gen2_reader::Gen2Reader;

/* Gen2 related data types */

/// Enums for gen2
mod gen2_enums;
/// Structs for gen2
mod gen2_structs;
pub use gen2_enums::*;
pub use gen2_structs::*;

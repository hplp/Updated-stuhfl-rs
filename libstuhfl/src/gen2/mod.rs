//! Module pertaining to the Gen2 RFID protocol.

/* Note: All of these submodules are
 * so closely related, there's no point in
 * separating their namespaces... */

/* Reader Submodule */
mod gen2_reader;
pub use gen2_reader::Gen2Reader;

/* Gen2 related data types */
mod gen2_enums;
mod gen2_structs;
pub use gen2_enums::*;
pub use gen2_structs::*;

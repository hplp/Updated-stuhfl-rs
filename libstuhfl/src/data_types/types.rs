use super::structs::*;
use crate::error::Error;

/// Function type to be used with inventory_runner
pub type CallbackFn = dyn Fn(InventoryTag) + Send;

/// Custom result type that always uses [`libstuhfl::Error`] as error type.
pub type Result<T> = core::result::Result<T, Error>;

/// HexID type to be used only for XPC numbers.
pub type Xpc = HexID;

impl From<ffi::STUHFL_T_InventoryTagXPC> for Xpc {
    fn from(xpc: ffi::STUHFL_T_InventoryTagXPC) -> Xpc {
        Epc {
            id: Vec::from(&xpc.data[..xpc.length as usize]),
        }
    }
}

/// HexID type to be used only for EPC numbers.
pub type Epc = HexID;

impl From<ffi::STUHFL_T_InventoryTagEPC> for Epc {
    fn from(epc: ffi::STUHFL_T_InventoryTagEPC) -> Epc {
        Epc {
            id: Vec::from(&epc.data[..epc.length as usize]),
        }
    }
}

/// HexID type to be used only for TID numbers.
pub type Tid = HexID;

impl From<ffi::STUHFL_T_InventoryTagTID> for Tid {
    fn from(tid: ffi::STUHFL_T_InventoryTagTID) -> Tid {
        Tid {
            id: Vec::from(&tid.data[..tid.length as usize]),
        }
    }
}

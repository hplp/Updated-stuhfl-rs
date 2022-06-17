#[macro_use]
extern crate enum_primitive;

extern crate num;
use num::FromPrimitive;

#[macro_use]
extern crate derive_builder;

use std::{fmt,mem};

extern crate ffi;

mod errors;
pub use errors::*;

fn proc_err (code: ffi::STUHFL_T_RET_CODE) -> Result<(), Error> {
    if code == ffi::STUHFL_ERR_NONE {
        Ok(())
    } else {
        Err(Error::from_u32(code).unwrap())
    }
}

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u8)]
    pub enum Antenna {
        Antenna1 = ffi::STUHFL_D_ANTENNA_1 as u8,
        Antenna2 = ffi::STUHFL_D_ANTENNA_2 as u8,
        Antenna3 = ffi::STUHFL_D_ANTENNA_3 as u8,
        Antenna4 = ffi::STUHFL_D_ANTENNA_4 as u8,
        AntennaAlt = ffi::STUHFL_D_ANTENNA_ALT as u8
    }
}

// enum_from_primitive! {
//     #[derive(Debug, Copy, Clone, PartialEq)]
//     #[repr(u8)]
//     pub enum Profile {
//         Custom = ffi::STUHFL_D_PROFILE_CUSTOM as u8,
//         Europe = ffi::STUHFL_D_PROFILE_EUROPE as u8,
//         Usa = ffi::STUHFL_D_PROFILE_USA as u8,
//         Japan = ffi::STUHFL_D_PROFILE_JAPAN as u8,
//         China = ffi::STUHFL_D_PROFILE_CHINA as u8,
//         China2 = ffi::STUHFL_D_PROFILE_CHINA2 as u8,
//     }
// }

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u8)]
    pub enum TuningAlgorithm {
        None = ffi::STUHFL_D_TUNING_ALGO_NONE as u8,
        Fast = ffi::STUHFL_D_TUNING_ALGO_FAST as u8,
        Exact = ffi::STUHFL_D_TUNING_ALGO_EXACT as u8,
        GroupedExact = ffi::STUHFL_D_TUNING_ALGO_GROUPED_EXACT as u8,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct VersionNum {
    pub major: u8,
    pub minor: u8,
    pub micro: u8,
    pub nano: u8
}

impl fmt::Display for VersionNum {
    fn fmt(&self, f: &mut fmt::Formatter::<'_>) -> fmt::Result {
        write!(f, "v{}.{}.{}.{}", self.major, self.minor, self.micro, self.nano)
    }
}

impl From <ffi::STUHFL_T_Version> for VersionNum {
    fn from(v: ffi::STUHFL_T_Version) -> Self {
        VersionNum {
            major: v.major,
            minor: v.minor,
            micro: v.micro,
            nano: v.nano
        }
    }
}

#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub info: String
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter::<'_>) -> fmt::Result {
        write!(f, "{}", self.info)
    }
}

#[derive(Debug, Clone)]
pub struct Version {
    pub sw_ver: VersionNum,
    pub hw_ver: VersionNum,
    pub sw_info: VersionInfo,
    pub hw_info: VersionInfo
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter::<'_>) -> fmt::Result {
        write!(f, "SW: {}, {}. HW: {}, {}.", self.sw_ver, self.sw_info, self.hw_ver, self.hw_info)
    }
}

pub struct TxOutputLevel (i8);

#[derive(Builder, Clone, Copy)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct TxRxCfg {
    /// Transmission output level (dB). See control register 3 for further info. Valid range [0dB..-19dB].
    #[builder(default="-2")]
    tx_output_level: i8,
    /// Reciever sensitivity level (dB). Valid range [-17dB..+19dB].
    #[builder(default="-3")]
    rx_sensitivity_level: i8,
    /// Antenna to be used.
    #[builder(default="Antenna::Antenna1")]
    antenna: Antenna,
    /// Time in ms for alternating the antennas when alternating mode is used.
    #[builder(default="1")]
    alternate_antenna_interval: u16,
}

impl TxRxCfg {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_TxRxCfg {
        ffi::STUHFL_T_ST25RU3993_TxRxCfg {
            txOutputLevel: self.tx_output_level,
            rxSensitivity: self.rx_sensitivity_level,
            usedAntenna: self.antenna as u8,
            alternateAntennaInterval: self.alternate_antenna_interval,
            rfu: 3 // RFU defined in firmware...
        }
    }
}

impl TxRxCfgBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(i) = self.tx_output_level {
            if i > 0 || i < -19 {
                return Err("tx_output_level invalid: see docs for details".to_owned());
            }
        }

        if let Some(i) = self.rx_sensitivity_level {
            if i > 19 || i < -17 {
                return Err("rx_sensitivity_level invalid: see docs for details".to_owned());
            }
        }

        Ok(())
    }
}

#[derive(Builder)]
pub struct Gen2Cfg<'a> {
    /// Antenna configuration
    tx_rx_cfg: &'a TxRxCfg,
    
}

mod st25ru3993;
pub use st25ru3993::*;

#[cfg(test)]
mod tests;
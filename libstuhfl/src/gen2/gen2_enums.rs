use super::gen2_structs::*;
use crate::ffi;

// Very similar to the Antenna enum in enums.rs
enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    /// One of four Gen2 Sessions
    pub enum Gen2Session {
        /// Session 0
        Session0 = ffi::STUHFL_D_GEN2_SESSION_S0 as u8,
        /// Session 1
        Session1 = ffi::STUHFL_D_GEN2_SESSION_S1 as u8,
        /// Session 2
        Session2 = ffi::STUHFL_D_GEN2_SESSION_S2 as u8,
        /// Session 3
        Session3 = ffi::STUHFL_D_GEN2_SESSION_S3 as u8,
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    /// Query tags who's inventoried flag is A or B.
    ///
    /// These values are from the GS1 Standard.
    pub enum Gen2QueryTarget {
        /// Target A
        A = 0b0,
        /// Target B
        B = 0b1,
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    /// TARI values are the length of time to represent a
    /// binary 0 using the Gen 2 standard (in microseconds)
    pub enum Gen2Tari {
        /// 6.25 μs tari
        Six = ffi::STUHFL_D_GEN2_TARI_6_25 as u8,
        /// 12.5 μs tari
        Twelve = ffi::STUHFL_D_GEN2_TARI_12_50 as u8,
        /// 25 μs tari
        TwentyFive = ffi::STUHFL_D_GEN2_TARI_25_00 as u8,
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    /// BLF is the Backscatter Link Frequency of the transmission (in kHz)
    pub enum Gen2Blf {
        /// 40 kHz BLF
        Forty = ffi::STUHFL_D_GEN2_BLF_40 as u8,
        /// 160 kHz BLF
        OneHundredSixty = ffi::STUHFL_D_GEN2_BLF_160 as u8,
        /// 213 kHz BLF
        TwoHundredThirteen = ffi::STUHFL_D_GEN2_BLF_213 as u8,
        /// 256 kHz BLF
        TwoHundredFiftySix = ffi::STUHFL_D_GEN2_BLF_256 as u8,
        /// 320 kHz BLF
        ThreeHundredTwenty = ffi::STUHFL_D_GEN2_BLF_320 as u8,
        /// 640 kHz BLF
        SixHundredForty = ffi::STUHFL_D_GEN2_BLF_640 as u8,
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    /// Gen2 protocol coding scheme
    pub enum Gen2Coding {
        /// FM0 Coding
        Fm0 = ffi::STUHFL_D_GEN2_CODING_FM0 as u8,
        /// Miller 2 Coding
        Miller2 = ffi::STUHFL_D_GEN2_CODING_MILLER2 as u8,
        /// Miller 4 Coding
        Miller4 = ffi::STUHFL_D_GEN2_CODING_MILLER4 as u8,
        /// Miller 8 Coding
        Miller8 = ffi::STUHFL_D_GEN2_CODING_MILLER8 as u8,
    }
}

#[derive(Copy, Clone)]
/// AdaptiveQ Configuration. The Q factor determines how many slots are made
/// during a query event (e.g. inventorying). Each tag chooses a random number
/// from 0..2^Q, which is used in access commands. Note that if this is too small,
/// there will be slot collisions. If it's too big, there will be overhead.
pub enum Gen2AdaptiveQ {
    /// Enable AdaptiveQ Algorithm (default)
    Enable(Gen2AdaptiveQCfg),
    /// Set manual Q
    Disable(u8),
}

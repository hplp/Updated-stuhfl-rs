use super::structs::*;
use std::fmt;

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq, Debug)]
    #[repr(u8)]
    /// Represents a physical antenna on an RFID reader.
    /// See ST25RU3993 manual for details.
    pub enum Antenna {
        /// Antenna 1 (default).
        Antenna1 = ffi::STUHFL_D_ANTENNA_1 as u8,
        /// Antenna 2.
        Antenna2 = ffi::STUHFL_D_ANTENNA_2 as u8,
        /// Antenna 3.
        Antenna3 = ffi::STUHFL_D_ANTENNA_3 as u8,
        /// Antenna 4.
        Antenna4 = ffi::STUHFL_D_ANTENNA_4 as u8,
    }
}

impl fmt::Display for Antenna {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Antenna::Antenna1 => "Antenna 1",
                Antenna::Antenna2 => "Antenna 2",
                Antenna::Antenna3 => "Antenna 3",
                Antenna::Antenna4 => "Antenna 4",
            }
        )
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    /// Profiles defined in firmware for channel configurations.
    /// See [`ChannelListCfg`] for details.
    pub enum Profile {
        /// European profile
        Europe = ffi::STUHFL_D_PROFILE_EUROPE as u8,
        /// United states profile
        Usa = ffi::STUHFL_D_PROFILE_USA as u8,
        /// Japanese profile
        Japan = ffi::STUHFL_D_PROFILE_JAPAN as u8,
        /// Chinese profile
        China = ffi::STUHFL_D_PROFILE_CHINA as u8,
        /// Chinese profile (alternative)
        China2 = ffi::STUHFL_D_PROFILE_CHINA2 as u8,
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    /// Different types of tuning algorithms. The faster
    /// algorithms are generally less accurate.
    pub enum TuningAlgorithm {
        /// # No tuning algorithm.
        ///
        /// This algorithm simply leaves the reader untuned (not recommended). Use [`TuningAlgorithm::Fast`]
        /// if you really need to avoid the performance penalty from tuning.
        None = ffi::STUHFL_D_TUNING_ALGO_NONE as u8,
        /// # Simple automatic tuning function.
        ///
        /// This algorithm tries to find an optimized tuner setting (minimal reflected power).
        /// The function starts at the current tuner setting and modifies the tuner caps
        /// until a setting with a minimum of reflected power is found.When changing the tuner
        /// further leads to an increase of reflected power the algorithm stops.
        ///
        /// Note that, although the algorithm has been optimized to not immediately stop
        /// at local minima of reflected power, it still might not find the tuner setting with
        /// the lowest reflected power. The algorithm of [`TuningAlgorithm::Exact`] is probably
        /// producing better results, but it is slower.
        Fast = ffi::STUHFL_D_TUNING_ALGO_FAST as u8,
        /// # Sophisticated automatic tuning function.
        ///
        /// This algorithm tries to find an optimized tuner setting (minimal reflected power).
        /// The function splits the 3-dimensional tuner-setting-space (axis are Cin, Clen and Cout)
        /// into segments and searches in each segment (by using [`TuningAlgorithm::Fast`])
        /// for its local minimum of reflected power.
        ///
        /// The tuner setting (point in tuner-setting-space) which has the lowest reflected
        /// power is returned in parameter res. This algorithm has a much higher probability
        /// to find the tuner setting with the lowest reflected power than [`TuningAlgorithm::Fast`]
        /// but on the other hand takes much longer.
        Exact = ffi::STUHFL_D_TUNING_ALGO_EXACT as u8,
        /// # Enhanced Sophisticated automatic tuning function.
        ///
        /// This algorithm tries to find an optimized tuner setting (minimal reflected power).
        /// The function splits the 3-dimensional tuner-setting-space (axis are Cin, Clen and Cout)
        /// into segments and get reflected power for each of them. A [`TuningAlgorithm::Fast`]
        /// is then run on the 3 segments with minimum of reflected power.
        ///
        /// The tuner setting (point in tuner-setting-space)
        /// which has the lowest reflected power is then returned in parameter res.
        /// This algorithm has a much higher probability to find the tuner setting with the
        /// lowest reflected power than [`TuningAlgorithm::Fast`] and is faster than [`TuningAlgorithm::Exact`].
        GroupedExact = ffi::STUHFL_D_TUNING_ALGO_GROUPED_EXACT as u8,
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    /// Query Selection.
    ///
    /// These values are from the GS1 Standard
    pub enum QuerySel {
        /// All targets
        All = 0b00,
        /// Selected targets
        Sel = 0b10,
        /// Not selected targets
        NotSel = 0b11,
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    /// Frequency hopping modes
    pub enum FreqHopMode {
        /// Only the max sending time is considered
        IgnoreMin = ffi::STUHFL_D_FREQUENCY_HOP_MODE_IGNORE_MIN as u8,
        /// Power off when min_sending_time is expired and wait until maxSendingTime before frequency is hopped
        PowerSave = ffi::STUHFL_D_FREQUENCY_HOP_MODE_POWER_SAVE as u8,
        /// Perform hopping as soon as min_sending_time is expired and continue with next hopping frequency
        Fast = ffi::STUHFL_D_FREQUENCY_HOP_MODE_FAST as u8,
        /// Perform hopping as described in Fast mode, but take care that all frequencies get the same time
        /// in total to avoid violating FCC rules
        FastFcc = ffi::STUHFL_D_FREQUENCY_HOP_MODE_FAST_FCC as u8,
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq, Debug)]
    #[repr(u8)]
    /// Tuning Status
    pub enum TuningStatus {
        /// Untuned
        Untuned = ffi::STUHFL_D_TUNING_STATUS_UNTUNED as u8,
        /// Tuning
        Tuning = ffi::STUHFL_D_TUNING_STATUS_TUNING as u8,
        /// Tuned
        Tuned = ffi::STUHFL_D_TUNING_STATUS_TUNED as u8,
    }
}

impl fmt::Display for TuningStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TuningStatus::Untuned => "Untuned",
                TuningStatus::Tuning => "Tuning",
                TuningStatus::Tuned => "Tuned",
            }
        )
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    /// Memory banks. See [`st25ru3993::read_gen2()`] for details.
    pub enum MemoryBank {
        /// EPC Memory bank
        Epc = ffi::STUHFL_D_GEN2_MEMORY_BANK_EPC as u8,
        /// Reserved Memory bank
        Reserved = ffi::STUHFL_D_GEN2_MEMORY_BANK_RESERVED as u8,
        /// TID Memory bank
        Tid = ffi::STUHFL_D_GEN2_MEMORY_BANK_TID as u8,
        /// User Memory bank
        User = ffi::STUHFL_D_GEN2_MEMORY_BANK_USER as u8,
    }
}

/// Listen-Before-Talk configuration.
#[derive(Copy, Clone)]
pub enum Lbt {
    /// Enable Listen-Before-Talk
    Enable(LbtCfg),
    /// Disable Listen-Before-Talk (default)
    Disable,
}

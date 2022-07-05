use super::*;

pub(crate) trait AsFFI<T> {
    fn as_ffi(&self) -> T;
}

/// Returns a builder to create the structure
/// ```
/// use libstuhfl::{Gen2Cfg, Builder};
/// 
/// let cfg = Gen2Cfg::builder()
///     .build()
///     .expect("Failed to build configuration");
/// ```
pub trait Builder<T>
where T: std::default::Default {
    /// Default implementation, see [`Builder<T>`]
    fn builder() -> T {
        T::default()
    }
}

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
        /// Alternative antenna.
        AntennaAlt = ffi::STUHFL_D_ANTENNA_ALT as u8
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

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    /// Gen2 Memory banks. See [`st25ru3993::read_gen2()`] for details.
    pub enum Gen2MemoryBank {
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

#[derive(Copy, Clone, PartialEq, PartialOrd)]
/// Contains the version numbering for ST Hardware & Software.
/// `major.minor.micro.nano`
pub struct VersionNum {
    /// Major version
    pub major: u8,
    /// Minor version
    pub minor: u8,
    /// Micro version
    pub micro: u8,
    /// Nano version
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

#[derive(Clone)]
/// Contains a version descriptor string
pub struct VersionInfo {
    /// Information about version
    pub info: String
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter::<'_>) -> fmt::Result {
        write!(f, "{}", self.info)
    }
}

#[derive(Clone)]
/// Contains version of firmware & software of ST chip
pub struct Version {
    /// Firmware version
    pub sw_ver: VersionNum,
    /// Hardware version
    pub hw_ver: VersionNum,
    /// Firmware description
    pub sw_info: VersionInfo,
    /// Hardware description
    pub hw_info: VersionInfo
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter::<'_>) -> fmt::Result {
        write!(f, "SW: {}, {}. HW: {}, {}.", self.sw_ver, self.sw_info, self.hw_ver, self.hw_info)
    }
}

#[derive(Builder, Copy, Clone)]
#[builder(build_fn(validate = "Self::validate"))]
/// Contains antenna configuration settings. See [`Self::builder()`] for details.
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

impl Builder<TxRxCfgBuilder> for TxRxCfg {}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_TxRxCfg> for TxRxCfg {
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

#[derive(Copy, Clone)]
/// AdaptiveQ Configuration. The Q factor determines how many slots are made
/// during a query event (e.g. inventorying). Each tag chooses a random number
/// from 0..2^Q, which is used in access commands. Note that if this is too small,
/// there will be slot collisions. If it's too big, there will be overhead.
pub enum Gen2AdaptiveQ {
    /// Enable AdaptiveQ Algorithm (default)
    Enable(Gen2AdaptiveQCfg),
    /// Set manual Q
    Disable(u8)
}

#[derive(Builder, Copy, Clone)]
/// AdaptiveQ Algorithm configuration. This contains parameters for determining
/// the Q value automatically. See [`Gen2AdaptiveQ`] for details.
#[builder(build_fn(validate = "Self::validate"))]
pub struct Gen2AdaptiveQCfg {
    /// Q Starting value
    #[builder(default="6")]
    start_q: u8,
    /// Minimum Q Value
    #[builder(default="2")]
    min_q: u8,
    /// Maximum Q Value (max 15)
    #[builder(default="ffi::STUHFL_D_GEN2_MAXQ as u8")]
    max_q: u8,
    /// Q Algorithm option
    #[builder(default="false")]
    adjust_nic: bool,
    /// Q Algorithm option
    #[builder(default="false")]
    single_adjust: bool,
    /// Q Algorithm option
    #[builder(default="false")]
    use_ceil_floor: bool,
    /// Q Algorithm option
    #[builder(default="false")]
    reset_after_round: bool,
}

impl Builder<Gen2AdaptiveQCfgBuilder> for Gen2AdaptiveQCfg {}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_Gen2_Anticollision> for Gen2AdaptiveQ {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_Gen2_Anticollision {
        match self {
            Gen2AdaptiveQ::Enable(conf) => {
                let options
                    = if conf.adjust_nic {ffi::STUHFL_D_USE_QUERY_ADJUST_NIC as u8} else {0}
                    | if conf.single_adjust {ffi::STUHFL_D_SINGLE_ADJUST as u8} else {0}
                    | if conf.use_ceil_floor {ffi::STUHFL_D_USE_CEIL_FLOOR as u8} else {0}
                    | if conf.reset_after_round {ffi::STUHFL_D_RESET_Q_AFTER_ROUND as u8} else {0};

                ffi::STUHFL_T_ST25RU3993_Gen2_Anticollision {
                    adaptiveQ: true,
                    startQ: conf.start_q,
                    minQ: conf.min_q,
                    maxQ: conf.max_q,
                    options,
                    ..Default::default()
                }
            },
            // use all firmware defaults
            Gen2AdaptiveQ::Disable(q) => ffi::STUHFL_T_ST25RU3993_Gen2_Anticollision {
                adaptiveQ: false,
                startQ: *q,
                ..Default::default()
            }
        }        
    }
}

impl Gen2AdaptiveQCfgBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(max) = self.max_q {
            if let Some(min) = self.min_q {
                if max <= min {
                    return Err("max_q must be greater than min_q".to_owned())
                }
            }

            if max > ffi::STUHFL_D_GEN2_MAXQ as u8 {
                return Err("max_q too large: see docs for details".to_owned())
            }
        }

        Ok(())
    }
}

#[derive(Builder, Copy, Clone)]
/// Options regarding Gen2 inventory rounds. This is part of [`Gen2InventoryCfg`].
pub struct Gen2InventoryOptions {
    /// Fast Inventory enabling. If set to false, normal inventory round will be performed.
    /// If set to true, fast inventory rounds will be performed.
    #[builder(default="true")]
    fast: bool,
    /// Automatic Acknowledgement enabling. If set to false, inventory rounds will be triggered 
    /// by the firmware, otherwise the commands will be sent automatically.
    #[builder(default="true")]
    auto_ack: bool,
    /// Enable reading TID's during inventory rounds
    #[builder(default="true")]
    read_tid: bool,
}

impl Builder<Gen2InventoryOptionsBuilder> for Gen2InventoryOptions {}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_Gen2_InventoryOption> for Gen2InventoryOptions {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_Gen2_InventoryOption {
        ffi::STUHFL_T_ST25RU3993_Gen2_InventoryOption {
            fast: self.fast,
            autoAck: self.auto_ack,
            readTID: self.read_tid,
        }
    }
}

#[derive(Builder, Copy, Clone)]
/// Auto-tuning algorithm parameters. This is used during `inventory_runner` cycles.
pub struct AutoTuning {
    /// Auto-tuning check interval (in inventory rounds)
    #[builder(default="7")]
    interval: u16,
    /// Devation from (I^2+Q^2) to trigger a retuning
    #[builder(default="20")]
    level: u8,
    /// Algorithm to use for automatic tuning
    #[builder(default="TuningAlgorithm::Fast")]
    algo: TuningAlgorithm,
    /// Do false positive detection check
    #[builder(default="true")]
    false_positive_detect: bool,
}

impl Builder<AutoTuningBuilder> for AutoTuning {}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_AutoTuning> for AutoTuning {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_AutoTuning {
        ffi::STUHFL_T_ST25RU3993_AutoTuning {
            interval: self.interval,
            level: self.level,
            algorithm: self.algo as u8,
            falsePositiveDetection: self.false_positive_detect,
        }
    }
}

#[derive(Builder, Copy, Clone)]
/// Gen2 Query paremeter parameters. These are used by the firmware
/// during Query events (all data exchanges between reader and tag
/// require Query events).
pub struct Gen2QueryParams {
    /// QUERY command Sel field
    #[builder(default = "QuerySel::All")]
    sel: QuerySel,
    /// QUERY session
    #[builder(default = "Gen2Session::Session0")]
    session: Gen2Session,
    /// QUERY target field
    #[builder(default = "Gen2QueryTarget::A")]
    target: Gen2QueryTarget,
    /// Automatically change target between A and B
    #[builder(default = "true")]
    toggle_target: bool,
    /// If set to true and the target shall be toggled in inventory,
    /// an additional inventory round before the target is toggled will
    /// be executed. This allows "weak" transponders an additional
    /// chance to reply.
    #[builder(default = "true")]
    target_depletion_mode: bool,
}

impl Builder<Gen2QueryParamsBuilder> for Gen2QueryParams {}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_Gen2_QueryParams> for Gen2QueryParams {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_Gen2_QueryParams {
        ffi::STUHFL_T_ST25RU3993_Gen2_QueryParams {
            sel: self.sel as u8,
            session: self.session as u8,
            target: self.target as u8,
            toggleTarget: self.toggle_target,
            targetDepletionMode: self.target_depletion_mode,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
/// Automatic reciever sensitivity setting. Currently this is
/// only a boolean, as the algorithm parameters in the firmware
/// are too vague.
pub struct AutoRxSensitivity {
    enable: bool
}

impl From<bool> for AutoRxSensitivity {
    fn from(enable: bool) -> Self {
        Self {
            enable
        }
    }
}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_AdaptiveSensitivity> for AutoRxSensitivity {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_AdaptiveSensitivity {
        ffi::STUHFL_T_ST25RU3993_AdaptiveSensitivity {
            adaptiveRx: self.enable,
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
/// Automatic transmission strength setting. Currently this is
/// only a boolean, as the algorithm parameters in the firmware
/// are too vague.
pub struct AutoTxStrength {
    enable: bool
}

impl From<bool> for AutoTxStrength {
    fn from(enable: bool) -> Self {
        Self {
            enable
        }
    }
}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_AdaptiveOutputPower> for AutoTxStrength {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_AdaptiveOutputPower {
        ffi::STUHFL_T_ST25RU3993_AdaptiveOutputPower {
            adaptiveTx: self.enable,
            ..Default::default()
        }
    }
}

#[derive(Builder, Copy, Clone)]
/// Settings for Gen2 Inventorying. Note that [`Gen2InventoryOptions`] is a only subset of
/// these settings. See [`Self::builder()`] for details.
/// See also: [`Gen2Cfg`]
pub struct Gen2InventoryCfg {
    /// Extra inventory options
    #[builder(default = "Gen2InventoryOptionsBuilder::default().build().unwrap()")]
    options: Gen2InventoryOptions,
    /// Automatic Q adjustment settings (Anti-collision)
    #[builder(default = "Gen2AdaptiveQ::Enable(Gen2AdaptiveQCfgBuilder::default().build().unwrap())")]
    adaptive_q: Gen2AdaptiveQ,
    /// Automatic tuning settings
    #[builder(default = "AutoTuningBuilder::default().build().unwrap()")]
    auto_tuning: AutoTuning,
    /// Parameters to QUERY commands
    #[builder(default = "Gen2QueryParamsBuilder::default().build().unwrap()")]
    query_params: Gen2QueryParams,
    /// Adaptive RX Sensitivity options
    #[builder(default = "AutoRxSensitivity::from(true)")]
    auto_rx_sensitivity: AutoRxSensitivity,
    /// Adaptive TX Strength options
    #[builder(default = "AutoTxStrength::from(true)")]
    auto_tx_strength: AutoTxStrength,
}

impl Builder<Gen2InventoryCfgBuilder> for Gen2InventoryCfg {}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_Gen2_InventoryCfg> for Gen2InventoryCfg {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_Gen2_InventoryCfg {
        ffi::STUHFL_T_ST25RU3993_Gen2_InventoryCfg {
            inventoryOption: self.options.as_ffi(),
            antiCollision: self.adaptive_q.as_ffi(),
            autoTuning: self.auto_tuning.as_ffi(),
            queryParams: self.query_params.as_ffi(),
            adaptiveSensitivity: self.auto_rx_sensitivity.as_ffi(),
            adaptiveOutputPower: self.auto_tx_strength.as_ffi(),
        }
    }
}

#[derive(Builder, Copy, Clone)]
/// Gen2 protocol settings. These factors affect the transmission
/// speed and reliability of the air protocol.
pub struct Gen2ProtocolCfg {
    /// Tari setting
    #[builder(default = "Gen2Tari::Six")]
    tari: Gen2Tari,
    /// Backscatter link frequency factor
    #[builder(default = "Gen2Blf::ThreeHundredTwenty")]
    blf: Gen2Blf,
    /// Coding
    #[builder(default = "Gen2Coding::Miller2")]
    coding: Gen2Coding,
    /// Short or long preamble (true means long)
    #[builder(default = "ffi::STUHFL_D_TREXT_ON != 0")]
    trext: bool,
}

impl Builder<Gen2ProtocolCfgBuilder> for Gen2ProtocolCfg {}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_Gen2_ProtocolCfg> for Gen2ProtocolCfg {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_Gen2_ProtocolCfg {
        ffi::STUHFL_T_ST25RU3993_Gen2_ProtocolCfg {
            tari: self.tari as u8,
            blf: self.blf as u8,
            coding: self.coding as u8,
            trext: self.trext,
        }
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

#[derive(Builder, Copy, Clone)]
/// Configuration settings for Listen-Before-Talk. See [`Lbt`] for details.
pub struct LbtCfg {
    /// Length of listening period
    #[builder(default = "1")]
    listening_time: u16,
    /// Idle time for LBT
    #[builder(default = "0")]
    idle_time: u16,
    /// RSSI threshold value
    #[builder(default = "31")]
    rssi_log_threshold: u8,
}

impl Builder<LbtCfgBuilder> for LbtCfg {}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_FreqLBT> for Lbt {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_FreqLBT {
        match self {
            Lbt::Enable(cfg) => ffi::STUHFL_T_ST25RU3993_FreqLBT {
                listeningTime: cfg.listening_time,
                idleTime: cfg.idle_time,
                rssiLogThreshold: cfg.rssi_log_threshold,
                skipLBTcheck: false
            },
            Lbt::Disable => ffi::STUHFL_T_ST25RU3993_FreqLBT {
                skipLBTcheck: true,
                ..Default::default()
            }
        }
    }
}

/// Capacitance values of self-jamming PI-capacitor network. See
/// ST25RU3993 self-jamming implementation details. See [`ChannelListCfg`]
/// for details.
#[derive(Copy, Clone)]
pub struct TuningCaps {
    /// IN capacitance of tuning network
    cin: u8,
    /// LEN capacitance of tuning network
    clen: u8,
    /// OUT capacitance of tuning network
    cout: u8,
}

impl Default for TuningCaps {
    fn default() -> Self {
        Self {
            cin: ffi::STUHFL_D_DEFAULT_CAP as u8,
            clen: ffi::STUHFL_D_DEFAULT_CAP as u8,
            cout: ffi::STUHFL_D_DEFAULT_CAP as u8,
        }
    }
}

impl TuningCaps {
    /// Create tuning caps from manually-specified values.
    pub fn from(cin: u8, clen: u8, cout: u8) -> Self {
        Self {
            cin, clen, cout,
        }
    }
}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_Caps> for TuningCaps {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_Caps {
        ffi::STUHFL_T_ST25RU3993_Caps {
            cin: self.cin,
            clen: self.clen,
            cout: self.cout,
        }
    }
}

#[derive(Copy, Clone)]
/// A single frequency configuration in a [`ChannelListCfg`].
pub struct ChannelItem {
    /// Frequency to be used for channel item (Hz)
    frequency: u32,
    /// Tuning capacitor values
    caps: [TuningCaps; 2],
}

impl ChannelItem {
    /// Create [`ChannelItem`] from frequency (Hz), using default tuning
    /// capacitor values (recommended).
    pub fn from_freq(frequency: u32) -> Self {
        Self {
            frequency,
            caps: [TuningCaps::default(); 2],
        }
    }

    /// Create [`ChannelItem`] from frequency (Hz), manually specifying
    /// each tuning capacitor value. Advanced usage only!
    pub fn from(frequency: u32, caps: [TuningCaps; 2]) -> Self {
        Self {
            frequency,
            caps,
        }
    }
}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_ChannelItem> for ChannelItem {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_ChannelItem {
        ffi::STUHFL_T_ST25RU3993_ChannelItem {
            frequency: self.frequency,
            caps: [self.caps[0].as_ffi(), self.caps[1].as_ffi()],
            ..Default::default()
        }
    }
}

#[derive(Clone)]
/// Contains a list of [`ChannelItem`]. This represents all
/// the frequencies the reader can try to use during transmission.
pub struct ChannelListCfg {
    item_list: Vec<ChannelItem>,
}

impl ChannelListCfg {
    /// Create a ChannelList manually (advanced usage).
    pub fn from(item_list: &[ChannelItem]) -> Self {
        Self {
            item_list: Vec::from(item_list)
        }
    }

    /// Create a ChannelList using a profile specified in the firmware
    pub fn from_profile(profile: Profile) -> Self {
        Self {
            item_list: profile_to_item_list(profile)
        }
    }
}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_ChannelList> for ChannelListCfg {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_ChannelList {
        ffi::STUHFL_T_ST25RU3993_ChannelList {
            numFrequencies: self.item_list.len() as u8,
            itemList: item_list_to_ffi(&self.item_list), // TODO
            ..Default::default()
        }
    }
}

#[derive(Builder, Copy, Clone)]
/// Frequency hopping configuration. See [`Self::builder()`] for more.
pub struct FreqHopCfg {
    /// Max sending time before frequency hopping is performed. Minimum value: 40ms
    #[builder(default = "400")]
    max_sending_time: u16,
    /// Minimum sending time before frequency hopping is performed.
    #[builder(default = "400")]
    min_sending_time: u16,
    /// Hopping Mode
    #[builder(default = "FreqHopMode::IgnoreMin")]
    mode: FreqHopMode,
}

impl Builder<FreqHopCfgBuilder> for FreqHopCfg {}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_FreqHop> for FreqHopCfg {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_FreqHop {
        ffi::STUHFL_T_ST25RU3993_FreqHop {
            maxSendingTime: self.max_sending_time,
            minSendingTime: self.min_sending_time,
            mode: self.mode as u8,
            ..Default::default()
        }
    }
}

#[derive(Builder, Clone)]
/// Gen2 Master configuration
pub struct Gen2Cfg {
    /// Antenna configuration
    #[builder(default = "TxRxCfg::builder().build().unwrap()")]
    pub(crate) tx_rx_cfg: TxRxCfg,
    /// Settings for inventorying tags
    #[builder(default = "Gen2InventoryCfg::builder().build().unwrap()")]
    pub(crate) inv_cfg: Gen2InventoryCfg,
    /// Gen2 protocol configuration
    #[builder(default = "Gen2ProtocolCfg::builder().build().unwrap()")]
    pub(crate) proto_cfg: Gen2ProtocolCfg,
    /// Listen before talk configuration
    #[builder(default = "Lbt::Disable")]
    pub(crate) lbt: Lbt,
    /// Channel list configuration
    #[builder(default = "ChannelListCfg::from_profile(Profile::Europe)")]
    pub(crate) channel_list: ChannelListCfg,
    /// Frequency hopping configuration
    #[builder(default = "FreqHopCfg::builder().build().unwrap()")]
    pub(crate) freq_hop: FreqHopCfg,
}

impl Builder<Gen2CfgBuilder> for Gen2Cfg {}

#[derive(Debug, PartialEq, Clone)]
/// Container for hexadecimal-based ID values such as TID, XPC, and EPC.
pub struct HexID {
    pub(crate) id: Vec<u8>
}

impl HexID {
    /// Create a HexID from a list of integers.
    pub fn from_id(id: Vec<u8>) -> HexID {
        HexID{id}
    }

    /// Get a HexID as a list of integers.
    pub fn get_id(&self) -> &[u8] {
        &self.id[..]
    }
}

impl std::ops::Index<usize> for HexID {
    type Output = u8;

    fn index(&self, i: usize) -> &Self::Output {
        &self.id[i]
    }
}

impl fmt::Display for HexID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut id = self.id.iter().fold(String::new(), |a, b| a + &format!("{:02X}", b) + ":");
        id.pop();
        write!(f, "{}", &id)
    }
}

/// HexID type to be used only for XPC numbers.
pub type Xpc = HexID;

impl From<ffi::STUHFL_T_InventoryTagXPC> for Xpc {
    fn from(xpc: ffi::STUHFL_T_InventoryTagXPC) -> Xpc {
        Xpc { id: Vec::from(&xpc.data[..xpc.length as usize]) }
    }
}

/// HexID type to be used only for EPC numbers.
pub type Epc = HexID;

impl From<ffi::STUHFL_T_InventoryTagEPC> for Epc {
    fn from(epc: ffi::STUHFL_T_InventoryTagEPC) -> Epc {
        Epc { id: Vec::from(&epc.data[..epc.length as usize]) }
    }
}

/// HexID type to be used only for TID numbers.
pub type Tid = HexID;

impl From<ffi::STUHFL_T_InventoryTagTID> for Tid {
    fn from(tid: ffi::STUHFL_T_InventoryTagTID) -> Tid {
        Tid { id: Vec::from(&tid.data[..tid.length as usize]) }
    }
}

#[derive(Clone, PartialEq)]
/// Contains all data related to an RFID tag found during an inventory cycle.
/// These values are populated automatically by the firmware.
pub struct InventoryTag {
    /// Tag detection slot ID
    pub slot_id: u32,
    /// Tag detection time stamp in ms after starting inventory
    pub timestamp: u32,
    /// Antenna at which tag was detected
    pub antenna: Antenna,
    /// AGC (Automatic Gain Control) measured when tag found
    pub agc: u8,
    /// I part of tag logarithmic RSSI
    pub rssi_log_i: u8,
    /// Q part of tag logarithmic RSSI
    pub rssi_log_q: u8,
    /// I part of tag linear RSSI
    pub rssi_lin_i: i8,
    /// Q part of tag linear RSSI
    pub rssi_lin_q: i8,
    /// Tag PC
    pub pc: [u8; 2],
    /// Tag XPC
    pub xpc: Xpc,
    /// Tag EPC
    pub epc: Epc,
    /// Tag TID
    pub tid: Tid,
}

impl From<ffi::STUHFL_T_InventoryTag> for InventoryTag {
    fn from(tag: ffi::STUHFL_T_InventoryTag) -> InventoryTag {
        InventoryTag {
            slot_id: tag.slotId,
            timestamp: tag.timestamp,
            antenna: Antenna::from_u8(tag.antenna).unwrap(),
            agc: tag.agc,
            rssi_log_i: tag.rssiLogI,
            rssi_log_q: tag.rssiLogQ,
            rssi_lin_i: tag.rssiLinI,
            rssi_lin_q: tag.rssiLinQ,
            pc: tag.pc,
            xpc: Xpc::from(tag.xpc),
            epc: Epc::from(tag.epc),
            tid: Tid::from(tag.tid),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
/// Statistics related to an inventory run. These settings are
/// generated by the firmware.
pub struct InventoryStatistics {
    /// Timestamp since last statistics update.
    pub timestamp: u32,
    /// Inventory rounds already complete.
    pub round_count: u32,
    /// Reader tuning status.
    pub tuning_status: TuningStatus,
    /// RSSI log mean value. Updated with each found tag.
    pub rssi_log_mean: u8,
    /// Reader sensitivity.
    pub sensitivity: i8,
    /// Current Q, may vary if adaptive Q is enabled.
    pub final_q: u8,
    /// Currently used frequency.
    pub frequency: u32,
    /// ADC value, measured between each round.
    pub adc: u16,
    /// Number of detected tags.
    pub tag_count: u32,
    /// Number of empty slots.
    pub empty_slot_count: u32,
    /// Number of executed slots.
    pub slot_count: u32,
    /// Number of collisions.
    pub collision_count: u32,
    /// Number of Header errors.
    pub preamble_err_count: u32,
    /// Number of CRC errors
    pub crc_err_count: u32,
    /// Number of RX count errorsnd due to error during EPC reception.
    pub rx_count_err_count: u32,
    /// Number of ACK resends due to error duing EPC reception.
    pub resend_ack_count: u32,
    /// Number of noise suspicon events. 
    /// Event is triggered when ACK is resended and no response is recieved.
    pub noise_suspicion_count: u32,
}

impl From<ffi::STUHFL_T_InventoryStatistics> for InventoryStatistics {
    fn from(stats: ffi::STUHFL_T_InventoryStatistics) -> InventoryStatistics {
        InventoryStatistics {
            timestamp: stats.timestamp,
            round_count: stats.roundCnt,
            tuning_status: TuningStatus::from_u8(stats.tuningStatus).unwrap(),
            rssi_log_mean: stats.rssiLogMean,
            sensitivity: stats.sensitivity,
            final_q: stats.Q,
            frequency: stats.frequency,
            adc: stats.adc,
            tag_count: stats.tagCnt,
            empty_slot_count: stats.emptySlotCnt,
            slot_count: stats.slotCnt,
            collision_count: stats.collisionCnt,
            preamble_err_count: stats.preambleErrCnt,
            crc_err_count: stats.crcErrCnt,
            rx_count_err_count: stats.rxCountErrCnt,
            resend_ack_count: stats.resendAckCnt,
            noise_suspicion_count: stats.noiseSuspicionCnt,
        }
    }
}

/// Function type to be used with inventory_runner
pub type CallbackFn = dyn Fn(InventoryTag) + Send;

#[derive(Copy, Clone, PartialEq)]
/// Contains settings used to issue custom commands to the RFID reader.
/// Warning: expect_header **does nothing** when CRC is disabled.
pub struct Gen2CustomCommand {
    /// Enable sending and recieving CRC codes in the packets.
    pub use_crc: bool,
    /// Enable sending and recieving RN16 codes in the packets.
    pub use_rn16: bool,
    /// Gen2 Command code designation. Note: only 16-bit commands
    /// are supported. This includes all *custom* and *proprietary*
    /// command codes according to the standard.
    pub command_code: u16,
    /// Whether or not to expect a header bit in recieved packets.
    /// Note that [`self.use_crc = true`] is **required** for this to work
    /// as expected.
    pub expect_header: bool,
}

/// Data to be sent with a custom RFID command
pub struct Gen2CustomCommandData<'a> {
    pub(crate) num_bits: u16,
    pub(crate) bytes: &'a [u8],
}

impl<'a> Gen2CustomCommandData<'a> {
    /// Create a data packet using a slice of bytes and a bit length.
    /// Note: if num_bits is too small or large this will fail.
    pub fn new(num_bits: usize, bytes: &'a [u8]) -> Option<Self> {
        let max_bits = bytes.len() * 8;
        let min_bits = max_bits - 8;

        if num_bits >= max_bits || num_bits < min_bits {
            None
        } else {
            Some(Self {
                num_bits: num_bits as u16,
                bytes
            })
        }
    }
}

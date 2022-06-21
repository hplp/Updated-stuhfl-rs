use super::*;

pub(crate) trait AsFFI<T> {
    fn as_ffi(&self) -> T;
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
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
//     #[derive(Copy, Clone, PartialEq)]
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
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    pub enum TuningAlgorithm {
        None = ffi::STUHFL_D_TUNING_ALGO_NONE as u8,
        Fast = ffi::STUHFL_D_TUNING_ALGO_FAST as u8,
        Exact = ffi::STUHFL_D_TUNING_ALGO_EXACT as u8,
        GroupedExact = ffi::STUHFL_D_TUNING_ALGO_GROUPED_EXACT as u8,
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    /// Query Selection.
    // These values are from the GS1 Standard
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
    /// Query tags who's inventoried flag is A or B
    // These values are from the GS1 Standard
    pub enum Gen2QueryTarget {
        /// Target A
        A = 0b0,
        B = 0b1,
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    /// TARI values are the length of time to represent a
    /// binary 0 using the Gen 2 standard
    pub enum Gen2Tari {
        /// 6.25 μs
        Six = ffi::STUHFL_D_GEN2_TARI_6_25 as u8,
        /// 12.5 μs
        Twelve = ffi::STUHFL_D_GEN2_TARI_12_50 as u8,
        /// 25 μs
        TwentyFive = ffi::STUHFL_D_GEN2_TARI_25_00 as u8,
    }
}

enum_from_primitive! {
    #[derive(Copy, Clone, PartialEq)]
    #[repr(u8)]
    /// BLF is the Backscatter Link Frequency of the transmission
    pub enum Gen2Blf {
        /// 40 kHz
        Forty = ffi::STUHFL_D_GEN2_BLF_40 as u8,
        /// 160 kHz
        OneHundredSixty = ffi::STUHFL_D_GEN2_BLF_160 as u8,
        /// 213 kHz
        TwoHundredThirteen = ffi::STUHFL_D_GEN2_BLF_213 as u8,
        /// 256 kHz
        TwoHundredFiftySix = ffi::STUHFL_D_GEN2_BLF_256 as u8,
        /// 320 kHz
        ThreeHundredTwenty = ffi::STUHFL_D_GEN2_BLF_320 as u8,
        /// 640 kHz
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

#[derive(Clone, PartialEq, PartialOrd)]
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

#[derive(Clone)]
pub struct VersionInfo {
    pub info: String
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter::<'_>) -> fmt::Result {
        write!(f, "{}", self.info)
    }
}

#[derive(Clone)]
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

#[derive(Builder, Clone)]
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

#[derive(Clone)]
pub enum Gen2AdaptiveQ {
    /// Configure Adaptive Q
    Enable(Gen2AdaptiveQCfg),
    /// Set manual Q
    Disable(u8)
}

#[derive(Builder, Clone)]
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

#[derive(Builder, Clone)]
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

impl AsFFI<ffi::STUHFL_T_ST25RU3993_Gen2_InventoryOption> for Gen2InventoryOptions {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_Gen2_InventoryOption {
        ffi::STUHFL_T_ST25RU3993_Gen2_InventoryOption {
            fast: self.fast,
            autoAck: self.auto_ack,
            readTID: self.read_tid,
        }
    }
}

#[derive(Builder, Clone)]
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

#[derive(Builder, Clone)]
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

#[derive(Builder, Clone)]
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

#[derive(Builder, Clone)]
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

/// Listen Before Talk
#[derive(Clone)]
pub enum Lbt {
    Enable(LbtCfg),
    Disable,
}

#[derive(Builder, Clone)]
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

#[derive(Builder)]
pub struct Gen2Cfg {
    /// Antenna configuration
    #[builder(default = "TxRxCfgBuilder::default().build().unwrap()")]
    pub(crate) tx_rx_cfg: TxRxCfg,
    /// Settings for inventorying tags
    #[builder(default = "Gen2InventoryCfgBuilder::default().build().unwrap()")]
    pub(crate) inv_cfg: Gen2InventoryCfg,
    /// Gen2 protocol configuration
    #[builder(default = "Gen2ProtocolCfgBuilder::default().build().unwrap()")]
    pub(crate) proto_cfg: Gen2ProtocolCfg,
    /// Listen before talk configuration
    #[builder(default = "Lbt::Disable")]
    pub(crate) lbt: Lbt,
}

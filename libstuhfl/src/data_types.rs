use super::*;

pub(crate) trait AsFFI<T> {
    fn as_ffi(&self) -> T;
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

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
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
    #[derive(Debug, Copy, Clone, PartialEq)]
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
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u8)]
    /// Query tags who's inventoried flag is A or B
    // These values are from the GS1 Standard
    pub enum Gen2QueryTarget {
        /// Target A
        A = 0b0,
        B = 0b1,
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

#[derive(Copy, Clone, PartialEq)]
pub enum Gen2AdaptiveQ {
    /// Configure Adaptive Q
    Enable(Gen2AdaptiveQCfg),
    /// Set manual Q
    Disable(u8)
}

#[derive(Builder, Clone, Copy, PartialEq)]
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
        match *self {
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
                startQ: q,
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

#[derive(Builder, Clone, Copy)]
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

#[derive(Builder, Clone, Copy)]
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

#[derive(Builder, Clone, Copy)]
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

#[derive(Builder)]
pub struct Gen2InventoryCfg {
    /// Extra inventory options
    options: Gen2InventoryOptions,
    /// Automatic Q adjustment settings (Anti-collision)
    adaptive_q: Gen2AdaptiveQ,
    /// Automatic tuning settings
    auto_tuning: AutoTuning,
    /// Parameters to QUERY commands
    query_params: Gen2QueryParams,
    /// Adaptive RX Sensitivity options
    auto_rx_sensitivity: AutoRxSensitivity,
    /// Adaptive TX Strength options
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

#[derive(Builder)]
/// TODO
pub struct Gen2ProtocolCfg {
    /// Tari setting
    tari: u8,
    /// Backscatter link frequnecy factor
    blf: u8,
    /// Coding
    coding: u8,
    /// Short or long preamble (true means long)
    trext: bool,
}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_Gen2_ProtocolCfg> for Gen2ProtocolCfg {
    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_Gen2_ProtocolCfg {
        ffi::STUHFL_T_ST25RU3993_Gen2_ProtocolCfg {
            tari: self.tari,
            blf: self.blf,
            coding: self.coding,
            trext: self.trext,
        }
    }
}

#[derive(Builder)]
pub struct Gen2Cfg<'a> {
    /// Antenna configuration
    pub(crate) tx_rx_cfg: &'a TxRxCfg,
    /// Settings for inventorying tags
    pub(crate) inv_cfg: &'a Gen2InventoryCfg,
    /// Gen2 protocol configuration
    pub(crate) proto_cfg: &'a Gen2ProtocolCfg,
}

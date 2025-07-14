use super::gen2_enums::*;
use crate::data_types::*;
use crate::ffi;

// CB 7/14/25: Similar to 'structs.rs', this file holds default settings that can be 
//             altered to change the Gen2Cfg configuration. For example, on line 290:
//
//             #[builder(default = "ChannelListCfg::from_profile(Profile::Custom)")]
//
//             'Custom' can be changed to the desired Cfg profile, which can be found in 'enums.rs'
//             from lines 55 to 72

#[derive(Builder, Copy, Clone)]
/// AdaptiveQ Algorithm configuration. This contains parameters for determining
/// the Q value automatically. See [`Gen2AdaptiveQ`] for details.
#[builder(build_fn(validate = "Self::validate"))]
pub struct Gen2AdaptiveQCfg {
    /// Q Starting value
    #[builder(default = "6")]
    start_q: u8,

    /// Minimum Q Value
    #[builder(default = "2")]
    min_q: u8,

    /// Maximum Q Value (max 15)
    #[builder(default = "ffi::STUHFL_D_GEN2_MAXQ as u8")]
    max_q: u8,

    /// Q Algorithm option
    #[builder(default = "false")]
    adjust_nic: bool,

    /// Q Algorithm option
    #[builder(default = "false")]
    single_adjust: bool,

    /// Q Algorithm option
    #[builder(default = "false")]
    use_ceil_floor: bool,

    /// Q Algorithm option
    #[builder(default = "false")]
    reset_after_round: bool,
}

//Is this anything?
impl Builder<Gen2AdaptiveQCfgBuilder> for Gen2AdaptiveQCfg {}

impl AsFFI<ffi::STUHFL_T_ST25RU3993_Gen2_Anticollision> for Gen2AdaptiveQ {
    // INPUTS: Self(Gen2AdaptiveQ)
    // OUTPUTS: ffi::STUHFL_T_ST25RU3993_Gen2_Anticollision

    fn as_ffi(&self) -> ffi::STUHFL_T_ST25RU3993_Gen2_Anticollision {
        match self {
            //If in Enable Configuration
            Gen2AdaptiveQ::Enable(conf) => {
                let options = if conf.adjust_nic {
                    ffi::STUHFL_D_USE_QUERY_ADJUST_NIC as u8
                } else {
                    0
                } | if conf.single_adjust {
                    ffi::STUHFL_D_SINGLE_ADJUST as u8
                } else {
                    0
                } | if conf.use_ceil_floor {
                    ffi::STUHFL_D_USE_CEIL_FLOOR as u8
                } else {
                    0
                } | if conf.reset_after_round {
                    ffi::STUHFL_D_RESET_Q_AFTER_ROUND as u8
                } else {
                    0
                };

                // Creating an Anticollison variable with the control flow above
                // to decide 'options'
                ffi::STUHFL_T_ST25RU3993_Gen2_Anticollision {
                    adaptiveQ: true,
                    startQ: conf.start_q,
                    minQ: conf.min_q,
                    maxQ: conf.max_q,
                    options,
                    ..Default::default()
                }
            }
            //If in Disable Configuration
            // use all firmware defaults
            Gen2AdaptiveQ::Disable(q) => ffi::STUHFL_T_ST25RU3993_Gen2_Anticollision {
                adaptiveQ: false,
                startQ: *q,
                ..Default::default()
            },
        }
    }
}

impl Gen2AdaptiveQCfgBuilder {
    /// Validates the configuration by ensuring that the
    /// min_q and max_q parameters make sense
    fn validate(&self) -> core::result::Result<(), String> {
        if let Some(max) = self.max_q {
            if let Some(min) = self.min_q {
                if max <= min {
                    return Err("max_q must be greater than min_q".to_owned());
                }
            }

            if max > ffi::STUHFL_D_GEN2_MAXQ as u8 {
                return Err("max_q too large: see docs for details".to_owned());
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
    #[builder(default = "true")]
    fast: bool,
    /// Automatic Acknowledgement enabling. If set to false, inventory rounds will be triggered
    /// by the firmware, otherwise the commands will be sent automatically.
    #[builder(default = "true")]
    auto_ack: bool,
    /// Enable reading TID's during inventory rounds
    #[builder(default = "true")]
    read_tid: bool,
}

impl Builder<Gen2InventoryOptionsBuilder> for Gen2InventoryOptions {}

// Converts 'Gen2InventoryOptions' to 'ffi::STUHFL_T_ST25RU3993_Gen2_InventoryOption'
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

// Converts 'Gen2QueryParams' to 'ffi::STUHFL_T_ST25RU3993_Gen2_QueryParams'
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

#[derive(Builder, Copy, Clone)]
/// Settings for Gen2 Inventorying. Note that [`Gen2InventoryOptions`] is a only subset of
/// these settings. See [`Self::builder()`] for details.
/// See also: [`Gen2Cfg`]
pub struct Gen2InventoryCfg {
    /// Extra inventory options
    #[builder(default = "Gen2InventoryOptionsBuilder::default().build().unwrap()")]
    options: Gen2InventoryOptions,

    /// Automatic Q adjustment settings (Anti-collision)
    #[builder(
        default = "Gen2AdaptiveQ::Enable(Gen2AdaptiveQCfgBuilder::default().build().unwrap())"
    )]
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

// Converts 'Gen2InventoryCfg' to 'ffi::STUHFL_T_ST25RU3993_Gen2_InventoryCfg'
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

// Converts 'Gen2ProtocolCfg' to 'ffi::STUHFL_T_ST25RU3993_Gen2_ProtocolCfg'
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
    // CB 7/9/25: Default configuration has been set to the Custom profile
    #[builder(default = "ChannelListCfg::from_profile(Profile::Custom)")]
    pub(crate) channel_list: ChannelListCfg,

    /// Frequency hopping configuration
    #[builder(default = "FreqHopCfg::builder().build().unwrap()")]
    pub(crate) freq_hop: FreqHopCfg,
}

impl Builder<Gen2CfgBuilder> for Gen2Cfg {}

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
    /// length of command data in bits
    pub(crate) num_bits: u16,
    /// array of data bytes
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
                bytes,
            })
        }
    }
}

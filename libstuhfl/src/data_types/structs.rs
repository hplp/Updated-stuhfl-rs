use super::{enums::*, traits::*, types::*};
use crate::helpers::{item_list_to_ffi, profile_to_item_list};
use enum_primitive::FromPrimitive;
use std::fmt;

/// Password for authentication during various protocol commands.
/// For a list of which commands require/support password authentication,
/// see [`ProtocolReader`].
///
/// # Example
/// ```
/// use libstuhfl::prelude::*;
/// # fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
/// # let mut reader = unsafe{DummyReader::new()};
///
/// let password = Password::from([0x55, 0x55, 0x55, 0x55]);
///
/// let bytes = reader.read(MemoryBank::User, 0x00, 8, Some(password))?;
///
/// # Ok(())}
/// ```
#[derive(Copy, Clone)]
pub struct Password([u8; 4]);

impl Password {
    /// Returns the password as [`[u8; 4]`].
    /// This function consumes self
    pub fn into_inner(self) -> [u8; 4] {
        self.0
    }
}

impl From<[u8; 4]> for Password {
    fn from(password: [u8; 4]) -> Self {
        Self(password)
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
    pub nano: u8,
}

impl fmt::Display for VersionNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "v{}.{}.{}.{}",
            self.major, self.minor, self.micro, self.nano
        )
    }
}

impl From<ffi::STUHFL_T_Version> for VersionNum {
    fn from(v: ffi::STUHFL_T_Version) -> Self {
        VersionNum {
            major: v.major,
            minor: v.minor,
            micro: v.micro,
            nano: v.nano,
        }
    }
}

#[derive(Clone)]
/// Contains a version descriptor string
pub struct VersionInfo {
    /// Information about version
    pub info: String,
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    pub hw_info: VersionInfo,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SW: {}, {}. HW: {}, {}.",
            self.sw_ver, self.sw_info, self.hw_ver, self.hw_info
        )
    }
}

#[derive(Builder, Copy, Clone)]
#[builder(build_fn(validate = "Self::validate"))]
/// Contains antenna configuration settings. See [`Self::builder()`] for details.
pub struct TxRxCfg {
    /// Transmission output level (dB). See control register 3 for further info. Valid range [0dB..-19dB].
    #[builder(default = "-2")]
    tx_output_level: i8,
    /// Reciever sensitivity level (dB). Valid range [-17dB..+19dB].
    #[builder(default = "-3")]
    rx_sensitivity_level: i8,
    /// Antenna to be used.
    #[builder(default = "Antenna::Antenna1")]
    antenna: Antenna,
    /// Time in ms for alternating the antennas when alternating mode is used.
    #[builder(default = "1")]
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
            rfu: 3, // RFU defined in firmware...
        }
    }
}

impl TxRxCfgBuilder {
    fn validate(&self) -> core::result::Result<(), String> {
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

#[derive(Builder, Copy, Clone)]
/// Auto-tuning algorithm parameters. This is used during `inventory_runner` cycles.
pub struct AutoTuning {
    /// Auto-tuning check interval (in inventory rounds)
    #[builder(default = "7")]
    interval: u16,
    /// Devation from (I^2+Q^2) to trigger a retuning
    #[builder(default = "20")]
    level: u8,
    /// Algorithm to use for automatic tuning
    #[builder(default = "TuningAlgorithm::Fast")]
    algo: TuningAlgorithm,
    /// Do false positive detection check
    #[builder(default = "true")]
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

#[derive(Copy, Clone, PartialEq)]
/// Automatic reciever sensitivity setting. Currently this is
/// only a boolean, as the algorithm parameters in the firmware
/// are too vague.
pub struct AutoRxSensitivity {
    enable: bool,
}

impl From<bool> for AutoRxSensitivity {
    fn from(enable: bool) -> Self {
        Self { enable }
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
    enable: bool,
}

impl From<bool> for AutoTxStrength {
    fn from(enable: bool) -> Self {
        Self { enable }
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
                skipLBTcheck: false,
            },
            Lbt::Disable => ffi::STUHFL_T_ST25RU3993_FreqLBT {
                skipLBTcheck: true,
                ..Default::default()
            },
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
        Self { cin, clen, cout }
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
        Self { frequency, caps }
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
            item_list: Vec::from(item_list),
        }
    }

    /// Create a ChannelList using a profile specified in the firmware
    pub fn from_profile(profile: Profile) -> Self {
        Self {
            item_list: profile_to_item_list(profile),
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

#[derive(Debug, PartialEq, Clone)]
/// Container for hexadecimal-based ID values such as TID, XPC, and EPC.
pub struct HexID {
    pub(crate) id: Vec<u8>,
}

impl HexID {
    /// Create a HexID from a list of integers.
    pub fn from_id(id: Vec<u8>) -> HexID {
        HexID { id }
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
        let mut id = self
            .id
            .iter()
            .fold(String::new(), |a, b| a + &format!("{:02X}", b) + ":");
        id.pop();
        write!(f, "{}", &id)
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

impl fmt::Display for InventoryTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Slot ID: {}
            Timestamp: {}
            Antenna: {}
            AGC: {}
            RSSI log i: {}
            RSSI log q: {}
            RSSI lin i: {}
            RSSI lin q: {}",
            self.slot_id,
            self.timestamp,
            self.antenna,
            self.agc,
            self.rssi_log_i,
            self.rssi_log_q,
            self.rssi_lin_i,
            self.rssi_lin_q,
        )
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

impl fmt::Display for InventoryStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Timestamp: {}
            Round Count: {}
            Tuning Status: {}
            RSSI log mean: {}
            Sensitivity: {}
            Final Q: {}
            Frequency: {} Hz
            ADC: {}
            Tag Count: {}
            Empty Slot Count: {}
            Collision Count: {}
            Preamble Error Count: {}
            CRC Error Count: {}
            RX Count Error Count: {}
            Resend Ack Count: {}
            Noise Suspicion Count: {}",
            self.timestamp,
            self.round_count,
            self.tuning_status,
            self.rssi_log_mean,
            self.sensitivity,
            self.final_q,
            self.frequency,
            self.adc,
            self.tag_count,
            self.empty_slot_count,
            self.collision_count,
            self.preamble_err_count,
            self.crc_err_count,
            self.rx_count_err_count,
            self.resend_ack_count,
            self.noise_suspicion_count,
        )
    }
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

impl InventoryStatistics {
    pub(crate) fn new() -> Self {
        Self {
            timestamp: 0,
            round_count: 0,
            tuning_status: TuningStatus::Untuned,
            rssi_log_mean: 0,
            sensitivity: 0,
            final_q: 0,
            frequency: 0,
            adc: 0,
            tag_count: 0,
            empty_slot_count: 0,
            slot_count: 0,
            collision_count: 0,
            preamble_err_count: 0,
            crc_err_count: 0,
            rx_count_err_count: 0,
            resend_ack_count: 0,
            noise_suspicion_count: 0,
        }
    }
}

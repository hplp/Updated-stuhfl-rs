use super::{enums::*, structs::*, types::*};
use crate::gen2;
use crate::helpers::proc_err;
use std::mem::zeroed;

pub(crate) trait AsFFI<T> {
    fn as_ffi(&self) -> T;
}

/// Returns a builder to create the structure
pub trait Builder<T>
where
    T: std::default::Default,
{
    /// Default implementation, see [`Builder<T>`]
    fn builder() -> T {
        T::default()
    }
}

/// Marker trait for reader types. Used exclusively in blanket
/// implementation of [`BasicReader`]
pub unsafe trait GenericReader {}

/// Basic RFID Reader functionality.
pub unsafe trait BasicReader {
    fn get_version(&self) -> Result<Version>;
    fn configure_gen2(self, configuration: &gen2::Gen2Cfg) -> Result<gen2::Gen2Reader>;
}

pub trait ProtocolReader {
    fn tune(&mut self, algo: TuningAlgorithm) -> Result<()>;
    fn inventory_once(&self) -> Result<(InventoryStatistics, Vec<InventoryTag>)>;
    fn inventory(&mut self, num_rounds: u32, cb: Box<CallbackFn>) -> Result<InventoryStatistics>;
    fn select(&mut self, epc: &Epc) -> Result<()>;
    fn read(
        &mut self,
        bank: MemoryBank,
        word_address: u32,
        num_bytes: u8,
        password: Option<Password>,
    ) -> Result<Vec<u8>>;
    fn write(
        &mut self,
        bank: MemoryBank,
        word_adddress: u32,
        data: [u8; 2],
        password: Option<Password>,
    ) -> Result<()>;
}

unsafe impl<T> GenericReader for T where T: ProtocolReader {}

unsafe impl<T> BasicReader for T
where
    T: Sized + GenericReader,
{
    fn get_version(&self) -> Result<Version> {
        // Create structs to be filled by function
        let mut sw_ver: ffi::STUHFL_T_Version = unsafe { zeroed() };
        let mut hw_ver: ffi::STUHFL_T_Version = unsafe { zeroed() };
        let mut sw_info: ffi::STUHFL_T_VersionInfo = unsafe { zeroed() };
        let mut hw_info: ffi::STUHFL_T_VersionInfo = unsafe { zeroed() };

        // Attempt to get board version
        unsafe { proc_err(ffi::Get_BoardVersion(&mut sw_ver, &mut hw_ver))? }
        unsafe { proc_err(ffi::Get_BoardInfo(&mut sw_info, &mut hw_info))? }

        // Move structs to safe memory
        let sw_ver = VersionNum::from(sw_ver);
        let hw_ver = VersionNum::from(hw_ver);

        // Recreate info as a "Rust safe" string
        let sw_info: String = unsafe {
            std::ffi::CStr::from_ptr(&sw_info.info as *const _)
                .to_string_lossy()
                .to_string()
        };
        let hw_info: String = unsafe {
            std::ffi::CStr::from_ptr(&hw_info.info as *const _)
                .to_string_lossy()
                .to_string()
        };

        // Return safe version
        Ok(Version {
            sw_ver,
            hw_ver,
            sw_info: VersionInfo { info: sw_info },
            hw_info: VersionInfo { info: hw_info },
        })
    }

    fn configure_gen2(self, configuration: &gen2::Gen2Cfg) -> Result<gen2::Gen2Reader> {
        // Set up antenna configuration
        let mut tx_rx_cfg = configuration.tx_rx_cfg.as_ffi();
        unsafe { proc_err(ffi::Set_TxRxCfg(&mut tx_rx_cfg))? }

        // Set up inventory configuration
        let mut inv_cfg = configuration.inv_cfg.as_ffi();
        unsafe { proc_err(ffi::Set_Gen2_InventoryCfg(&mut inv_cfg))? }

        // Set up protocol configuration
        let mut proto_cfg = configuration.proto_cfg.as_ffi();
        unsafe { proc_err(ffi::Set_Gen2_ProtocolCfg(&mut proto_cfg))? }

        // Set up lbt configuraiton
        let mut lbt = configuration.lbt.as_ffi();
        unsafe { proc_err(ffi::Set_FreqLBT(&mut lbt))? }

        // Set up channel list configuration
        let mut channel_list = configuration.channel_list.as_ffi();
        unsafe { proc_err(ffi::Set_ChannelList(&mut channel_list))? }

        // Set up frequency hopping configuration
        let mut freq_hop = configuration.freq_hop.as_ffi();
        unsafe { proc_err(ffi::Set_FreqHop(&mut freq_hop))? }

        // Clear select configuration
        let mut gen2_select = ffi::STUHFL_T_Gen2_Select {
            mode: ffi::STUHFL_D_GEN2_SELECT_MODE_CLEAR_LIST as u8,
            ..Default::default()
        };
        unsafe { proc_err(ffi::Gen2_Select(&mut gen2_select))? }

        Ok(unsafe { gen2::Gen2Reader::new() })
    }
}

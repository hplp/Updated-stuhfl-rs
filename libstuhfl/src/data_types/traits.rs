use super::{enums::*, structs::*, types::*};
use crate::error::Result;
use crate::gen2;
use crate::helpers::proc_err;
use crate::reader::Reader;
use std::mem::zeroed;

/// Allows various datatypes to be converted
/// into their C representation within the library.
pub(crate) trait AsFFI<T> {
    /// Does the actual conversion
    fn as_ffi(&self) -> T;
}

// CB 7/14/25: The gen2_config uses this builder to configure itself using default settings

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

/// Basic RFID Reader functionality. Includes commands that don't
/// depend on any particular protocol. See [`ProtocolReader`] for
/// commands that depend on the protocol.
///
/// # Safety
///
/// This trait must only be implemented for types that handle
/// reader instances. Otherwise the C library's state can
/// be corrupted. This is enforced through the HasConnection trait
pub unsafe trait BasicReader: Sized + ConnectionHolder {
    /// # Getting the reader version
    ///
    /// This function returns a [`Version`] instance with the reader's
    /// firmware and hardware information. Note: the version is normally
    /// automatically checked for compatibility during reader construction.
    ///
    /// # Example
    ///
    /// ```
    /// use libstuhfl::prelude::*;
    /// # fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    /// # let mut reader = unsafe{DummyReader::new()};
    ///
    /// let version = reader.get_version().expect("Failed to get reader version");
    ///
    /// println!("Reader version: {}", &version);
    ///
    /// # Ok(())
    /// # }
    /// ```
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

    /// # Configuring reader
    ///
    /// This function removes any protocol-specific configuration from the reader.
    ///
    fn disconfigure(self) -> Reader {
        Reader::new(self.steal_connection())
    }

    /// # Configuring reader
    ///
    /// This function configures the reader for use of the Gen2 protocol.
    /// See [`gen2::Gen2Cfg`] for details. Note: all settings have valid defaults,
    /// however most can be overrided. For usage see [`gen2::Gen2Reader`] and
    /// [`ProtocolReader`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use libstuhfl::prelude::*;
    /// use libstuhfl::gen2::*;
    /// # fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    ///
    /// let mut reader = Reader::autoconnect()?;
    ///
    /// let gen2_cfg = Gen2Cfg::builder()
    ///     .build()?;
    ///
    /// let gen2_reader = reader
    ///     .configure_gen2(&gen2_cfg)
    ///     .expect("Failed to configure reader");
    ///
    /// # Ok(())
    /// # }
    /// ```
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

        Ok(gen2::Gen2Reader::new(self.steal_connection()))
    }

    /// Tests whether a connected reader is compatible with the
    /// middle-ware library. Note that this should be *already*
    /// called by the constructor, so there is no need for
    /// users to run this method.
    fn test_compatible(&self) -> Result<bool> {
        /// Defined in firmware
        const LOWEST_SW_VER: VersionNum = VersionNum {
            major: 3,
            minor: 1,
            micro: 0,
            nano: 0,
        };
        /// Defined in firmware
        const LOWEST_HW_VER: VersionNum = VersionNum {
            major: 1,
            minor: 1,
            micro: 0,
            nano: 0,
        };

        // Determine board version
        let ver = self.get_version()?;

        // Check minimum version satisfied
        Ok(ver.sw_ver >= LOWEST_SW_VER && ver.hw_ver >= LOWEST_HW_VER)
    }

    /// # Disconnecting from reader
    ///
    /// This function disconnects from the reader.
    ///
    /// # Safety
    ///
    /// This puts the reader in an invalid state. It should only
    /// be called by the [`Drop`] implementation.
    ///
    unsafe fn disconnect(&mut self) -> Result<()> {
        proc_err(ffi::Disconnect())
    }

    /// # Adjusting antenna output power
    // CB 6/19/25: could this be used to increase range?
    fn set_antenna_power(&mut self, on: bool, timeout: u16, frequency: u32) -> Result<()> {
        unsafe {
            proc_err(ffi::Set_AntennaPower(
                &mut ffi::STUHFL_T_ST25RU3993_AntennaPower {
                    mode: on as u8,
                    timeout,
                    frequency,
                },
            ))
        }
    }
    
    /// # Adjusting amplifier power configuration
    fn set_power_amplifier_cfg(&mut self, external: bool) -> Result<()> {
        unsafe {
            proc_err(ffi::Set_PowerAmplifierCfg(
                &mut ffi::STUHFL_T_ST25RU3993_PowerAmplifierCfg { external },
            ))
        }
    }
}

/// Protocol-Specific reader commands. These include any commands that require
/// the reader to be configured to use an RFID protocol. See [`BasicReader`]
/// for more.
///
/// # Safety
///
/// This trait must only be implemented for types that manage reader instances
/// (see [`BasicReader`]). Furthermore, all readers implementing this trait must
/// be able to handle the full funcitonality defined in this trait.
pub unsafe trait ProtocolReader: BasicReader {

    /// # Tuning reader
    ///
    /// Tune the reader using the specified tuning algorithm.
    /// See [`TuningAlgorithm`] for details. Note: This must
    /// be done before issuing other commands. If you would
    /// like to intentionally use the reader untuned, use
    /// [`TuningAlgorithm::None`].
    ///
    /// # Example
    ///
    /// ```
    /// use libstuhfl::prelude::*;
    /// # fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    /// # let mut reader = unsafe{DummyReader::new()};
    ///
    /// reader.tune(TuningAlgorithm::Exact)?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn tune(&mut self, algo: TuningAlgorithm) -> Result<()>;

    /// # Inventorying tags
    ///
    /// There are two ways to inventory tags, using this command or
    /// the [`ProtocolReader::inventory()`] command. This command will only
    /// run a single round and return a vector of tags. Running this
    /// in a loop will cause a huge amount of unecessary memory allocations
    /// and copies, so it is recommended to use [`ProtocolReader::inventory()`]
    /// if several rounds are needed.
    ///
    /// # Example
    /// ```
    /// use libstuhfl::prelude::*;
    /// # fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    /// # let mut reader = unsafe{DummyReader::new()};
    ///
    /// reader.tune(TuningAlgorithm::Exact)?;
    /// let (stats, tags) = reader.inventory_once()?;
    ///
    /// println!("Inventory Statistics:");
    /// println!("{}", &stats);
    ///
    /// println!("Tags found:");
    /// for tag in &tags {
    ///     println!("{}", tag.epc);
    /// }
    ///
    /// # Ok(())}
    fn inventory_once(&self) -> Result<(InventoryStatistics, Vec<InventoryTag>)>;

    /// # Inventorying tags (threaded)
    ///
    /// There are two ways to inventory tags, using this command or
    /// the [`ProtocolReader::inventory_once()`] command. This command will run `num_rounds`
    /// inventory rounds, and for each round where tags are discovered the
    /// `data_cb` will be called. Note: this is a blocking call.
    ///
    /// # Example
    /// ```
    /// use std::sync::{Arc, Mutex};
    /// use libstuhfl::prelude::*;
    /// # fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    /// # let mut reader = unsafe{DummyReader::new()};
    ///
    /// reader.tune(TuningAlgorithm::Exact)?;
    ///
    /// // create thread-safe shared vector of tags
    /// let tags = Arc::new(Mutex::new(Vec::new()));
    /// let tags2 = Arc::clone(&tags);
    ///
    /// // create callback function
    /// let callback = move |tag| {
    ///     let mut tags = tags2.lock().unwrap();
    ///     tags.push(tag);
    /// };
    ///
    /// let stats = reader.inventory(20, Box::new(callback))?;
    ///
    /// println!("Inventory Statistics:");
    /// println!("{}", &stats);
    ///
    /// let tags = tags.lock().unwrap();
    ///
    /// println!("Tags found:");
    /// for tag in &*tags {
    ///     println!("{}", tag.epc);
    /// }
    ///
    /// # Ok(())}
    /// ```
    ///
    /// # Known Issues
    ///
    /// Currently, there is no way to prematurely stop the inventory.
    /// This would ideally happen automatically when the callback panics,
    /// or allow for the callback to control the end of the inventory
    /// in general.
    ///
    /// # Panics
    ///
    /// Any panics generated in `data_cb` will be caught by
    /// a wrapper callback. This function should never panic.
    fn inventory(&mut self, num_rounds: u32, cb: Box<CallbackFn>) -> Result<InventoryStatistics>;

    /// # Selecting a tag
    ///
    /// This function allows you to select an invdividual tag using its EPC number.
    /// By doing so, access functions such as read can be safely used.
    ///
    /// # Example
    /// ```
    /// use libstuhfl::prelude::*;
    /// # fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    /// # let mut reader = unsafe{DummyReader::new()};
    ///
    /// reader.tune(TuningAlgorithm::Exact)?;
    /// let (_stats, tags) = reader.inventory_once()?;
    ///
    /// if tags.is_empty() { panic!("Tags list is empty") }
    ///
    /// reader.select(&tags[0].epc)?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn select(&mut self, epc: &Epc) -> Result<()>;

    /// # Reading a tag
    ///
    /// This command allows a tag to be read. Note: be sure to
    /// select a tag first, or the read order will be random.
    ///
    /// # Example
    /// ```
    /// use libstuhfl::prelude::*;
    /// # fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    /// # let mut reader = unsafe{DummyReader::new()};
    ///
    /// reader.tune(TuningAlgorithm::Exact)?;
    /// let (_stats, tags) = reader.inventory_once()?;
    ///
    /// if tags.is_empty() { panic!("Tags list is empty") }
    ///
    /// for tag in tags {
    ///     println!("Found tag {}", &tag.epc);
    ///     let epc = reader.read(MemoryBank::Epc, 0x02, 12, None)?;
    ///     assert!(epc == tag.epc.get_id());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn read(
        &mut self,
        bank: MemoryBank,
        word_address: u32,
        num_bytes: u8,
        password: Option<Password>,
    ) -> Result<Vec<u8>>;

    /// # Writing to a tag
    ///
    /// This command allows you to write to a tag. Note that
    /// you should select a tag before writing.
    ///
    /// # Example
    /// ```
    /// use libstuhfl::prelude::*;
    /// # fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    /// # let mut reader = unsafe{DummyReader::new()};
    ///
    /// reader.tune(TuningAlgorithm::Exact)?;
    /// let (_stats, tags) = reader.inventory_once()?;
    ///
    /// if tags.is_empty() { panic!("Tags list is empty") }
    /// reader.select(&tags[0].epc)?;
    ///
    /// reader.write(MemoryBank::User, 0x00, [0x55, 0x55], None)?;
    /// let bytes = reader.read(MemoryBank::User, 0x00, 2, None)?;
    /// # let bytes = [0x55, 0x55];
    ///
    /// assert_eq!(bytes, [0x55, 0x55]);
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn write(
        &mut self,
        bank: MemoryBank,
        word_adddress: u32,
        data: [u8; 2],
        password: Option<Password>,
    ) -> Result<()>;
}

/// Used to prove that a reader has an active connection.
pub trait ConnectionHolder {
    /// This trait is impossible to implement without having sole
    /// access to a valid connection.
    fn steal_connection(self) -> Connection;
}

use crate::data_types::*;
use crate::error::Error;
use crate::helpers::proc_err;

#[cfg(feature = "port_scanning")]
use serialport as sp;

/// Main reader struct. See [`BasicReader`] for more details.
pub struct Reader {}

unsafe impl GenericReader for Reader {}

impl Reader {
    /// # Connecting to reader
    ///
    /// Using [`Self::autoconnect()`] requires the `port_scanning` feature. This method scans
    /// all available USB TTY/COM ports on the computer and checks their vendor & product
    /// ID's. Upon finding a port successfully, the [`Self::connect()`] method is automatically
    /// invoked.
    ///
    /// # Example
    /// ```no_run
    /// # use libstuhfl::prelude::*;
    /// # fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    ///
    /// // automatically with port scanning
    /// #[cfg(feature = "port_scanning")]
    /// let reader = Reader::autoconnect()?;
    ///
    /// // manually without port scanning
    /// #[cfg(not(feature = "port_scanning"))]
    /// let reader = Reader::connect("/dev/ttyUSB0")?;
    ///
    /// # Ok(())}
    /// ```
    ///
    /// # Errors
    ///
    /// This function errors if the reader cannot be safely connected to. A [`Error::GeneralIo`]
    /// error may be issued if no valid ports can be found/opened. See [`Self::connect()`] for
    /// more info.
    #[cfg(feature = "port_scanning")]
    pub fn autoconnect() -> Result<Self> {
        let mut found_port: Option<String> = None;

        if let Ok(ports) = sp::available_ports() {
            for port in ports {
                if let sp::SerialPortType::UsbPort(port_info) = port.port_type {
                    if port_info.vid == 0x403
                        && port_info.pid == 0x6015
                        && sp::new(&port.port_name, 9600).open().is_ok()
                    {
                        found_port = Some(port.port_name);
                    }
                }
            }
        }

        // Try connecting to port
        if let Some(found_port) = found_port {
            Self::connect(&found_port)
        } else {
            Err(Error::GeneralIo)
        }
    }

    /// # Connecting to reader
    ///
    /// Using [`Self::connect()`] will attempt to connect to the reader using the
    /// user specified port. Note that it's up to the caller to ensure that this port
    /// is valid and in an opened state. After connecting to the reader, a check is
    /// done on the firmware and hardware versions of the board to ensure compatibility.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use libstuhfl::prelude::*;
    /// # fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    ///
    /// #[cfg(unix)]
    /// let reader = Reader::connect("/dev/ttyUSB0")?;
    ///
    /// #[cfg(windows)]
    /// let reader = Reader::connect("COM6")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function errors if the reader cannot be safely connected to. The function
    /// will return [`Error::None`] if the reader's firmware or hardware is incompatible
    /// with this library.
    pub fn connect(port: &str) -> Result<Self> {
        // Copy the port so that its "safe" from C
        let port = std::ffi::CString::new(port).expect("Failed to convert string");

        // Connect to board
        unsafe { proc_err(ffi::Connect(port.as_ptr() as *mut _))? }

        // Wait so that board has time to connect
        std::thread::sleep(std::time::Duration::from_micros(600000));

        // Construct new instance
        let board = Self {};

        // Test compatibility
        if board.test_compatible()? {
            Ok(board)
        } else {
            eprintln!("Warning: Incompatible Board or Library Version Detected. Please verify that your FW is up to date.");
            Err(Error::None)
        }
    }

    /// Tests whether a connected reader is compatible with this
    /// middleware library.
    fn test_compatible(&self) -> Result<bool> {
        // Defined in firmware
        const LOWEST_SW_VER: VersionNum = VersionNum {
            major: 3,
            minor: 1,
            micro: 0,
            nano: 0,
        };
        // Defined in firmware
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
}

/// A reader used for test purposes. All of its functions do nothing
/// and should always succeed.
pub struct DummyReader();
impl ProtocolReader for DummyReader {
    fn tune(&mut self, _algo: TuningAlgorithm) -> Result<()> {
        Ok(())
    }

    fn inventory_once(&self) -> Result<(InventoryStatistics, Vec<InventoryTag>)> {
        Ok((InventoryStatistics::new(), Vec::new()))
    }

    fn inventory(&mut self, _num_rounds: u32, _cb: Box<CallbackFn>) -> Result<InventoryStatistics> {
        Ok(InventoryStatistics::new())
    }

    fn select(&mut self, _epc: &Epc) -> Result<()> {
        Ok(())
    }

    fn read(
        &mut self,
        _bank: MemoryBank,
        _word_address: u32,
        _num_bytes: u8,
        _password: Option<Password>,
    ) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }

    fn write(
        &mut self,
        _bank: MemoryBank,
        _word_adddress: u32,
        _data: [u8; 2],
        _password: Option<Password>,
    ) -> Result<()> {
        Ok(())
    }
}

impl Default for DummyReader {
    fn default() -> Self {
        Self {}
    }
}

/*
impl ST25RU3993 {

    /// # Connecting to reader
    ///
    /// Using [`Self::from_port()`] will attempt to connect to the reader using the
    /// user specified port. Note that it's up to the caller to ensure that this port
    /// is valid and in an opened state. After connecting to the reader, a check is
    /// done on the firmware and hardware versions of the board to ensure compatibility.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use libstuhfl::*;
    /// # fn main() -> core::result::Result<(), Box<dyn Error>> {
    /// // Open the reader using a specific port
    /// # let reader = ST25RU3993::new()?;
    ///
    /// #[cfg(unix)]
    /// let reader = ST25RU3993::from_port("/dev/ttyUSB0")?;
    ///
    /// #[cfg(windows)]
    /// let reader = ST25RU3993::from_port("COM6")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function errors if the reader cannot be safely connected to. The function
    /// will return [`Error::None`] if the reader's firmware or hardware is incompatible
    /// with this library.
    ///
    pub fn from_port(port: &str) -> Result<Self> {
        unsafe {
            // Copy the port so that its "safe" from C
            let port = std::ffi::CString::new(port).expect("Failed to convert string");

            // Connect to board
            proc_err(ffi::Connect(port.as_ptr() as *mut _))?;

            // Wait so that board has time to connect
            std::thread::sleep(std::time::Duration::from_micros(600000));

            // Create object so we can test version
            let board = ST25RU3993 {
                protocol: None
            };

            // Test compatibility
            if board.test_compatible()? {
                Ok(board)
            } else {
                eprintln!("Warning: Incompatible Board or Library Version Detected. Please verify that your FW is up to date.");
                Err(Error::None)
            }
        }
    }

    /// # Getting the board version
    ///
    /// This function returns a [`Version`] instance with the reader's
    /// firmware and hardware information. See [`Self`] for usage.
    pub fn get_board_version(&self) -> Result<Version> {
        unsafe {
            // Create structs to be filled by function
            let mut sw_ver: ffi::STUHFL_T_Version = mem::zeroed();
            let mut hw_ver: ffi::STUHFL_T_Version = mem::zeroed();
            let mut sw_info: ffi::STUHFL_T_VersionInfo = mem::zeroed();
            let mut hw_info: ffi::STUHFL_T_VersionInfo = mem::zeroed();

            // Attempt to get board version
            proc_err(ffi::Get_BoardVersion(&mut sw_ver, &mut hw_ver))?;
            proc_err(ffi::Get_BoardInfo(&mut sw_info, &mut hw_info))?;

            // Move structs to safe memory
            let sw_ver = VersionNum::from(sw_ver);
            let hw_ver = VersionNum::from(hw_ver);

            // Recreate info as a "Rust safe" string
            let sw_info: String = std::ffi::CStr::from_ptr(&sw_info.info as *const _).to_string_lossy().to_string();
            let hw_info: String = std::ffi::CStr::from_ptr(&hw_info.info as *const _).to_string_lossy().to_string();

            // Return safe version
            Ok(Version {
                sw_ver,
                hw_ver,
                sw_info: VersionInfo {info: sw_info},
                hw_info: VersionInfo {info: hw_info},
            })
        }
    }

    /// Tests whether the board is compatible or not.
    /// (This is called automatically by [`Self::from_port()`], so it isn't
    /// user accessible, otherwise people would run it uneccessarily)
    fn test_compatible(&self) -> Result<bool> {
        const LOWEST_SW_VER: VersionNum = VersionNum {
            major: 3,
            minor: 1,
            micro: 0,
            nano: 0
        };
        const LOWEST_HW_VER: VersionNum = VersionNum {
            major: 1,
            minor: 1,
            micro: 0,
            nano: 0
        };

        let ver = self.get_board_version()?;

        Ok(ver.sw_ver >= LOWEST_SW_VER && ver.hw_ver >= LOWEST_HW_VER)
    }

    /// # Configuring reader
    ///
    /// This function configures the reader for use of the Gen2 protocol.
    /// See [`Gen2Cfg`] for details. Note: all settings have valid defaults,
    /// however most can be overrided.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use libstuhfl::*;
    /// # #[cfg(feature = "port_scanning")]
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// // Configure reader for use with Gen2 standard
    ///
    /// let mut reader = ST25RU3993::new()?; // new requires port scanning feature
    ///
    /// let cfg = Gen2Cfg::builder().build()?;
    ///
    /// reader.configure_gen2(&cfg)?;
    /// # Ok(())
    /// # }
    /// # #[cfg(not(feature = "port_scanning"))]
    /// # fn main() {}
    /// ```
    ///
    pub fn configure_gen2(&mut self, gen2_cfg: &Gen2Cfg) -> Result<()> {
        // Reset protocol, in case of invalid state
        self.protocol = None;

        // Set up antenna configuration
        let mut tx_rx_cfg = gen2_cfg.tx_rx_cfg.as_ffi();
        unsafe {proc_err(ffi::Set_TxRxCfg(&mut tx_rx_cfg))?}

        // Set up inventory configuration
        let mut inv_cfg = gen2_cfg.inv_cfg.as_ffi();
        unsafe {proc_err(ffi::Set_Gen2_InventoryCfg(&mut inv_cfg))?}

        // Set up protocol configuration
        let mut proto_cfg = gen2_cfg.proto_cfg.as_ffi();
        unsafe {proc_err(ffi::Set_Gen2_ProtocolCfg(&mut proto_cfg))?}

        // Set up lbt configuraiton
        let mut lbt = gen2_cfg.lbt.as_ffi();
        unsafe {proc_err(ffi::Set_FreqLBT(&mut lbt))?}

        // Set up channel list configuration
        let mut channel_list = gen2_cfg.channel_list.as_ffi();
        unsafe {proc_err(ffi::Set_ChannelList(&mut channel_list))?}

        // Set up frequency hopping configuration
        let mut freq_hop = gen2_cfg.freq_hop.as_ffi();
        unsafe {proc_err(ffi::Set_FreqHop(&mut freq_hop))?}

        // Clear select configuration
        let mut gen2_select = ffi::STUHFL_T_Gen2_Select{
            mode: ffi::STUHFL_D_GEN2_SELECT_MODE_CLEAR_LIST as u8,
            ..Default::default()
        };
        unsafe {proc_err(ffi::Gen2_Select(&mut gen2_select))?}

        // Enable Gen2 protocol commands for reader
        self.protocol = Some(Protocol::Gen2);
        Ok(())
    }

    /// # Tuning reader
    ///
    /// Tune the reader using the specified tuning algorithm. Note: must be called
    /// **after** configuring a protocol, in order to know which antenna and frequencies
    /// to tune. See [`TuningAlgorithm`] for details.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use libstuhfl::*;
    /// # #[cfg(feature = "port_scanning")]
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// // Tune reader using configuration for Gen2 standard
    ///
    /// let mut reader = ST25RU3993::new()?; // new requires port scanning feature
    ///
    /// let cfg = Gen2Cfg::builder().build()?;
    ///
    /// reader.configure_gen2(&cfg)?;
    ///
    /// reader.tune_freqs(TuningAlgorithm::Exact)?;
    /// # Ok(())
    /// # }
    /// # #[cfg(not(feature = "port_scanning"))]
    /// # fn main() {}
    /// ```
    ///
    /// # Errors
    ///
    /// If no protocol is configured this will return [`Error::None`]. Otherwise
    /// errors are generated by firmware.
    ///
    pub fn tune_freqs(&mut self, algo: TuningAlgorithm) -> Result<()> {
        if self.protocol.is_none() {
            eprintln!("Error: Must configure a protocol before tuning");
            return Err(Error::None)
        }

        // None does nothing
        if algo == TuningAlgorithm::None {
            return Ok(())
        }

        // Get the current reader settings, we need to know which antenna is in use
        let mut tx_rx_cfg = ffi::STUHFL_T_ST25RU3993_TxRxCfg::default();
        unsafe {proc_err(ffi::Get_TxRxCfg(&mut tx_rx_cfg))?}

        // Create a tune configuration using the antenna & algorithm
        let mut tune_cfg = ffi::STUHFL_T_ST25RU3993_TuneCfg{
            antenna: tx_rx_cfg.usedAntenna,
            algorithm: algo as u8,
            tuneAll: true,
            ..Default::default()
        };

        // Tune the reader using the configuration
        unsafe {proc_err(ffi::TuneChannel(&mut tune_cfg))?}

        Ok(())
    }

    /// # Inventorying tags
    ///
    /// There are two ways to inventory tags, using this command or
    /// the [`Self::inventory_runner()`] command. This command will only
    /// run a single round and return a vector of tags. Running this
    /// in a loop will cause a huge amount of unecessary memory allocations
    /// and copies, so it is recommended to use [`Self::inventory_runner()`]
    /// if several rounds are needed.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use libstuhfl::*;
    /// # #[cfg(feature = "port_scanning")]
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// // Inventory Gen2 tags
    ///
    /// let mut reader = ST25RU3993::new()?; // new requires port scanning feature
    ///
    /// let cfg = Gen2Cfg::builder().build()?;
    ///
    /// reader.configure_gen2(&cfg)?;
    ///
    /// reader.tune_freqs(TuningAlgorithm::Exact)?;
    ///
    /// let (statistics, tags) = reader.inventory()?;
    ///
    /// println!("{:#?}", statistics);
    ///
    /// for tag in tags {
    ///     println!("[{}]: {}", tag.slot_id, tag.epc);
    /// }
    /// # Ok(())
    /// # }
    /// # #[cfg(not(feature = "port_scanning"))]
    /// # fn main() {}
    /// ```
    pub fn inventory(&mut self) -> Result<(InventoryStatistics, Vec<InventoryTag>)> {
        if self.protocol.is_none() { return Err(Error::None) };

        // create tag data storage location
        let mut tag_data: [ffi::STUHFL_T_InventoryTag; ffi::STUHFL_D_MAX_TAG_LIST_SIZE as usize] = unsafe{std::mem::zeroed()};

        // create tag data storage container
        let mut inv_data = ffi::STUHFL_T_InventoryData{
            tagList: &mut tag_data as _,
            tagListSizeMax: tag_data.len() as u16,
            ..Default::default()
        };

        // customize inventory options
        let mut inv_option = ffi::STUHFL_T_InventoryOption {
            options: ffi::STUHFL_D_INVENTORYREPORT_OPTION_NONE as u8,
            ..Default::default()
        };

        // run the inventory
        unsafe{proc_err(ffi::Gen2_Inventory(&mut inv_option, &mut inv_data))?}

        // save data into iterator
        let tags = tag_data[..inv_data.statistics.tagCnt as usize]
            .iter()
            .map(|tag| InventoryTag::from(*tag))
            .collect();

        let statistics = InventoryStatistics::from(inv_data.statistics);

        Ok((statistics, tags))
    }

    /// # Inventorying tags (threaded)
    ///
    /// There are two ways to inventory tags, using this command or
    /// the [`Self::inventory()`] command. This command will run `num_rounds`
    /// inventory rounds, and for each round where tags are discovered the
    /// `data_cb` will be called. Note: this is a blocking call.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use libstuhfl::*;
    /// # #[cfg(feature = "port_scanning")]
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// // Inventory Gen2 tags using inventory_runner
    /// # use std::sync::{Arc,Mutex};
    ///
    /// let mut reader = ST25RU3993::new()?; // new requires port scanning feature
    ///
    /// let cfg = Gen2Cfg::builder().build()?;
    ///
    /// reader.configure_gen2(&cfg)?;
    ///
    /// reader.tune_freqs(TuningAlgorithm::Exact)?;
    ///
    /// // create atomic vector of tags
    /// let tags = Arc::new(Mutex::new(Vec::new()));
    /// let tags2 = Arc::clone(&tags);
    ///
    /// // create callback function
    /// let callback = move |tag| {
    ///     let mut tags = tags2.lock().unwrap();
    ///     tags.push(tag);
    /// };
    ///
    /// // run inventory
    /// let statitistics = reader.inventory_runner(20, Box::new(callback))?;
    ///
    /// println!("Inventory Statistics:\n{:#?}", statitistics);
    /// println!("Found tags:");
    ///
    /// // lock tags
    /// let tags = tags.lock().unwrap();
    ///
    /// // read tags
    /// for tag in &*tags {
    ///     println!("{}", tag.epc);
    /// }
    /// # Ok(())
    /// # }
    /// # #[cfg(not(feature = "port_scanning"))]
    /// # fn main() {}
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return [`Error::None`] if no protocol
    /// has been configured, or if `num_rounds == 0`. If the callback
    /// panics then the funciton will return [`Error::Generic`].
    ///
    /// # Panics
    ///
    /// Any panics generated in `data_cb` will be caught by
    /// a wrapper callback. This function should never panic.
    pub fn inventory_runner(&mut self, num_rounds: u32, data_cb: Box<CallbackFn>) -> Result<InventoryStatistics> {
        if self.protocol.is_none() { return Err(Error::None) };

        if num_rounds == 0 {
            eprintln!("Error: num_rounds = 0 not yet implemented!");
            return Err(Error::None)
        }

        // create tag data storage location
        let mut tag_data: [ffi::STUHFL_T_InventoryTag; ffi::STUHFL_D_MAX_TAG_LIST_SIZE as usize] = unsafe{std::mem::zeroed()};

        // create tag data storage container
        let mut inv_data = ffi::STUHFL_T_InventoryData{
            tagList: &mut tag_data as _,
            tagListSizeMax: tag_data.len() as u16,
            ..Default::default()
        };

        // customize inventory options
        let mut inv_option = ffi::STUHFL_T_InventoryOption {
            options: ffi::STUHFL_D_INVENTORYREPORT_OPTION_NONE as u8,
            roundCnt: num_rounds,
            ..Default::default()
        };

        // Save callback function
        let mut cb_holder = CB_HOLDER.lock().unwrap();
        *cb_holder = Some(data_cb);
        drop(cb_holder);

        // Call inventory (blocking)
        let result = unsafe{proc_err(ffi::Inventory_RunnerStart(&mut inv_option, Some(cycle_cb), None, &mut inv_data))};

        let cb_success = if CB_HOLDER.is_poisoned() {
            Err(Error::Generic)
        } else {
            Ok(())
        };

        // Delete callback function
        let mut guard = match CB_HOLDER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        *guard = None;

        // Return any errors due to inventory failing
        result?;

        // Return any errors due to poisoning
        cb_success?;

        let statistics = InventoryStatistics::from(inv_data.statistics);

        Ok(statistics)
    }

    /// # Selecting a tag
    ///
    /// This function allows you to select an invdividual tag using its EPC number.
    /// By doing so, access functions such as read can be safely used.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use libstuhfl::*;
    /// # #[cfg(feature = "port_scanning")]
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// // Select a gen2 tag
    ///
    /// let mut reader = ST25RU3993::new()?; // new requires port scanning feature
    ///
    /// let cfg = Gen2Cfg::builder().build()?;
    ///
    /// reader.configure_gen2(&cfg)?;
    ///
    /// reader.tune_freqs(TuningAlgorithm::Exact)?;
    ///
    /// let (_statistics, tags) = reader.inventory()?;
    ///
    /// if tags.is_empty() { panic!("Tags list is empty") }
    ///
    /// // note: this selects the first tag found
    /// reader.select_gen2(&tags[0].epc)?;
    /// # Ok(())
    /// # }
    /// # #[cfg(not(feature = "port_scanning"))]
    /// # fn main() {}
    /// ```
    ///
    pub fn select_gen2(&mut self, epc: &Epc) -> Result<()> {

        let mut mask = [0; 32];

        for (i, x) in epc.id[..std::cmp::min(32, epc.id.len())].iter().enumerate() {
            mask[i] = *x;
        }

        let mut sel = ffi::STUHFL_T_Gen2_Select {
            mode: ffi::STUHFL_D_GEN2_SELECT_MODE_CLEAR_AND_ADD as u8,
            target: ffi::STUHFL_D_GEN2_TARGET_SL as u8,
            action: 0,
            memoryBank: ffi::STUHFL_D_GEN2_MEMORY_BANK_EPC as u8,
            maskBitPointer: 0x20,
            maskBitLength: if epc.id.len() >= ffi::STUHFL_D_GEN2_MAX_SELECT_MASK_LENGTH as usize {
                0xFF
            } else {
                epc.id.len() as u8 * 8
            },
            mask,
            truncation: 0,
        };

        unsafe{proc_err(ffi::Gen2_Select(&mut sel))?}

        Ok(())
    }

    /// # Reading a tag
    ///
    /// This command allows a tag to be read. Note: be sure to
    /// select a tag first, or the read order will be random.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use libstuhfl::*;
    /// # #[cfg(feature = "port_scanning")]
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// // Read gen2 tags' EPC memory bank
    ///
    /// let mut reader = ST25RU3993::new()?; // new requires port scanning feature
    ///
    /// let cfg = Gen2Cfg::builder().build()?;
    ///
    /// reader.configure_gen2(&cfg)?;
    ///
    /// reader.tune_freqs(TuningAlgorithm::Exact)?;
    ///
    /// let (_statistics, tags) = reader.inventory()?;
    ///
    /// for tag in tags {
    ///     println!("Found tag {}", &tag.epc);
    ///     reader.select_gen2(&tag.epc)?;
    ///     let epc = reader.read_gen2(MemoryBank::Epc, 0x02, 12, None)?;
    ///     assert!(epc == tag.epc.get_id());
    /// }
    /// # Ok(())
    /// # }
    /// # #[cfg(not(feature = "port_scanning"))]
    /// # fn main() {}
    /// ```
    pub fn read_gen2(&mut self, memory_bank: MemoryBank, word_ptr: u32, num_bytes: u8, password: Option<[u8; 4]>) -> Result<Vec<u8>> {
        // Make sure protocol is set up first
        if self.protocol.is_none() { return Err(Error::None) };

        let mut read_struct = ffi::STUHFL_T_Read {
            wordPtr: word_ptr,
            memoryBank: memory_bank as u8,
            numBytesToRead: num_bytes,
            pwd: if let Some(pwd) = password {
                pwd
            } else {
                [0; 4]
            },
            numReadBytes: 0,
            data: [0; 64]
        };

        // Call read
        unsafe{proc_err(ffi::Gen2_Read(&mut read_struct))?}

        // Create vector from read bytes
        let result = Vec::from(&read_struct.data[..read_struct.numReadBytes as usize]);

        // Return result
        Ok(result)
    }

    /// # Writing to a tag
    ///
    /// This command allows you to write to a tag. Note that
    /// you should select a tag before writing.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use libstuhfl::*;
    /// # #[cfg(feature = "port_scanning")]
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// // Write to gen2 tag's User memory bank
    ///
    /// let mut reader = ST25RU3993::new()?; // new requires port scanning feature
    ///
    /// let cfg = Gen2Cfg::builder().build()?;
    ///
    /// reader.configure_gen2(&cfg)?;
    ///
    /// reader.tune_freqs(TuningAlgorithm::Exact)?;
    ///
    /// let (_statistics, tags) = reader.inventory()?;
    ///
    /// if tags.is_empty() { panic!("No tags found") }
    ///
    /// reader.select_gen2(&tags[0].epc)?;
    ///
    /// let reply = reader.write_gen2(Gen2MemoryBank::User, 0x00, [0x55, 0x55], None)?;
    ///
    /// println!("Tag reply: {}", reply);
    ///
    /// assert_eq!(reader.read_gen2(Gen2MemoryBank::User, 0x00, 2, None)?, [0x55, 0x55]);
    /// # Ok(())
    /// # }
    /// # #[cfg(not(feature = "port_scanning"))]
    /// # fn main() {}
    /// ```
    pub fn write_gen2(&mut self, memory_bank: MemoryBank, word_ptr: u32, data: [u8; 2], password: Option<[u8; 4]>) -> Result<u8> {
        // Make sure protocol is set up first
        if self.protocol.is_none() { return Err(Error::None) };

        let mut write_struct = ffi::STUHFL_T_Write {
            wordPtr: word_ptr,
            memoryBank: memory_bank as u8,
            pwd: if let Some(pwd) = password {
                pwd
            } else {
                [0; 4]
            },
            data,
            tagReply: 0,
        };

        unsafe{proc_err(ffi::Gen2_Write(&mut write_struct))?}

        Ok(write_struct.tagReply)
    }

    /// # Sending Custom & Proprietary Gen2 Commands
    ///
    /// This command allows you to define and send custom Gen2 commands.
    /// This requires first defining a [`Gen2CustomCommand`], then calculating
    /// how many bits must be transmitted and recieved (see below). You can also
    /// optionally send data inside the transmission packet. A password may also
    /// be supplied for authentication with the tag.
    ///
    /// ## Note for calculating packet lengths:
    ///
    /// The length of the sending packet is handled completely automatically. This
    /// value is calculated using the following formula:
    ///
    /// command (16 bits) + data length (optional, variable) + CRC16 (optional) + RN16 (optional)
    ///
    /// The length of the recieved packet already takes into account the header (optional),
    /// CRC16 (optional) and RN16 (optional). It the value given to this function should
    /// simply be the length of the command's *data* fields to be recieved.
    ///
    /// ## Note on command codes:
    ///
    /// While command codes *can* vary in length according to the standard, this
    /// function assumes you are using a 16-bit long command code. This is valid for
    /// any *custom* or *reserved* command according to the Gen2 standard. If you
    /// need a different length, consider using the designated command or calling
    /// the FFI directly.
    ///
    /// ## Returns
    ///
    /// On a successful command run, this command will return the data recieved from
    /// the command. This does not include the header or CRC (if enabled), however it
    /// WILL include the RN16 handle (if enabled). The RN16 can safely be disregarded.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use libstuhfl::*;
    /// # #[cfg(feature = "port_scanning")]
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// // Send Custom Gen2 Command (GetUID for EM4325)
    ///
    /// let mut reader = ST25RU3993::new()?; // new requires port scanning feature
    ///
    /// let cfg = Gen2Cfg::builder().build()?;
    ///
    /// reader.configure_gen2(&cfg)?;
    ///
    /// reader.tune_freqs(TuningAlgorithm::Exact)?;
    ///
    /// let (_statistics, tags) = reader.inventory()?;
    ///
    /// if tags.is_empty() { panic!("No tags found") }
    ///
    /// reader.select_gen2(&tags[0].epc)?;
    ///
    /// let allocation_class = tags[0].tid[0];
    /// println!("Found tag {} with allocation class {:02X}", &tags[0].epc, allocation_class);
    ///
    /// // Create custom command: GetUID for EM4325
    /// let get_uid = Gen2CustomCommand {
    ///     command_code: 0xE000,
    ///     use_crc: true,
    ///     use_rn16: true,
    ///     expect_header: true,
    /// };
    ///
    /// let uid_len = match allocation_class {
    ///     0xE0 => 64,
    ///     0xE3 => 80,
    ///     0xE2 => 96,
    ///     0x44 | 0x45 | 0x46 | 0x47 => 64,
    ///     _ => panic!("unknown allocation class")
    /// };
    ///
    /// let uid = reader.custom_gen2(&get_uid, None, uid_len, None)?;
    /// println!("Tag UID: {:02X?}", &uid[..uid.len() - 2]); // Last 2 bytes are RN16 code
    /// # Ok(())
    /// # }
    /// # #[cfg(not(feature = "port_scanning"))]
    /// # fn main() {}
    /// ```
    pub fn custom_gen2(&mut self, command: &Gen2CustomCommand, data_to_send: Option<Gen2CustomCommandData>, bits_to_recieve: u16, password: Option<[u8; 4]>) -> Result<Vec<u8>> {
        // Make sure protocol is set up first
        if self.protocol.is_none() { return Err(Error::None) };

        // Determine password
        let pwd = password.unwrap_or([0; 4]);

        // Determine command to send
        let cmd = match (command.use_crc, command.expect_header) {
            // transmission with CRC
            (true, false) => ffi::STUHFL_D_GEN2_GENERIC_CMD_CRC as u8,
             // transmission with CRC, expecting header bit
            (true, true)  => ffi::STUHFL_D_GEN2_GENERIC_CMD_CRC_EXPECT_HEAD as u8,
            // transmission without CRC
            (false, _)    => ffi::STUHFL_D_GEN2_GENERIC_CMD_NO_CRC as u8,
        };

        // Generate data to send
        #[allow(non_snake_case)]
        let mut sndData = [0; 64];

        // Copy command code
        let cmd_code = command.command_code.to_be_bytes();
        sndData[0] = cmd_code[0];
        sndData[1] = cmd_code[1];

        // Determine length of data to send
        #[allow(non_snake_case)]
        let mut sndDataBitLength = 16;

        // Copy data to be sent
        if let Some(data) = data_to_send {
            for (i, byte) in data.bytes.iter().enumerate() {
                sndData[i + 2] = *byte;
            }
            sndDataBitLength += data.num_bits;
        }

        // Account for RN16 in response packet
        #[allow(non_snake_case)]
        let expectedRcvDataBitLength = bits_to_recieve + if command.use_rn16 { 16 } else { 0 };

        // Create command parameter struct
        let mut generic_cmd_struct = ffi::STUHFL_T_Gen2_GenericCmd {
            pwd,
            cmd,
            noResponseTime: 0xFF, // 20 ms
            expectedRcvDataBitLength,
            sndDataBitLength,
            appendRN16: command.use_rn16,
            sndData,
            rcvDataLength: 0, // this gets populated by firmware
            rcvData: [0; 128] // this also gets populated by firmware
        };

        // Send command
        unsafe{proc_err(ffi::Gen2_GenericCmd(&mut generic_cmd_struct))?};

        Ok(Vec::from(&generic_cmd_struct.rcvData[..generic_cmd_struct.rcvDataLength as usize]))
    }
}

/// Wrapper for user specified callback funciton. This catches any unwind panics, and
/// processes the inventory data from FFI form into Rust form.
///
/// # Panics
///
/// Any panics will be caught by the `catch_unwind`, then turned into an error.
/// However, doing so **will** poison the Mutex.
extern "C" fn cycle_cb(data: *mut ffi::STUHFL_T_InventoryData) -> ffi::STUHFL_T_RET_CODE {
    let cb_wrapper = std::panic::catch_unwind(|| {
        // Get user defined callback function
        let cb_holder = CB_HOLDER.lock().unwrap();

        // Access callback function
        let cb_fn = cb_holder.as_ref().unwrap();

        // Access data from behind pointer
        let data = unsafe{&*data};

        // Copy every scanned tag into the vector
        for i in 0..data.tagListSize {
            // Index pointer to array and convert it to InventoryTag
            let tag = InventoryTag::from(unsafe{*data.tagList.offset(i as isize)});
            // Let caller handle values
            cb_fn(tag);
        }
    });

    if cb_wrapper.is_err() {
        // callback unwrapped, mutex now poisoned
        unsafe{ffi::Inventory_RunnerStop()};
        Error::Generic as ffi::STUHFL_T_RET_CODE
    } else {
        // callback finished
        Error::None as ffi::STUHFL_T_RET_CODE
    }
}

/// # Disconnect from reader
///
/// This is handled automatically by the Drop implementation. If the reader fails
/// to disconnect for some reason, a warning message will be printed.
impl Drop for ST25RU3993 {
    fn drop(&mut self) {
        unsafe {
            // close the connection to the reader
            if proc_err(ffi::Disconnect()).is_err() {
                eprintln!("ERROR: Couldn't disconnect from reader during call to Drop()")
            };
        }
    }
}
*/

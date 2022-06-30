use super::*;

use std::sync::{Arc, Mutex};

#[allow(dead_code)]
enum Protocol {
    Gen2,
    Gb29768,
    Iso6b
}

/// Represents an ST25RU3993 Reader.
/// ```no_run
/// use libstuhfl::{ST25RU3993, Version};
/// 
/// // with port scanning
/// # #[cfg(feature = "port_scanning")]
/// let mut reader = ST25RU3993::new().expect("Couldn't connect to reader");
/// 
/// // without port scanning
/// # #[cfg(not(feature = "port_scanning"))]
/// let mut reader = ST25RU3993::from_port("/dev/ttyUSB0").expect("Couldn't connect to reader");
/// 
/// let version = reader.get_board_version().expect("Failed to get board version");
/// 
/// println!("Reader version: {}", &version);
/// ```
pub struct ST25RU3993 {
    protocol: Option<Protocol>,
}

#[cfg(feature = "port_scanning")]
extern crate serialport as sp;

impl ST25RU3993 {
    /// Create a new ST25RU3993 RFID reader by automatically scanning ports
    #[cfg(feature = "port_scanning")]
    pub fn new() -> Result<Self, Error> {
        let mut found_port: Option<String> = None;
    
        if let Ok(ports) = sp::available_ports() {
            for port in ports {
                if let sp::SerialPortType::UsbPort(port_info) = port.port_type {
                    if port_info.vid == 0x403 && port_info.pid == 0x6015 {
                        sp::new(&port.port_name, 9600).open().expect("Couldn't open port!");
                        found_port = Some(port.port_name)
                    }
                }
            }
        }
        
        let found_port = found_port.expect("Reader not found on any ports");

        // hand over creation to normal constructor
        ST25RU3993::from_port(&found_port)
    }

    /// Create anew ST25RU3993 RFID reader using a path to a specific port
    pub fn from_port(port: &str) -> Result<Self, Error> {
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

    /// Queries software & hardware information from the reader
    pub fn get_board_version(&self) -> Result<Version, Error> {
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
    /// (This is called automatically by the constructor)
    fn test_compatible(&self) -> Result<bool, Error> {
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

    /// Tune the reader using the specified tuning algorithm. Note: must be called
    /// AFTER configuring a protocol (requires knoweldge of active antenna).
    pub fn tune_freqs(&mut self, algo: TuningAlgorithm) -> Result<(), Error> {
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

    /// Configures the reader for use of the Gen2 protocol
    pub fn configure_gen2(&mut self, gen2_cfg: &Gen2Cfg) -> Result<(), Error> {
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

    /// Inventories tags for the selected protocol (1 Round). 
    /// Note: be sure to configure a protocol & tune the reader first!
    pub fn inventory(&mut self) -> Result<(InventoryStatistics, Vec<InventoryTag>), Error> {
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

    /// Inventories tags for the selected protocol (N-Rounds, blocking call).
    /// Note: be sure to configure a protocol & tune the reader first!
    pub fn inventory_runner(&mut self, num_rounds: u32, data_cb: Box<CallbackFn>) -> Result<InventoryStatistics, Error> {
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
        unsafe{proc_err(ffi::Inventory_RunnerStart(&mut inv_option, Some(cycle_cb), None, &mut inv_data))?}

        // Delete callback function
        let mut cb_holder = CB_HOLDER.lock().unwrap();
        *cb_holder = None;

        let statistics = InventoryStatistics::from(inv_data.statistics);

        Ok(statistics)
    }

    /// Selects a tag using its EPC number
    ///```no_run
    /// // connect to reader
    /// let mut reader = ST25RU3993::new()?;
    /// 
    /// // set gen2 configuration
    /// let gen2_cfg = Gen2Cfg::builder()
    ///     .build()?;
    /// 
    /// // apply the settings
    /// reader.configure_gen2(&gen2_cfg)?;
    /// 
    /// // tune reader
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
    /// // run inventory with 100 rounds
    /// let statitistics = reader.inventory_runner(100, Box::new(callback))?;
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
    /// ```
    pub fn select_gen2(&mut self, epc: &Epc) -> Result<(), Error> {

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

    /// Reads data from a selected tag.
    /// Make sure to call select_gen2 first!
    pub fn read_gen2(&mut self, memory_bank: Gen2MemoryBank, word_ptr: u32, num_bytes: u8, password: Option<[u8; 4]>) -> Result<Vec<u8>, Error> {
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
}

lazy_static! {
    // Note: In rust 1.63 this will no longer require the lazy_static crate.
    static ref CB_HOLDER: Mutex<Option<Box<CallbackFn>>> = Mutex::new(None);
}

extern "C" fn cycle_cb(data: *mut ffi::STUHFL_T_InventoryData) -> ffi::STUHFL_T_RET_CODE {
    if std::panic::catch_unwind(|| {
        // Get user defined callback function
        let cb_holder = CB_HOLDER.lock().unwrap();
        let cb_fn = cb_holder.as_ref().unwrap();

        // Access data from behind pointer
        let data = unsafe{&*data};

        // Copy every scanned tag into the vector
        for i in 0..data.tagListSize {
            // Index pointer to array and convert it to InventoryTag
            let tag = InventoryTag::from(unsafe{*data.tagList.offset(i as isize)});
            // Let caller handle values
            cb_fn(InventoryTag::from(tag));
        }
    }).is_err() {
        // callback unwrapped
        Error::Generic as ffi::STUHFL_T_RET_CODE
    } else {
        // callback finished
        Error::None as ffi::STUHFL_T_RET_CODE
    }
}

/// Automatically handles disconnecting from the reader.
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

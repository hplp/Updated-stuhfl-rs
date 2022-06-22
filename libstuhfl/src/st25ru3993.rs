use super::*;

#[allow(dead_code)]
enum Protocol {
    Gen2,
    Gb29768,
    Iso6b
}

mod gen2;

/// Represents an ST25RU3993 Reader.
/// ```no_run
/// use libstuhfl::{ST25RU3993, Version};
/// 
/// let mut reader = ST25RU3993::new().expect("Couldn't connect to reader");
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

        dbg!("Found port: {}", &found_port);

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

    /// Private function to run the specified tuning algorithm on the reader
    fn tune_freqs(&mut self, algo: TuningAlgorithm) -> Result<(), Error> {
        if algo == TuningAlgorithm::None {
            return Ok(())
        }

        unsafe {
            let mut tx_rx_cfg = ffi::STUHFL_T_ST25RU3993_TxRxCfg::default();

            proc_err(ffi::Get_TxRxCfg(&mut tx_rx_cfg))?;

            let mut tune_cfg = ffi::STUHFL_T_ST25RU3993_TuneCfg{
                antenna: tx_rx_cfg.usedAntenna,
                algorithm: algo as u8,
                tuneAll: true,
                ..Default::default()
            };
            
            proc_err(ffi::TuneChannel(&mut tune_cfg))?;
        }

        Ok(())
    }

    /// Recreates the setupGen2Config() command found in the STUHFL demo program
    #[warn(deprecated)]
    pub fn setup_gen2_config(&mut self, single_tag: bool, freq_hopping: bool, antenna: Antenna) -> Result<(), Error> {
        gen2::setup_gen2_config(self, single_tag, freq_hopping, antenna)?;

        // Reader successfully set up gen2 configuration
        self.protocol = Some(Protocol::Gen2);
        Ok(())
    }

    /// Configures the reader for using the Gen2 protocol
    pub fn configure_gen2(&mut self, gen2_cfg: &Gen2Cfg) -> Result<(), Error> {
        // Set up antenna configuration
        let mut tx_rx_cfg = gen2_cfg.tx_rx_cfg.as_ffi();
        unsafe {proc_err(ffi::Set_TxRxCfg(&mut tx_rx_cfg))?}

        // Set up inventory configuration
        let mut inv_cfg = gen2_cfg.inv_cfg.as_ffi();
        unsafe {proc_err(ffi::Set_Gen2_InventoryCfg(&mut inv_cfg))?};

        // Set up protocol configuration
        let mut proto_cfg = gen2_cfg.proto_cfg.as_ffi();
        unsafe {proc_err(ffi::Set_Gen2_ProtocolCfg(&mut proto_cfg))?};

        // Set up lbt configuraiton
        let mut lbt = gen2_cfg.lbt.as_ffi();
        unsafe {proc_err(ffi::Set_FreqLBT(&mut lbt))?};

        // Set up channel list configuration
        // TODO

        // Set up frequency hopping configuration
        // TODO

        // Set up select configuraiton
        // TODO

        // Enable Gen2 protocol commands for reader
        self.protocol = Some(Protocol::Gen2);
        Ok(())
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

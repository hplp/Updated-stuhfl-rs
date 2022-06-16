use super::*;

enum Protocol {
    Gen2,
    Gb29768,
    Iso6b
}

mod gen2;

pub struct ST25RU3993 {
    protocol: Option<Protocol>,
}

impl ST25RU3993 {
    pub fn new(port: &str) -> Result<Self, Error> {
        unsafe {
            // Copy the port so that its "safe" from C
            let mut port = port.to_owned();

            // Connect to board
            proc_err(ffi::Connect(port.as_mut_ptr() as _))?;
            
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
                sw_ver: sw_ver,
                hw_ver: hw_ver,
                sw_info: VersionInfo {info: sw_info},
                hw_info: VersionInfo {info: hw_info},
            })
        }
    }

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

    fn tune_freqs(&mut self, algo: TuningAlgorithm) -> Result<(), Error> {
        if algo == TuningAlgorithm::None {
            return Ok(())
        }

        unsafe {
            let mut tx_rx_cfg = ffi::STUHFL_T_ST25RU3993_TxRxCfg::default();

            proc_err(ffi::Get_TxRxCfg(&mut tx_rx_cfg))?;

            let mut tune_cfg = ffi::STUHFL_T_ST25RU3993_TuneCfg::default();
            tune_cfg.antenna = tx_rx_cfg.usedAntenna;
            tune_cfg.algorithm = algo as u8;
            tune_cfg.tuneAll = true;
            
            proc_err(ffi::TuneChannel(&mut tune_cfg))?;
        }

        Ok(())
    }

    pub fn setup_gen2_config(&mut self, single_tag: bool, freq_hopping: bool, antenna: Antenna) -> Result<(), Error> {
        gen2::setup_gen2_config(self, single_tag, freq_hopping, antenna)?;

        // Reader successfully set up gen2 configuration
        self.protocol = Some(Protocol::Gen2);
        Ok(())
    }
}

impl Drop for ST25RU3993 {
    fn drop(&mut self) {
        unsafe {
            // close the connection to the reader
            proc_err(ffi::Disconnect()).unwrap();
        }
    }
}

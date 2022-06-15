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
            // copy the port so that its safe from C
            let mut port = String::from(port);
            match ffi::Connect(port.as_mut_ptr() as *mut i8) {
                ffi::STUHFL_ERR_NONE => {
                    // Wait so that board has time to connect
                    std::thread::sleep(std::time::Duration::from_micros(600000));
                    let board = ST25RU3993 {
                        protocol: None,
                    };
                    match board.test_compatible() {
                        Ok(true) => Ok(board),
                        Ok(false) => {
                            eprintln!("Warning: Incompatible Board or Library Version Detected. Please verify that your FW is up to date.");
                            Err(Error::None)
                        },
                        Err(e) => Err(e)
                    }
                },
                x => Err(Error::from_u32(x).unwrap())
            }
        }
    }

    pub fn get_board_version(&self) -> Result<Version, Error> {
        let mut ret;

        unsafe {
            // create structs to be filled by function
            let mut sw_ver: ffi::STUHFL_T_Version = mem::zeroed();
            let mut hw_ver: ffi::STUHFL_T_Version = mem::zeroed();
            let mut sw_info: ffi::STUHFL_T_VersionInfo = mem::zeroed();
            let mut hw_info: ffi::STUHFL_T_VersionInfo = mem::zeroed();
            // attempt to get board version
            ret = ffi::Get_BoardVersion(&mut sw_ver, &mut hw_ver);
            if ret != ffi::STUHFL_ERR_NONE {return Err(Error::from_u32(ret).unwrap())};

            ret = ffi::Get_BoardInfo(&mut sw_info, &mut hw_info);
            if ret != ffi::STUHFL_ERR_NONE {return Err(Error::from_u32(ret).unwrap())};

            // move structs to safe memory
            let sw_ver = VersionNum::from(sw_ver);
            let hw_ver = VersionNum::from(hw_ver);
            // recreate info as a Rust safe string
            let sw_info: String = std::ffi::CStr::from_ptr(&sw_info.info as *const _).to_string_lossy().to_string();
            let hw_info: String = std::ffi::CStr::from_ptr(&hw_info.info as *const _).to_string_lossy().to_string();
            
            // return safe version
            Ok(Version {
                sw_ver: sw_ver,
                hw_ver: hw_ver,
                sw_info: VersionInfo {info: sw_info},
                hw_info: VersionInfo {info: hw_info},
            })
        }
    }

    pub fn test_compatible(&self) -> Result<bool, Error> {
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

        match self.get_board_version() {
            Ok(ver) => Ok(ver.sw_ver >= LOWEST_SW_VER && ver.hw_ver >= LOWEST_HW_VER),
            Err(e) => Err(e)
        }
    }

    fn tune_freqs(&mut self, algo: TuningAlgorithm) -> Result<(), Error> {
        if algo == TuningAlgorithm::None {
            return Ok(())
        }

        unsafe {
            let mut ret;

            let mut tx_rx_cfg = ffi::STUHFL_T_ST25RU3993_TxRxCfg::default();

            ret = ffi::Get_TxRxCfg(&mut tx_rx_cfg);
            if ret != ffi::STUHFL_ERR_NONE {return Err(Error::from_u32(ret).unwrap())};

            let mut tune_cfg = ffi::STUHFL_T_ST25RU3993_TuneCfg::default();
            tune_cfg.antenna = tx_rx_cfg.usedAntenna;
            tune_cfg.algorithm = algo as u8;
            tune_cfg.tuneAll = true;
            
            ret = ffi::TuneChannel(&mut tune_cfg);
            if ret != ffi::STUHFL_ERR_NONE {return Err(Error::from_u32(ret).unwrap())};
        }

        Ok(())
    }

    pub fn setup_gen2_config(&mut self, single_tag: bool, freq_hopping: bool, antenna: Antenna) -> Result<(), Error> {
        match gen2::setup_gen2_config(self, single_tag, freq_hopping, antenna) {
            Ok(_) => {}
            Err(e) => return Err(e)
        }

        // Reader successfully set up gen2 configuration
        self.protocol = Some(Protocol::Gen2);
        Ok(())
    }
}

impl Drop for ST25RU3993 {
    fn drop(&mut self) {
        unsafe {
            // close the connection to the reader
            match ffi::Disconnect() {
                ffi::STUHFL_ERR_NONE => {},
                x => {
                    eprintln!("Error while disconnecting from reader: {}", Error::from_u32(x).unwrap())
                }
            }
        }
    }
}

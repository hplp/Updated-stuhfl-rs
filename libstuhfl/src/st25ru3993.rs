use super::*;

pub struct ST25RU3993 {
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
                    Ok(ST25RU3993 {})
                },
                x => Err(Error::from_u32(x).unwrap())
            }
        }
    }

    pub fn get_board_version(&self) -> Result<Version, Error> {
        unsafe {
            // create structs to be filled by function
            let mut sw_ver: ffi::STUHFL_T_Version = mem::zeroed();
            let mut hw_ver: ffi::STUHFL_T_Version = mem::zeroed();
            let mut sw_info: ffi::STUHFL_T_VersionInfo = mem::zeroed();
            let mut hw_info: ffi::STUHFL_T_VersionInfo = mem::zeroed();
            // attempt to get board version
            match ffi::Get_BoardVersion(&mut sw_ver, &mut hw_ver) {
                ffi::STUHFL_ERR_NONE => {
                    // attempt to get board info
                    match ffi::Get_BoardInfo(&mut sw_info, &mut hw_info) {
                        ffi::STUHFL_ERR_NONE => {
                            // move structs to safe memory
                            let sw_ver = VersionNum {
                                major: sw_ver.major,
                                minor: sw_ver.minor,
                                micro: sw_ver.micro,
                                nano: sw_ver.nano,
                            };
                            let hw_ver = VersionNum {
                                major: hw_ver.major,
                                minor: hw_ver.minor,
                                micro: hw_ver.micro,
                                nano: hw_ver.nano,
                            };
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
                        },
                        x => Err(Error::from_u32(x).unwrap())
                    }
                },
                x => Err(Error::from_u32(x).unwrap())
            }
        }
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

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::derivable_impls)]

#[cfg(test)]
#[cfg(feature = "reader_tests")]
extern crate serialport;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod initializers;
pub use initializers::*;

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "reader_tests")]
    fn connect_to_reader() {
        use super::*;
        use serialport as sp;
        use std::ffi::{CStr, CString};
        use std::mem;

        let mut found_port: Option<String> = None;

        if let Ok(ports) = sp::available_ports() {
            for port in ports {
                if let sp::SerialPortType::UsbPort(port_info) = port.port_type {
                    if port_info.vid == 0x403 && port_info.pid == 0x6015 {
                        sp::new(&port.port_name, 9600)
                            .open()
                            .expect("Couldn't open port!");
                        found_port = Some(port.port_name)
                    }
                }
            }
        }

        let found_port = found_port.expect("Reader not found on any ports");

        let port = CString::new(found_port).expect("Couldn't create string");
        let ptr = port.into_raw();

        unsafe {
            let mut ret = Connect(ptr);
            let _ = CString::from_raw(ptr);

            std::thread::sleep(std::time::Duration::from_micros(600000));

            let mut swVer: STUHFL_T_Version = mem::zeroed();
            let mut hwVer: STUHFL_T_Version = mem::zeroed();

            let mut swInfo: STUHFL_T_VersionInfo = mem::zeroed();
            let mut hwInfo: STUHFL_T_VersionInfo = mem::zeroed();

            ret |= Get_BoardVersion(&mut swVer, &mut hwVer);
            ret |= Get_BoardInfo(&mut swInfo, &mut hwInfo);

            print!("\n-------------------------------------------------------\nSW: V{}.{}.{}.{}, {}\nHW: V{}.{}.{}.{}, {}\n-------------------------------------------------------\n\n",
            swVer.major, swVer.minor, swVer.micro, swVer.nano, CStr::from_ptr(&swInfo.info as *const _).to_string_lossy(),
            hwVer.major, hwVer.minor, hwVer.micro, hwVer.nano, CStr::from_ptr(&hwInfo.info as *const _).to_string_lossy());

            ret |= Disconnect();

            match ret {
                r if r == (STUHFL_ERR_NONE) => {}
                r => panic!("Unknown Error: {}", r as i32),
            }
        }
    }
}

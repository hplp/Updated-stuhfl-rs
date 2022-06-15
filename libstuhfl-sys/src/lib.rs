#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

//mod bindings;
//use bindings::*;

// Tests that actually use the reader

mod initializers;
pub use initializers::*;

#[cfg(unix)]
#[cfg(test)]
mod tests {
    use std::mem;
    use std::ffi::*;
    use super::*;

    #[test]
    fn connect_to_reader() {
        let port = CString::new("/dev/ttyUSB0").expect("Couldn't create string");
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
                r if r == (STUHFL_ERR_NONE) => {},
                r => panic!("Unknown Error: {}", r as i32)
            }
        }
    }
}
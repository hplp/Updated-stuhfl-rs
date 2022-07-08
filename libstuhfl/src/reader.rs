//! Module containing an RFID reader which
//! can be instantiated by connecting to it.

use crate::data_types::*;
use crate::error::{Error, Result};
use crate::helpers::proc_err;

#[cfg(feature = "port_scanning")]
use serialport as sp;

/// Main reader struct. See [`BasicReader`] for more usage.
pub struct Reader;

unsafe impl BasicReader for Reader {}

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
    /// use libstuhfl::prelude::*;
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
    /// use libstuhfl::prelude::*;
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
}

/// A [`ProtocolReader`] implementation used for test purposes.
/// All of its functions do nothing and should always succeed.
pub struct DummyReader();

unsafe impl BasicReader for DummyReader {
    fn get_version(&self) -> Result<Version> {
        Ok(Version {
            sw_ver: VersionNum {
                major: 3,
                minor: 1,
                micro: 0,
                nano: 0,
            },
            sw_info: VersionInfo {
                info: "STUHFL SDK Evaluation FW @ STM32L4x6".to_owned(),
            },
            hw_ver: VersionNum {
                major: 1,
                minor: 1,
                micro: 0,
                nano: 0,
            },
            hw_info: VersionInfo {
                info: "ST25RU3993-EVAL Board.".to_owned(),
            },
        })
    }

    fn configure_gen2(
        self,
        _configuration: &crate::gen2::Gen2Cfg,
    ) -> Result<crate::gen2::Gen2Reader> {
        Ok(unsafe { crate::gen2::Gen2Reader::new() })
    }

    fn test_compatible(&self) -> Result<bool> {
        Ok(true)
    }
}

unsafe impl ProtocolReader for DummyReader {
    fn tune(&mut self, _algo: TuningAlgorithm) -> Result<()> {
        Ok(())
    }

    fn inventory_once(&self) -> Result<(InventoryStatistics, Vec<InventoryTag>)> {
        Ok((
            InventoryStatistics::new(),
            vec![InventoryTag {
                slot_id: 0,
                timestamp: 0,
                antenna: Antenna::Antenna1,
                agc: 0,
                rssi_lin_i: 0,
                rssi_lin_q: 0,
                rssi_log_i: 0,
                rssi_log_q: 0,
                pc: [0, 0],
                xpc: Xpc::from_id(Vec::new()),
                epc: Epc::from_id(Vec::new()),
                tid: Tid::from_id(Vec::new()),
            }],
        ))
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

impl DummyReader {
    /// Constructs a new [`DummyReader`].
    ///
    /// # Safety
    ///
    /// This struct is for testing purposes only. Certain
    /// trait implementations may or may not be safe to
    /// use. For example, [`ProtocolReader::configure_gen`]
    /// will create a corrupt instance of [`Gen2Reader`].
    pub unsafe fn new() -> Self {
        Self {}
    }
}

/*
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

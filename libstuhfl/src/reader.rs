//! Module containing an RFID reader which
//! can be instantiated by connecting to it.

use crate::data_types::*;
use crate::error::{Error, Result};

#[cfg(feature = "port-scanning")]
use serialport as sp;

/// Main reader struct. See [`BasicReader`] for more usage.
pub struct Reader {
    /// Holds connection
    connection: Connection,
}

impl ConnectionHolder for Reader {
    fn steal_connection(self) -> Connection {
        self.connection
    }
}

unsafe impl BasicReader for Reader {}

impl Reader {
    /// # Connecting to reader
    ///
    /// Using [`Self::autoconnect()`] requires the `port-scanning` feature. This method scans
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
    /// #[cfg(feature = "port-scanning")]
    /// let reader = Reader::autoconnect()?;
    ///
    /// // manually without port scanning
    /// #[cfg(not(feature = "port-scanning"))]
    /// let reader = Reader::connect("/dev/ttyUSB0")?;
    ///
    /// # Ok(())}
    /// ```
    ///
    /// # Errors
    ///
    /// This function errors if the reader cannot be safely connected to. A [`Error::GeneralIo`]
    /// error may be issued if no valid ports can be found/opened. See [`Reader::connect()`] for
    /// more info.
    #[cfg(feature = "port-scanning")]
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
    /// Using [`Reader::connect()`] will attempt to connect to the reader using the
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
        // Establish connection
        let connection = Connection::new(port)?;

        // Construct new instance
        let board = Self { connection };

        // Test compatibility
        if board.test_compatible()? {
            Ok(board)
        } else {
            eprintln!("Warning: Incompatible Board or Library Version Detected. Please verify that your FW is up to date.");
            Err(Error::None)
        }
    }

    /// Used for constructing new instances during conversion between
    /// reader types.
    pub(crate) fn new(connection: Connection) -> Self {
        Self { connection }
    }
}

/// A fake struct that behaves similarly to a [`ProtocolReader`],
/// except none of its functions actually do anything. This is
/// mostly used to help write documentation.
pub struct DummyReader;

impl DummyReader {
    /// Always returns the same version, see source.
    pub fn get_version(&self) -> Result<Version> {
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

    /// Always returns [`Ok(true)`].
    pub fn test_compatible(&self) -> Result<bool> {
        Ok(true)
    }

    /// Always returns [`Ok(())`]
    pub fn tune(&mut self, _algo: TuningAlgorithm) -> Result<()> {
        Ok(())
    }

    /// Always returns an empty [`InventoryStatistics`] and a single tag (with empty values)
    pub fn inventory_once(&self) -> Result<(InventoryStatistics, Vec<InventoryTag>)> {
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

    /// Always returns an empty [`InventoryStatistics`].
    pub fn inventory(
        &mut self,
        _num_rounds: u32,
        _cb: Box<CallbackFn>,
    ) -> Result<InventoryStatistics> {
        Ok(InventoryStatistics::new())
    }

    /// Always returns [`Ok(())`]
    pub fn select(&mut self, _epc: &Epc) -> Result<()> {
        Ok(())
    }

    /// Always returns an empty vector
    pub fn read(
        &mut self,
        _bank: MemoryBank,
        _word_address: u32,
        _num_bytes: u8,
        _password: Option<Password>,
    ) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }

    /// Always returns [`Ok(())`]
    pub fn write(
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
    /// This struct is for testing purposes only.
    /// It looks like a reader in documentation tests
    /// but does nothing.
    pub unsafe fn new() -> Self {
        Self {}
    }
}

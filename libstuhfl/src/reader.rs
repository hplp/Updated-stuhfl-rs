//! Module containing an RFID reader which
//! can be instantiated by connecting to it.

use crate::data_types::*;
use crate::error::{Error, Result};

#[cfg(feature = "port-scanning")]
use serialport as sp;

// CB 7/14/25: Imported for 'parse()' to use
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// CB 7/7/25: Added text parse function. This function is used to convert the 50-line text
//            file 'GUI_Tuning_Results.txt' into an array of tuples. This array holds the
//            capacitor values for Antenna 1 of the reader. The way the file is formatted is very specific.
//            The text in the file comes from a single line of the ST GUI tuning result.
//            There must be 50 lines in total, and each line starting with the number that labels the set of capacitors.
//            There are 2 sets of capacitor values that correspond to the specific frequency (in MHz).
//            The first set is for Antenna 1, the second for Antenna 2. If Antenna 2 is not being used,
//            then the second set can be deleted, but it makes no difference either way.
//
//            File must be named 'GUI_Tuning_Results.txt' and be in the 'libstuhfl' folder

/* GUI_Tuning_Results.txt Example
0:{902750, (24,17,1), (9,19,15)}, 
1:{915250, (25,17,0), (9,25,15)}, 
2:{903250, (27,20,0), (9,19,15)}, 
3:{915750, (25,17,0), (9,25,15)}, 
4:{903750, (23,17,1), (9,19,15)}, 
5:{916250, (11,7,2), (9,14,14)}, 
6:{904250, (27,20,0), (9,19,15)}, 
7:{916750, (25,17,0), (9,14,14)}, 
8:{904750, (27,20,0), (9,19,15)}, 
9:{917250, (9,6,1), (9,14,14)}, 
.
.
.
48:{914750, (25,18,0), (9,25,15)}, 
49:{927250, (11,7,0), (9,30,15)},
*/

// CB 7/14/25: This function parses a text file to return an array
//             of 50 tuples containing capacitor values for the reader

pub(crate) fn parse() -> io::Result<[(u8, u8, u8); 50]> {
    let mut a: String = String::from("");
    let mut b: String = String::from("");
    let mut c: String = String::from("");

    let mut tracker: u8 = 0;
    let mut counter = 0;
    let mut caps: [(u8, u8, u8); 50] = [(0, 0, 0); 50];

    let file: File = File::open("GUI_Tuning_Results.txt")?;
    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        let line: String = line?;

        for _char in line.chars() {
            if tracker == 1 && _char != ')' && _char != ',' {
                a.push(_char);
            } else if tracker == 2 && _char != ')' && _char != ',' {
                b.push(_char);
            } else if tracker == 3 && _char != ')' && _char != ',' {
                c.push(_char);
            }

            if _char == '(' {
                tracker += 1;
            } else if _char == ',' && tracker > 0 {
                tracker += 1;
            }

            if tracker > 3 {
                break;
            }
        }

        caps[counter].0 = a.trim().parse::<u8>().unwrap();
        caps[counter].1 = b.trim().parse::<u8>().unwrap();
        caps[counter].2 = c.trim().parse::<u8>().unwrap();
        tracker = 0;
        a = String::from("");
        b = String::from("");
        c = String::from("");
        counter += 1;
    }

    Ok(caps)
    //caps
}

//

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

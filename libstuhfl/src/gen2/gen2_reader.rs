use crate::data_types::*;
use crate::error::{Error, Result};
use crate::gen2::*;
use crate::helpers::{ebv_formatter, proc_err};
use std::sync::Mutex;

/// A reader compatible with the Gen2 standard.
/// To instantiate this struct, see [`BasicReader::configure_gen2`].
pub struct Gen2Reader {
    /// keeps track of whether or not the reader is tuned
    is_tuned: bool,
    /// manages connection to reader
    connection: Connection,
}

impl Gen2Reader {
    /// Creates an instance of self, must be private to
    /// ensure that this doesn't 'leak' out to the end
    /// user. Otherwise the state might not be valid.
    pub(crate) fn new(connection: Connection) -> Self {
        Self {
            is_tuned: false,
            connection,
        }
    }

    /// Workaround for firmware issues with read command.
    /// Uses custom command in background.
    pub fn read_alt(
        &mut self,
        bank: MemoryBank,
        word_address: u32,
        word_count: u8,
        password: Option<Password>,
    ) -> Result<Vec<u8>> {
        let pwd = password
            .unwrap_or_else(|| Password::from([0; 4]))
            .into_inner();

        let ebv = ebv_formatter(word_address);

        let mut data = [0; 64];

        data[0] = 0b1100_0010;
        data[1] = (bank as u8) << 6;

        let ebv_len = ebv.len();

        for (i, byte) in ebv.iter().enumerate() {
            data[i + 1] |= byte >> 2;
            data[i + 2] |= byte << 6;
        }

        data[ebv_len + 1] |= word_count >> 2;
        data[ebv_len + 2] |= word_count << 6;

        let snd_len = 18 + 8 * ebv_len as u16;
        let rcv_len = 16 * (word_count as u16) + 16; // account for rn16

        let mut cmd = ffi::STUHFL_T_Gen2_GenericCmd {
            cmd: ffi::STUHFL_D_GEN2_GENERIC_CMD_CRC_EXPECT_HEAD as u8,
            pwd,
            noResponseTime: 0xFF,
            expectedRcvDataBitLength: rcv_len,
            sndDataBitLength: snd_len,
            appendRN16: true,
            sndData: data,
            rcvDataLength: 0,
            rcvData: [0; 128],
        };

        unsafe { proc_err(ffi::Gen2_GenericCmd(&mut cmd))? }

        // discard the last 2 bytes (RN16)
        Ok(Vec::from(&cmd.rcvData[..(cmd.rcvDataLength - 2) as usize]))
    }

    /// # Sending Custom & Proprietary Gen2 Commands
    ///
    /// This command allows you to define and send custom Gen2 commands.
    /// This requires first defining a [`Gen2CustomCommand`], then calculating
    /// how many bits must be transmitted and recieved (see below). You can also
    /// optionally send data inside the transmission packet. A password may also
    /// be supplied for authentication with the tag.
    ///
    /// ## Note for calculating packet lengths:
    ///
    /// The length of the sending packet is handled completely automatically. This
    /// value is calculated using the following formula:
    ///
    /// command (16 bits) + data length (optional, variable) + CRC16 (optional) + RN16 (optional)
    ///
    /// The length of the recieved packet already takes into account the header (optional),
    /// CRC16 (optional) and RN16 (optional). It the value given to this function should
    /// simply be the length of the command's *data* fields to be recieved.
    ///
    /// ## Note on command codes:
    ///
    /// While command codes *can* vary in length according to the standard, this
    /// function assumes you are using a 16-bit long command code. This is valid for
    /// any *custom* or *reserved* command according to the Gen2 standard. If you
    /// need a different length, consider using the designated command or calling
    /// the FFI directly.
    ///
    /// ## Returns
    ///
    /// On a successful command run, this command will return the data recieved from
    /// the command. This does not include the header or CRC (if enabled), however it
    /// WILL include the RN16 handle (if enabled). The RN16 can safely be disregarded.
    ///
    /// # Example
    /// ```no_run
    /// use libstuhfl::prelude::*;
    /// use libstuhfl::gen2::*;
    /// # fn main() -> core::result::Result<(), Box<dyn std::error::Error>> {
    ///
    /// let mut reader = Reader::autoconnect()?;
    ///
    /// let gen2_cfg = Gen2Cfg::builder()
    ///     .build()?;
    ///
    /// let mut reader = reader.configure_gen2(&gen2_cfg)?;
    ///
    /// reader.tune(TuningAlgorithm::Exact)?;
    ///
    /// let (_stats, tags) = reader.inventory_once()?;
    ///
    /// if tags.is_empty() { panic!("No tags found") }
    ///
    /// reader.select(&tags[0].epc)?;
    ///
    /// let allocation_class = tags[0].tid[0];
    /// println!("Found tag {} with allocation class {:02X}", &tags[0].epc, allocation_class);
    ///
    /// // Create custom command: GetUID for EM4325
    /// let get_uid = Gen2CustomCommand {
    ///     command_code: 0xE000,
    ///     use_crc: true,
    ///     use_rn16: true,
    ///     expect_header: true,
    /// };
    ///
    /// let uid_len = match allocation_class {
    ///     0xE0 => 64,
    ///     0xE3 => 80,
    ///     0xE2 => 96,
    ///     0x44 | 0x45 | 0x46 | 0x47 => 64,
    ///     _ => panic!("unknown allocation class")
    /// };
    ///
    /// let uid = reader.custom_cmd(&get_uid, None, uid_len, None)?;
    /// println!("Tag UID: {:02X?}", &uid[..uid.len() - 2]); // Last 2 bytes are RN16 code
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn custom_cmd(
        &mut self,
        command: &Gen2CustomCommand,
        data_to_send: Option<Gen2CustomCommandData>,
        bits_to_recieve: u16,
        password: Option<Password>,
    ) -> Result<Vec<u8>> {
        // Determine password
        let pwd = password
            .unwrap_or_else(|| Password::from([0; 4]))
            .into_inner();

        // Determine command to send
        let cmd = match (command.use_crc, command.expect_header) {
            // transmission with CRC
            (true, false) => ffi::STUHFL_D_GEN2_GENERIC_CMD_CRC as u8,
            // transmission with CRC, expecting header bit
            (true, true) => ffi::STUHFL_D_GEN2_GENERIC_CMD_CRC_EXPECT_HEAD as u8,
            // transmission without CRC
            (false, _) => ffi::STUHFL_D_GEN2_GENERIC_CMD_NO_CRC as u8,
        };

        // Generate data to send
        #[allow(non_snake_case)]
        let mut sndData = [0; 64];

        // Copy command code
        let cmd_code = command.command_code.to_be_bytes();
        sndData[0] = cmd_code[0];
        sndData[1] = cmd_code[1];

        // Determine length of data to send
        #[allow(non_snake_case)]
        let mut sndDataBitLength = 16;

        // Copy data to be sent
        if let Some(data) = data_to_send {
            for (i, byte) in data.bytes.iter().enumerate() {
                sndData[i + 2] = *byte;
            }
            sndDataBitLength += data.num_bits;
        }

        // Account for RN16 in response packet
        #[allow(non_snake_case)]
        let expectedRcvDataBitLength = bits_to_recieve + if command.use_rn16 { 16 } else { 0 };

        // Create command parameter struct
        let mut generic_cmd_struct = ffi::STUHFL_T_Gen2_GenericCmd {
            pwd,
            cmd,
            noResponseTime: 0xFF, // 20 ms
            expectedRcvDataBitLength,
            sndDataBitLength,
            appendRN16: command.use_rn16,
            sndData,
            rcvDataLength: 0,  // this gets populated by firmware
            rcvData: [0; 128], // this also gets populated by firmware
        };

        // Send command
        unsafe { proc_err(ffi::Gen2_GenericCmd(&mut generic_cmd_struct))? };

        Ok(Vec::from(
            &generic_cmd_struct.rcvData[..generic_cmd_struct.rcvDataLength as usize],
        ))
    }
}

lazy_static! {
    /// CB_HOLDER contains a reference to a user-specified callback function
    /// used for multithreaded synchronous inventory_runner execution

    // Note: In rust 1.63 this will no longer require the lazy_static crate.
    static ref CB_HOLDER: Mutex<Option<Box<CallbackFn>>> = Mutex::new(None);
}

impl ConnectionHolder for Gen2Reader {
    fn steal_connection(self) -> Connection {
        self.connection
    }
}

unsafe impl BasicReader for Gen2Reader {}

unsafe impl ProtocolReader for Gen2Reader {
    fn tune(&mut self, algo: TuningAlgorithm) -> Result<()> {
        // None does nothing
        if algo == TuningAlgorithm::None {
            return Ok(());
        }

        // Get the current reader settings, we need to know which antenna is in use
        let mut tx_rx_cfg = ffi::STUHFL_T_ST25RU3993_TxRxCfg::default();
        unsafe { proc_err(ffi::Get_TxRxCfg(&mut tx_rx_cfg))? }

        // Create a tune configuration using the antenna & algorithm
        let mut tune_cfg = ffi::STUHFL_T_ST25RU3993_TuneCfg {
            antenna: tx_rx_cfg.usedAntenna,
            algorithm: algo as u8,
            tuneAll: true,
            ..Default::default()
        };

        // Tune the reader using the configuration
        unsafe { proc_err(ffi::TuneChannel(&mut tune_cfg))? }

        // Mark tuned status
        self.is_tuned = true;

        Ok(())
    }

    fn inventory_once(&self) -> Result<(InventoryStatistics, Vec<InventoryTag>)> {
        // Require tuning
        if !self.is_tuned {
            return Err(Error::Generic);
        }

        // create tag data storage location
        let mut tag_data: [ffi::STUHFL_T_InventoryTag; ffi::STUHFL_D_MAX_TAG_LIST_SIZE as usize] =
            unsafe { std::mem::zeroed() };

        // create tag data storage container
        let mut inv_data = ffi::STUHFL_T_InventoryData {
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
        unsafe { proc_err(ffi::Gen2_Inventory(&mut inv_option, &mut inv_data))? }

        // save data into iterator
        let tags = tag_data[..inv_data.statistics.tagCnt as usize]
            .iter()
            .map(|tag| InventoryTag::from(*tag))
            .collect();

        let statistics = InventoryStatistics::from(inv_data.statistics);

        Ok((statistics, tags))
    }

    fn inventory(&mut self, num_rounds: u32, cb: Box<CallbackFn>) -> Result<InventoryStatistics> {
        // Require tuning
        if !self.is_tuned {
            return Err(Error::Generic);
        }

        if num_rounds == 0 {
            eprintln!("Error: num_rounds = 0 not yet implemented!");
            return Err(Error::None);
        }

        // create tag data storage location
        let mut tag_data: [ffi::STUHFL_T_InventoryTag; ffi::STUHFL_D_MAX_TAG_LIST_SIZE as usize] =
            unsafe { std::mem::zeroed() };

        // create tag data storage container
        let mut inv_data = ffi::STUHFL_T_InventoryData {
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
        *cb_holder = Some(cb);
        drop(cb_holder);

        // Call inventory (blocking)
        let result = unsafe {
            proc_err(ffi::Inventory_RunnerStart(
                &mut inv_option,
                Some(cycle_cb),
                None,
                &mut inv_data,
            ))
        };

        let cb_success = if CB_HOLDER.is_poisoned() {
            Err(Error::Generic)
        } else {
            Ok(())
        };

        // Delete callback function
        let mut guard = match CB_HOLDER.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        *guard = None;

        // Return any errors due to inventory failing
        result?;

        // Return any errors due to poisoning
        cb_success?;

        let statistics = InventoryStatistics::from(inv_data.statistics);

        Ok(statistics)
    }

    fn select(&mut self, epc: &Epc) -> Result<()> {
        // Require tuning
        if !self.is_tuned {
            return Err(Error::Generic);
        }

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

        unsafe { proc_err(ffi::Gen2_Select(&mut sel))? }

        Ok(())
    }

    fn read(
        &mut self,
        bank: MemoryBank,
        word_address: u32,
        num_bytes: u8,
        password: Option<Password>,
    ) -> Result<Vec<u8>> {
        // Require tuning
        if !self.is_tuned {
            return Err(Error::Generic);
        }

        let mut read_struct = ffi::STUHFL_T_Read {
            wordPtr: word_address,
            memoryBank: bank as u8,
            numBytesToRead: num_bytes,
            pwd: if let Some(pwd) = password {
                pwd.into_inner()
            } else {
                [0; 4]
            },
            numReadBytes: 0,
            data: [0; 64],
        };

        // Call read
        unsafe { proc_err(ffi::Gen2_Read(&mut read_struct))? }

        // Create vector from read bytes
        let result = Vec::from(&read_struct.data[..read_struct.numReadBytes as usize]);

        // Return result
        Ok(result)
    }

    fn write(
        &mut self,
        bank: MemoryBank,
        word_adddress: u32,
        data: [u8; 2],
        password: Option<Password>,
    ) -> Result<()> {
        // Require tuning
        if !self.is_tuned {
            return Err(Error::Generic);
        }

        let mut write_struct = ffi::STUHFL_T_Write {
            wordPtr: word_adddress,
            memoryBank: bank as u8,
            pwd: if let Some(pwd) = password {
                pwd.into_inner()
            } else {
                [0; 4]
            },
            data,
            tagReply: 0,
        };

        unsafe { proc_err(ffi::Gen2_Write(&mut write_struct))? }

        Ok(())
    }
}

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
        let data = unsafe { &*data };

        // Copy every scanned tag into the vector
        for i in 0..data.tagListSize {
            // Index pointer to array and convert it to InventoryTag
            let tag = InventoryTag::from(unsafe { *data.tagList.offset(i as isize) });
            // Let caller handle values
            cb_fn(tag);
        }
    });

    if cb_wrapper.is_err() {
        // callback unwrapped, mutex now poisoned
        unsafe { ffi::Inventory_RunnerStop() };
        Error::Generic as ffi::STUHFL_T_RET_CODE
    } else {
        // callback finished
        Error::None as ffi::STUHFL_T_RET_CODE
    }
}

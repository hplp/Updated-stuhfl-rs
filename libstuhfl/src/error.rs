use std::fmt;

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    /// Error type used throughout the STUHFL library. The `None` error
    /// will be returned from recoverable warnings, usually due to
    /// improper usage. (Successfull `None` values are not returned
    /// as errors)
    ///
    /// The following errors are used within the application:
    ///  - [`Error::Generic`]
    ///  - [`Error::None`]
    ///  - [`Error::NoMem`]
    ///  - [`Error::Busy`]
    ///  - [`Error::GeneralIo`]
    ///  - [`Error::Timeout`]
    ///  - [`Error::Request`]
    ///  - [`Error::NoMsg`]
    ///  - [`Error::Param`]
    ///  - [`Error::Proto`]
    ///
    /// The following errors are primarily raised by the ST25RU3993 itself:
    ///  - [`Error::ChipNoResp`]
    ///  - [`Error::ChipHeader`]
    ///  - [`Error::ChipPreamble`]
    ///  - [`Error::ChipRxCount`]
    ///  - [`Error::ChipFifo`]
    ///  - [`Error::ChipColl`]
    ///
    /// The following are upper level protocol errors for Gen2:
    ///  - [`Error::Gen2Select`]
    ///  - [`Error::Gen2Access`]
    ///  - [`Error::Gen2ReqRn`]
    ///  - [`Error::Gen2ChannelTimeout`]
    ///
    /// The following are Gen2 V2 Errors:
    ///  - [`Error::Gen2Other`]
    ///  - [`Error::Gen2NotSupported`]
    ///  - [`Error::Gen2Privileges`]
    ///  - [`Error::Gen2MemOverRun`]
    ///  - [`Error::Gen2MemLocked`]
    ///  - [`Error::Gen2Crypto`]
    ///  - [`Error::Gen2Encapsulation`]
    ///  - [`Error::Gen2RespBufOverflow`]
    ///  - [`Error::Gen2SecurityTimeout`]
    ///  - [`Error::Gen2PowerShortage`]
    ///  - [`Error::Gen2Nonspecific`]
    ///
    /// The following are upper level protocol errors for GB-29768:
    ///  - [`Error::Gb29768PowerShortage`]
    ///  - [`Error::Gb29768PermissionError`]
    ///  - [`Error::Gb29768StorageOverflow`]
    ///  - [`Error::Gb29768StorageLocked`]
    ///  - [`Error::Gb29768PasswordError`]
    ///  - [`Error::Gb29768AuthError`]
    ///  - [`Error::Gb29768AccessError`]
    ///  - [`Error::Gb29768AccessTimeout`]
    ///  - [`Error::Gb29768Other`]
    ///
    /// The following are upper level protocol erros for ISO-6b:
    ///  - [`Error::Iso6bNoTag`]
    ///  - [`Error::Iso6bIRQ`]
    ///  - [`Error::Iso6bRegFIFO`]
    ///  - [`Error::Iso6bOther`]
    ///  - [`Error::Iso6bAccessTimeout`]
    ///
    pub enum Error {
        /// Generic errors
        Generic = ffi::STUHFL_ERR_GENERIC as u32,
        /// No errors occurred (Note: This could be a warning. See [`Self`])
        None = ffi::STUHFL_ERR_NONE as u32,
        /// Not enough memory to perform the requested operation
        NoMem = ffi::STUHFL_ERR_NOMEM as u32,
        /// Device or resource busy
        Busy = ffi::STUHFL_ERR_BUSY as u32,
        /// General IO. Usually happens when reader is disconnected
        /// or can't be reached.
        GeneralIo = ffi::STUHFL_ERR_IO as u32,
        /// Error due to timeout
        Timeout = ffi::STUHFL_ERR_TIMEOUT as u32,
        /// Invalid request or requested function can't be executed at the moment
        Request = ffi::STUHFL_ERR_REQUEST as u32,
        /// No message of desired type
        NoMsg = ffi::STUHFL_ERR_NOMSG as u32,
        /// Parameter error
        Param = ffi::STUHFL_ERR_PARAM as u32,
        /// Protocol error
        Proto = ffi::STUHFL_ERR_PROTO as u32,
        /// No response
        ChipNoResp = ffi::STUHFL_ERR_CHIP_NORESP as u32,
        /// Header
        ChipHeader = ffi::STUHFL_ERR_CHIP_HEADER as u32,
        /// Preamble
        ChipPreamble = ffi::STUHFL_ERR_CHIP_PREAMBLE as u32,
        /// Chip RX count
        ChipRxCount = ffi::STUHFL_ERR_CHIP_RXCOUNT as u32,
        /// Chip CRC
        ChipCrc = ffi::STUHFL_ERR_CHIP_CRCERROR as u32,
        /// Chip First-In-First-Out buffer
        ChipFifo = ffi::STUHFL_ERR_CHIP_FIFO as u32,
        /// Chip Collision
        ChipColl = ffi::STUHFL_ERR_CHIP_COLL as u32,
        /// Reflected Power
        ReflectedPower = ffi::STUHFL_ERR_REFLECTED_POWER as u32,
        /// Gen2 Select
        Gen2Select = ffi::STUHFL_ERR_GEN2_SELECT as u32,
        /// Gen2 Access
        Gen2Access = ffi::STUHFL_ERR_GEN2_ACCESS as u32,
        /// Gen2 Request Random Number
        Gen2ReqRn = ffi::STUHFL_ERR_GEN2_REQRN as u32,
        /// Gen2 Channel Timeout
        Gen2ChannelTimeout = ffi::STUHFL_ERR_GEN2_CHANNEL_TIMEOUT as u32,
        /// Gen2 Other
        Gen2Other = ffi::STUHFL_ERR_GEN2_ERRORCODE_OTHER as u32,
        /// Gen2 Unsupported Command
        Gen2NotSupported = ffi::STUHFL_ERR_GEN2_ERRORCODE_NOTSUPPORTED as u32,
        /// Gen2 Invalid Priviledges
        Gen2Privileges = ffi::STUHFL_ERR_GEN2_ERRORCODE_PRIVILEGES as u32,
        /// Gen2 Memory Overrun
        Gen2MemOverRun = ffi::STUHFL_ERR_GEN2_ERRORCODE_MEMOVERRUN as u32,
        /// Gen2 Memory Locked
        Gen2MemLocked = ffi::STUHFL_ERR_GEN2_ERRORCODE_MEMLOCKED as u32,
        /// Gen2 Crypto
        Gen2Crypto = ffi::STUHFL_ERR_GEN2_ERRORCODE_CRYPTO as u32,
        /// Gen2 Encapsulation
        Gen2Encapsulation = ffi::STUHFL_ERR_GEN2_ERRORCODE_ENCAPSULATION as u32,
        /// Gen2 Response Buffer Overflow
        Gen2RespBufOverflow = ffi::STUHFL_ERR_GEN2_ERRORCODE_RESPBUFOVERFLOW as u32,
        /// Gen2 Security Timeout
        Gen2SecurityTimeout = ffi::STUHFL_ERR_GEN2_ERRORCODE_SECURITYTIMEOUT as u32,
        /// Gen2 Power Shortage
        Gen2PowerShortage = ffi::STUHFL_ERR_GEN2_ERRORCODE_POWER_SHORTAGE as u32,
        /// Gen2 Nonspecific
        Gen2Nonspecific = ffi::STUHFL_ERR_GEN2_ERRORCODE_NONSPECIFIC as u32,
        /// GB-29768 Power Shortage
        Gb29768PowerShortage = ffi::STUHFL_ERR_GB29768_POWER_SHORTAGE as u32,
        /// GB-29768 Permissions Error
        Gb29768PermissionError = ffi::STUHFL_ERR_GB29768_PERMISSION_ERROR as u32,
        /// GB-29768 Storage Overflow
        Gb29768StorageOverflow = ffi::STUHFL_ERR_GB29768_STORAGE_OVERFLOW as u32,
        /// GB-29768 Storage Locked
        Gb29768StorageLocked = ffi::STUHFL_ERR_GB29768_STORAGE_LOCKED as u32,
        /// GB-29768 Password Error
        Gb29768PasswordError = ffi::STUHFL_ERR_GB29768_PASSWORD_ERROR as u32,
        /// GB-29768 Authentication Error
        Gb29768AuthError = ffi::STUHFL_ERR_GB29768_AUTH_ERROR as u32,
        /// GB-29768 Access Error
        Gb29768AccessError = ffi::STUHFL_ERR_GB29768_ACCESS_ERROR as u32,
        /// GB-29768 Access Timeout
        Gb29768AccessTimeout = ffi::STUHFL_ERR_GB29768_ACCESS_TIMEOUT_ERROR as u32,
        /// GB-29768 Other
        Gb29768Other = ffi::STUHFL_ERR_GB29768_OTHER as u32,
        /// ISO-6b No Tag
        Iso6bNoTag = ffi::STUHFL_ERR_ISO6B_NOTAG as u32,
        /// ISO-6b IRQ
        Iso6bIRQ = ffi::STUHFL_ERR_ISO6B_IRQ as u32,
        /// ISO-6b First In First Out Buffer
        Iso6bRegFIFO = ffi::STUHFL_ERR_ISO6B_REG_FIFO as u32,
        /// ISO-6b Other
        Iso6bOther = ffi::STUHFL_ERR_ISO6B_OTHER as u32,
        /// ISO-6b Access Timeout
        Iso6bAccessTimeout = ffi::STUHFL_ERR_ISO6B_ACCESS_TIMEOUT as u32,
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} Error",
            match *self {
                Error::Generic => "Generic",
                Error::None => "None",
                Error::NoMem => "No Memory",
                Error::Busy => "Busy",
                Error::GeneralIo => "General IO",
                Error::Timeout => "Timeout",
                Error::Request => "Request",
                Error::NoMsg => "No Message",
                Error::Param => "Parameter",
                Error::Proto => "Protocol",
                Error::ChipNoResp => "No Response",
                Error::ChipHeader => "Header",
                Error::ChipPreamble => "Preamble",
                Error::ChipRxCount => "Chip Rx Count",
                Error::ChipCrc => "Chip CRC",
                Error::ChipFifo => "Chip FIFO",
                Error::ChipColl => "Chip COLL",
                Error::ReflectedPower => "Reflected Power",
                Error::Gen2Select => "Gen2 Select",
                Error::Gen2Access => "Gen2 Access",
                Error::Gen2ReqRn => "Gen2 Request RN",
                Error::Gen2ChannelTimeout => "Gen2 Channel Timeout",
                Error::Gen2Other => "Gen2 Other",
                Error::Gen2NotSupported => "Gen2 Not Supported",
                Error::Gen2Privileges => "Gen2 Privileges",
                Error::Gen2MemOverRun => "Gen2 Memory Overrun",
                Error::Gen2MemLocked => "Gen2 Memory Locked",
                Error::Gen2Crypto => "Gen2 Crypto",
                Error::Gen2Encapsulation => "Gen2 Encapsulation",
                Error::Gen2RespBufOverflow => "Gen2 Response Buffer Overflow",
                Error::Gen2SecurityTimeout => "Gen2 Security Timeout",
                Error::Gen2PowerShortage => "Gen2 Power Shortage",
                Error::Gen2Nonspecific => "Gen2 Nonspecific",
                Error::Gb29768PowerShortage => "Gb29768 Power Shortage",
                Error::Gb29768PermissionError => "Gb29768 Permission Error",
                Error::Gb29768StorageOverflow => "Gb29768 Storage Overflow",
                Error::Gb29768StorageLocked => "Gb29768 Storage Locked",
                Error::Gb29768PasswordError => "Gb29768 Password Error",
                Error::Gb29768AuthError => "Gb29768 Authentication Error",
                Error::Gb29768AccessError => "Gb29768 Access Error",
                Error::Gb29768AccessTimeout => "Gb29768 Access Timeout",
                Error::Gb29768Other => "Gb29768 Other",
                Error::Iso6bNoTag => "Iso6b No Tag",
                Error::Iso6bIRQ => "Iso6b IRQ",
                Error::Iso6bRegFIFO => "Iso6b Register FIFO",
                Error::Iso6bOther => "Iso6b Other",
                Error::Iso6bAccessTimeout => "Iso6b Access Timeout",
            }
        )
    }
}

impl std::error::Error for Error {}

impl From<Error> for String {
    fn from(e: Error) -> String {
        format!["{}", e]
    }
}

use super::*;

enum_from_primitive! {
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[repr(u32)]
    pub enum Error {
        Generic = ffi::STUHFL_ERR_GENERIC,
        None = ffi::STUHFL_ERR_NONE,
        NoMem = ffi::STUHFL_ERR_NOMEM as u32,
        Busy = ffi::STUHFL_ERR_BUSY as u32,
        GeneralIO = ffi::STUHFL_ERR_IO as u32,
        Timeout = ffi::STUHFL_ERR_TIMEOUT as u32,
        Request = ffi::STUHFL_ERR_REQUEST as u32,
        NoMsg = ffi::STUHFL_ERR_NOMSG as u32,
        Param = ffi::STUHFL_ERR_PARAM as u32,
        Proto = ffi::STUHFL_ERR_PROTO as u32,
        NoResp = ffi::STUHFL_ERR_CHIP_NORESP as u32,
        Header = ffi::STUHFL_ERR_CHIP_HEADER as u32,
        Preamble = ffi::STUHFL_ERR_CHIP_PREAMBLE as u32,
        ChipRxCount = ffi::STUHFL_ERR_CHIP_RXCOUNT as u32,
        ChipCRC = ffi::STUHFL_ERR_CHIP_CRCERROR as u32,
        ChipFIFO = ffi::STUHFL_ERR_CHIP_FIFO as u32,
        ChipCOLL = ffi::STUHFL_ERR_CHIP_COLL as u32,
        ReflectedPower = ffi::STUHFL_ERR_REFLECTED_POWER as u32,
        Gen2Select = ffi::STUHFL_ERR_GEN2_SELECT as u32,
        Gen2Access = ffi::STUHFL_ERR_GEN2_ACCESS as u32,
        Gen2ReqRN = ffi::STUHFL_ERR_GEN2_REQRN as u32,
        Gen2ChannelTimeout = ffi::STUHFL_ERR_GEN2_CHANNEL_TIMEOUT as u32,
        Gen2Other = ffi::STUHFL_ERR_GEN2_ERRORCODE_OTHER as u32,
        Gen2NotSupported = ffi::STUHFL_ERR_GEN2_ERRORCODE_NOTSUPPORTED as u32,
        Gen2Privileges = ffi::STUHFL_ERR_GEN2_ERRORCODE_PRIVILEGES as u32,
        Gen2MemOverRun = ffi::STUHFL_ERR_GEN2_ERRORCODE_MEMOVERRUN as u32,
        Gen2MemLocked = ffi::STUHFL_ERR_GEN2_ERRORCODE_MEMLOCKED as u32,
        Gen2Crypto = ffi::STUHFL_ERR_GEN2_ERRORCODE_CRYPTO as u32,
        Gen2Encapsulation = ffi::STUHFL_ERR_GEN2_ERRORCODE_ENCAPSULATION as u32,
        Gen2RespBufOverflow = ffi::STUHFL_ERR_GEN2_ERRORCODE_RESPBUFOVERFLOW as u32,
        Gen2SecurityTimeout = ffi::STUHFL_ERR_GEN2_ERRORCODE_SECURITYTIMEOUT as u32,
        Gen2PowerShortage = ffi::STUHFL_ERR_GEN2_ERRORCODE_POWER_SHORTAGE as u32,
        Gen2Nonspecific = ffi::STUHFL_ERR_GEN2_ERRORCODE_NONSPECIFIC as u32,
        Gb29768PowerShortage = ffi::STUHFL_ERR_GB29768_POWER_SHORTAGE as u32,
        Gb29768PermissionError = ffi::STUHFL_ERR_GB29768_PERMISSION_ERROR as u32,
        Gb29768StorageOverflow = ffi::STUHFL_ERR_GB29768_STORAGE_OVERFLOW as u32,
        Gb29768StorageLocked = ffi::STUHFL_ERR_GB29768_STORAGE_LOCKED as u32,
        Gb29768PasswordError = ffi::STUHFL_ERR_GB29768_PASSWORD_ERROR as u32,
        Gb29768AuthError = ffi::STUHFL_ERR_GB29768_AUTH_ERROR as u32,
        Gb29768AccessError = ffi::STUHFL_ERR_GB29768_ACCESS_ERROR as u32,
        Gb29768AccessTimeout = ffi::STUHFL_ERR_GB29768_ACCESS_TIMEOUT_ERROR as u32,
        Gb29768Other = ffi::STUHFL_ERR_GB29768_OTHER as u32,
        Iso6bNoTag = ffi::STUHFL_ERR_ISO6B_NOTAG as u32,
        Iso6bIRQ = ffi::STUHFL_ERR_ISO6B_IRQ as u32,
        Iso6bRegFIFO = ffi::STUHFL_ERR_ISO6B_REG_FIFO as u32,
        Iso6bOther = ffi::STUHFL_ERR_ISO6B_OTHER as u32,
        Iso6bAccessTimeout = ffi::STUHFL_ERR_ISO6B_ACCESS_TIMEOUT as u32,
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Error",
        match *self {
            Error::Generic => "Generic",
            Error::None => "None",
            Error::NoMem => "No Memory",
            Error::Busy => "Busy",
            Error::GeneralIO => "General IO",
            Error::Timeout => "Timeout",
            Error::Request => "Request",
            Error::NoMsg => "No Message",
            Error::Param => "Parameter",
            Error::Proto => "Protocol",
            Error::NoResp => "No Response",
            Error::Header => "Header",
            Error::Preamble => "Preamble",
            Error::ChipRxCount => "Chip Rx Count",
            Error::ChipCRC => "Chip CRC",
            Error::ChipFIFO => "Chip FIFO",
            Error::ChipCOLL => "Chip COLL",
            Error::ReflectedPower => "Reflected Power",
            Error::Gen2Select => "Gen2 Select",
            Error::Gen2Access => "Gen2 Access",
            Error::Gen2ReqRN => "Gen2 Request RN",
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
            Error::Iso6bAccessTimeout => "Iso6b Access Timeout"
        })
    }
}

impl std::error::Error for Error {}

impl From<Error> for String {
    fn from(e: Error) -> String {
        format!["{}", e]
    }
}

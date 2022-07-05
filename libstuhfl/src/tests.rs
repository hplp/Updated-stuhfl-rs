use super::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[cfg(feature = "reader_tests")]
extern crate serial_test;
#[cfg(feature = "reader_tests")]
use serial_test::*;

#[test]
fn version_test() {
    let va = VersionNum{ major: 2, minor: 0, micro: 0, nano: 0 };
    let vb = VersionNum{ major: 1, minor: 0, micro: 0, nano: 0 };
    
    assert!(va > vb);

    let vb = VersionNum{ major: 0, minor: 3, micro: 0, nano: 0 };
    assert!(va > vb);

    let vb = VersionNum{ major: 2, minor: 1, micro: 0, nano: 0 };
    assert!(vb > va);
}

#[test]
fn builder_test() -> TestResult {
    // Builder should have valid defaults for all configuration values
    Gen2CfgBuilder::default().build()?;

    // Alternative syntax: get default builder from Gen2Cfg itself
    Gen2Cfg::builder().build()?;

    Ok(())
}

#[test]
fn hex_id_test() -> TestResult {
    let id: Vec<u8> = vec![226, 0, 66, 22, 97, 128, 96, 21, 0, 149, 24, 56];

    let epc = Epc::from_id(id);

    assert_eq!(epc.to_string(), "E2:00:42:16:61:80:60:15:00:95:18:38");

    Ok(())
}

#[test]
#[serial]
#[cfg(feature = "reader_tests")]
fn check_reader_version() -> TestResult {

    let reader = ST25RU3993::new()?;

    let version = reader.get_board_version()?;

    println!("Board version: {}", &version);

    Ok(())
}

#[test]
#[serial]
#[cfg(feature = "reader_tests")]
fn gen2_configure() -> TestResult {

    let mut reader = ST25RU3993::new()?;

    let gen2_config = Gen2Cfg::builder()
        .build()?;

    reader.configure_gen2(&gen2_config)?;

    reader.tune_freqs(TuningAlgorithm::Exact)?;

    Ok(())
}

#[test]
#[serial]
#[cfg(feature = "reader_tests")]
fn gen2_inventory() -> TestResult {

    // connect to reader
    let mut reader = ST25RU3993::new()?;

    // set gen2 configuration
    let gen2_cfg = Gen2Cfg::builder()
        .build()?;
    
    // apply the settings
    reader.configure_gen2(&gen2_cfg)?;

    // tune reader
    reader.tune_freqs(TuningAlgorithm::Exact)?;
    
    // run inventory
    let (statitistics, tags) = reader.inventory()?;

    println!("Inventory Statistics:\n{:#?}", statitistics);
    println!("Found tags:");

    for tag in tags {
        println!("{}", tag.epc);
    }

    Ok(())
}

#[test]
#[serial]
#[cfg(feature = "reader_tests")]
fn gen2_inventory_runner() -> TestResult {
    use std::sync::{Arc, Mutex};

    // connect to reader
    let mut reader = ST25RU3993::new()?;

    // set gen2 configuration
    let gen2_cfg = Gen2Cfg::builder()
        .build()?;
    
    // apply the settings
    reader.configure_gen2(&gen2_cfg)?;

    // tune reader
    reader.tune_freqs(TuningAlgorithm::Exact)?;

    // create atomic vector of tags
    let tags = Arc::new(Mutex::new(Vec::new()));
    let tags2 = Arc::clone(&tags);

    // create callback function
    let callback = move |tag| {
        let mut tags = tags2.lock().unwrap();
        tags.push(tag);
    };

    // run inventory
    let statitistics = reader.inventory_runner(20, Box::new(callback))?;

    println!("Inventory Statistics:\n{:#?}", statitistics);
    println!("Found tags:");

    // lock tags
    let tags = tags.lock().unwrap();

    // read tags
    for tag in &*tags {
        println!("{}", tag.epc);
    }

    Ok(())
}

#[test]
#[serial]
#[cfg(feature = "reader_tests")]
fn gen2_inventory_runner_error() -> TestResult {
    // connect to reader
    let mut reader = ST25RU3993::new()?;

    // set gen2 configuration
    let gen2_cfg = Gen2Cfg::builder()
        .build()?;
    
    // apply the settings
    reader.configure_gen2(&gen2_cfg)?;

    // tune reader
    reader.tune_freqs(TuningAlgorithm::Exact)?;

    // create callback function
    let callback = |_tag| {
        panic!()
    };

    // run inventory
    assert!(reader.inventory_runner(20, Box::new(callback)).is_err());

    Ok(())
}

#[test]
#[serial]
#[cfg(feature = "reader_tests")]
fn gen2_read() -> TestResult {

    // connect to reader
    let mut reader = ST25RU3993::new()?;

    // set gen2 configuration
    let gen2_cfg = Gen2Cfg::builder()
        .build()?;

    // apply the settings
    reader.configure_gen2(&gen2_cfg)?;

    // tune the reader
    reader.tune_freqs(TuningAlgorithm::Exact)?;

    let (_, tags) = reader.inventory()?;

    for tag in tags {
        println!("Found tag {}", &tag.epc);
        reader.select_gen2(&tag.epc)?;
        let epc = reader.read_gen2(Gen2MemoryBank::Epc, 0x02, 12, None)?;
        assert!(epc == tag.epc.get_id());
    }

    Ok(())
}

#[test]
#[serial]
#[cfg(feature = "reader_tests")]
fn gen2_write() -> TestResult {
    // Connect to reader
    let mut reader = ST25RU3993::new()?;

    // Set configuration
    let gen2_cfg = Gen2Cfg::builder()
        .build()?;
    
    // Apply configuration
    reader.configure_gen2(&gen2_cfg)?;

    // Tune reader
    reader.tune_freqs(TuningAlgorithm::Exact)?;

    // Run an inventory
    let (_, tags) = reader.inventory()?;

    if tags.is_empty() { panic!("No tags found") }

    reader.select_gen2(&tags[0].epc)?;

    let reply = reader.write_gen2(Gen2MemoryBank::User, 0x00, [0x55, 0x55], None)?;

    println!("Tag reply: {}", reply);

    assert_eq!(reader.read_gen2(Gen2MemoryBank::User, 0x00, 2, None)?, [0x55, 0x55]);

    Ok(())
}

#[test]
#[serial]
#[cfg(feature = "reader_tests")]
fn gen2_custom_ffi() -> TestResult {
    // Connect to reader
    let mut reader = ST25RU3993::new().expect("failed to connect to reader");

    // Set configuration
    let gen2_cfg = Gen2Cfg::builder()
        .build()?;
    
    // Apply configuration
    reader.configure_gen2(&gen2_cfg)?;

    // Tune reader
    reader.tune_freqs(TuningAlgorithm::Exact)?;

    // Run an inventory
    let (_, tags) = reader.inventory().expect("failed to inventory tags");

    // Select tag
    if tags.is_empty() { panic!("No tags found") }
    reader.select_gen2(&tags[0].epc).expect("failed to select tag");

    // Chose number of bytes to read
    let nb_words = 2u8;

    // Create struct
    let mut generic_cmd_struct = ffi::STUHFL_T_Gen2_GenericCmd {
        cmd: ffi::STUHFL_D_GEN2_GENERIC_CMD_CRC_EXPECT_HEAD as u8,
        noResponseTime: 0xFF,
        appendRN16: true,
        sndDataBitLength: 26, // CMD[8] + BANK[2] + PTR[EBV,8] + LEN[8]
        sndData: [0; 64],
        expectedRcvDataBitLength: ((nb_words*2*8) + 16) as u16, // READ_BYTES * 16 + RN16
        // defaults
        pwd: [0; 4],
        rcvData: [0; 128],
        rcvDataLength: 0,
    };

    // override sndData
    generic_cmd_struct.sndData[0] = 0xC2;
    generic_cmd_struct.sndData[1] = 0xC0;
    generic_cmd_struct.sndData[2] = nb_words >> 2;
    generic_cmd_struct.sndData[3] = nb_words << 6;

    // Write memory to test against
    reader.write_gen2(Gen2MemoryBank::User, 0x00, [0x73, 0x42], None)?;
    reader.write_gen2(Gen2MemoryBank::User, 0x01, [0x53, 0x13], None)?;

    // Read using custom read command
    unsafe{proc_err(ffi::Gen2_GenericCmd(&mut generic_cmd_struct))?};

    // Check read bytes
    assert_eq!(generic_cmd_struct.rcvData[..nb_words as usize * 2], [0x73, 0x42, 0x53, 0x13]);

    Ok(())
}

#[test]
#[serial]
#[cfg(feature = "reader_tests")]
fn gen2_custom() -> TestResult {
    // Connect to reader
    let mut reader = ST25RU3993::new().expect("failed to connect to reader");

    // Set configuration
    let gen2_cfg = Gen2Cfg::builder()
        .build()?;
    
    // Apply configuration
    reader.configure_gen2(&gen2_cfg)?;

    // Tune reader
    reader.tune_freqs(TuningAlgorithm::Exact)?;

    // Run an inventory
    let (_, tags) = reader.inventory().expect("failed to inventory tags");

    // Select a tag
    if tags.is_empty() { panic!("No tags found") }
    reader.select_gen2(&tags[0].epc).expect("failed to select tag");

    // Determine tag's allocation class
    let allocation_class = tags[0].tid[0];
    println!("Found tag {} with allocation class {:02X}", &tags[0].epc, allocation_class);

    // Create custom command: GetUID for EM4325
    let get_uid = Gen2CustomCommand {
        command_code: 0xE000,
        use_crc: true,
        use_rn16: true,
        expect_header: true,
    };

    // The response's length is dependant on the UID's length
    // which is determined by allocation class
    let uid_len = match allocation_class {
        0xE0 => 64,
        0xE3 => 80,
        0xE2 => 96,
        0x44 | 0x45 | 0x46 | 0x47 => 64,
        _ => panic!("unknown allocation class")
    };

    // Get tag's UID
    let uid = reader.custom_gen2(&get_uid, None, uid_len, None)?;
    println!("Tag UID: {:02X?}", &uid[..uid.len() - 2]); // Last 2 bytes are RN16 code

    Ok(())
}

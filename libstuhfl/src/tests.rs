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
fn gen2_inventory_continuous() -> TestResult {

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
    let (statitistics, tags) = reader.inventory_runner(20)?;

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

    // run the inventory (TODO - use the tags found to select with)
    let (_, tags) = reader.inventory()?;

    // must find tags to continue with test
    if tags.is_empty() {
        return Err(Box::new("Empty tag list!".to_owned()));
    }

    // print found tags
    println!("Found tags:");
    for tag in tags {
        println!("{}", tag.epc);
    }

    // select the first found tag
    reader.select_gen2(tags[0].epc)?;

    // read from the tag
    let data = reader.read_gen2(Gen2MemoryBank::User, 0xEC, 3, None)?;

    // print read bytes
    println!("Read bytes {}: {:?}", data.len(), data);

    Ok(())
}

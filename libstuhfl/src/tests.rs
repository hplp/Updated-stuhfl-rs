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
fn gen2_inventory_ffi() -> TestResult {

    // connect to reader
    let mut reader = ST25RU3993::new()?;

    // set adaptive q settings
    let adaptive_q_cfg = Gen2AdaptiveQCfg::builder()
        .start_q(4)
        .build()?;
    
    // set query parameters
    let query_params = Gen2QueryParams::builder()
        .target_depletion_mode(false)
        .build()?;

    // set inventory configuration
    let inventory_cfg = Gen2InventoryCfg::builder()
        .adaptive_q(Gen2AdaptiveQ::Enable(adaptive_q_cfg))
        .query_params(query_params)
        .build()?;

    // set gen2 configuration
    let gen2_cfg = Gen2Cfg::builder()
        .inv_cfg(inventory_cfg)
        .build()?;
    
    // apply the settings
    reader.configure_gen2(&gen2_cfg)?;

    // tune reader
    reader.tune_freqs(TuningAlgorithm::None)?;

    // create tag data storage location
    let mut tag_data: [ffi::STUHFL_T_InventoryTag; ffi::STUHFL_D_MAX_TAG_LIST_SIZE as usize] = unsafe{std::mem::zeroed()};

    // create tag data storage container
    let mut inv_data = ffi::STUHFL_T_InventoryData{
        tagList: &mut tag_data as _,
        tagListSizeMax: tag_data.len() as u16,
        ..Default::default()
    };

    // customize inventory options
    let mut inv_option = ffi::STUHFL_T_InventoryOption{
        roundCnt: 2000,
        ..Default::default()
    };
    inv_option.options |= ffi::STUHFL_D_INVENTORYREPORT_OPTION_HEARTBEAT as u8;

    unsafe{proc_err(ffi::Gen2_Inventory(&mut inv_option, &mut inv_data))?}

    println!("Inventory Info:\n{:#?}", inv_data);

    let tag_cnt = inv_data.statistics.tagCnt as usize;

    println!("Tag Info:\n{:#?}", &tag_data[0..tag_cnt]);

    Ok(())
}

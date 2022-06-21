use super::*;

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
#[serial]
#[cfg(feature = "reader_tests")]
fn check_reader_version() -> Result<(), String> {

    let reader = ST25RU3993::new()?;

    let version = reader.get_board_version()?;

    println!("Board version: {}", &version);

    Ok(())
}

#[test]
#[serial]
#[cfg(feature = "reader_tests")]
fn gen2_configure_ffi() -> Result<(), String> {

    let mut reader = ST25RU3993::new()?;

    reader.setup_gen2_config(false, true, Antenna::Antenna1)?;

    Ok(())
}

#[test]
#[serial]
#[cfg(feature = "reader_tests")]
fn gen2_configure() -> Result<(), String> {

    let mut reader = ST25RU3993::new()?;

    let tx_rx_cfg = TxRxCfgBuilder::default()
        .build()?;

    let gen2_config = Gen2CfgBuilder::default()
        .tx_rx_cfg(&tx_rx_cfg)
        .build()?;

    reader.configure_gen2(&gen2_config)?;

    Ok(())

}

#[test]
#[serial]
#[cfg(feature = "reader_tests")]
fn gen2_inventory_ffi() -> Result<(), String> {

    // connect to reader
    let mut reader = ST25RU3993::new()?;

    // setup gen2
    reader.setup_gen2_config(false, true, Antenna::Antenna1)?;

    unsafe {
        // tweak gen2 settings
        let mut inv_gen2_cfg = ffi::STUHFL_T_ST25RU3993_Gen2_InventoryCfg::default();
        inv_gen2_cfg.antiCollision.startQ = 4;
        inv_gen2_cfg.antiCollision.adaptiveQ = true;
        inv_gen2_cfg.queryParams.targetDepletionMode = false;

        // save gen2 settings
        proc_err(ffi::Set_Gen2_InventoryCfg(&mut inv_gen2_cfg))?;

        // create tag data storage location
        let mut tag_data: [ffi::STUHFL_T_InventoryTag; ffi::STUHFL_D_MAX_TAG_LIST_SIZE as usize] = std::mem::zeroed();

        // create tag data storage container
        let mut inv_data = ffi::STUHFL_T_InventoryData{
            tagList: &mut tag_data as _,
            tagListSizeMax: ffi::STUHFL_D_MAX_TAG_LIST_SIZE as u16,
            ..Default::default()
        };

        // customize inventory options
        let mut inv_option = ffi::STUHFL_T_InventoryOption{
            roundCnt: 2000,
            ..Default::default()
        };
        inv_option.options |= ffi::STUHFL_D_INVENTORYREPORT_OPTION_HEARTBEAT as u8;

        proc_err(ffi::Gen2_Inventory(&mut inv_option, &mut inv_data))?;

        println!("Inventory Info:\n{:#?}", inv_data);

        println!("Tag Info:\n{:?}", tag_data);
    }

    Ok(())
}

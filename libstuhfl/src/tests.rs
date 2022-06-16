use super::*;
use st25ru3993::*;

extern crate serial_test;
use serial_test::*;

#[cfg(unix)]
#[test]
#[serial]
fn check_reader_version() {
    let reader = ST25RU3993::new("/dev/ttyUSB0").expect("Couldn't connect to reader");

    let version = reader.get_board_version().expect("Couldn't get version");

    println!("Board version: {}", version);
}

#[cfg(unix)]
#[test]
#[serial]
fn gen2_configure() {
    let mut reader = ST25RU3993::new("/dev/ttyUSB0").expect("Couldn't connect to reader");

    reader.setup_gen2_config(false, true, Antenna::Antenna1).expect("Couldn't Configure Reader");
}

#[cfg(unix)]
#[test]
#[serial]
fn gen2_inventory() {
    // connect reader
    let mut reader = ST25RU3993::new("/dev/ttyUSB0").expect("Couldn't connect to reader");

    // setup gen2
    reader.setup_gen2_config(false, true, Antenna::Antenna1).expect("Couldn't Configure Reader");

    let mut ret;

    unsafe {
        // tweak gen2 settings
        let mut inv_gen2_cfg = ffi::STUHFL_T_ST25RU3993_Gen2_InventoryCfg::default();
        inv_gen2_cfg.antiCollision.startQ = 4;
        inv_gen2_cfg.antiCollision.adaptiveQ = true;
        inv_gen2_cfg.queryParams.targetDepletionMode = false;

        // save gen2 settings
        ret = ffi::Set_Gen2_InventoryCfg(&mut inv_gen2_cfg);
        if ret != ffi::STUHFL_ERR_NONE { panic!("Error: {}", Error::from_u32(ret).unwrap()) };

        // create tag data storage location
        let mut tag_data: [ffi::STUHFL_T_InventoryTag; ffi::STUHFL_D_MAX_TAG_LIST_SIZE as usize] = std::mem::zeroed();

        // create tag data storage container
        let mut inv_data = ffi::STUHFL_T_InventoryData::default();
        inv_data.tagList = &mut tag_data as _;
        inv_data.tagListSizeMax = ffi::STUHFL_D_MAX_TAG_LIST_SIZE as u16;

        // customize inventory options
        let mut inv_option = ffi::STUHFL_T_InventoryOption::default();
        inv_option.roundCnt = 2000;
        inv_option.options |= ffi::STUHFL_D_INVENTORYREPORT_OPTION_HEARTBEAT as u8;

        ret = ffi::Gen2_Inventory(&mut inv_option, &mut inv_data);
        if ret != ffi::STUHFL_ERR_NONE { panic!("Error: {}", Error::from_u32(ret).unwrap()) };

        println!("Inventory Info:\n{:?}", inv_data);

        println!("Tag Info:\n{:?}", tag_data);
    }
}

use super::*;
use st25ru3993::*;

extern crate serial_test;
use serial_test::*;

extern crate serialport;
use serialport as sp;

#[cfg(unix)]
#[test]
#[serial]
fn check_reader_version() -> Result<(), Error> {

    let mut found_port: Option<String> = None;
    
    if let Ok(ports) = sp::available_ports() {
        for port in ports {
            if let sp::SerialPortType::UsbPort(port_info) = port.port_type {
                if port_info.vid == 0x403 && port_info.pid == 0x6015 {
                    sp::new(&port.port_name, 9600).open().expect("Couldn't open port!");
                    found_port = Some(port.port_name)
                }
            }
        }
    }
    
    let found_port = found_port.expect("Reader not found on any ports");

    println!("Found Port: {}", &found_port);

    let reader = ST25RU3993::new(&found_port).expect("Couldn't connect to reader");

    let version = reader.get_board_version().expect("Couldn't get reader version");

    println!("Board version: {}", &version);

    Ok(())
}

#[cfg(unix)]
#[test]
#[serial]
fn gen2_configure() -> Result<(), Error> {

    let mut found_port: Option<String> = None;
    
    if let Ok(ports) = sp::available_ports() {
        for port in ports {
            if let sp::SerialPortType::UsbPort(port_info) = port.port_type {
                if port_info.vid == 0x403 && port_info.pid == 0x6015 {
                    sp::new(&port.port_name, 9600).open().expect("Couldn't open port!");
                    found_port = Some(port.port_name)
                }
            }
        }
    }
    
    let found_port = found_port.expect("Reader not found on any ports");

    let mut reader = ST25RU3993::new(&found_port).expect("Couldn't connect to reader");

    reader.setup_gen2_config(false, true, Antenna::Antenna1).expect("Couldn't configure reader");

    Ok(())
}

#[cfg(unix)]
#[test]
#[serial]
fn gen2_inventory() -> Result<(), Error> {

    let mut found_port: Option<String> = None;
    
    if let Ok(ports) = sp::available_ports() {
        for port in ports {
            if let sp::SerialPortType::UsbPort(port_info) = port.port_type {
                if port_info.vid == 0x403 && port_info.pid == 0x6015 {
                    sp::new(&port.port_name, 9600).open().expect("Couldn't open port!");
                    found_port = Some(port.port_name)
                }
            }
        }
    }
    
    let found_port = found_port.expect("Reader not found on any ports");

    // connect to reader
    let mut reader = ST25RU3993::new(&found_port).expect("Couldn't connect to reader");

    // setup gen2
    reader.setup_gen2_config(false, true, Antenna::Antenna1).expect("Couldn't configure reader");

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
        let mut inv_data = ffi::STUHFL_T_InventoryData::default();
        inv_data.tagList = &mut tag_data as _;
        inv_data.tagListSizeMax = ffi::STUHFL_D_MAX_TAG_LIST_SIZE as u16;

        // customize inventory options
        let mut inv_option = ffi::STUHFL_T_InventoryOption::default();
        inv_option.roundCnt = 2000;
        inv_option.options |= ffi::STUHFL_D_INVENTORYREPORT_OPTION_HEARTBEAT as u8;

        proc_err(ffi::Gen2_Inventory(&mut inv_option, &mut inv_data))?;

        println!("Inventory Info:\n{:#?}", inv_data);

        println!("Tag Info:\n{:?}", tag_data);
    }

    Ok(())
}

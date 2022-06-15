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
fn configure_gen2() {
    let mut reader = ST25RU3993::new("/dev/ttyUSB0").expect("Couldn't connect to reader");

    reader.setup_gen2_config(false, true, Antenna::Antenna1).expect("Couldn't Configure Reader");
}

use super::*;
use st25ru3993::*;

#[cfg(unix)]
#[test]
fn connect_to_reader() {
    let reader = ST25RU3993::new("/dev/ttyUSB0").expect("Couldn't connect to reader!");

    let version = reader.get_board_version().expect("Couldn't get version!");

    println!("Board version: {}", version);
}

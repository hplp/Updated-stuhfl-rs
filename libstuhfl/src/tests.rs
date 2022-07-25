use crate::prelude::*;

type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

#[cfg(feature = "reader-tests")]
extern crate serial_test;
#[cfg(feature = "reader-tests")]
use rand::prelude::*;
#[cfg(feature = "reader-tests")]
use serial_test::*;

#[test]
fn version_comparison() {
    let va = VersionNum {
        major: 2,
        minor: 0,
        micro: 0,
        nano: 0,
    };
    let vb = VersionNum {
        major: 1,
        minor: 0,
        micro: 0,
        nano: 0,
    };

    assert!(va > vb);

    let vb = VersionNum {
        major: 0,
        minor: 3,
        micro: 0,
        nano: 0,
    };
    assert!(va > vb);

    let vb = VersionNum {
        major: 2,
        minor: 1,
        micro: 0,
        nano: 0,
    };
    assert!(vb > va);
}

#[test]
fn cfg_builder() -> TestResult {
    use crate::gen2::*;

    // Builder should have valid defaults for all configuration values
    Gen2CfgBuilder::default().build()?;

    // Alternative syntax: get default builder from Gen2Cfg itself
    Gen2Cfg::builder().build()?;

    Ok(())
}

#[test]
fn hex_id() -> TestResult {
    let id: Vec<u8> = vec![226, 0, 66, 22, 97, 128, 96, 21, 0, 149, 24, 56];

    let epc = Epc::from_id(id);

    assert_eq!(epc.to_string(), "E2:00:42:16:61:80:60:15:00:95:18:38");

    Ok(())
}

#[cfg(feature = "reader-tests")]
#[test]
#[serial]
fn check_reader_version() -> TestResult {
    let reader = Reader::autoconnect()?;

    let version = reader.get_version()?;

    println!("Board version: {}", &version);

    Ok(())
}

#[cfg(feature = "reader-tests")]
mod gen2 {
    use super::*;
    use crate::gen2::*;

    #[test]
    #[serial]
    fn configure() -> TestResult {
        let reader = Reader::autoconnect()?;

        let gen2_config = Gen2Cfg::builder().build()?;

        let mut reader = reader.configure_gen2(&gen2_config)?;

        reader.tune(TuningAlgorithm::Exact)?;

        Ok(())
    }

    #[test]
    #[serial]
    fn inventory_once() -> TestResult {
        let reader = Reader::autoconnect()?;

        let gen2_config = Gen2Cfg::builder().build()?;

        let mut reader = reader.configure_gen2(&gen2_config)?;

        reader.tune(TuningAlgorithm::Exact)?;

        let (stats, tags) = reader.inventory_once()?;

        println!("Inventory Statistics:{}", stats);
        println!("Found tags:");
        for tag in tags {
            println!("{}", tag.epc);
        }

        Ok(())
    }

    #[test]
    #[serial]
    fn inventory_runner() -> TestResult {
        use std::sync::{Arc, Mutex};

        let reader = Reader::autoconnect()?;

        let gen2_config = Gen2Cfg::builder().build()?;

        let mut reader = reader.configure_gen2(&gen2_config)?;

        reader.tune(TuningAlgorithm::Exact)?;

        let tags = Arc::new(Mutex::new(Vec::new()));
        let tags2 = Arc::clone(&tags);

        let callback = move |tag| {
            let mut tags = tags2.lock().unwrap();
            tags.push(tag);
        };

        let stats = reader.inventory(20, Box::new(callback))?;

        let tags = tags.lock().unwrap();

        println!("Inventory Statistics:{}", stats);
        println!("Found tags:");
        for tag in &*tags {
            println!("{}", tag.epc);
        }

        Ok(())
    }

    #[test]
    #[serial]
    fn read() -> TestResult {
        let reader = Reader::autoconnect()?;

        let gen2_config = Gen2Cfg::builder().build()?;

        let mut reader = reader.configure_gen2(&gen2_config)?;

        reader.tune(TuningAlgorithm::Exact)?;

        let (_stats, tags) = reader.inventory_once()?;

        println!("Found tags:");
        for tag in tags {
            println!("Found tag {}", &tag.epc);
            reader.select(&tag.epc)?;
            let epc = reader.read(MemoryBank::Epc, 0x02, 12, None)?;
            assert!(epc == tag.epc.get_id());
        }

        Ok(())
    }

    #[test]
    #[serial]
    fn read_alt() -> TestResult {
        let reader = Reader::autoconnect()?;

        let gen2_config = Gen2Cfg::builder().build()?;

        let mut reader = reader.configure_gen2(&gen2_config)?;

        reader.tune(TuningAlgorithm::Exact)?;

        let (_stats, tags) = reader.inventory_once()?;

        if tags.is_empty() {
            panic!("No tags found")
        }

        reader.select(&tags[0].epc)?;

        let bytes = reader.read_alt(MemoryBank::User, 0xEC, 1, None)?;

        println!("Read bytes: {:0X?}", bytes);

        Ok(())
    }

    #[test]
    #[serial]
    fn write() -> TestResult {
        let reader = Reader::autoconnect()?;

        let gen2_config = Gen2Cfg::builder().build()?;

        let mut reader = reader.configure_gen2(&gen2_config)?;

        reader.tune(TuningAlgorithm::Exact)?;

        let (_stats, tags) = reader.inventory_once()?;

        if tags.is_empty() {
            panic!("No tags found")
        }

        reader.select(&tags[0].epc)?;

        reader.write(MemoryBank::User, 0x00, [0x55, 0x55], None)?;

        let bytes_read = reader.read(MemoryBank::User, 0x00, 2, None)?;

        assert_eq!(bytes_read, [0x55, 0x55]);

        Ok(())
    }

    #[test]
    #[serial]
    fn write_high_addr() -> TestResult {
        let reader = Reader::autoconnect()?;

        let gen2_config = Gen2Cfg::builder().build()?;

        let mut reader = reader.configure_gen2(&gen2_config)?;

        reader.tune(TuningAlgorithm::Exact)?;

        let (_stats, tags) = reader.inventory_once()?;

        if tags.is_empty() {
            panic!("No tags found")
        }

        reader.select(&tags[0].epc)?;

        let bytes_backup = reader.read_alt(MemoryBank::User, 0xEC, 1, None)?;

        let new_word: u16 = rand::thread_rng().gen();
        let new_bytes = new_word.to_be_bytes();

        reader.write(MemoryBank::User, 0xEC, new_bytes, None)?;

        let bytes_read = reader.read_alt(MemoryBank::User, 0xEC, 1, None)?;

        assert_eq!(bytes_read, new_bytes);

        println!("Wrote bytes: {bytes_read:?}");

        reader.write(
            MemoryBank::User,
            0xEC,
            [bytes_backup[0], bytes_backup[1]],
            None,
        )?;

        assert_eq!(
            reader.read_alt(MemoryBank::User, 0xEC, 1, None)?,
            bytes_backup
        );

        Ok(())
    }

    #[test]
    #[serial]
    fn custom() -> TestResult {
        let reader = Reader::autoconnect()?;

        let gen2_config = Gen2Cfg::builder().build()?;

        let mut reader = reader.configure_gen2(&gen2_config)?;

        reader.tune(TuningAlgorithm::Exact)?;

        let (_stats, tags) = reader.inventory_once()?;

        if tags.is_empty() {
            panic!("No tags found")
        }

        reader.select(&tags[0].epc)?;

        // Determine tag's allocation class
        let allocation_class = tags[0].tid[0];
        println!(
            "Found tag {} with allocation class {:02X}",
            &tags[0].epc, allocation_class
        );

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
            _ => panic!("unknown allocation class"),
        };

        // Get tag's UID
        let uid = reader.custom_cmd(&get_uid, None, uid_len, None)?;
        println!("Tag UID: {:02X?}", &uid[..uid.len() - 2]); // Last 2 bytes are RN16 code

        Ok(())
    }
}

use crate::prelude::*;

type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

#[cfg(feature = "reader_tests")]
extern crate serial_test;
#[cfg(feature = "reader_tests")]
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

#[cfg(feature = "reader_tests")]
#[test]
#[serial]
fn check_reader_version() -> TestResult {
    let reader = Reader::autoconnect()?;

    let version = reader.get_version()?;

    println!("Board version: {}", &version);

    Ok(())
}

#[cfg(feature = "reader_tests")]
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
    fn read_custom() -> TestResult {
        use crate::helpers::proc_err;

        let reader = Reader::autoconnect()?;

        let gen2_config = Gen2Cfg::builder().build()?;

        let mut reader = reader.configure_gen2(&gen2_config)?;

        reader.tune(TuningAlgorithm::Exact)?;

        let (_stats, tags) = reader.inventory_once()?;

        if tags.is_empty() {
            panic!("No tags found")
        }

        reader.select(&tags[0].epc)?;

        fn ebv_formatter(mut d: u32) -> Vec<u8> {
            let mut v = Vec::new();

            loop {
                let b = (d & 0b0111_1111) as u8; // push 7 least significant bits
                d >>= 7; // remove least significant bits
                if d > 0 {
                    v.push(b | 0b1000_0000);
                } else {
                    v.push(b);
                    break;
                }
            }

            v
        }

        let addr = 0xEC;

        let ebv = ebv_formatter(addr);

        let b = MemoryBank::User;

        let wc = 1;

        let mut d = [0; 64];

        d[0] = 0b1100_0010;
        d[1] = (b as u8) << 6;

        for (i, byte) in ebv.iter().enumerate() {
            d[i + 1] |= byte >> 2;
            d[i + 2] |= byte << 6;
        }

        d[ebv.len() + 1] |= wc >> 2;
        d[ebv.len() + 2] |= wc << 6;

        let mut cmd = ffi::STUHFL_T_Gen2_GenericCmd {
            cmd: ffi::STUHFL_D_GEN2_GENERIC_CMD_CRC_EXPECT_HEAD as u8,
            pwd: [0, 0, 0, 0],
            noResponseTime: 0xFF,
            expectedRcvDataBitLength: 16 * (wc as u16),
            sndDataBitLength: 34 + 8 * ebv.len() as u16,
            appendRN16: true,
            sndData: d,
            rcvDataLength: 0,
            rcvData: [0; 128],
        };

        println!("Attempting to send packet: {:02X?}", d);

        unsafe { proc_err(ffi::Gen2_GenericCmd(&mut cmd))? }

        println!(
            "Read bytes {:02X?}",
            &cmd.rcvData[..cmd.rcvDataLength as usize]
        );

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

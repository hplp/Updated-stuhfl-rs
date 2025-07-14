//! Helper functions used within the crate

use crate::data_types::*;
use crate::error::{Error, Result};
// CB 7/9/25: Add prelude to use 'parse()' function from 'reader.rs'
use crate::prelude::*;
use enum_primitive::FromPrimitive;

/// Converts the profile enum into an 'item list' aka Vec<ChannelItem>
pub(crate) fn profile_to_item_list(profile: Profile) -> Vec<ChannelItem> {
    match profile {
        Profile::Europe => {
            vec![
                ChannelItem::from(
                    865700,
                    [TuningCaps::from(12, 12, 14), TuningCaps::from(12, 9, 16)],
                ),
                ChannelItem::from(
                    866300,
                    [TuningCaps::from(12, 12, 14), TuningCaps::from(11, 9, 16)],
                ),
                ChannelItem::from(
                    866900,
                    [TuningCaps::from(11, 12, 14), TuningCaps::from(11, 9, 16)],
                ),
                ChannelItem::from(
                    867500,
                    [TuningCaps::from(11, 12, 14), TuningCaps::from(11, 9, 16)],
                ),
            ]
        }
        Profile::Usa => {
            vec![
                ChannelItem::from(
                    902750,
                    [TuningCaps::from(13, 24, 12), TuningCaps::from(9, 19, 15)],
                ),
                ChannelItem::from(
                    903250,
                    [TuningCaps::from(13, 24, 12), TuningCaps::from(9, 19, 15)],
                ),
                ChannelItem::from(
                    903750,
                    [TuningCaps::from(13, 24, 12), TuningCaps::from(9, 19, 15)],
                ),
                ChannelItem::from(
                    904250,
                    [TuningCaps::from(13, 24, 12), TuningCaps::from(9, 19, 15)],
                ),
                ChannelItem::from(
                    904750,
                    [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 19, 15)],
                ),
                ChannelItem::from(
                    905250,
                    [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 19, 15)],
                ),
                ChannelItem::from(
                    905750,
                    [TuningCaps::from(9, 19, 14), TuningCaps::from(10, 25, 15)],
                ),
                ChannelItem::from(
                    906250,
                    [TuningCaps::from(9, 8, 12), TuningCaps::from(10, 25, 15)],
                ),
                ChannelItem::from(
                    906750,
                    [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 21, 15)],
                ),
                ChannelItem::from(
                    907250,
                    [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 21, 15)],
                ),
                ChannelItem::from(
                    907750,
                    [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 21, 15)],
                ),
                ChannelItem::from(
                    908250,
                    [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 21, 15)],
                ),
                ChannelItem::from(
                    908750,
                    [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 21, 15)],
                ),
                ChannelItem::from(
                    909250,
                    [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 21, 15)],
                ),
                ChannelItem::from(
                    909750,
                    [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 22, 15)],
                ),
                ChannelItem::from(
                    910250,
                    [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 22, 15)],
                ),
                ChannelItem::from(
                    910750,
                    [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 22, 15)],
                ),
                ChannelItem::from(
                    911250,
                    [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 22, 15)],
                ),
                ChannelItem::from(
                    911750,
                    [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 22, 15)],
                ),
                ChannelItem::from(
                    912250,
                    [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 22, 15)],
                ),
                ChannelItem::from(
                    912750,
                    [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 8, 13)],
                ),
                ChannelItem::from(
                    913250,
                    [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 8, 13)],
                ),
                ChannelItem::from(
                    913750,
                    [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 8, 13)],
                ),
                ChannelItem::from(
                    914250,
                    [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 25, 15)],
                ),
                ChannelItem::from(
                    914750,
                    [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 25, 15)],
                ),
                ChannelItem::from(
                    915250,
                    [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 25, 15)],
                ),
                ChannelItem::from(
                    915750,
                    [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 25, 15)],
                ),
                ChannelItem::from(
                    916250,
                    [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 14, 14)],
                ),
                ChannelItem::from(
                    916750,
                    [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 14, 14)],
                ),
                ChannelItem::from(
                    917250,
                    [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 14, 14)],
                ),
                ChannelItem::from(
                    917750,
                    [TuningCaps::from(9, 10, 12), TuningCaps::from(9, 14, 14)],
                ),
                ChannelItem::from(
                    918250,
                    [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 27, 15)],
                ),
                ChannelItem::from(
                    918750,
                    [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 27, 15)],
                ),
                ChannelItem::from(
                    919250,
                    [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    919750,
                    [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    920250,
                    [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    920750,
                    [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    921250,
                    [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    921750,
                    [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    922250,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    922750,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)],
                ),
                ChannelItem::from(
                    923250,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)],
                ),
                ChannelItem::from(
                    923750,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)],
                ),
                ChannelItem::from(
                    924250,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)],
                ),
                ChannelItem::from(
                    924750,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)],
                ),
                ChannelItem::from(
                    925250,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)],
                ),
                ChannelItem::from(
                    925750,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)],
                ),
                ChannelItem::from(
                    926250,
                    [TuningCaps::from(8, 6, 12), TuningCaps::from(9, 30, 15)],
                ),
                ChannelItem::from(
                    926750,
                    [TuningCaps::from(8, 6, 12), TuningCaps::from(9, 30, 15)],
                ),
                ChannelItem::from(
                    927250,
                    [TuningCaps::from(8, 6, 12), TuningCaps::from(9, 30, 15)],
                ),
            ]
        }
        Profile::Japan => {
            vec![
                ChannelItem::from(
                    920500,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    920700,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    920900,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    921100,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    921300,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    921500,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    921700,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    921900,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    922100,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)],
                ),
            ]
        }
        Profile::China => {
            vec![
                ChannelItem::from(
                    840625,
                    [TuningCaps::from(15, 21, 15), TuningCaps::from(13, 15, 18)],
                ),
                ChannelItem::from(
                    840875,
                    [TuningCaps::from(15, 21, 15), TuningCaps::from(13, 15, 18)],
                ),
                ChannelItem::from(
                    841125,
                    [TuningCaps::from(15, 21, 15), TuningCaps::from(13, 15, 18)],
                ),
                ChannelItem::from(
                    841375,
                    [TuningCaps::from(15, 21, 15), TuningCaps::from(13, 15, 18)],
                ),
                ChannelItem::from(
                    841625,
                    [TuningCaps::from(15, 21, 15), TuningCaps::from(13, 15, 18)],
                ),
                ChannelItem::from(
                    841875,
                    [TuningCaps::from(15, 21, 15), TuningCaps::from(13, 15, 18)],
                ),
                ChannelItem::from(
                    842125,
                    [TuningCaps::from(13, 14, 15), TuningCaps::from(13, 15, 18)],
                ),
                ChannelItem::from(
                    842375,
                    [TuningCaps::from(13, 14, 15), TuningCaps::from(13, 15, 18)],
                ),
                ChannelItem::from(
                    842625,
                    [TuningCaps::from(13, 14, 15), TuningCaps::from(16, 20, 17)],
                ),
                ChannelItem::from(
                    842875,
                    [TuningCaps::from(13, 14, 15), TuningCaps::from(16, 20, 17)],
                ),
                ChannelItem::from(
                    843125,
                    [TuningCaps::from(15, 23, 15), TuningCaps::from(16, 20, 17)],
                ),
                ChannelItem::from(
                    843375,
                    [TuningCaps::from(15, 23, 15), TuningCaps::from(16, 20, 17)],
                ),
                ChannelItem::from(
                    843625,
                    [TuningCaps::from(15, 23, 15), TuningCaps::from(16, 20, 17)],
                ),
                ChannelItem::from(
                    843875,
                    [TuningCaps::from(15, 23, 15), TuningCaps::from(16, 20, 17)],
                ),
                ChannelItem::from(
                    844125,
                    [TuningCaps::from(15, 23, 15), TuningCaps::from(16, 20, 17)],
                ),
                ChannelItem::from(
                    844375,
                    [TuningCaps::from(15, 23, 15), TuningCaps::from(16, 20, 17)],
                ),
            ]
        }
        Profile::China2 => {
            vec![
                ChannelItem::from(
                    920500,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    920700,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    920900,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    921100,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    921300,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    921500,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    921700,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    921900,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)],
                ),
                ChannelItem::from(
                    922100,
                    [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)],
                ),
            ]
        }
        // CB 7/9/25: Added custom profile that uses manual tuning from GUI results
        //            Uses USA frequencies
        Profile::Custom => {
            let ant1_cap_list: [(u8, u8, u8); 50] = parse().unwrap();
            vec![
                ChannelItem::from(
                    902750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[0].0,
                            ant1_cap_list[0].1,
                            ant1_cap_list[0].2,
                        ),
                        TuningCaps::from(9, 19, 15),
                    ],
                ),
                ChannelItem::from(
                    903250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[1].0,
                            ant1_cap_list[1].1,
                            ant1_cap_list[1].2,
                        ),
                        TuningCaps::from(9, 19, 15),
                    ],
                ),
                ChannelItem::from(
                    903750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[2].0,
                            ant1_cap_list[2].1,
                            ant1_cap_list[2].2,
                        ),
                        TuningCaps::from(9, 19, 15),
                    ],
                ),
                ChannelItem::from(
                    904250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[3].0,
                            ant1_cap_list[3].1,
                            ant1_cap_list[3].2,
                        ),
                        TuningCaps::from(9, 19, 15),
                    ],
                ),
                ChannelItem::from(
                    904750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[4].0,
                            ant1_cap_list[4].1,
                            ant1_cap_list[4].2,
                        ),
                        TuningCaps::from(9, 19, 15),
                    ],
                ),
                ChannelItem::from(
                    905250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[5].0,
                            ant1_cap_list[5].1,
                            ant1_cap_list[5].2,
                        ),
                        TuningCaps::from(9, 19, 15),
                    ],
                ),
                ChannelItem::from(
                    905750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[6].0,
                            ant1_cap_list[6].1,
                            ant1_cap_list[6].2,
                        ),
                        TuningCaps::from(10, 25, 15),
                    ],
                ),
                ChannelItem::from(
                    906250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[7].0,
                            ant1_cap_list[7].1,
                            ant1_cap_list[7].2,
                        ),
                        TuningCaps::from(10, 25, 15),
                    ],
                ),
                ChannelItem::from(
                    906750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[8].0,
                            ant1_cap_list[8].1,
                            ant1_cap_list[8].2,
                        ),
                        TuningCaps::from(9, 21, 15),
                    ],
                ),
                ChannelItem::from(
                    907250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[9].0,
                            ant1_cap_list[9].1,
                            ant1_cap_list[9].2,
                        ),
                        TuningCaps::from(9, 21, 15),
                    ],
                ),
                ChannelItem::from(
                    907750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[10].0,
                            ant1_cap_list[10].1,
                            ant1_cap_list[10].2,
                        ),
                        TuningCaps::from(9, 21, 15),
                    ],
                ),
                ChannelItem::from(
                    908250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[11].0,
                            ant1_cap_list[11].1,
                            ant1_cap_list[11].2,
                        ),
                        TuningCaps::from(9, 21, 15),
                    ],
                ),
                ChannelItem::from(
                    908750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[12].0,
                            ant1_cap_list[12].1,
                            ant1_cap_list[12].2,
                        ),
                        TuningCaps::from(9, 21, 15),
                    ],
                ),
                ChannelItem::from(
                    909250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[13].0,
                            ant1_cap_list[13].1,
                            ant1_cap_list[13].2,
                        ),
                        TuningCaps::from(9, 21, 15),
                    ],
                ),
                ChannelItem::from(
                    909750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[14].0,
                            ant1_cap_list[14].1,
                            ant1_cap_list[14].2,
                        ),
                        TuningCaps::from(9, 22, 15),
                    ],
                ),
                ChannelItem::from(
                    910250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[15].0,
                            ant1_cap_list[15].1,
                            ant1_cap_list[15].2,
                        ),
                        TuningCaps::from(9, 22, 15),
                    ],
                ),
                ChannelItem::from(
                    910750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[16].0,
                            ant1_cap_list[16].1,
                            ant1_cap_list[16].2,
                        ),
                        TuningCaps::from(9, 22, 15),
                    ],
                ),
                ChannelItem::from(
                    911250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[17].0,
                            ant1_cap_list[17].1,
                            ant1_cap_list[17].2,
                        ),
                        TuningCaps::from(9, 22, 15),
                    ],
                ),
                ChannelItem::from(
                    911750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[18].0,
                            ant1_cap_list[18].1,
                            ant1_cap_list[18].2,
                        ),
                        TuningCaps::from(9, 22, 15),
                    ],
                ),
                ChannelItem::from(
                    912250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[19].0,
                            ant1_cap_list[19].1,
                            ant1_cap_list[19].2,
                        ),
                        TuningCaps::from(9, 22, 15),
                    ],
                ),
                ChannelItem::from(
                    912750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[20].0,
                            ant1_cap_list[20].1,
                            ant1_cap_list[20].2,
                        ),
                        TuningCaps::from(9, 8, 13),
                    ],
                ),
                ChannelItem::from(
                    913250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[21].0,
                            ant1_cap_list[21].1,
                            ant1_cap_list[21].2,
                        ),
                        TuningCaps::from(9, 8, 13),
                    ],
                ),
                ChannelItem::from(
                    913750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[22].0,
                            ant1_cap_list[22].1,
                            ant1_cap_list[22].2,
                        ),
                        TuningCaps::from(9, 8, 13),
                    ],
                ),
                ChannelItem::from(
                    914250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[23].0,
                            ant1_cap_list[23].1,
                            ant1_cap_list[23].2,
                        ),
                        TuningCaps::from(9, 25, 15),
                    ],
                ),
                ChannelItem::from(
                    914750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[24].0,
                            ant1_cap_list[24].1,
                            ant1_cap_list[24].2,
                        ),
                        TuningCaps::from(9, 25, 15),
                    ],
                ),
                ChannelItem::from(
                    915250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[25].0,
                            ant1_cap_list[25].1,
                            ant1_cap_list[25].2,
                        ),
                        TuningCaps::from(9, 25, 15),
                    ],
                ),
                ChannelItem::from(
                    915750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[26].0,
                            ant1_cap_list[26].1,
                            ant1_cap_list[26].2,
                        ),
                        TuningCaps::from(9, 25, 15),
                    ],
                ),
                ChannelItem::from(
                    916250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[27].0,
                            ant1_cap_list[27].1,
                            ant1_cap_list[27].2,
                        ),
                        TuningCaps::from(9, 14, 14),
                    ],
                ),
                ChannelItem::from(
                    916750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[28].0,
                            ant1_cap_list[28].1,
                            ant1_cap_list[28].2,
                        ),
                        TuningCaps::from(9, 14, 14),
                    ],
                ),
                ChannelItem::from(
                    917250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[29].0,
                            ant1_cap_list[29].1,
                            ant1_cap_list[29].2,
                        ),
                        TuningCaps::from(9, 14, 14),
                    ],
                ),
                ChannelItem::from(
                    917750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[30].0,
                            ant1_cap_list[30].1,
                            ant1_cap_list[30].2,
                        ),
                        TuningCaps::from(9, 14, 14),
                    ],
                ),
                ChannelItem::from(
                    918250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[31].0,
                            ant1_cap_list[31].1,
                            ant1_cap_list[31].2,
                        ),
                        TuningCaps::from(9, 27, 15),
                    ],
                ),
                ChannelItem::from(
                    918750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[32].0,
                            ant1_cap_list[32].1,
                            ant1_cap_list[32].2,
                        ),
                        TuningCaps::from(9, 27, 15),
                    ],
                ),
                ChannelItem::from(
                    919250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[33].0,
                            ant1_cap_list[33].1,
                            ant1_cap_list[33].2,
                        ),
                        TuningCaps::from(9, 29, 15),
                    ],
                ),
                ChannelItem::from(
                    919750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[34].0,
                            ant1_cap_list[34].1,
                            ant1_cap_list[34].2,
                        ),
                        TuningCaps::from(9, 29, 15),
                    ],
                ),
                ChannelItem::from(
                    920250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[35].0,
                            ant1_cap_list[35].1,
                            ant1_cap_list[35].2,
                        ),
                        TuningCaps::from(9, 29, 15),
                    ],
                ),
                ChannelItem::from(
                    920750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[36].0,
                            ant1_cap_list[36].1,
                            ant1_cap_list[36].2,
                        ),
                        TuningCaps::from(9, 29, 15),
                    ],
                ),
                ChannelItem::from(
                    921250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[37].0,
                            ant1_cap_list[37].1,
                            ant1_cap_list[37].2,
                        ),
                        TuningCaps::from(9, 29, 15),
                    ],
                ),
                ChannelItem::from(
                    921750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[38].0,
                            ant1_cap_list[38].1,
                            ant1_cap_list[38].2,
                        ),
                        TuningCaps::from(9, 29, 15),
                    ],
                ),
                ChannelItem::from(
                    922250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[39].0,
                            ant1_cap_list[39].1,
                            ant1_cap_list[39].2,
                        ),
                        TuningCaps::from(9, 29, 15),
                    ],
                ),
                ChannelItem::from(
                    922750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[40].0,
                            ant1_cap_list[40].1,
                            ant1_cap_list[40].2,
                        ),
                        TuningCaps::from(9, 30, 15),
                    ],
                ),
                ChannelItem::from(
                    923250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[41].0,
                            ant1_cap_list[41].1,
                            ant1_cap_list[41].2,
                        ),
                        TuningCaps::from(9, 30, 15),
                    ],
                ),
                ChannelItem::from(
                    923750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[42].0,
                            ant1_cap_list[42].1,
                            ant1_cap_list[42].2,
                        ),
                        TuningCaps::from(9, 30, 15),
                    ],
                ),
                ChannelItem::from(
                    924250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[43].0,
                            ant1_cap_list[43].1,
                            ant1_cap_list[43].2,
                        ),
                        TuningCaps::from(9, 30, 15),
                    ],
                ),
                ChannelItem::from(
                    924750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[44].0,
                            ant1_cap_list[44].1,
                            ant1_cap_list[44].2,
                        ),
                        TuningCaps::from(9, 30, 15),
                    ],
                ),
                ChannelItem::from(
                    925250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[45].0,
                            ant1_cap_list[45].1,
                            ant1_cap_list[45].2,
                        ),
                        TuningCaps::from(9, 30, 15),
                    ],
                ),
                ChannelItem::from(
                    925750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[46].0,
                            ant1_cap_list[46].1,
                            ant1_cap_list[46].2,
                        ),
                        TuningCaps::from(9, 30, 15),
                    ],
                ),
                ChannelItem::from(
                    926250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[47].0,
                            ant1_cap_list[47].1,
                            ant1_cap_list[47].2,
                        ),
                        TuningCaps::from(9, 30, 15),
                    ],
                ),
                ChannelItem::from(
                    926750,
                    [
                        TuningCaps::from(
                            ant1_cap_list[48].0,
                            ant1_cap_list[48].1,
                            ant1_cap_list[48].2,
                        ),
                        TuningCaps::from(9, 30, 15),
                    ],
                ),
                ChannelItem::from(
                    927250,
                    [
                        TuningCaps::from(
                            ant1_cap_list[49].0,
                            ant1_cap_list[49].1,
                            ant1_cap_list[49].2,
                        ),
                        TuningCaps::from(9, 30, 15),
                    ],
                ),
            ]
        }
    }
}

/// Converts a list of channel items into the ffi version
pub(crate) fn item_list_to_ffi(
    item_list: &[ChannelItem],
) -> [ffi::STUHFL_T_ST25RU3993_ChannelItem; 53] {
    let mut ffi_list = [ffi::STUHFL_T_ST25RU3993_ChannelItem::default(); 53];

    for (i, x) in item_list.iter().enumerate() {
        ffi_list[i] = x.as_ffi();
    }

    ffi_list
}

/// Helps with error handling by converting the type into a proper rust result type
pub(crate) fn proc_err(code: ffi::STUHFL_T_RET_CODE) -> Result<()> {
    if code == ffi::STUHFL_ERR_NONE {
        Ok(())
    } else {
        Err(Error::from_u32(code).unwrap())
    }
}

/// Formates data into the EBV format
pub(crate) fn ebv_formatter(mut d_in: u32) -> Vec<u8> {
    let mut v = Vec::new();
    let mut first_run = true;

    // runs backwards
    loop {
        // save 7 bit chunk
        let chunk = (d_in & 0b0111_1111) as u8;

        // discard chunk from data
        d_in >>= 7;

        // first chunk has marker
        if first_run {
            v.push(chunk);
            first_run = false;
        } else {
            v.push(chunk | 0b1000_0000);
        }

        // stop when we encode all data
        if d_in == 0 {
            break;
        }
    }

    v.reverse();

    v
}

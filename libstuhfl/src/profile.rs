use super::*;

pub(crate) fn profile_to_item_list(profile: Profile) -> Vec<ChannelItem> {
    match profile {
        Profile::Europe => {
            vec![
                ChannelItem::from(865700, [TuningCaps::from(12, 12, 14), TuningCaps::from(12, 9, 16)]),
                ChannelItem::from(866300, [TuningCaps::from(12, 12, 14), TuningCaps::from(11, 9, 16)]),
                ChannelItem::from(866900, [TuningCaps::from(11, 12, 14), TuningCaps::from(11, 9, 16)]),
                ChannelItem::from(867500, [TuningCaps::from(11, 12, 14), TuningCaps::from(11, 9, 16)]),
            ]
        },
        Profile::Usa => {
            vec![
                ChannelItem::from(902750, [TuningCaps::from(13, 24, 12), TuningCaps::from(9, 19, 15)]),
                ChannelItem::from(903250, [TuningCaps::from(13, 24, 12), TuningCaps::from(9, 19, 15)]),
                ChannelItem::from(903750, [TuningCaps::from(13, 24, 12), TuningCaps::from(9, 19, 15)]),
                ChannelItem::from(904250, [TuningCaps::from(13, 24, 12), TuningCaps::from(9, 19, 15)]),
                ChannelItem::from(904750, [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 19, 15)]),
                ChannelItem::from(905250, [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 19, 15)]),
                ChannelItem::from(905750, [TuningCaps::from(9, 19, 14), TuningCaps::from(10, 25, 15)]),
                ChannelItem::from(906250, [TuningCaps::from(9, 8, 12), TuningCaps::from(10, 25, 15)]),
                ChannelItem::from(906750, [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 21, 15)]),
                ChannelItem::from(907250, [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 21, 15)]),
                ChannelItem::from(907750, [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 21, 15)]),
                ChannelItem::from(908250, [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 21, 15)]),
                ChannelItem::from(908750, [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 21, 15)]),
                ChannelItem::from(909250, [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 21, 15)]),
                ChannelItem::from(909750, [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 22, 15)]),
                ChannelItem::from(910250, [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 22, 15)]),
                ChannelItem::from(910750, [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 22, 15)]),
                ChannelItem::from(911250, [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 22, 15)]),
                ChannelItem::from(911750, [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 22, 15)]),
                ChannelItem::from(912250, [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 22, 15)]),
                ChannelItem::from(912750, [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 8, 13)]),
                ChannelItem::from(913250, [TuningCaps::from(9, 23, 14), TuningCaps::from(9, 8, 13)]),
                ChannelItem::from(913750, [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 8, 13)]),
                ChannelItem::from(914250, [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 25, 15)]),
                ChannelItem::from(914750, [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 25, 15)]),
                ChannelItem::from(915250, [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 25, 15)]),
                ChannelItem::from(915750, [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 25, 15)]),
                ChannelItem::from(916250, [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 14, 14)]),
                ChannelItem::from(916750, [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 14, 14)]),
                ChannelItem::from(917250, [TuningCaps::from(9, 8, 12), TuningCaps::from(9, 14, 14)]),
                ChannelItem::from(917750, [TuningCaps::from(9, 10, 12), TuningCaps::from(9, 14, 14)]),
                ChannelItem::from(918250, [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 27, 15)]),
                ChannelItem::from(918750, [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 27, 15)]),
                ChannelItem::from(919250, [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(919750, [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(920250, [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(920750, [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(921250, [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(921750, [TuningCaps::from(12, 24, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(922250, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(922750, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)]),
                ChannelItem::from(923250, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)]),
                ChannelItem::from(923750, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)]),
                ChannelItem::from(924250, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)]),
                ChannelItem::from(924750, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)]),
                ChannelItem::from(925250, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)]),
                ChannelItem::from(925750, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)]),
                ChannelItem::from(926250, [TuningCaps::from(8, 6, 12), TuningCaps::from(9, 30, 15)]),
                ChannelItem::from(926750, [TuningCaps::from(8, 6, 12), TuningCaps::from(9, 30, 15)]),
                ChannelItem::from(927250, [TuningCaps::from(8, 6, 12), TuningCaps::from(9, 30, 15)]),
            ]
        },
        Profile::Japan => {
            vec![
                ChannelItem::from(920500, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(920700, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(920900, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(921100, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(921300, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(921500, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(921700, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(921900, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(922100, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)]),
            ]
        },
        Profile::China => {
            vec![
                ChannelItem::from(840625, [TuningCaps::from(15, 21, 15), TuningCaps::from(13, 15, 18)]),
                ChannelItem::from(840875, [TuningCaps::from(15, 21, 15), TuningCaps::from(13, 15, 18)]),
                ChannelItem::from(841125, [TuningCaps::from(15, 21, 15), TuningCaps::from(13, 15, 18)]),
                ChannelItem::from(841375, [TuningCaps::from(15, 21, 15), TuningCaps::from(13, 15, 18)]),
                ChannelItem::from(841625, [TuningCaps::from(15, 21, 15), TuningCaps::from(13, 15, 18)]),
                ChannelItem::from(841875, [TuningCaps::from(15, 21, 15), TuningCaps::from(13, 15, 18)]),
                ChannelItem::from(842125, [TuningCaps::from(13, 14, 15), TuningCaps::from(13, 15, 18)]),
                ChannelItem::from(842375, [TuningCaps::from(13, 14, 15), TuningCaps::from(13, 15, 18)]),
                ChannelItem::from(842625, [TuningCaps::from(13, 14, 15), TuningCaps::from(16, 20, 17)]),
                ChannelItem::from(842875, [TuningCaps::from(13, 14, 15), TuningCaps::from(16, 20, 17)]),
                ChannelItem::from(843125, [TuningCaps::from(15, 23, 15), TuningCaps::from(16, 20, 17)]),
                ChannelItem::from(843375, [TuningCaps::from(15, 23, 15), TuningCaps::from(16, 20, 17)]),
                ChannelItem::from(843625, [TuningCaps::from(15, 23, 15), TuningCaps::from(16, 20, 17)]),
                ChannelItem::from(843875, [TuningCaps::from(15, 23, 15), TuningCaps::from(16, 20, 17)]),
                ChannelItem::from(844125, [TuningCaps::from(15, 23, 15), TuningCaps::from(16, 20, 17)]),
                ChannelItem::from(844375, [TuningCaps::from(15, 23, 15), TuningCaps::from(16, 20, 17)]),
            ]
        },
        Profile::China2 => {
            vec![
                ChannelItem::from(920500, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(920700, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(920900, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(921100, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(921300, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(921500, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(921700, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(921900, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 29, 15)]),
                ChannelItem::from(922100, [TuningCaps::from(10, 16, 12), TuningCaps::from(9, 30, 15)]),
            ]
        },
    }
}

pub(crate) fn item_list_to_ffi(item_list: &[ChannelItem]) -> [ffi::STUHFL_T_ST25RU3993_ChannelItem; 53] {
    let mut ffi_list = [ffi::STUHFL_T_ST25RU3993_ChannelItem::default(); 53];

    for (i, x) in item_list.iter().enumerate() {
        ffi_list[i] = x.as_ffi();
    }

    ffi_list
}

use super::*;

impl Default for STUHFL_T_ST25RU3993_Register {
    fn default() -> Self {
        Self { addr: 0, data: 0 }
    }
}

impl Default for STUHFL_T_ST25RU3993_RwdConfig {
    fn default() -> Self {
        Self {
            id: STUHFL_D_RWD_CFG_ID_POWER_DOWN_MODE as u8,
            value: STUHFL_D_POWER_NORMAL as u8,
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_RxFilter {
    fn default() -> Self {
        Self {
            blf: STUHFL_D_GEN2_BLF_256 as u8,
            coding: STUHFL_D_GEN2_CODING_MILLER8 as u8,
            value: 0x34,
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_FilterCalibration {
    fn default() -> Self {
        Self {
            blf: STUHFL_D_GEN2_BLF_256 as u8,
            coding: STUHFL_D_GEN2_CODING_MILLER8 as u8,
            highPass: 0x08,
            lowPass: 0x08,
        }
    }
}

pub const STUHFL_D_ANTENNA_POWER_MODE_ON: u8 = 0x00;
pub const STUHFL_D_ANTENNA_POWER_MODE_OFF: u8 = 0xFF;
impl Default for STUHFL_T_ST25RU3993_AntennaPower {
    fn default() -> Self {
        Self {
            mode: STUHFL_D_ANTENNA_POWER_MODE_OFF as u8,
            timeout: 0,
            frequency: STUHFL_D_DEFAULT_FREQUENCY as u32,
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_TxRxCfg {
    fn default() -> Self {
        Self {
            txOutputLevel: -2,
            rxSensitivity: 3,
            usedAntenna: STUHFL_D_ANTENNA_1 as u8,
            alternateAntennaInterval: 1,
            rfu: 3,
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_Gen2_InventoryOption {
    fn default() -> Self {
        Self {
            fast: true,
            autoAck: false,
            readTID: false,
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_Gen2_Anticollision {
    fn default() -> Self {
        Self {
            adaptiveQ: true,
            startQ: 6,
            minQ: 2,
            maxQ: STUHFL_D_GEN2_MAXQ as u8,
            options: 0,
            C1: [5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
            C2: [
                35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
            ],
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_AutoTuning {
    fn default() -> Self {
        Self {
            interval: 7,
            level: 20,
            algorithm: STUHFL_D_TUNING_ALGO_FAST as u8,
            falsePositiveDetection: true,
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_Gen2_QueryParams {
    fn default() -> Self {
        Self {
            sel: 0,
            session: STUHFL_D_GEN2_SESSION_S0 as u8,
            target: STUHFL_D_GEN2_TARGET_A as u8,
            toggleTarget: true,
            targetDepletionMode: false,
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_AdaptiveSensitivity {
    fn default() -> Self {
        Self {
            adaptiveRx: true,
            startRx: 3,
            minRx: -17,
            maxRx: 19,
            retuneWhenAdjust: false,
            ivTag: 0,
            ivEmpty: 8,
            ivColllision: -1,
            ivPreample: -7,
            ivCrc: 0,
            ivHeader: 0,
            ivRxCount: -7,
            ivStopBit: 0,
            ivResendAck: 0,
            ivNoiseSuspicion: 0,
            decThreshold: [-120; 20],
            incThreshold: [100; 20],
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_AdaptiveOutputPower {
    fn default() -> Self {
        Self {
            adaptiveTx: false,
            startTx: -2,
            minTx: 0,
            maxTx: -19,
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_Gen2_InventoryCfg {
    fn default() -> Self {
        Self {
            inventoryOption: STUHFL_T_ST25RU3993_Gen2_InventoryOption::default(),
            antiCollision: STUHFL_T_ST25RU3993_Gen2_Anticollision::default(),
            autoTuning: STUHFL_T_ST25RU3993_AutoTuning::default(),
            queryParams: STUHFL_T_ST25RU3993_Gen2_QueryParams::default(),
            adaptiveSensitivity: STUHFL_T_ST25RU3993_AdaptiveSensitivity::default(),
            adaptiveOutputPower: STUHFL_T_ST25RU3993_AdaptiveOutputPower::default(),
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_Gen2_ProtocolCfg {
    fn default() -> Self {
        Self {
            tari: STUHFL_D_GEN2_TARI_25_00 as u8,
            blf: STUHFL_D_GEN2_BLF_256 as u8,
            coding: STUHFL_D_GEN2_CODING_MILLER8 as u8,
            trext: STUHFL_D_TREXT_ON != 0,
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_FreqLBT {
    fn default() -> Self {
        Self {
            listeningTime: 1,
            idleTime: 0,
            rssiLogThreshold: 31,
            skipLBTcheck: true,
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_Caps {
    fn default() -> Self {
        Self {
            cin: STUHFL_D_DEFAULT_CAP as u8,
            clen: STUHFL_D_DEFAULT_CAP as u8,
            cout: STUHFL_D_DEFAULT_CAP as u8,
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_ChannelItem {
    fn default() -> Self {
        Self {
            frequency: STUHFL_D_DEFAULT_FREQUENCY,
            caps: [STUHFL_T_ST25RU3993_Caps::default(); 2],
            rfu1: 0,
            rfu2: 0,
        }
    }
}

impl STUHFL_T_ST25RU3993_ChannelItem {
    #[allow(clippy::type_complexity)]
    fn from_raw(raw: (u32, ((u8, u8, u8), (u8, u8, u8)), u8, u8)) -> Self {
        Self {
            frequency: raw.0,
            caps: [
                STUHFL_T_ST25RU3993_Caps {
                    cin: raw.1 .0 .0,
                    clen: raw.1 .0 .1,
                    cout: raw.1 .0 .2,
                },
                STUHFL_T_ST25RU3993_Caps {
                    cin: raw.1 .1 .0,
                    clen: raw.1 .1 .1,
                    cout: raw.1 .1 .2,
                },
            ],
            rfu1: raw.2,
            rfu2: raw.3,
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_ChannelList {
    fn default() -> Self {
        Self {
            persistent: false,
            numFrequencies: 1,
            channelListIdx: 0,
            itemList: [STUHFL_T_ST25RU3993_ChannelItem::default(); 53],
        }
    }
}

impl STUHFL_T_ST25RU3993_ChannelList {
    pub fn from_profile(profile: u8) -> Self {
        let mut list = Self::default();

        match profile as u32 {
            STUHFL_D_PROFILE_EUROPE => {
                list.numFrequencies = 4;
                list.itemList[0] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    865700,
                    ((12, 12, 14), (12, 9, 16)),
                    0,
                    0,
                ));
                list.itemList[1] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    866300,
                    ((12, 12, 14), (11, 9, 16)),
                    0,
                    0,
                ));
                list.itemList[2] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    866900,
                    ((11, 12, 14), (11, 9, 16)),
                    0,
                    0,
                ));
                list.itemList[3] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    867500,
                    ((11, 12, 14), (11, 9, 16)),
                    0,
                    0,
                ));
            }
            STUHFL_D_PROFILE_USA => {
                list.numFrequencies = 50;
                list.itemList[0] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    902750,
                    ((13, 24, 12), (9, 19, 15)),
                    0,
                    0,
                ));
                list.itemList[1] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    903250,
                    ((13, 24, 12), (9, 19, 15)),
                    0,
                    0,
                ));
                list.itemList[2] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    903750,
                    ((13, 24, 12), (9, 19, 15)),
                    0,
                    0,
                ));
                list.itemList[3] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    904250,
                    ((13, 24, 12), (9, 19, 15)),
                    0,
                    0,
                ));
                list.itemList[4] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    904750,
                    ((9, 8, 12), (9, 19, 15)),
                    0,
                    0,
                ));
                list.itemList[5] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    905250,
                    ((9, 8, 12), (9, 19, 15)),
                    0,
                    0,
                ));
                list.itemList[6] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    905750,
                    ((9, 19, 14), (10, 25, 15)),
                    0,
                    0,
                ));
                list.itemList[7] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    906250,
                    ((9, 8, 12), (10, 25, 15)),
                    0,
                    0,
                ));
                list.itemList[8] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    906750,
                    ((9, 8, 12), (9, 21, 15)),
                    0,
                    0,
                ));
                list.itemList[9] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    907250,
                    ((9, 8, 12), (9, 21, 15)),
                    0,
                    0,
                ));
                list.itemList[10] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    907750,
                    ((9, 8, 12), (9, 21, 15)),
                    0,
                    0,
                ));
                list.itemList[11] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    908250,
                    ((9, 8, 12), (9, 21, 15)),
                    0,
                    0,
                ));
                list.itemList[12] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    908750,
                    ((9, 23, 14), (9, 21, 15)),
                    0,
                    0,
                ));
                list.itemList[13] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    909250,
                    ((9, 23, 14), (9, 21, 15)),
                    0,
                    0,
                ));
                list.itemList[14] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    909750,
                    ((9, 23, 14), (9, 22, 15)),
                    0,
                    0,
                ));
                list.itemList[15] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    910250,
                    ((9, 23, 14), (9, 22, 15)),
                    0,
                    0,
                ));
                list.itemList[16] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    910750,
                    ((9, 23, 14), (9, 22, 15)),
                    0,
                    0,
                ));
                list.itemList[17] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    911250,
                    ((9, 23, 14), (9, 22, 15)),
                    0,
                    0,
                ));
                list.itemList[18] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    911750,
                    ((9, 23, 14), (9, 22, 15)),
                    0,
                    0,
                ));
                list.itemList[19] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    912250,
                    ((9, 23, 14), (9, 22, 15)),
                    0,
                    0,
                ));
                list.itemList[20] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    912750,
                    ((9, 23, 14), (9, 8, 13)),
                    0,
                    0,
                ));
                list.itemList[21] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    913250,
                    ((9, 23, 14), (9, 8, 13)),
                    0,
                    0,
                ));
                list.itemList[22] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    913750,
                    ((9, 8, 12), (9, 8, 13)),
                    0,
                    0,
                ));
                list.itemList[23] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    914250,
                    ((9, 8, 12), (9, 25, 15)),
                    0,
                    0,
                ));
                list.itemList[24] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    914750,
                    ((9, 8, 12), (9, 25, 15)),
                    0,
                    0,
                ));
                list.itemList[25] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    915250,
                    ((9, 8, 12), (9, 25, 15)),
                    0,
                    0,
                ));
                list.itemList[26] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    915750,
                    ((9, 8, 12), (9, 25, 15)),
                    0,
                    0,
                ));
                list.itemList[27] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    916250,
                    ((9, 8, 12), (9, 14, 14)),
                    0,
                    0,
                ));
                list.itemList[28] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    916750,
                    ((9, 8, 12), (9, 14, 14)),
                    0,
                    0,
                ));
                list.itemList[29] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    917250,
                    ((9, 8, 12), (9, 14, 14)),
                    0,
                    0,
                ));
                list.itemList[30] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    917750,
                    ((9, 10, 12), (9, 14, 14)),
                    0,
                    0,
                ));
                list.itemList[31] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    918250,
                    ((12, 24, 12), (9, 27, 15)),
                    0,
                    0,
                ));
                list.itemList[32] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    918750,
                    ((12, 24, 12), (9, 27, 15)),
                    0,
                    0,
                ));
                list.itemList[33] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    919250,
                    ((12, 24, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[34] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    919750,
                    ((12, 24, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[35] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    920250,
                    ((12, 24, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[36] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    920750,
                    ((12, 24, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[37] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    921250,
                    ((12, 24, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[38] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    921750,
                    ((12, 24, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[39] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    922250,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[40] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    922750,
                    ((10, 16, 12), (9, 30, 15)),
                    0,
                    0,
                ));
                list.itemList[41] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    923250,
                    ((10, 16, 12), (9, 30, 15)),
                    0,
                    0,
                ));
                list.itemList[42] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    923750,
                    ((10, 16, 12), (9, 30, 15)),
                    0,
                    0,
                ));
                list.itemList[43] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    924250,
                    ((10, 16, 12), (9, 30, 15)),
                    0,
                    0,
                ));
                list.itemList[44] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    924750,
                    ((10, 16, 12), (9, 30, 15)),
                    0,
                    0,
                ));
                list.itemList[45] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    925250,
                    ((10, 16, 12), (9, 30, 15)),
                    0,
                    0,
                ));
                list.itemList[46] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    925750,
                    ((10, 16, 12), (9, 30, 15)),
                    0,
                    0,
                ));
                list.itemList[47] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    926250,
                    ((8, 6, 12), (9, 30, 15)),
                    0,
                    0,
                ));
                list.itemList[48] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    926750,
                    ((8, 6, 12), (9, 30, 15)),
                    0,
                    0,
                ));
                list.itemList[49] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    927250,
                    ((8, 6, 12), (9, 30, 15)),
                    0,
                    0,
                ));
            }
            STUHFL_D_PROFILE_JAPAN => {
                list.numFrequencies = 9;
                list.itemList[0] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    920500,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[1] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    920700,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[2] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    920900,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[3] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    921100,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[4] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    921300,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[5] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    921500,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[6] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    921700,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[7] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    921900,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[8] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    922100,
                    ((10, 16, 12), (9, 30, 15)),
                    0,
                    0,
                ));
            }
            STUHFL_D_PROFILE_CHINA => {
                list.numFrequencies = 16;
                list.itemList[0] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    840625,
                    ((15, 21, 15), (13, 15, 18)),
                    0,
                    0,
                ));
                list.itemList[1] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    840875,
                    ((15, 21, 15), (13, 15, 18)),
                    0,
                    0,
                ));
                list.itemList[2] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    841125,
                    ((15, 21, 15), (13, 15, 18)),
                    0,
                    0,
                ));
                list.itemList[3] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    841375,
                    ((15, 21, 15), (13, 15, 18)),
                    0,
                    0,
                ));
                list.itemList[4] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    841625,
                    ((15, 21, 15), (13, 15, 18)),
                    0,
                    0,
                ));
                list.itemList[5] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    841875,
                    ((15, 21, 15), (13, 15, 18)),
                    0,
                    0,
                ));
                list.itemList[6] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    842125,
                    ((13, 14, 15), (13, 15, 18)),
                    0,
                    0,
                ));
                list.itemList[7] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    842375,
                    ((13, 14, 15), (13, 15, 18)),
                    0,
                    0,
                ));
                list.itemList[8] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    842625,
                    ((13, 14, 15), (16, 20, 17)),
                    0,
                    0,
                ));
                list.itemList[9] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    842875,
                    ((13, 14, 15), (16, 20, 17)),
                    0,
                    0,
                ));
                list.itemList[10] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    843125,
                    ((15, 23, 15), (16, 20, 17)),
                    0,
                    0,
                ));
                list.itemList[11] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    843375,
                    ((15, 23, 15), (16, 20, 17)),
                    0,
                    0,
                ));
                list.itemList[12] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    843625,
                    ((15, 23, 15), (16, 20, 17)),
                    0,
                    0,
                ));
                list.itemList[13] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    843875,
                    ((15, 23, 15), (16, 20, 17)),
                    0,
                    0,
                ));
                list.itemList[14] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    844125,
                    ((15, 23, 15), (16, 20, 17)),
                    0,
                    0,
                ));
                list.itemList[15] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    844375,
                    ((15, 23, 15), (16, 20, 17)),
                    0,
                    0,
                ));
            }
            STUHFL_D_PROFILE_CHINA2 => {
                list.numFrequencies = 16;
                list.itemList[0] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    920500,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[1] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    920700,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[2] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    920900,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[3] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    921100,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[4] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    921300,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[5] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    921500,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[6] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    921700,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[7] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    921900,
                    ((10, 16, 12), (9, 29, 15)),
                    0,
                    0,
                ));
                list.itemList[8] = STUHFL_T_ST25RU3993_ChannelItem::from_raw((
                    922100,
                    ((10, 16, 12), (9, 30, 15)),
                    0,
                    0,
                ));
            }
            _ => panic!("Error: Invalid profile attempted to be initialized"),
        }

        list
    }
}

impl Default for STUHFL_T_ST25RU3993_FreqHop {
    fn default() -> Self {
        Self {
            maxSendingTime: 400,
            minSendingTime: 400,
            mode: STUHFL_D_FREQUENCY_HOP_MODE_IGNORE_MIN as u8,
            rfu: 0,
        }
    }
}

impl Default for STUHFL_T_Gen2_Select {
    fn default() -> Self {
        Self {
            mode: STUHFL_D_GEN2_SELECT_MODE_CLEAR_LIST as u8,
            target: STUHFL_D_GEN2_TARGET_S0 as u8,
            action: 0,
            memoryBank: STUHFL_D_GEN2_MEMORY_BANK_EPC as u8,
            mask: [0; 32],
            maskBitPointer: 0,
            maskBitLength: 0,
            truncation: false as u8,
        }
    }
}

impl Default for STUHFL_T_ST25RU3993_TuneCfg {
    fn default() -> Self {
        Self {
            falsePositiveDetection: true,
            persistent: false,
            channelListIdx: 0,
            antenna: STUHFL_D_ANTENNA_1 as u8,
            algorithm: STUHFL_D_TUNING_ALGO_GROUPED_EXACT as u8,
            tuneAll: false,
        }
    }
}

impl Default for STUHFL_T_InventoryStatistics {
    fn default() -> Self {
        Self {
            timestamp: 0,
            roundCnt: 0,
            tuningStatus: STUHFL_D_TUNING_STATUS_UNTUNED as u8,
            rssiLogMean: 0,
            sensitivity: 0,
            Q: 0,
            frequency: 0,
            adc: 0,
            tagCnt: 0,
            emptySlotCnt: 0,
            slotCnt: 0,
            collisionCnt: 0,
            preambleErrCnt: 0,
            crcErrCnt: 0,
            headerErrCnt: 0,
            rxCountErrCnt: 0,
            resendAckCnt: 0,
            noiseSuspicionCnt: 0,
        }
    }
}

impl Default for STUHFL_T_InventoryData {
    fn default() -> Self {
        unsafe {
            Self {
                statistics: STUHFL_T_InventoryStatistics::default(),
                tagList: std::mem::zeroed(),
                tagListSize: 0,
                tagListSizeMax: 0,
            }
        }
    }
}

impl Default for STUHFL_T_InventoryOption {
    fn default() -> Self {
        Self {
            rssiMode: STUHFL_D_RSSI_MODE_2NDBYTE as u8,
            roundCnt: 0,
            inventoryDelay: 0,
            options: 0x00,
        }
    }
}

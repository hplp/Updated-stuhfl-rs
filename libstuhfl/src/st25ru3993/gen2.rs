use super::*;

pub fn setup_gen2_config(reader: &mut ST25RU3993, single_tag: bool, freq_hopping: bool, antenna: Antenna) -> Result<(), Error> {
    // Set up tx_rx_cfg to default firmware values
    let mut tx_rx_cfg = ffi::STUHFL_T_ST25RU3993_TxRxCfg::default();
    tx_rx_cfg.usedAntenna = antenna as u8;

    // Set up inv_gen2_cfg to default firmware values
    let mut inv_gen2_cfg = ffi::STUHFL_T_ST25RU3993_Gen2_InventoryCfg::default();
    inv_gen2_cfg.inventoryOption.fast = true;
    inv_gen2_cfg.inventoryOption.autoAck = false;
    inv_gen2_cfg.antiCollision.startQ = if single_tag {0} else {4};
    inv_gen2_cfg.antiCollision.adaptiveQ = !single_tag;
    inv_gen2_cfg.queryParams.toggleTarget = true;
    inv_gen2_cfg.queryParams.targetDepletionMode = true;
    inv_gen2_cfg.adaptiveSensitivity.adaptiveRx = false;
    inv_gen2_cfg.adaptiveOutputPower.adaptiveTx = false;

    // Set up gen2_protocol_cfg
    let mut gen2_protocol_cfg = ffi::STUHFL_T_ST25RU3993_Gen2_ProtocolCfg::default();
    // gen2_protocol_cfg.blf = ffi::STUHFL_D_GEN2_BLF_320 as u8;
    // gen2_protocol_cfg.coding = ffi::STUHFL_D_GEN2_CODING_MILLER2 as u8;
    // gen2_protocol_cfg.tari = ffi::STUHFL_D_GEN2_TARI_6_25 as u8;
    // gen2_protocol_cfg.trext = ffi::STUHFL_D_TREXT_ON != 0;

    // Set up freq_lbt
    let mut freq_lbt = ffi::STUHFL_T_ST25RU3993_FreqLBT::default();
    freq_lbt.listeningTime = 0;

    // Set up channel_list
    let mut channel_list = ffi::STUHFL_T_ST25RU3993_ChannelList::default();
    channel_list.persistent = false;
    channel_list.channelListIdx = 0;
    if freq_hopping {
        channel_list = ffi::STUHFL_T_ST25RU3993_ChannelList::from_profile(ffi::STUHFL_D_PROFILE_EUROPE as u8);
    } else {
        channel_list.numFrequencies = 1;
        channel_list.itemList[0].frequency = ffi::STUHFL_D_DEFAULT_FREQUENCY;
    }

    // Set up freq_hop
    let mut freq_hop = ffi::STUHFL_T_ST25RU3993_FreqHop::default();

    // Set up gen2_select
    let mut gen2_select = ffi::STUHFL_T_Gen2_Select::default();
    gen2_select.mode = ffi::STUHFL_D_GEN2_SELECT_MODE_CLEAR_LIST as u8;

    unsafe {
        // Apply reader settings and check for errors along the way
        proc_err(ffi::Set_TxRxCfg(&mut tx_rx_cfg))?;
        proc_err(ffi::Set_Gen2_InventoryCfg(&mut inv_gen2_cfg))?;
        proc_err(ffi::Set_Gen2_ProtocolCfg(&mut gen2_protocol_cfg))?;
        proc_err(ffi::Set_FreqLBT(&mut freq_lbt))?;
        proc_err(ffi::Set_ChannelList(&mut channel_list))?;
        proc_err(ffi::Set_FreqHop(&mut freq_hop))?;
        proc_err(ffi::Gen2_Select(&mut gen2_select))?;

        // tune the reader frequencies
        match reader.tune_freqs(TuningAlgorithm::Exact) {
            Ok(_) => {},
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
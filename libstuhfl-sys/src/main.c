// "DemoInteractive" recreation attempt
// Nathan Rowan

#include "stuhfl_evalAPI.h"

void printTagList(STUHFL_T_InventoryOption *invOption, STUHFL_T_InventoryData *invData);
void setupGen2Config(bool singleTag, bool freqHopping, int antenna);
static void tuneFreqs(uint8_t tuningAlgo);

int main()
{
    char port[] = "/dev/ttyUSB0";

    STUHFL_T_RET_CODE ret = Connect(port);

    usleep(600000);

    STUHFL_T_Version swVer = STUHFL_O_VERSION_INIT();
    STUHFL_T_Version hwVer = STUHFL_O_VERSION_INIT();
    STUHFL_T_VersionInfo swInfo = STUHFL_O_VERSION_INFO_INIT();
    STUHFL_T_VersionInfo hwInfo = STUHFL_O_VERSION_INFO_INIT();

    ret |= Get_BoardVersion(&swVer, &hwVer);
    ret |= Get_BoardInfo(&swInfo, &hwInfo);

    printf("\n-------------------------------------------------------\nSW: V%d.%d.%d.%d, %s\nHW: V%d.%d.%d.%d, %s\n-------------------------------------------------------\n\n",
           swVer.major, swVer.minor, swVer.micro, swVer.nano, swInfo.info,
           hwVer.major, hwVer.minor, hwVer.micro, hwVer.nano, hwInfo.info);

    bool exit = false;
    char cmd;
    char input[64];

    while (!exit)
    {
        printf("Choose an action:\n");
        printf("\t1) Inventory Gen2 Tags (1 round)\n");
        printf("\tq) Quit\n");

        scanf("%63s", input);
        cmd = input[0];

        switch (cmd)
        {
        case 'q':
        case 'Q':
            printf("Exiting...\n");
            ret |= Disconnect();
            exit = true;
            break;
        case '1':
            printf("Running Inventory...\n");

            setupGen2Config(false, true, STUHFL_D_ANTENNA_1);

            // apply data storage location, where the found TAGs shall be stored
            STUHFL_T_InventoryTag tagData[STUHFL_D_MAX_TAG_LIST_SIZE];

            // Set inventory data and print all found tags
            STUHFL_T_InventoryData invData = STUHFL_O_INVENTORY_DATA_INIT();
            invData.tagList = tagData;
            invData.tagListSizeMax = STUHFL_D_MAX_TAG_LIST_SIZE;

            STUHFL_T_InventoryOption invOption = STUHFL_O_INVENTORY_OPTION_INIT(); // Init with default values

            Gen2_Inventory(&invOption, &invData);

            printTagList(&invOption, &invData);
            break;
        }
    }
}

void setupGen2Config(bool singleTag, bool freqHopping, int antenna)
{
    STUHFL_T_ST25RU3993_TxRxCfg TxRxCfg = STUHFL_O_ST25RU3993_TXRX_CFG_INIT(); // Set to FW default values
    TxRxCfg.usedAntenna = (uint8_t)antenna;
    Set_TxRxCfg(&TxRxCfg);

    STUHFL_T_ST25RU3993_Gen2_InventoryCfg invGen2Cfg = STUHFL_O_ST25RU3993_GEN2_INVENTORY_CFG_INIT(); // Set to FW default values
    invGen2Cfg.inventoryOption.fast = true;
    invGen2Cfg.inventoryOption.autoAck = false;
    invGen2Cfg.antiCollision.startQ = singleTag ? 0 : 4;
    invGen2Cfg.antiCollision.adaptiveQ = !singleTag;
    invGen2Cfg.queryParams.toggleTarget = true;
    invGen2Cfg.queryParams.targetDepletionMode = true;
    invGen2Cfg.adaptiveSensitivity.adaptiveRx = false;
    invGen2Cfg.adaptiveOutputPower.adaptiveTx = false;
    Set_Gen2_InventoryCfg(&invGen2Cfg);

    //
    STUHFL_T_ST25RU3993_Gen2_ProtocolCfg gen2ProtocolCfg = STUHFL_O_ST25RU3993_GEN2_PROTOCOL_CFG_INIT(); // Set to FW default values
    Set_Gen2_ProtocolCfg(&gen2ProtocolCfg);

    STUHFL_T_ST25RU3993_FreqLBT freqLBT = STUHFL_O_ST25RU3993_FREQ_LBT_INIT(); // Set to FW default values
    freqLBT.listeningTime = 0;
    Set_FreqLBT(&freqLBT);

    STUHFL_T_ST25RU3993_ChannelList channelList = STUHFL_O_ST25RU3993_CHANNEL_LIST_INIT();
    channelList.persistent = false;
    channelList.channelListIdx = 0;
    if (freqHopping)
    {
        channelList = (STUHFL_T_ST25RU3993_ChannelList)STUHFL_O_ST25RU3993_CHANNEL_LIST_EUROPE_INIT();
    }
    else
    {
        channelList.numFrequencies = 1;
        channelList.itemList[0].frequency = STUHFL_D_DEFAULT_FREQUENCY;
    }
    Set_ChannelList(&channelList); // Nota: Profile is implicitely switched to STUHFL_D_PROFILE_NEWTUNING

    STUHFL_T_ST25RU3993_FreqHop freqHop = STUHFL_O_ST25RU3993_FREQ_HOP_INIT(); // Set to FW default values
    Set_FreqHop(&freqHop);

    STUHFL_T_Gen2_Select Gen2Select = STUHFL_O_GEN2_SELECT_INIT(); // Set to FW default values
    Gen2Select.mode = STUHFL_D_GEN2_SELECT_MODE_CLEAR_LIST;        // Clear all Select filters
    Gen2_Select(&Gen2Select);

    printf("Tuning Profile frequencies: algo: STUHFL_D_TUNING_ALGO_EXACT\n");
    tuneFreqs(STUHFL_D_TUNING_ALGO_EXACT);
}

static void tuneFreqs(uint8_t tuningAlgo)
{
    if (tuningAlgo == STUHFL_D_TUNING_ALGO_NONE)
    {
        return;
    }

    STUHFL_T_ST25RU3993_TxRxCfg txRxCfg = STUHFL_O_ST25RU3993_TXRX_CFG_INIT();

    // Get current antenna
    Get_TxRxCfg(&txRxCfg);

    STUHFL_T_ST25RU3993_TuneCfg tuneCfg = STUHFL_O_ST25RU3993_TUNE_CFG_INIT();
    tuneCfg.antenna = txRxCfg.usedAntenna;
    tuneCfg.algorithm = tuningAlgo;
    tuneCfg.tuneAll = true;
    TuneChannel(&tuneCfg);
}

void printTagList(STUHFL_T_InventoryOption *invOption, STUHFL_T_InventoryData *invData)
{
    //
    printf("\n\n--- Inventory Option ---\n");
    printf("rssiMode    : %d\n", invOption->rssiMode);
    printf("reportMode  : %d\n", invOption->options);
    printf("\n");

    printf("--- Round Info ---\n");
    printf("tuningStatus: %s\n", invData->statistics.tuningStatus == STUHFL_D_TUNING_STATUS_UNTUNED ? "UNTUNED" : (invData->statistics.tuningStatus == STUHFL_D_TUNING_STATUS_TUNING ? "TUNING" : "TUNED"));
    printf("roundCnt    : %d\n", invData->statistics.roundCnt);
    printf("sensitivity : %d\n", invData->statistics.sensitivity);
    printf("Q           : %d\n", invData->statistics.Q);
    printf("adc         : %d\n", invData->statistics.adc);
    printf("frequency   : %d\n", invData->statistics.frequency);
    printf("tagCnt      : %d\n", invData->statistics.tagCnt);
    printf("empty Slots : %d\n", invData->statistics.emptySlotCnt);
    printf("collisions  : %d\n", invData->statistics.collisionCnt);
    printf("preampleErr : %d\n", invData->statistics.preambleErrCnt);
    printf("crcErr      : %d\n\n", invData->statistics.crcErrCnt);

    // print transponder information for TagList
    for (int tagIdx = 0; tagIdx < invData->tagListSize; tagIdx++)
    {
        printf("\n\n--- %03d ---\n", tagIdx + 1);
        printf("agc         : %d\n", invData->tagList[tagIdx].agc);
        printf("rssiLogI    : %d\n", invData->tagList[tagIdx].rssiLogI);
        printf("rssiLogQ    : %d\n", invData->tagList[tagIdx].rssiLogQ);
        printf("rssiLinI    : %d\n", invData->tagList[tagIdx].rssiLinI);
        printf("rssiLinQ    : %d\n", invData->tagList[tagIdx].rssiLinQ);
        printf("pc          : ");
        for (int i = 0; i < STUHFL_D_MAX_PC_LENGTH; i++)
        {
            printf("%02x ", invData->tagList[tagIdx].pc[i]);
        }
        printf("\nepcLen      : %d\n", invData->tagList[tagIdx].epc.length);
        printf("epc         : ");
        for (int i = 0; i < invData->tagList[tagIdx].epc.length; i++)
        {
            printf("%02x ", invData->tagList[tagIdx].epc.data[i]);
        }
        printf("\ntidLen      : %d\n", invData->tagList[tagIdx].tid.length);
        printf("tid         : ");
        for (int i = 0; i < invData->tagList[tagIdx].tid.length; i++)
        {
            printf("%02x ", invData->tagList[tagIdx].tid.data[i]);
        }
    }
    printf("\n");
}
/******************************************************************************
  * \attention
  *
  * <h2><center>&copy; COPYRIGHT(c) 2022 STMicroelectronics</center></h2>
  *
  * Licensed under ST MYLIBERTY SOFTWARE LICENSE AGREEMENT (the "License");
  * You may not use this file except in compliance with the License.
  * You may obtain a copy of the License at:
  *
  *        www.st.com/myliberty
  *
  * Unless required by applicable law or agreed to in writing, software 
  * distributed under the License is distributed on an "AS IS" BASIS, 
  * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied,
  * AND SPECIFICALLY DISCLAIMING THE IMPLIED WARRANTIES OF MERCHANTABILITY,
  * FITNESS FOR A PARTICULAR PURPOSE, AND NON-INFRINGEMENT.
  * See the License for the specific language governing permissions and
  * limitations under the License.
  *
******************************************************************************/


//
#if !defined __STUHFL_EVALAPI_H
#define __STUHFL_EVALAPI_H

#include "stuhfl.h"
#include "stuhfl_al.h"
#include "stuhfl_sl.h"
#include "stuhfl_sl_gen2.h"
#include "stuhfl_sl_gb29768.h"
#include "stuhfl_sl_iso6b.h"
#include "stuhfl_dl_ST25RU3993.h"

//
#ifdef __cplusplus
extern "C"
{
#endif //__cplusplus

// --------------------------------------------------------------------------
// General
// --------------------------------------------------------------------------
/**
    * Connect to ST25RU3993 based EVAL board.
    * @param szComPort: The port name were the ST25RU3993 board is connected.
    + The string must be null terminated.
    *
    * @return error code
    */
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Connect(char *szComPort);
/**
    * Disconnect from a current connected board
    * @param none
    *
    * @return error code
    */
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Disconnect(void);

/**
    * Read the board software and hardware information\n
    * @param swVersion: See STUHFL_T_Version struct for further info
    * @param hwVersion: See STUHFL_T_Version struct for further info
    *
    * @return error code
    */
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_BoardVersion(STUHFL_T_Version *swVersion, STUHFL_T_Version *hwVersion);
/**
    * Get human readable software and hardware device information\n
    * @param swInfo: See STUHFL_T_VersionInfo struct for further info
    * @param hwInfo: See STUHFL_T_VersionInfo struct for further info
    *
    * @return error code
    */
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_BoardInfo(STUHFL_T_VersionInfo *swInfo, STUHFL_T_VersionInfo *hwInfo);

/**
    * Reboot FW\n
    * NOTE: This function will never return, as the reboot is triggered immediately
    *
    * @return: never returns
    */
STUHFL_DLL_API void CALL_CONV Reboot(void);

/**
    * Shutdown FW and enter STM32 ROM bootloader of EVAL board.\n
    *
    * @return: never returns
    */
STUHFL_DLL_API void CALL_CONV EnterBootloader(void);

// --------------------------------------------------------------------------
// Configurations
// --------------------------------------------------------------------------
/**
 * Writes to ST25RU3993 register at address addr the value and prepares tx answer to host.\n
 * or Runs Direct commands if given address addr fits a Direct command code.\n
 * @param reg: See STUHFL_T_ST25RU3993_Register struct for further info
 *
 * @return error code
 */
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_Register(STUHFL_T_ST25RU3993_Register *reg);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_RegisterMultiple(STUHFL_T_ST25RU3993_Register **reg, uint8_t numReg);

/**
 * Reads one ST25RU3993 register at address and puts the value into the reply to the host.\n
 * @param reg: See STUHFL_T_ST25RU3993_Register struct for further info \n
 *
 * @return error code
 */
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_Register(STUHFL_T_ST25RU3993_Register *reg);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_RegisterMultiple(uint8_t numReg, STUHFL_T_ST25RU3993_Register **reg);

/**
 * Set reader configuration
 * @param rwdCfg: See STUHFL_T_ST25RU3993_RwdConfig struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_RwdCfg(STUHFL_T_ST25RU3993_RwdConfig *rwdCfg);
/**
 * Get reader configuration
 * @param rwdCfg: See STUHFL_T_ST25RU3993_RwdConfig struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_RwdCfg(STUHFL_T_ST25RU3993_RwdConfig *rwdCfg);

/**
 * Set reader Rx Filter configuration for Gen2 config
 * @param rxFilter: See STUHFL_T_ST25RU3993_RxFilter struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_Gen2_RxFilter(STUHFL_T_ST25RU3993_RxFilter *rxFilter);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_Gen2_RxFilterMultiple(uint8_t numRxFilter, STUHFL_T_ST25RU3993_RxFilter **rxFilter);
/**
 * Get reader Rx Filter configuration of Gen2 config
 * @param rxFilter: See STUHFL_T_ST25RU3993_RxFilter struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_Gen2_RxFilter(STUHFL_T_ST25RU3993_RxFilter *rxFilter);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_Gen2_RxFilterMultiple(uint8_t numRxFilter, STUHFL_T_ST25RU3993_RxFilter **rxFilter);

/**
 * Set reader Rx Filter configuration for Gb29768 config
 * @param rxFilter: See STUHFL_T_ST25RU3993_RxFilter struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_Gb29768_RxFilter(STUHFL_T_ST25RU3993_RxFilter *rxFilter);
/**
 * Get reader Rx Filter configuration of Gb29768 config
 * @param rxFilter: See STUHFL_T_ST25RU3993_RxFilter struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_Gb29768_RxFilter(STUHFL_T_ST25RU3993_RxFilter *rxFilter);

/**
 * Set reader Rx Filter calibration for Gen2
 * @param rxFilter: See STUHFL_T_ST25RU3993_FilterCalibration struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_Gen2_FilterCalibration(STUHFL_T_ST25RU3993_FilterCalibration *fCal);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_Gen2_FilterCalibrationMultiple(uint8_t numFCal, STUHFL_T_ST25RU3993_FilterCalibration **fCal);
/**
 * Get reader Rx Filter calibration from Gen2
 * @param rxFilter: See STUHFL_T_ST25RU3993_FilterCalibration struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_Gen2_FilterCalibration(STUHFL_T_ST25RU3993_FilterCalibration *fCal);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_Gen2_FilterCalibrationMultiple(uint8_t numFCal, STUHFL_T_ST25RU3993_FilterCalibration **fCal);

/**
 * Set reader Rx Filter calibration for Gb29768
 * @param rxFilter: See STUHFL_T_ST25RU3993_FilterCalibration struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_Gb29768_FilterCalibration(STUHFL_T_ST25RU3993_FilterCalibration *fCal);
/**
 * Get reader Rx Filter calibration from Gb29768
 * @param rxFilter: See STUHFL_T_ST25RU3993_FilterCalibration struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_Gb29768_FilterCalibration(STUHFL_T_ST25RU3993_FilterCalibration *fCal);

/**
  * Set antenna power
  * @param antPwr: See STUHFL_T_ST25RU3993_AntennaPower struct for further info
  *
  * @return error code
 */
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_AntennaPower(STUHFL_T_ST25RU3993_AntennaPower *antPwr);
/**
 * Get antenna power
 * @param antPwr: See STUHFL_T_ST25RU3993_AntennaPower struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_AntennaPower(STUHFL_T_ST25RU3993_AntennaPower *antPwr);



#if OLD_FREQUENCY_TUNING
/**
 * Set frequency profile
 * @param freqProfile: See STUHFL_T_ST25RU3993_FreqProfile struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_DEPRECATED STUHFL_T_RET_CODE CALL_CONV Set_FreqProfile(STUHFL_T_ST25RU3993_FreqProfile *freqProfile);
/**
 * Add frequency to custom profile
 * @param freqProfileAdd2Custom: See STUHFL_T_ST25RU3993_FreqProfileAddToCustom struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_DEPRECATED STUHFL_T_RET_CODE CALL_CONV Set_FreqProfileAddToCustom(STUHFL_T_ST25RU3993_FreqProfileAddToCustom *freqProfileAdd2Custom);
#endif

/**
 * Set frequency channel list
 * @param channelList: See STUHFL_T_ST25RU3993_ChannelList struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_ChannelList(STUHFL_T_ST25RU3993_ChannelList *channelList);
/**
 * Set frequency hopping time
 * @param freqHop: See STUHFL_T_ST25RU3993_FreqHop struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_FreqHop(STUHFL_T_ST25RU3993_FreqHop *freqHop);
/**
 * Set listen before talk configuration
 * @param freqLBT: See STUHFL_T_ST25RU3993_FreqLBT struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_FreqLBT(STUHFL_T_ST25RU3993_FreqLBT *freqLBT);
/**
 * Set continuous modulation configuration
 * @param freqContMod: See STUHFL_T_ST25RU3993_FreqContinuousModulation struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_FreqContinuousModulation(STUHFL_T_ST25RU3993_FreqContinuousModulation *freqContMod);

/**
 * Get frequency RSSI
 * @param freqRSSI: See STUHFL_T_ST25RU3993_FreqRssi struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_FreqRSSI(STUHFL_T_ST25RU3993_FreqRssi *freqRSSI);
/**
 * Get reflected power
 * @param freqProfileCustomAppend: See STUHFL_T_ST25RU3993_FreqReflectedPowerInfo struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_FreqReflectedPower(STUHFL_T_ST25RU3993_FreqReflectedPowerInfo *freqReflectedPower);

#if OLD_FREQUENCY_TUNING
/**
 * Get info of current used profile
 * @param freqProfileInfo: See STUHFL_T_ST25RU3993_FreqProfileInfo struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_DEPRECATED STUHFL_T_RET_CODE CALL_CONV Get_FreqProfileInfo(STUHFL_T_ST25RU3993_FreqProfileInfo *freqProfileInfo);
#endif

/**
 * Get frequency channel list
 * @param channelList: See STUHFL_T_ST25RU3993_ChannelList struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_ChannelList(STUHFL_T_ST25RU3993_ChannelList *channelList);
/**
 * Get frequency hopping time
 * @param freqHop: See STUHFL_T_ST25RU3993_FreqHop struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_FreqHop(STUHFL_T_ST25RU3993_FreqHop *freqHop);
/**
 * Get listen before talk configuration
 * @param freqLBT: See STUHFL_T_ST25RU3993_FreqLBT struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_FreqLBT(STUHFL_T_ST25RU3993_FreqLBT *freqLBT);

/**
 * Set protocols timings
 * @param gen2Timings: See STUHFL_T_ST25RU3993_Gen2_Timings struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_Gen2_Timings(STUHFL_T_ST25RU3993_Gen2_Timings *gen2Timings);
/**
 * Set Gen2 configuration
 * @param gen2ProtocolCfg: See STUHFL_T_ST25RU3993_Gen2_ProtocolCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_Gen2_ProtocolCfg(STUHFL_T_ST25RU3993_Gen2_ProtocolCfg *gen2ProtocolCfg);
/**
 * Set Gb29768 configuration
 * @param gb29768ProtocolCfg: See STUHFL_T_ST25RU3993_Gb29768_ProtocolCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_Gb29768_ProtocolCfg(STUHFL_T_ST25RU3993_Gb29768_ProtocolCfg *gb29768ProtocolCfg);
/**
 * Set Iso6b configuration
 * @param iso6bProtocolCfg: See STUHFL_T_ST25RU3993_Iso6b_ProtocolCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_Iso6b_ProtocolCfg(STUHFL_T_ST25RU3993_Iso6b_ProtocolCfg *iso6bProtocolCfg);
/**
 * Set TxRx configuration
 * @param txRxCfg: See STUHFL_T_ST25RU3993_TxRxCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_TxRxCfg(STUHFL_T_ST25RU3993_TxRxCfg *txRxCfg);
/**
 * Set power amplifier configuration
 * @param paCfg: See STUHFL_T_ST25RU3993_PowerAmplifierCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_PowerAmplifierCfg(STUHFL_T_ST25RU3993_PowerAmplifierCfg *paCfg);
/**
 * Set general Gen2 inventory configuration
 * @param invGen2Cfg: See STUHFL_T_ST25RU3993_Gen2_InventoryCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_Gen2_InventoryCfg(STUHFL_T_ST25RU3993_Gen2_InventoryCfg *invGen2Cfg);
/**
 * Set general Gb29768 inventory configuration
 * @param invGb29768Cfg: See STUHFL_T_ST25RU3993_Gb29768_InventoryCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_Gb29768_InventoryCfg(STUHFL_T_ST25RU3993_Gb29768_InventoryCfg *invGb29768Cfg);
/**
 * Set general Iso6b inventory configuration
 * @param invIso6bCfg: See STUHFL_T_ST25RU3993_Iso6b_InventoryCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_Iso6b_InventoryCfg(STUHFL_T_ST25RU3993_Iso6b_InventoryCfg *invIso6bCfg);

/**
 * Get protocols timings
 * @param gen2Timings: See STUHFL_T_ST25RU3993_Gen2_Timings struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_Gen2_Timings(STUHFL_T_ST25RU3993_Gen2_Timings *gen2Timings);
/**
 * Get Gen2 configuration
 * @param gen2ProtocolCfg: See STUHFL_T_ST25RU3993_Gen2_ProtocolCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_Gen2_ProtocolCfg(STUHFL_T_ST25RU3993_Gen2_ProtocolCfg *gen2ProtocolCfg);
/**
 * Get Gb29768 configuration
 * @param gb29768ProtocolCfg: See STUHFL_T_ST25RU3993_Gb29768_ProtocolCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_Gb29768_ProtocolCfg(STUHFL_T_ST25RU3993_Gb29768_ProtocolCfg *gb29768ProtocolCfg);
/**
 * Get Iso6b configuration
 * @param iso6bProtocolCfg: See STUHFL_T_ST25RU3993_Iso6b_ProtocolCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_Iso6b_ProtocolCfg(STUHFL_T_ST25RU3993_Iso6b_ProtocolCfg *iso6bProtocolCfg);
/**
 * Get TxRx configuration
 * @param txRxCfg: See STUHFL_T_ST25RU3993_TxRxCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_TxRxCfg(STUHFL_T_ST25RU3993_TxRxCfg *txRxCfg);
/**
 * Get power amplifier configuration
 * @param paCfg: See STUHFL_T_ST25RU3993_PowerAmplifierCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_PowerAmplifierCfg(STUHFL_T_ST25RU3993_PowerAmplifierCfg *paCfg);
/**
 * Get general Gen2 inventory configuration
 * @param invGen2Cfg: See STUHFL_T_ST25RU3993_Gen2_InventoryCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_Gen2_InventoryCfg(STUHFL_T_ST25RU3993_Gen2_InventoryCfg *invGen2Cfg);
/**
 * Get general Gb29768 inventory configuration
 * @param invGb29768Cfg: See STUHFL_T_ST25RU3993_Gb29768_InventoryCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_Gb29768_InventoryCfg(STUHFL_T_ST25RU3993_Gb29768_InventoryCfg *invGb29768Cfg);
/**
 * Get general Iso6b inventory configuration
 * @param invIso6bCfg: See STUHFL_T_ST25RU3993_Iso6b_InventoryCfg struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_Iso6b_InventoryCfg(STUHFL_T_ST25RU3993_Iso6b_InventoryCfg *invIso6bCfg);




// --------------------------------------------------------------------------
// Tuning
// --------------------------------------------------------------------------
#if OLD_FREQUENCY_TUNING
/**
 * Set Cin, Clen and Cout
 * @param tuning: See STUHFL_T_ST25RU3993_Tuning struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_DEPRECATED STUHFL_T_RET_CODE CALL_CONV Set_Tuning(STUHFL_T_ST25RU3993_Tuning *tuning);
#endif
/**
 * Set Cin, Clen and Cout at given antenna
 * @param tuning: See STUHFL_T_ST25RU3993_TuningCaps struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Set_TuningCaps(STUHFL_T_ST25RU3993_TuningCaps *tuning);

#if OLD_FREQUENCY_TUNING
/**
 * Set tuning table entry
 * @param tuningTableEntry: See STUHFL_T_ST25RU3993_TuningTableEntry struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_DEPRECATED STUHFL_T_RET_CODE CALL_CONV Set_TuningTableEntry(STUHFL_T_ST25RU3993_TuningTableEntry *tuningTableEntry);
/**
 * Revert to default tuning
 * @param set: See STUHFL_T_ST25RU3993_TunerTableSet struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_DEPRECATED STUHFL_T_RET_CODE CALL_CONV Set_TuningTableDefault(STUHFL_T_ST25RU3993_TunerTableSet *set);
/**
 * Save current configured tuning table to flash
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_DEPRECATED STUHFL_T_RET_CODE CALL_CONV Set_TuningTableSave2Flash(void);
/**
 * Delete tuning table entries
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_DEPRECATED STUHFL_T_RET_CODE CALL_CONV Set_TuningTableEmpty(void);


/**
 * Get Cin, Clen and Cout configuration
 * @param tuning: See STUHFL_T_ST25RU3993_Tuning struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_DEPRECATED STUHFL_T_RET_CODE CALL_CONV Get_Tuning(STUHFL_T_ST25RU3993_Tuning *tuning);
#endif

/**
 * Get Cin, Clen and Cout from given antenna
 * @param tuning: See STUHFL_T_ST25RU3993_TuningCaps struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Get_TuningCaps(STUHFL_T_ST25RU3993_TuningCaps *tuning);

#if OLD_FREQUENCY_TUNING
/**
 * Get tuning table entry
 * @param tuningTableEntry: See STUHFL_T_ST25RU3993_TuningTableEntry struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_DEPRECATED STUHFL_T_RET_CODE CALL_CONV Get_TuningTableEntry(STUHFL_T_ST25RU3993_TuningTableEntry *tuningTableEntry);
/**
 * Set current tuning table info
 * @param tuningTableInfo: See STUHFL_T_ST25RU3993_TuningTableInfo struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_DEPRECATED STUHFL_T_RET_CODE CALL_CONV Get_TuningTableInfo(STUHFL_T_ST25RU3993_TuningTableInfo *tuningTableInfo);

/**
 * Start tuning
 * @param tune: See STUHFL_T_ST25RU3993_Tune struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_DEPRECATED STUHFL_T_RET_CODE CALL_CONV Tune(STUHFL_T_ST25RU3993_Tune *tune);
#endif

/**
 * Start tuning
 * @param tuneCfg: See STUHFL_T_ST25RU3993_Tune struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV TuneChannel(STUHFL_T_ST25RU3993_TuneCfg *tuneCfg);



// --------------------------------------------------------------------------
// Gen2
// --------------------------------------------------------------------------
/**
 * Perform Gen2 Inventory depending on the current inventory and gen2 configuration
 * @param invOption: See STUHFL_T_InventoryOption struct for further info
 * @param invData: See STUHFL_T_InventoryData struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_Inventory(STUHFL_T_InventoryOption *invOption, STUHFL_T_InventoryData *invData);
/**
 * Perform Gen2 Select command to select or filter transponders
 * @param selData: See STUHFL_T_Gen2_Select struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_Select(STUHFL_T_Gen2_Select *selData);
/**
 * Perform Gen2 Read command to read from the Gen2 transponders
 * @param readData: See STUHFL_T_Read struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_Read(STUHFL_T_Gen2_Read *readData);
/**
 * Perform Gen2 Write command to write data to Gen2 transponders
 * @param writeData: See STUHFL_T_Write struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_Write(STUHFL_T_Gen2_Write *writeData);
/**
 * Perform Gen2 Block Write command to write block data to Gen2 transponders
 * @param writeData: See STUHFL_T_BlockWrite struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_BlockWrite(STUHFL_T_Gen2_BlockWrite *blockWrite);
/**
 * Perform Gen2 Lock command to lock transponders
 * @param lockData: See STUHFL_T_Gen2_Lock struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_Lock(STUHFL_T_Gen2_Lock *lockData);
/**
 * Perform Gen2 Kill command to kill transponders.
 * NOTE: After this command your transponders will not be functional anymore
 * @param killData: See STUHFL_T_Kill struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_Kill(STUHFL_T_Gen2_Kill *killData);
/**
 * Perform generic Gen2 bit exchange
 * @param genericCmd: See STUHFL_T_Gen2_GenericCmd struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_GenericCmd(STUHFL_T_Gen2_GenericCmd *genericCmd);
/**
 * Perform RSSI measurement during Gen2 Query command
 * @param queryMeasureRssi: See STUHFL_T_Gen2_QueryMeasureRssi struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_QueryMeasureRssi(STUHFL_T_Gen2_QueryMeasureRssi *queryMeasureRssi);

/**
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_Challenge(STUHFL_T_Gen2_Challenge *challenge);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_Authenticate(STUHFL_T_Gen2_Authenticate *authenticate);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_AuthComm(STUHFL_T_Gen2_AuthComm *authComm);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_SecureComm(STUHFL_T_Gen2_SecureComm *secureComm);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_KeyUpdate(STUHFL_T_Gen2_KeyUpdate *keyUpdate);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_TagPrivilege (STUHFL_T_Gen2_TagPrivilege *tagPrivilege);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_ReadBuffer (STUHFL_T_Gen2_ReadBuffer *readBuffer);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gen2_Untraceable (STUHFL_T_Gen2_Untraceable *untraceable);
//
//STUHFL_T_RET_CODE Gen2_FileOpen(STUHFL_T_Gen2_FileOpen *fileOpen);
//STUHFL_T_RET_CODE Gen2_FileList(STUHFL_T_Gen2_FileList *fileList);
//STUHFL_T_RET_CODE Gen2_FilePrivilege(STUHFL_T_Gen2_FilePrivilege *filePrivilege);
//STUHFL_T_RET_CODE Gen2_FileSetup(STUHFL_T_Gen2_FileSetup *fileSetup);



// --------------------------------------------------------------------------
// GB29768
// --------------------------------------------------------------------------
/**
 * Perform GB29768 Inventory depending on the current inventory and GB29768 configuration
 * @param invOption: See STUHFL_T_InventoryOption struct for further info
 * @param invData: See STUHFL_T_InventoryData struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gb29768_Inventory(STUHFL_T_InventoryOption *invOption, STUHFL_T_InventoryData *invData);
/**
 * Perform Gb29768 Sort command to select or filter transponders
 * Nota: If Sort defined on Matching flag, inventories and tag accesses are based on the assumption
 * the matching flags are only '1'
 * as a consequence the rule STUHFL_D_GB29768_RULE_MATCH0_ELSE_1 (0x03) is to be used to invert selection
 *
 * @param sortData: See STUHFL_T_Gb29768_Sort struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gb29768_Sort(STUHFL_T_Gb29768_Sort *sortData);
/**
 * Perform Gb29768 Read command to read from the Gb29768 transponders
 * @param readData: See STUHFL_T_Read struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gb29768_Read(STUHFL_T_Gb29768_Read *readData);
/**
 * Perform Gb29768 Write command to write data to Gb29768 transponders
 * @param writeData: See STUHFL_T_Write struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gb29768_Write(STUHFL_T_Gb29768_Write *writeData);
/**
 * Perform Gb29768 Lock command to lock transponders
 * @param lockData: See STUHFL_T_Gb29768_Lock struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gb29768_Lock(STUHFL_T_Gb29768_Lock *lockData);
/**
 * Perform Gb29768 Kill command to kill transponders.
 * NOTE: After this command your transponders will not be functional anymore
 * @param killData: See STUHFL_T_Kill struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gb29768_Kill(STUHFL_T_Gb29768_Kill *killData);
/**
 * Perform Gb29768 Erase command to erase transponders
 * @param lockData: See STUHFL_T_Gb29768_Erase struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Gb29768_Erase(STUHFL_T_Gb29768_Erase *eraseData);


// --------------------------------------------------------------------------
// ISO6B
// --------------------------------------------------------------------------
/**
 * Perform Iso6B Inventory depending on the current inventory and Iso6B configuration
 * @param invOption: See STUHFL_T_InventoryOption struct for further info
 * @param invData: See STUHFL_T_InventoryData struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Iso6b_Inventory(STUHFL_T_InventoryOption *invOption, STUHFL_T_InventoryData *invData);
/**
 * Perform Iso6b Select command to select or filter transponders
 * @param selData: See STUHFL_T_Iso6b_Select struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Iso6b_Select(STUHFL_T_Iso6b_Select *selData);
/**
 * Perform Iso6b Read command to read from the Iso6b transponders
 * @param readData: See STUHFL_T_Iso6b_Read struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Iso6b_Read(STUHFL_T_Iso6b_Read *readData);
/**
 * Perform Iso6b Write command to write data to Iso6b transponders
 * @param writeData: See STUHFL_T_Iso6b_Write struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Iso6b_Write(STUHFL_T_Iso6b_Write *writeData);


// --------------------------------------------------------------------------
// Inventory Runner
// --------------------------------------------------------------------------
/**
 * Function definition for callback when a transponder is found during inventory.
 * This callback is executed whenever a transponder is detected during the inventory
 * round.
 *
 * @param *data: inventory data of transponder that is given back in the callback with
 *               information about the transponder and the current reader status.
 *               See STUHFL_T_InventoryData struct for further info.
 *
 * @return error code
*/
typedef STUHFL_T_RET_CODE(*STUHFL_T_InventoryCycle)(STUHFL_T_InventoryData *data);

/**
 * Function definition for callback when inventory is finished.
 * This callback is executed after all inventory ounds are executed.
 *
 * @param *data: inventory data of transponder that is given back in the callback with
 *               information about the transponder and the current reader status.
 *               See STUHFL_T_InventoryData struct for further info.
 *
 * @return error code
*/
typedef STUHFL_T_RET_CODE(*STUHFL_T_InventoryFinished)(STUHFL_T_InventoryData *data);
/**
 * Start inventory runner
 * @param *option: set runner options. NOTE:
 *                 See STUHFL_T_InventoryOption struct for further info
 * @param cycleCallback: callback when a transponder is found during inventory
 * @param finishedCallback: callback when the inventory is finished
 * @param *data: inventory data of transponder that is given back in the callback with
 *               information about the transponder and the current reader status.
 *               See STUHFL_T_InventoryData struct for further info.
 *
 * NOTE: Starting the inventory runner is a blocking call. Whenever
 * transponders found the STUHFL_T_InventoryCycle callback is called.
 * Within this callback processing of the data should take place.
 *
 * To stop the runner and return from the Inventory_RunnerStart function
 * there are 2 possibilities.
 *   1. A call to Inventory_RunnerStop. Can be called from the callback
 *   2. When the number of requested rounds are executed
 *
 * @return error code
 */
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Inventory_RunnerStart(STUHFL_T_InventoryOption *option, STUHFL_T_InventoryCycle cycleCallback, STUHFL_T_InventoryFinished finishedCallback, STUHFL_T_InventoryData *data);

#ifdef USE_INVENTORY_EXT
/**
 * Start Extended inventory runner (Inventory runner data + slot info)
 * @param *option: set runner options. NOTE:
 *                 See STUHFL_T_InventoryOption struct for further info
 * @param cycleCallback: callback when a transponder is found during inventory
 * @param finishedCallback: callback when the inventory is finished
 * @param *data: inventory data of transponder that is given back in the callback with
 *               information about the transponder and the current reader status plus the slot info.
 *               See STUHFL_T_InventoryDataExt struct for further info.
 *
 * NOTE: Starting the inventory runner is a blocking call. Whenever
 * transponders found the STUHFL_T_InventoryCycle callback is called.
 * Within this callback processing of the data should take place.
 *
 * To stop the runner and return from the Inventory_RunnerStartExt function
 * there are 2 possibilities.
 *   1. A call to Inventory_RunnerStop. Can be called from the callback
 *   2. When the number of requested rounds are executed
 *
 * @return error code
 */
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Inventory_RunnerStartExt(STUHFL_T_InventoryOption *option, STUHFL_T_InventoryCycle cycleCallback, STUHFL_T_InventoryFinished finishedCallback, STUHFL_T_InventoryDataExt *data);
#endif  // USE_INVENTORY_EXT

/**
 * Stop current inventory runner
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV Inventory_RunnerStop(void);



#ifdef __cplusplus
}
#endif //__cplusplus

#endif // __STUHFL_EVALAPI_H

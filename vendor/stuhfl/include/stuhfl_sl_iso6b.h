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
#if !defined __STUHFL_SL_ISO6B_H
#define __STUHFL_SL_ISO6B_H

#include "stuhfl.h"
#include "stuhfl_sl.h"

#ifdef __cplusplus
extern "C"
{
#endif //__cplusplus

#pragma pack(push, 1)

#define STUHFL_D_ISO6B_SELECT_MODE_CLEAR_LIST      0
#define STUHFL_D_ISO6B_SELECT_MODE_ADD2LIST        1
#define STUHFL_D_ISO6B_SELECT_MODE_CLEAR_AND_ADD   2

#define STUHFL_D_ISO6B_GROUP_SELECT_EQ              0x00
#define STUHFL_D_ISO6B_GROUP_SELECT_NE              0x01
#define STUHFL_D_ISO6B_GROUP_SELECT_GT              0x02
#define STUHFL_D_ISO6B_GROUP_SELECT_LT              0x03
#define STUHFL_D_ISO6B_GROUP_UNSELECT_EQ            0x04
#define STUHFL_D_ISO6B_GROUP_UNSELECT_NE            0x05
#define STUHFL_D_ISO6B_GROUP_UNSELECT_GT            0x06
#define STUHFL_D_ISO6B_GROUP_UNSELECT_LT            0x07


#define STUHFL_D_ISO6B_MAX_READ_DATA_LEN     8U
typedef struct {
    uint8_t             address;                                    /**< I Param: byte address to which read data. */
    uint8_t             data[STUHFL_D_ISO6B_MAX_READ_DATA_LEN];     /**< O Param: Read data. */
} STUHFL_T_Iso6b_Read;
#define STUHFL_O_ISO6B_READ_INIT()        {0, {0, 0, 0, 0, 0, 0, 0, 0}}

typedef struct {
    uint8_t             address;            /**< I Param: byte address to which write data. */
    uint8_t             data;               /**< I Param: Byte to be written. */
    uint8_t             tagReply;           /**< O Param: Tag reply. */
} STUHFL_T_Iso6b_Write;
#define STUHFL_O_ISO6B_WRITE_INIT()        {0, 0, 0}

//typedef STUHFL_T_Kill           STUHFL_T_Iso6b_Kill;

#define STUHFL_D_ISO6B_SELECT_FILTER_LENGTH          8        /**< Select mask length. */
typedef struct {
    uint8_t                             mode;           /**< I Param: Select mode to be applied (CLEAR_LIST, ADD2LIST, CLEAR_AND_ADD). */
    uint8_t                             group;          /**< I Param: Elicit the tag behavior according to Iso6b Select command specification (GROUP_SELECT_EQ, GROUP_SELECT_NE, GROUP_UNSELECT_EQ, ...). */
    uint8_t                             address;        /**< I Param: Address of the 8-byte memory content to which select applies. */
    uint8_t                             bitMask;        /**< I Param: Bit mask to which filter applies. */
    uint8_t                             filter[STUHFL_D_ISO6B_SELECT_FILTER_LENGTH];       /**< I Param: Select mask filter. */
} STUHFL_T_Iso6b_Select;
#define STUHFL_O_ISO6B_SELECT_INIT()         {STUHFL_D_ISO6B_SELECT_MODE_CLEAR_LIST, STUHFL_D_ISO6B_GROUP_SELECT_EQ, 0x00, 0x00, {0, 0, 0, 0, 0, 0, 0, 0}}

//#define STUHFL_D_ISO6B_CONFIGURATION_ATTRIBUTE         0x00
//#define STUHFL_D_ISO6B_CONFIGURATION_SECURITYMODE      0x01
//
//#define STUHFL_D_ISO6B_ACTION_ATTRIBUTE_READWRITE      0x00
//#define STUHFL_D_ISO6B_ACTION_ATTRIBUTE_READUNWRITE    0x01
//#define STUHFL_D_ISO6B_ACTION_ATTRIBUTE_UNREADWRITE    0x02
//#define STUHFL_D_ISO6B_ACTION_ATTRIBUTE_UNREADUNWRITE  0x03
//
//#define STUHFL_D_ISO6B_ACTION_SECMODE_AUTH_RESERVED         0x00
//#define STUHFL_D_ISO6B_ACTION_SECMODE_AUTH_NOAUTH           0x01
//#define STUHFL_D_ISO6B_ACTION_SECMODE_AUTH_AUTH_NOSECCOMM   0x02
//#define STUHFL_D_ISO6B_ACTION_SECMODE_AUTH_AUTH_SECCOMM     0x03
//
//typedef struct {
//    uint8_t                 memoryBank;            /**< I Param: Bank (TagInfo, Coding, Security, User) on which apply the lock. */
//    uint8_t                 configuration;          /**< I Param: Configure attribute and security mode of storage area. */
//    uint8_t                 action;                 /**< I Param: Define how the lock operation is performed. */
//    uint8_t                 pwd[STUHFL_D_PASSWORD_LEN];      /**< I Param: Password. */
//} STUHFL_T_Iso6b_Lock;
//#define STUHFL_O_ISO6B_LOCK_INIT()         {STUHFL_D_ISO6B_AREA_TAGINFO, STUHFL_D_ISO6B_CONFIGURATION_ATTRIBUTE, STUHFL_D_ISO6B_ACTION_ATTRIBUTE_READWRITE, {0}}
//
//typedef struct {
//    uint8_t                             memoryBank;                    /**< I Param: Bank (TagInfo, Coding, Security, User) on which apply the erase. */
//    uint8_t                             numBytesToErase;               /**< I Param: Number of bytes to erase. */
//    uint32_t                            bytePtr;                       /**< I Param: Byte start address within bank for data to be erased. */
//    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];    /**< I Param: Password. */
//} STUHFL_T_Iso6b_Erase;
//#define STUHFL_O_ISO6B_ERASE_INIT()        {STUHFL_D_ISO6B_AREA_TAGINFO, 0, 0, {0}}

#pragma pack(pop)

/**
 * Perform Iso6b Inventory depending on the current inventory and Iso6b configuration
 * @param invOption: See STUHFL_T_InventoryOption struct for further info
 * @param invData: See STUHFL_T_InventoryData struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Iso6b_Inventory(STUHFL_T_InventoryOption *invOption, STUHFL_T_InventoryData *invData);
/**
 * Perform Iso6b Select command to select or filter transponders
 * @param selectData: See STUHFL_T_Iso6b_Select struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Iso6b_Select(STUHFL_T_Iso6b_Select *selectData);
/**
 * Perform Iso6b Read command to read from the Iso6b transponders
 * @param readData: See STUHFL_T_Iso6b_Read struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Iso6b_Read(STUHFL_T_Iso6b_Read *readData);
/**
 * Perform Iso6b Write command to write data to Iso6b transponders
 * @param writeData: See STUHFL_T_Iso6b_Write struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Iso6b_Write(STUHFL_T_Iso6b_Write *writeData);
//
///**
// * Perform Iso6b Lock command to lock transponders
// * @param lockData: See STUHFL_T_Iso6b_Lock struct for further info
// *
// * @return error code
//*/
//STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Iso6b_Lock(STUHFL_T_Iso6b_Lock *lockData);
//
///**
// * Perform Iso6b Kill command to kill transponders.
// * NOTE: After this command your transponders will not be functional anymore
// * @param killData: See STUHFL_T_Kill struct for further info
// *
// * @return error code
//*/
//STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Iso6b_Kill(STUHFL_T_Iso6b_Kill *killData);
//
///**
// * Perform Iso6b erase command to erase data in transponders.
// * @param eraseData: See STUHFL_T_Iso6b_Erase struct for further info
// *
// * @return error code
//*/
//STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Iso6b_Erase(STUHFL_T_Iso6b_Erase *eraseData);

#ifdef __cplusplus
}
#endif //__cplusplus

#endif // __STUHFL_SL_ISO6B_H

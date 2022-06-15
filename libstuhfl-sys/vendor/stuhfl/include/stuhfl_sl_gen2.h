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
#if !defined __STUHFL_SL_GEN2_H
#define __STUHFL_SL_GEN2_H

#include "stuhfl.h"
#include "stuhfl_sl.h"


#ifdef __cplusplus
extern "C"
{
#endif //__cplusplus

// - GEN2 -------------------------------------------------------------------

#pragma pack(push, 1)
#define STUHFL_D_GEN2_SELECT_MODE_CLEAR_LIST            0
#define STUHFL_D_GEN2_SELECT_MODE_ADD2LIST              1
#define STUHFL_D_GEN2_SELECT_MODE_CLEAR_AND_ADD         2
#define STUHFL_D_GEN2_SELECT_MODE_INVERT_SL             0x80

#define STUHFL_D_GEN2_TARGET_S0                  0
#define STUHFL_D_GEN2_TARGET_S1                  1
#define STUHFL_D_GEN2_TARGET_S2                  2
#define STUHFL_D_GEN2_TARGET_S3                  3
#define STUHFL_D_GEN2_TARGET_SL                  4

#define STUHFL_D_GEN2_MEMORY_BANK_RESERVED       0
#define STUHFL_D_GEN2_MEMORY_BANK_EPC            1
#define STUHFL_D_GEN2_MEMORY_BANK_TID            2
#define STUHFL_D_GEN2_MEMORY_BANK_USER           3

#define STUHFL_D_GEN2_MAX_SELECT_MASK_LENGTH          32  /**< Maximum select mask length. Limited to 255 bits by EPCglobal */
typedef struct {
    uint8_t                             mode;           /**< I Param: Select mode to be applied (CLEAR_LIST, ADD2LIST, CLEAR_AND_ADD). */
    uint8_t                             target;         /**< I Param: indicates whether the select modifies a tag's SL flag or its inventoried flag. */
    uint8_t                             action;         /**< I Param: Elicit the tag behavior according to Gen2 Select specification. */
    uint8_t                             memoryBank;     /**< I Param: Bank (File, EPC, TID, USER) on which apply the select. */
    uint8_t                             mask[STUHFL_D_GEN2_MAX_SELECT_MASK_LENGTH];   /**< I Param: Selection mask. */
    uint32_t                            maskBitPointer; /**< I Param: Bit starting address to which mask is applied (bit address). */
    uint8_t                             maskBitLength;  /**< I Param: Mask length in bits. */
    uint8_t                             truncation;     /**< I Param: Truncate enabling. Not supported yet, must be set to zero */
} STUHFL_T_Gen2_Select;
#define STUHFL_O_GEN2_SELECT_INIT()      {STUHFL_D_GEN2_SELECT_MODE_CLEAR_LIST, STUHFL_D_GEN2_TARGET_S0, 0, STUHFL_D_GEN2_MEMORY_BANK_EPC, {0}, 0, 0, false}

typedef STUHFL_T_Read           STUHFL_T_Gen2_Read;
typedef STUHFL_T_Write          STUHFL_T_Gen2_Write;
typedef STUHFL_T_BlockWrite     STUHFL_T_Gen2_BlockWrite;

#define STUHFL_D_GEN2_LOCK_MASK_ACTION_LEN            3
typedef struct {
    uint8_t                             mask[STUHFL_D_GEN2_LOCK_MASK_ACTION_LEN];   /**< I Param: Mask and actions field. */
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];           /**< I Param: Access password. If pwd set to 0x00000000 access command prior to lock is skipped. */
    uint8_t                             tagReply;                   /**< O Param: Tag reply. */
} STUHFL_T_Gen2_Lock;
#define STUHFL_O_GEN2_LOCK_INIT()        {{0}, {0}, 0}

typedef STUHFL_T_Kill           STUHFL_T_Gen2_Kill;

#define STUHFL_D_GEN2_GENERIC_CMD_CRC                   0x90
#define STUHFL_D_GEN2_GENERIC_CMD_CRC_EXPECT_HEAD       0x91
#define STUHFL_D_GEN2_GENERIC_CMD_NO_CRC                0x92

#define STUHFL_D_GEN2_GENERIC_CMD_MAX_SND_DATA_BYTES     64U    // 512/8
#define STUHFL_D_GEN2_GENERIC_CMD_MAX_RCV_DATA_BYTES     128U

typedef struct {
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];   /**< I Param: Access password. If pwd set to 0x00000000 access command prior to generic command is skipped. */
    uint8_t                             cmd;                            /**< I Param: Generic command. */
    uint8_t                             noResponseTime;                 /**< I Param: Tag response timeout. */
    uint16_t                            expectedRcvDataBitLength;       /**< I Param: Size in bits of expected received data. NOTE: For the direct commands 0x90 and 0x91 (Tranmission with CRC) CRC is handled by HW and need not to be included into the expected bit count. The received CRC will also not replied to the host. When using command 0x92 handling of any data integrity checking must be done manually.*/
    uint16_t                            sndDataBitLength;               /**< I Param: Size in bits of data sent to Tag. */
    bool                                appendRN16;                     /**< I Param: Append tag handle to generic command. */
    uint8_t                             sndData[STUHFL_D_GEN2_GENERIC_CMD_MAX_SND_DATA_BYTES];   /**< I Param: Data being sent to Tag. */
    uint16_t                            rcvDataLength;                  /**< O Param: Size in bytes of received data from Tag. */
    uint8_t                             rcvData[STUHFL_D_GEN2_GENERIC_CMD_MAX_RCV_DATA_BYTES];   /**< O Param: Data received from Tag. */
} STUHFL_T_Gen2_GenericCmd;
#define STUHFL_O_GEN2_GENERICCMD_INIT()      {{0}, 0, 0, 0, 0, true, {0}, 0, {0}}

typedef struct {
    uint32_t                            frequency;                  /**< I Param: Frequency. */
    uint8_t                             measureCnt;                 /**< I Param: Number of measures. */
    uint8_t                             agc[256];                   /**< O Param: AGC. */
    uint8_t                             rssiLogI[256];              /**< O Param: RSSI log. */
    uint8_t                             rssiLogQ[256];              /**< O Param: RSSI log. */
    int8_t                              rssiLinI[256];              /**< O Param: RSSI I Level. */
    int8_t                              rssiLinQ[256];              /**< O Param: RSSI Q Level. */
} STUHFL_T_Gen2_QueryMeasureRssi;
#define STUHFL_O_GEN2_QUERY_MEASURE_RSSI_INIT()      {0, 0, {0}, {0}, {0}, {0}, {0}}

// ..........................................................................
#define STUHFL_D_CSI_0                  0   // CSI_0: AES - 128
#define STUHFL_D_CSI_1                  1   // CSI_1: PRESENT - 80
#define STUHFL_D_CSI_2                  2   // CSI_2: ECC - DH
#define STUHFL_D_CSI_3                  3   // CSI_3: Grain - 128A
#define STUHFL_D_CSI_4                  4   // CSI_4: AES - OFB
#define STUHFL_D_CSI_5                  5   // CSI_5: XOR
#define STUHFL_D_CSI_6                  6   // CSI_6: ECDSA - ECDH
#define STUHFL_D_CSI_7                  7   // CSI_7: cryptoGPS
#define STUHFL_D_CSI_8                  8   // CSI_8: HB2
#define STUHFL_D_CSI_9                  9   // CSI_9: RAMON

#define STUHFL_D_MAX_MESSAGE_LENGTH     32U	//512U
#define STUHFL_D_MAX_RESPONSE_LENGTH    64U

#define STUHFL_D_CSI_0                  0
typedef struct {
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];
    bool                                incRepLen;								/**< I Param: IncRepLen specifies whether the Tag omits or includes length in its stored reply. */
    bool                                immed;									/**< I Param: Immed specifies whether a Tag concatenates response to its EPC when replying to an ACK or not. */
    uint8_t                             CSI;									/**< I Param: CSI that shall be used. See ISO29167-xx for further details */
    uint8_t                             message[STUHFL_D_MAX_MESSAGE_LENGTH];	/**< I Param: Message send to TAG */
    uint16_t                            messageBitLength;						/**< I Param: Message length in bits. */
} STUHFL_T_Gen2_Challenge;
#define STUHFL_O_GEN2_CHALLENGE_INIT() {{0}, true, true, STUHFL_D_CSI_0, {0}, 0}

typedef struct {
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];
    bool                                senRep;									/**< I Param: SenRep specifies whether a Tag backscatters its response or stores the response in its ResponseBuffer. */
    bool                                incRepLen;								/**< I Param: IncRepLen specifies whether the Tag omits or includes length in its stored reply. */
    uint8_t                             CSI;									/**< I Param: CSI that shall be used. See ISO29167-xx for further details */
    uint8_t                             message[STUHFL_D_MAX_MESSAGE_LENGTH];	/**< I Param: Message send to TAG */
    uint16_t                            messageBitLength;						/**< I Param: Message length in bits. */
    uint16_t                            responseBitLength;						/**< I/O Param: Expected and received response length in bits. Max = STUHFL_D_MAX_RESPONSE_LENGTH*8 */
    uint8_t                             response[STUHFL_D_MAX_RESPONSE_LENGTH];	/**< O Param: Tag response */
    bool                                responseHeaderBit;                      /**< O Param: Tag response header bit */
} STUHFL_T_Gen2_Authenticate;
#define STUHFL_O_GEN2_AUTHENTICATE_INIT() {{0}, true, true, STUHFL_D_CSI_0, {0}, 0, 0, {0}, false}

typedef struct {
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];
    bool                                incRepLen;
    uint8_t                             message[STUHFL_D_MAX_MESSAGE_LENGTH];
    uint16_t                            messageBitLength;
    uint16_t                            responseBitLength;						/**< I/O Param: Expected and received response length in bits. Max = STUHFL_D_MAX_RESPONSE_LENGTH*8 */
    uint8_t                             response[STUHFL_D_MAX_RESPONSE_LENGTH];	/**< O Param: Tag response */
    bool                                responseHeaderBit;                      /**< O Param: Tag response header bit */
} STUHFL_T_Gen2_AuthComm;
#define STUHFL_O_GEN2_AUTHCOMM_INIT() {{0}, true, {0}, 0, 0, false}

typedef struct {
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];
    bool                                senRep;
    bool                                incRepLen;
    uint8_t                             message[STUHFL_D_MAX_MESSAGE_LENGTH];
    uint16_t                            messageBitLength;
    uint16_t                            responseBitLength;						/**< I/O Param: Expected and received response length in bits. Max = STUHFL_D_MAX_RESPONSE_LENGTH*8 */
    uint8_t                             response[STUHFL_D_MAX_RESPONSE_LENGTH];	/**< O Param: Tag response */
    bool                                responseHeaderBit;                      /**< O Param: Tag response header bit */
} STUHFL_T_Gen2_SecureComm;
#define STUHFL_O_GEN2_SECURECOMM_INIT() {{0}, true, true, {0}, 0, 0, {0}, false}

typedef struct {
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];
    bool                                senRep;
    bool                                incRepLen;
    uint8_t                             keyID;
    uint8_t                             message[STUHFL_D_MAX_MESSAGE_LENGTH];
    uint16_t                            messageBitLength;
    uint16_t                            responseBitLength;						/**< I/O Param: Expected and received response length in bits. Max = STUHFL_D_MAX_RESPONSE_LENGTH*8 */
    uint8_t                             response[STUHFL_D_MAX_RESPONSE_LENGTH];	/**< O Param: Tag response */
    bool                                responseHeaderBit;                      /**< O Param: Tag response header bit */
} STUHFL_T_Gen2_KeyUpdate;
#define STUHFL_O_GEN2_KEYUPDATE_INIT() {{0}, true, true, 0, {0}, 0, 0, {0}, false}

typedef struct {
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];
    bool                                senRep;
    bool                                incRepLen;
    bool                                action;
    bool                                target;
    uint8_t                             keyID;
    uint16_t                            privilege;
    uint16_t                            responseBitLength;						/**< I/O Param: Expected and received response length in bits. Max = STUHFL_D_MAX_RESPONSE_LENGTH*8 */
    uint8_t                             response[STUHFL_D_MAX_RESPONSE_LENGTH];	/**< O Param: Tag response */
    bool                                responseHeaderBit;                      /**< O Param: Tag response header bit */
} STUHFL_T_Gen2_TagPrivilege;
#define STUHFL_O_GEN2_TAGPRIVILEGE_INIT() {{0}, true, true, false, true, 0, 0, 0, {0}, false}

#define STUHFL_D_MAX_READBUFFER_DATA_BYTES  0x0FFFU
typedef struct {
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];
    uint16_t                            wordPtr;
    uint16_t                            bitCount;
    uint16_t                            responseBitLength;						/**< I/O Param: Expected and received response length in bits. Max = STUHFL_D_MAX_RESPONSE_LENGTH*8 */
    uint8_t                             response[STUHFL_D_MAX_RESPONSE_LENGTH];	/**< O Param: Tag response */
    bool                                responseHeaderBit;                      /**< O Param: Tag response header bit */
} STUHFL_T_Gen2_ReadBuffer;
#define STUHFL_O_GEN2_READBUFFER_INIT() {{0}, 0, 0, 0, {0}, false}

#define STUHFL_D_UNTRACABLE_HIDE_TID_NONE       0
#define STUHFL_D_UNTRACABLE_HIDE_TID_SOME       1
#define STUHFL_D_UNTRACABLE_HIDE_TID_ALL        2
#define STUHFL_D_UNTRACABLE_RANGE_NORMAL        0
#define STUHFL_D_UNTRACABLE_RANGE_TOGGLE_TMP    1
#define STUHFL_D_UNTRACABLE_RANGE_REDUCED       2
typedef struct {
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];
    bool                                assertU;
    bool                                hideEPC;
    uint8_t                             newEPC_L;
    uint8_t                             hideTID;
    bool                                hideUser;
    uint8_t                             range;
    uint8_t                             tagErrorCode;                       /**< O Param: Error code received from TAG in case operation failed, otherwise 0 */
} STUHFL_T_Gen2_Untraceable;
#define STUHFL_O_GEN2_Untraceable_INIT() {{0}, false, true, 0, STUHFL_D_UNTRACABLE_HIDE_TID_ALL, true, STUHFL_D_UNTRACABLE_RANGE_REDUCED, 0}

#if 0 // RFU
typedef struct {
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];
    uint16_t                            fileNum;        /* I */
    uint16_t                            fileType;       /* O */
    uint16_t                            fileSize;       /* O */
    uint16_t                            blockSize;      /* O */
    uint8_t                             intPriv;        /* O */
    bool                                lastFile;       /* O */
} STUHFL_T_Gen2_FileOpen;
#define STUHFL_O_GEN2_FILEOPEN_INIT() {{0}, 0, 0, 0, 0, 0, false}

#define STUHFL_D_MAX_FILE_MESSAGES      256U
typedef struct {
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];
    uint16_t                            fileNum;
    uint16_t                            fileType;
    uint16_t                            fileSize;
    uint8_t                             intPriv;
} STUHFL_T_Gen2_FileMessage;
#define STUHFL_O_GEN2_FILEMESSAGE_INIT() {{0}, 0, 0, 0, 0, 0, false}

typedef struct {
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];
    bool                                senRep;         /* I */
    bool                                incRepLen;      /* I */
    uint16_t                            fileNum;        /* I */
    uint8_t                             additionalFiles;/* I */

    uint8_t                             numMessages;                            /* O: Number of messages in this reply */
    STUHFL_T_Gen2_FileMessage           messages[STUHFL_D_MAX_FILE_MESSAGES];   /* O: [FileNum, FileType, FileSize, IntPriv] */
    uint16_t                            blockSize;                              /* O: Block size in words */
    uint16_t                            availableFileSize;                      /* O: Allocateable memory in blocks */
} STUHFL_T_Gen2_FileList;
#define STUHFL_O_GEN2_FILELIST_INIT() {{0}, true, true, 0, 0, 0, {0}, 0, 0}


#define STUHFL_D_MAX_KEY_PRIVILEGE      256U
typedef struct {
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];
    uint8_t                             keyID;
    uint8_t                             privilege;
} STUHFL_T_Gen2_KeyPrivilege;

typedef struct {
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];
    bool                                senRep;         /* I */
    bool                                incRepLen;      /* I */
    uint8_t                             action;         /* I */
    uint8_t                             keyID;          /* I/0 */
    uint8_t                             provilege;      /* I/0 */
    uint16_t                            fileNum;        /* O: Number of messages in this reply */

    /* Relevant for action=0b110 (Read all keys) or action=0b111 (Modify all keys) */
    uint16_t                            numKeys;                        /* O: Number of messages in this reply */
    STUHFL_T_Gen2_KeyPrivilege          keyPrivilege[STUHFL_D_MAX_KEY_PRIVILEGE]; /* O: [FileNum, FileType, FileSize, IntPriv] */
} STUHFL_T_Gen2_FilePrivilege;
#define STUHFL_O_GEN2_FILEPRIVILEGE_INIT() {{0}, true, true, 0, 0, 0, 0, 0, {0}}

typedef struct {
    uint8_t                             pwd[STUHFL_D_PASSWORD_LEN];
    bool                                senRep;         /* I */
    bool                                incRepLen;      /* I */
    uint8_t                             fileType;       /* I */
    uint16_t                            fileSize;       /* I */
    uint16_t                            fileNum;        /* O: Number of messages in this reply */
} STUHFL_T_Gen2_FileSetup;
#define STUHFL_O_GEN2_FILESETUP_INIT() {{0}, true, true, 0, 0, 0}
#endif
#pragma pack(pop)

// --------------------------------------------------------------------------
/**
 * Perform Gen2 Inventory depending on the current inventory and gen2 configuration
 * @param invOption: See STUHFL_T_InventoryOption struct for further info
 * @param invData: See STUHFL_T_InventoryData struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_Inventory(STUHFL_T_InventoryOption *invOption, STUHFL_T_InventoryData *invData);
/**
 * Perform Gen2 Select command to select or filter transponders
 * @param selData: See STUHFL_T_Gen2_Select struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_Select(STUHFL_T_Gen2_Select *selData);
/**
 * Perform Gen2 Read command to read from the Gen2 transponders
 * @param readData: See STUHFL_T_Read struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_Read(STUHFL_T_Gen2_Read *readData);
/**
 * Perform Gen2 Write command to write data to Gen2 transponders
 * @param writeData: See STUHFL_T_Write struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_Write(STUHFL_T_Gen2_Write *writeData);
/**
 * Perform Gen2 Block Write command to write block data to Gen2 transponders
 * @param blockWrite: See STUHFL_T_BlockWrite struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_BlockWrite(STUHFL_T_Gen2_BlockWrite *blockWrite);
/**
 * Perform Gen2 Lock command to lock transponders
 * @param lockData: See STUHFL_T_Gen2_Lock struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_Lock(STUHFL_T_Gen2_Lock *lockData);
/**
 * Perform Gen2 Kill command to kill transponders.
 * NOTE: After this command your transponders will not be functional anymore
 * @param killData: See STUHFL_T_Kill struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_Kill(STUHFL_T_Gen2_Kill *killData);
/**
 * Perform generic Gen2 bit exchange
 * @param genericCmdDataSnd: See STUHFL_T_Gen2_GenericCmd struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_GenericCmd(STUHFL_T_Gen2_GenericCmd *genericCmd);
/**
 * Perform RSSI measurement during Gen2 Query commmand
 * @param queryMeasureRssi: See STUHFL_T_Gen2_QueryMeasureRssi struct for further info
 *
 * @return error code
*/
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_QueryMeasureRssi(STUHFL_T_Gen2_QueryMeasureRssi *queryMeasureRssi);

// ..........................................................................
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_Challenge      (STUHFL_T_Gen2_Challenge        *challenge);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_Authenticate	(STUHFL_T_Gen2_Authenticate     *authenticate);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_AuthComm		(STUHFL_T_Gen2_AuthComm		    *authComm);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_SecureComm     (STUHFL_T_Gen2_SecureComm       *secureComm);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_KeyUpdate      (STUHFL_T_Gen2_KeyUpdate        *keyUpdate);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_TagPrivilege   (STUHFL_T_Gen2_TagPrivilege     *tagPrivilege);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_ReadBuffer     (STUHFL_T_Gen2_ReadBuffer       *readBuffer);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_Untraceable    (STUHFL_T_Gen2_Untraceable      *untraceable);
#if 0 // RFU
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_FileOpen       (STUHFL_T_Gen2_FileOpen         *fileOpen);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_FileList       (STUHFL_T_Gen2_FileList         *fileList);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_FilePrivilege  (STUHFL_T_Gen2_FilePrivilege    *filePrivilege);
STUHFL_DLL_API STUHFL_T_RET_CODE CALL_CONV STUHFL_F_Gen2_FileSetup      (STUHFL_T_Gen2_FileSetup        *fileSetup);
#endif


#ifdef __cplusplus
}
#endif //__cplusplus

#endif // __STUHFL_SL_GEN2_H

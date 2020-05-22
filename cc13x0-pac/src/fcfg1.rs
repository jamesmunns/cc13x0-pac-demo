#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 160usize],
    #[doc = "0xa0 - Misc configurations"]
    pub misc_conf_1: MISC_CONF_1,
    _reserved1: [u8; 12usize],
    #[doc = "0xb0 - Internal. Only to be used through TI provided API."]
    pub baw_meas_5: BAW_MEAS_5,
    #[doc = "0xb4 - Internal. Only to be used through TI provided API."]
    pub baw_meas_4: BAW_MEAS_4,
    #[doc = "0xb8 - Internal. Only to be used through TI provided API."]
    pub baw_meas_3: BAW_MEAS_3,
    #[doc = "0xbc - Internal. Only to be used through TI provided API."]
    pub baw_meas_2: BAW_MEAS_2,
    #[doc = "0xc0 - Internal. Only to be used through TI provided API."]
    pub baw_meas_1: BAW_MEAS_1,
    #[doc = "0xc4 - Internal. Only to be used through TI provided API."]
    pub config_rf_frontend_div5: CONFIG_RF_FRONTEND_DIV5,
    #[doc = "0xc8 - Internal. Only to be used through TI provided API."]
    pub config_rf_frontend_div6: CONFIG_RF_FRONTEND_DIV6,
    #[doc = "0xcc - Internal. Only to be used through TI provided API."]
    pub config_rf_frontend_div10: CONFIG_RF_FRONTEND_DIV10,
    #[doc = "0xd0 - Internal. Only to be used through TI provided API."]
    pub config_rf_frontend_div12: CONFIG_RF_FRONTEND_DIV12,
    #[doc = "0xd4 - Internal. Only to be used through TI provided API."]
    pub config_rf_frontend_div15: CONFIG_RF_FRONTEND_DIV15,
    #[doc = "0xd8 - Internal. Only to be used through TI provided API."]
    pub config_rf_frontend_div30: CONFIG_RF_FRONTEND_DIV30,
    #[doc = "0xdc - Internal. Only to be used through TI provided API."]
    pub config_synth_div5: CONFIG_SYNTH_DIV5,
    #[doc = "0xe0 - Internal. Only to be used through TI provided API."]
    pub config_synth_div6: CONFIG_SYNTH_DIV6,
    #[doc = "0xe4 - Internal. Only to be used through TI provided API."]
    pub config_synth_div10: CONFIG_SYNTH_DIV10,
    #[doc = "0xe8 - Internal. Only to be used through TI provided API."]
    pub config_synth_div12: CONFIG_SYNTH_DIV12,
    #[doc = "0xec - Internal. Only to be used through TI provided API."]
    pub config_synth_div15: CONFIG_SYNTH_DIV15,
    #[doc = "0xf0 - Internal. Only to be used through TI provided API."]
    pub config_synth_div30: CONFIG_SYNTH_DIV30,
    #[doc = "0xf4 - Internal. Only to be used through TI provided API."]
    pub config_misc_adc_div5: CONFIG_MISC_ADC_DIV5,
    #[doc = "0xf8 - Internal. Only to be used through TI provided API."]
    pub config_misc_adc_div6: CONFIG_MISC_ADC_DIV6,
    #[doc = "0xfc - Internal. Only to be used through TI provided API."]
    pub config_misc_adc_div10: CONFIG_MISC_ADC_DIV10,
    #[doc = "0x100 - Internal. Only to be used through TI provided API."]
    pub config_misc_adc_div12: CONFIG_MISC_ADC_DIV12,
    #[doc = "0x104 - Internal. Only to be used through TI provided API."]
    pub config_misc_adc_div15: CONFIG_MISC_ADC_DIV15,
    #[doc = "0x108 - Internal. Only to be used through TI provided API."]
    pub config_misc_adc_div30: CONFIG_MISC_ADC_DIV30,
    _reserved24: [u8; 12usize],
    #[doc = "0x118 - Shadow of the DIE_ID_0 register in eFuse"]
    pub shdw_die_id_0: SHDW_DIE_ID_0,
    #[doc = "0x11c - Shadow of the DIE_ID_1 register in eFuse"]
    pub shdw_die_id_1: SHDW_DIE_ID_1,
    #[doc = "0x120 - Shadow of the DIE_ID_2 register in eFuse"]
    pub shdw_die_id_2: SHDW_DIE_ID_2,
    #[doc = "0x124 - Shadow of the DIE_ID_3 register in eFuse"]
    pub shdw_die_id_3: SHDW_DIE_ID_3,
    _reserved28: [u8; 16usize],
    #[doc = "0x138 - Internal. Only to be used through TI provided API."]
    pub shdw_osc_bias_ldo_trim: SHDW_OSC_BIAS_LDO_TRIM,
    #[doc = "0x13c - Internal. Only to be used through TI provided API."]
    pub shdw_ana_trim: SHDW_ANA_TRIM,
    _reserved30: [u8; 36usize],
    #[doc = "0x164 - lol"]
    pub flash_number: FLASH_NUMBER,
    _reserved31: [u8; 4usize],
    #[doc = "0x16c - lol2"]
    pub flash_coordinate: FLASH_COORDINATE,
    #[doc = "0x170 - Internal. Only to be used through TI provided API."]
    pub flash_e_p: FLASH_E_P,
    #[doc = "0x174 - Internal. Only to be used through TI provided API."]
    pub flash_c_e_p_r: FLASH_C_E_P_R,
    #[doc = "0x178 - Internal. Only to be used through TI provided API."]
    pub flash_p_r_pv: FLASH_P_R_PV,
    #[doc = "0x17c - Internal. Only to be used through TI provided API."]
    pub flash_eh_seq: FLASH_EH_SEQ,
    #[doc = "0x180 - Internal. Only to be used through TI provided API."]
    pub flash_vhv_e: FLASH_VHV_E,
    #[doc = "0x184 - Internal. Only to be used through TI provided API."]
    pub flash_pp: FLASH_PP,
    #[doc = "0x188 - Internal. Only to be used through TI provided API."]
    pub flash_prog_ep: FLASH_PROG_EP,
    #[doc = "0x18c - Internal. Only to be used through TI provided API."]
    pub flash_era_pw: FLASH_ERA_PW,
    #[doc = "0x190 - Internal. Only to be used through TI provided API."]
    pub flash_vhv: FLASH_VHV,
    #[doc = "0x194 - Internal. Only to be used through TI provided API."]
    pub flash_vhv_pv: FLASH_VHV_PV,
    #[doc = "0x198 - Internal. Only to be used through TI provided API."]
    pub flash_v: FLASH_V,
    _reserved43: [u8; 248usize],
    #[doc = "0x294 - User Identification. Reading this register or the ICEPICK_DEVICE_ID register is the only support way of identifying a device. The value of this register will be written to AON_WUC:JTAGUSERCODE by boot FW while in safezone."]
    pub user_id: USER_ID,
    _reserved44: [u8; 24usize],
    #[doc = "0x2b0 - Internal. Only to be used through TI provided API."]
    pub flash_otp_data3: FLASH_OTP_DATA3,
    #[doc = "0x2b4 - Internal. Only to be used through TI provided API."]
    pub ana2_trim: ANA2_TRIM,
    #[doc = "0x2b8 - Internal. Only to be used through TI provided API."]
    pub ldo_trim: LDO_TRIM,
    _reserved47: [u8; 44usize],
    #[doc = "0x2e8 - MAC BLE Address 0"]
    pub mac_ble_0: MAC_BLE_0,
    #[doc = "0x2ec - MAC BLE Address 1"]
    pub mac_ble_1: MAC_BLE_1,
    #[doc = "0x2f0 - MAC IEEE 802.15.4 Address 0"]
    pub mac_15_4_0: MAC_15_4_0,
    #[doc = "0x2f4 - MAC IEEE 802.15.4 Address 1"]
    pub mac_15_4_1: MAC_15_4_1,
    _reserved51: [u8; 16usize],
    #[doc = "0x308 - Internal. Only to be used through TI provided API."]
    pub flash_otp_data4: FLASH_OTP_DATA4,
    #[doc = "0x30c - Miscellaneous Trim Parameters"]
    pub misc_trim: MISC_TRIM,
    #[doc = "0x310 - Internal. Only to be used through TI provided API."]
    pub rcosc_hf_tempcomp: RCOSC_HF_TEMPCOMP,
    _reserved54: [u8; 4usize],
    #[doc = "0x318 - IcePick Device Identification Reading this register or the USER_ID register is the only support way of identifying a device."]
    pub icepick_device_id: ICEPICK_DEVICE_ID,
    #[doc = "0x31c - Factory Configuration (FCFG1) Revision"]
    pub fcfg1_revision: FCFG1_REVISION,
    #[doc = "0x320 - Misc OTP Data"]
    pub misc_otp_data: MISC_OTP_DATA,
    _reserved57: [u8; 32usize],
    #[doc = "0x344 - IO Configuration"]
    pub ioconf: IOCONF,
    _reserved58: [u8; 4usize],
    #[doc = "0x34c - Internal. Only to be used through TI provided API."]
    pub config_if_adc: CONFIG_IF_ADC,
    #[doc = "0x350 - Internal. Only to be used through TI provided API."]
    pub config_osc_top: CONFIG_OSC_TOP,
    #[doc = "0x354 - Internal. Only to be used through TI provided API."]
    pub config_rf_frontend: CONFIG_RF_FRONTEND,
    #[doc = "0x358 - Internal. Only to be used through TI provided API."]
    pub config_synth: CONFIG_SYNTH,
    #[doc = "0x35c - AUX_ADC Gain in Absolute Reference Mode"]
    pub soc_adc_abs_gain: SOC_ADC_ABS_GAIN,
    #[doc = "0x360 - AUX_ADC Gain in Relative Reference Mode"]
    pub soc_adc_rel_gain: SOC_ADC_REL_GAIN,
    _reserved64: [u8; 4usize],
    #[doc = "0x368 - AUX_ADC Temperature Offsets in Absolute Reference Mode"]
    pub soc_adc_offset_int: SOC_ADC_OFFSET_INT,
    #[doc = "0x36c - Internal. Only to be used through TI provided API."]
    pub soc_adc_ref_trim_and_offset_ext: SOC_ADC_REF_TRIM_AND_OFFSET_EXT,
    #[doc = "0x370 - Internal. Only to be used through TI provided API."]
    pub ampcomp_th1: AMPCOMP_TH1,
    #[doc = "0x374 - Internal. Only to be used through TI provided API."]
    pub ampcomp_th2: AMPCOMP_TH2,
    #[doc = "0x378 - Internal. Only to be used through TI provided API."]
    pub ampcomp_ctrl1: AMPCOMP_CTRL1,
    #[doc = "0x37c - Internal. Only to be used through TI provided API."]
    pub anabypass_value2: ANABYPASS_VALUE2,
    #[doc = "0x380 - Internal. Only to be used through TI provided API."]
    pub config_misc_adc: CONFIG_MISC_ADC,
    _reserved71: [u8; 4usize],
    #[doc = "0x388 - Internal. Only to be used through TI provided API."]
    pub volt_trim: VOLT_TRIM,
    #[doc = "0x38c - OSC Configuration"]
    pub osc_conf: OSC_CONF,
    _reserved73: [u8; 4usize],
    #[doc = "0x394 - Internal. Only to be used through TI provided API."]
    pub cap_trim: CAP_TRIM,
    #[doc = "0x398 - Internal. Only to be used through TI provided API."]
    pub misc_otp_data_1: MISC_OTP_DATA_1,
    #[doc = "0x39c - Power Down Current Control 20C"]
    pub pwd_curr_20c: PWD_CURR_20C,
    #[doc = "0x3a0 - Power Down Current Control 35C"]
    pub pwd_curr_35c: PWD_CURR_35C,
    #[doc = "0x3a4 - Power Down Current Control 50C"]
    pub pwd_curr_50c: PWD_CURR_50C,
    #[doc = "0x3a8 - Power Down Current Control 65C"]
    pub pwd_curr_65c: PWD_CURR_65C,
    #[doc = "0x3ac - Power Down Current Control 80C"]
    pub pwd_curr_80c: PWD_CURR_80C,
    #[doc = "0x3b0 - Power Down Current Control 95C"]
    pub pwd_curr_95c: PWD_CURR_95C,
    #[doc = "0x3b4 - Power Down Current Control 110C"]
    pub pwd_curr_110c: PWD_CURR_110C,
    #[doc = "0x3b8 - Power Down Current Control 125C"]
    pub pwd_curr_125c: PWD_CURR_125C,
}
#[doc = "Misc configurations\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_conf_1](misc_conf_1) module"]
pub type MISC_CONF_1 = crate::Reg<u32, _MISC_CONF_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_CONF_1;
#[doc = "`read()` method returns [misc_conf_1::R](misc_conf_1::R) reader structure"]
impl crate::Readable for MISC_CONF_1 {}
#[doc = "Misc configurations"]
pub mod misc_conf_1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baw_meas_5](baw_meas_5) module"]
pub type BAW_MEAS_5 = crate::Reg<u32, _BAW_MEAS_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAW_MEAS_5;
#[doc = "`read()` method returns [baw_meas_5::R](baw_meas_5::R) reader structure"]
impl crate::Readable for BAW_MEAS_5 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod baw_meas_5;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baw_meas_4](baw_meas_4) module"]
pub type BAW_MEAS_4 = crate::Reg<u32, _BAW_MEAS_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAW_MEAS_4;
#[doc = "`read()` method returns [baw_meas_4::R](baw_meas_4::R) reader structure"]
impl crate::Readable for BAW_MEAS_4 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod baw_meas_4;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baw_meas_3](baw_meas_3) module"]
pub type BAW_MEAS_3 = crate::Reg<u32, _BAW_MEAS_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAW_MEAS_3;
#[doc = "`read()` method returns [baw_meas_3::R](baw_meas_3::R) reader structure"]
impl crate::Readable for BAW_MEAS_3 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod baw_meas_3;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baw_meas_2](baw_meas_2) module"]
pub type BAW_MEAS_2 = crate::Reg<u32, _BAW_MEAS_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAW_MEAS_2;
#[doc = "`read()` method returns [baw_meas_2::R](baw_meas_2::R) reader structure"]
impl crate::Readable for BAW_MEAS_2 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod baw_meas_2;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baw_meas_1](baw_meas_1) module"]
pub type BAW_MEAS_1 = crate::Reg<u32, _BAW_MEAS_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAW_MEAS_1;
#[doc = "`read()` method returns [baw_meas_1::R](baw_meas_1::R) reader structure"]
impl crate::Readable for BAW_MEAS_1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod baw_meas_1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_rf_frontend_div5](config_rf_frontend_div5) module"]
pub type CONFIG_RF_FRONTEND_DIV5 = crate::Reg<u32, _CONFIG_RF_FRONTEND_DIV5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_RF_FRONTEND_DIV5;
#[doc = "`read()` method returns [config_rf_frontend_div5::R](config_rf_frontend_div5::R) reader structure"]
impl crate::Readable for CONFIG_RF_FRONTEND_DIV5 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_frontend_div5;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_rf_frontend_div6](config_rf_frontend_div6) module"]
pub type CONFIG_RF_FRONTEND_DIV6 = crate::Reg<u32, _CONFIG_RF_FRONTEND_DIV6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_RF_FRONTEND_DIV6;
#[doc = "`read()` method returns [config_rf_frontend_div6::R](config_rf_frontend_div6::R) reader structure"]
impl crate::Readable for CONFIG_RF_FRONTEND_DIV6 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_frontend_div6;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_rf_frontend_div10](config_rf_frontend_div10) module"]
pub type CONFIG_RF_FRONTEND_DIV10 = crate::Reg<u32, _CONFIG_RF_FRONTEND_DIV10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_RF_FRONTEND_DIV10;
#[doc = "`read()` method returns [config_rf_frontend_div10::R](config_rf_frontend_div10::R) reader structure"]
impl crate::Readable for CONFIG_RF_FRONTEND_DIV10 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_frontend_div10;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_rf_frontend_div12](config_rf_frontend_div12) module"]
pub type CONFIG_RF_FRONTEND_DIV12 = crate::Reg<u32, _CONFIG_RF_FRONTEND_DIV12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_RF_FRONTEND_DIV12;
#[doc = "`read()` method returns [config_rf_frontend_div12::R](config_rf_frontend_div12::R) reader structure"]
impl crate::Readable for CONFIG_RF_FRONTEND_DIV12 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_frontend_div12;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_rf_frontend_div15](config_rf_frontend_div15) module"]
pub type CONFIG_RF_FRONTEND_DIV15 = crate::Reg<u32, _CONFIG_RF_FRONTEND_DIV15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_RF_FRONTEND_DIV15;
#[doc = "`read()` method returns [config_rf_frontend_div15::R](config_rf_frontend_div15::R) reader structure"]
impl crate::Readable for CONFIG_RF_FRONTEND_DIV15 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_frontend_div15;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_rf_frontend_div30](config_rf_frontend_div30) module"]
pub type CONFIG_RF_FRONTEND_DIV30 = crate::Reg<u32, _CONFIG_RF_FRONTEND_DIV30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_RF_FRONTEND_DIV30;
#[doc = "`read()` method returns [config_rf_frontend_div30::R](config_rf_frontend_div30::R) reader structure"]
impl crate::Readable for CONFIG_RF_FRONTEND_DIV30 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_frontend_div30;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div5](config_synth_div5) module"]
pub type CONFIG_SYNTH_DIV5 = crate::Reg<u32, _CONFIG_SYNTH_DIV5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV5;
#[doc = "`read()` method returns [config_synth_div5::R](config_synth_div5::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV5 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div5;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div6](config_synth_div6) module"]
pub type CONFIG_SYNTH_DIV6 = crate::Reg<u32, _CONFIG_SYNTH_DIV6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV6;
#[doc = "`read()` method returns [config_synth_div6::R](config_synth_div6::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV6 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div6;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div10](config_synth_div10) module"]
pub type CONFIG_SYNTH_DIV10 = crate::Reg<u32, _CONFIG_SYNTH_DIV10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV10;
#[doc = "`read()` method returns [config_synth_div10::R](config_synth_div10::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV10 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div10;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div12](config_synth_div12) module"]
pub type CONFIG_SYNTH_DIV12 = crate::Reg<u32, _CONFIG_SYNTH_DIV12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV12;
#[doc = "`read()` method returns [config_synth_div12::R](config_synth_div12::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV12 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div12;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div15](config_synth_div15) module"]
pub type CONFIG_SYNTH_DIV15 = crate::Reg<u32, _CONFIG_SYNTH_DIV15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV15;
#[doc = "`read()` method returns [config_synth_div15::R](config_synth_div15::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV15 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div15;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div30](config_synth_div30) module"]
pub type CONFIG_SYNTH_DIV30 = crate::Reg<u32, _CONFIG_SYNTH_DIV30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH_DIV30;
#[doc = "`read()` method returns [config_synth_div30::R](config_synth_div30::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV30 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth_div30;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_misc_adc_div5](config_misc_adc_div5) module"]
pub type CONFIG_MISC_ADC_DIV5 = crate::Reg<u32, _CONFIG_MISC_ADC_DIV5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_MISC_ADC_DIV5;
#[doc = "`read()` method returns [config_misc_adc_div5::R](config_misc_adc_div5::R) reader structure"]
impl crate::Readable for CONFIG_MISC_ADC_DIV5 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_misc_adc_div5;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_misc_adc_div6](config_misc_adc_div6) module"]
pub type CONFIG_MISC_ADC_DIV6 = crate::Reg<u32, _CONFIG_MISC_ADC_DIV6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_MISC_ADC_DIV6;
#[doc = "`read()` method returns [config_misc_adc_div6::R](config_misc_adc_div6::R) reader structure"]
impl crate::Readable for CONFIG_MISC_ADC_DIV6 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_misc_adc_div6;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_misc_adc_div10](config_misc_adc_div10) module"]
pub type CONFIG_MISC_ADC_DIV10 = crate::Reg<u32, _CONFIG_MISC_ADC_DIV10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_MISC_ADC_DIV10;
#[doc = "`read()` method returns [config_misc_adc_div10::R](config_misc_adc_div10::R) reader structure"]
impl crate::Readable for CONFIG_MISC_ADC_DIV10 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_misc_adc_div10;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_misc_adc_div12](config_misc_adc_div12) module"]
pub type CONFIG_MISC_ADC_DIV12 = crate::Reg<u32, _CONFIG_MISC_ADC_DIV12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_MISC_ADC_DIV12;
#[doc = "`read()` method returns [config_misc_adc_div12::R](config_misc_adc_div12::R) reader structure"]
impl crate::Readable for CONFIG_MISC_ADC_DIV12 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_misc_adc_div12;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_misc_adc_div15](config_misc_adc_div15) module"]
pub type CONFIG_MISC_ADC_DIV15 = crate::Reg<u32, _CONFIG_MISC_ADC_DIV15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_MISC_ADC_DIV15;
#[doc = "`read()` method returns [config_misc_adc_div15::R](config_misc_adc_div15::R) reader structure"]
impl crate::Readable for CONFIG_MISC_ADC_DIV15 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_misc_adc_div15;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_misc_adc_div30](config_misc_adc_div30) module"]
pub type CONFIG_MISC_ADC_DIV30 = crate::Reg<u32, _CONFIG_MISC_ADC_DIV30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_MISC_ADC_DIV30;
#[doc = "`read()` method returns [config_misc_adc_div30::R](config_misc_adc_div30::R) reader structure"]
impl crate::Readable for CONFIG_MISC_ADC_DIV30 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_misc_adc_div30;
#[doc = "Shadow of the DIE_ID_0 register in eFuse\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_die_id_0](shdw_die_id_0) module"]
pub type SHDW_DIE_ID_0 = crate::Reg<u32, _SHDW_DIE_ID_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHDW_DIE_ID_0;
#[doc = "`read()` method returns [shdw_die_id_0::R](shdw_die_id_0::R) reader structure"]
impl crate::Readable for SHDW_DIE_ID_0 {}
#[doc = "Shadow of the DIE_ID_0 register in eFuse"]
pub mod shdw_die_id_0;
#[doc = "Shadow of the DIE_ID_1 register in eFuse\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_die_id_1](shdw_die_id_1) module"]
pub type SHDW_DIE_ID_1 = crate::Reg<u32, _SHDW_DIE_ID_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHDW_DIE_ID_1;
#[doc = "`read()` method returns [shdw_die_id_1::R](shdw_die_id_1::R) reader structure"]
impl crate::Readable for SHDW_DIE_ID_1 {}
#[doc = "Shadow of the DIE_ID_1 register in eFuse"]
pub mod shdw_die_id_1;
#[doc = "Shadow of the DIE_ID_2 register in eFuse\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_die_id_2](shdw_die_id_2) module"]
pub type SHDW_DIE_ID_2 = crate::Reg<u32, _SHDW_DIE_ID_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHDW_DIE_ID_2;
#[doc = "`read()` method returns [shdw_die_id_2::R](shdw_die_id_2::R) reader structure"]
impl crate::Readable for SHDW_DIE_ID_2 {}
#[doc = "Shadow of the DIE_ID_2 register in eFuse"]
pub mod shdw_die_id_2;
#[doc = "Shadow of the DIE_ID_3 register in eFuse\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_die_id_3](shdw_die_id_3) module"]
pub type SHDW_DIE_ID_3 = crate::Reg<u32, _SHDW_DIE_ID_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHDW_DIE_ID_3;
#[doc = "`read()` method returns [shdw_die_id_3::R](shdw_die_id_3::R) reader structure"]
impl crate::Readable for SHDW_DIE_ID_3 {}
#[doc = "Shadow of the DIE_ID_3 register in eFuse"]
pub mod shdw_die_id_3;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_osc_bias_ldo_trim](shdw_osc_bias_ldo_trim) module"]
pub type SHDW_OSC_BIAS_LDO_TRIM = crate::Reg<u32, _SHDW_OSC_BIAS_LDO_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHDW_OSC_BIAS_LDO_TRIM;
#[doc = "`read()` method returns [shdw_osc_bias_ldo_trim::R](shdw_osc_bias_ldo_trim::R) reader structure"]
impl crate::Readable for SHDW_OSC_BIAS_LDO_TRIM {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod shdw_osc_bias_ldo_trim;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_ana_trim](shdw_ana_trim) module"]
pub type SHDW_ANA_TRIM = crate::Reg<u32, _SHDW_ANA_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHDW_ANA_TRIM;
#[doc = "`read()` method returns [shdw_ana_trim::R](shdw_ana_trim::R) reader structure"]
impl crate::Readable for SHDW_ANA_TRIM {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod shdw_ana_trim;
#[doc = "lol\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_number](flash_number) module"]
pub type FLASH_NUMBER = crate::Reg<u32, _FLASH_NUMBER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_NUMBER;
#[doc = "`read()` method returns [flash_number::R](flash_number::R) reader structure"]
impl crate::Readable for FLASH_NUMBER {}
#[doc = "lol"]
pub mod flash_number;
#[doc = "lol2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_coordinate](flash_coordinate) module"]
pub type FLASH_COORDINATE = crate::Reg<u32, _FLASH_COORDINATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_COORDINATE;
#[doc = "`read()` method returns [flash_coordinate::R](flash_coordinate::R) reader structure"]
impl crate::Readable for FLASH_COORDINATE {}
#[doc = "lol2"]
pub mod flash_coordinate;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_e_p](flash_e_p) module"]
pub type FLASH_E_P = crate::Reg<u32, _FLASH_E_P>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_E_P;
#[doc = "`read()` method returns [flash_e_p::R](flash_e_p::R) reader structure"]
impl crate::Readable for FLASH_E_P {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_e_p;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_c_e_p_r](flash_c_e_p_r) module"]
pub type FLASH_C_E_P_R = crate::Reg<u32, _FLASH_C_E_P_R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_C_E_P_R;
#[doc = "`read()` method returns [flash_c_e_p_r::R](flash_c_e_p_r::R) reader structure"]
impl crate::Readable for FLASH_C_E_P_R {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_c_e_p_r;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_p_r_pv](flash_p_r_pv) module"]
pub type FLASH_P_R_PV = crate::Reg<u32, _FLASH_P_R_PV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_P_R_PV;
#[doc = "`read()` method returns [flash_p_r_pv::R](flash_p_r_pv::R) reader structure"]
impl crate::Readable for FLASH_P_R_PV {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_p_r_pv;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_eh_seq](flash_eh_seq) module"]
pub type FLASH_EH_SEQ = crate::Reg<u32, _FLASH_EH_SEQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_EH_SEQ;
#[doc = "`read()` method returns [flash_eh_seq::R](flash_eh_seq::R) reader structure"]
impl crate::Readable for FLASH_EH_SEQ {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_eh_seq;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_vhv_e](flash_vhv_e) module"]
pub type FLASH_VHV_E = crate::Reg<u32, _FLASH_VHV_E>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_VHV_E;
#[doc = "`read()` method returns [flash_vhv_e::R](flash_vhv_e::R) reader structure"]
impl crate::Readable for FLASH_VHV_E {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_vhv_e;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_pp](flash_pp) module"]
pub type FLASH_PP = crate::Reg<u32, _FLASH_PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_PP;
#[doc = "`read()` method returns [flash_pp::R](flash_pp::R) reader structure"]
impl crate::Readable for FLASH_PP {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_pp;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_prog_ep](flash_prog_ep) module"]
pub type FLASH_PROG_EP = crate::Reg<u32, _FLASH_PROG_EP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_PROG_EP;
#[doc = "`read()` method returns [flash_prog_ep::R](flash_prog_ep::R) reader structure"]
impl crate::Readable for FLASH_PROG_EP {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_prog_ep;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_era_pw](flash_era_pw) module"]
pub type FLASH_ERA_PW = crate::Reg<u32, _FLASH_ERA_PW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_ERA_PW;
#[doc = "`read()` method returns [flash_era_pw::R](flash_era_pw::R) reader structure"]
impl crate::Readable for FLASH_ERA_PW {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_era_pw;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_vhv](flash_vhv) module"]
pub type FLASH_VHV = crate::Reg<u32, _FLASH_VHV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_VHV;
#[doc = "`read()` method returns [flash_vhv::R](flash_vhv::R) reader structure"]
impl crate::Readable for FLASH_VHV {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_vhv;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_vhv_pv](flash_vhv_pv) module"]
pub type FLASH_VHV_PV = crate::Reg<u32, _FLASH_VHV_PV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_VHV_PV;
#[doc = "`read()` method returns [flash_vhv_pv::R](flash_vhv_pv::R) reader structure"]
impl crate::Readable for FLASH_VHV_PV {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_vhv_pv;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_v](flash_v) module"]
pub type FLASH_V = crate::Reg<u32, _FLASH_V>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_V;
#[doc = "`read()` method returns [flash_v::R](flash_v::R) reader structure"]
impl crate::Readable for FLASH_V {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_v;
#[doc = "User Identification. Reading this register or the ICEPICK_DEVICE_ID register is the only support way of identifying a device. The value of this register will be written to AON_WUC:JTAGUSERCODE by boot FW while in safezone.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user_id](user_id) module"]
pub type USER_ID = crate::Reg<u32, _USER_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USER_ID;
#[doc = "`read()` method returns [user_id::R](user_id::R) reader structure"]
impl crate::Readable for USER_ID {}
#[doc = "User Identification. Reading this register or the ICEPICK_DEVICE_ID register is the only support way of identifying a device. The value of this register will be written to AON_WUC:JTAGUSERCODE by boot FW while in safezone."]
pub mod user_id;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_otp_data3](flash_otp_data3) module"]
pub type FLASH_OTP_DATA3 = crate::Reg<u32, _FLASH_OTP_DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_OTP_DATA3;
#[doc = "`read()` method returns [flash_otp_data3::R](flash_otp_data3::R) reader structure"]
impl crate::Readable for FLASH_OTP_DATA3 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_otp_data3;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana2_trim](ana2_trim) module"]
pub type ANA2_TRIM = crate::Reg<u32, _ANA2_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANA2_TRIM;
#[doc = "`read()` method returns [ana2_trim::R](ana2_trim::R) reader structure"]
impl crate::Readable for ANA2_TRIM {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ana2_trim;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo_trim](ldo_trim) module"]
pub type LDO_TRIM = crate::Reg<u32, _LDO_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LDO_TRIM;
#[doc = "`read()` method returns [ldo_trim::R](ldo_trim::R) reader structure"]
impl crate::Readable for LDO_TRIM {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ldo_trim;
#[doc = "MAC BLE Address 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_ble_0](mac_ble_0) module"]
pub type MAC_BLE_0 = crate::Reg<u32, _MAC_BLE_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_BLE_0;
#[doc = "`read()` method returns [mac_ble_0::R](mac_ble_0::R) reader structure"]
impl crate::Readable for MAC_BLE_0 {}
#[doc = "MAC BLE Address 0"]
pub mod mac_ble_0;
#[doc = "MAC BLE Address 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_ble_1](mac_ble_1) module"]
pub type MAC_BLE_1 = crate::Reg<u32, _MAC_BLE_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_BLE_1;
#[doc = "`read()` method returns [mac_ble_1::R](mac_ble_1::R) reader structure"]
impl crate::Readable for MAC_BLE_1 {}
#[doc = "MAC BLE Address 1"]
pub mod mac_ble_1;
#[doc = "MAC IEEE 802.15.4 Address 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_15_4_0](mac_15_4_0) module"]
pub type MAC_15_4_0 = crate::Reg<u32, _MAC_15_4_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_15_4_0;
#[doc = "`read()` method returns [mac_15_4_0::R](mac_15_4_0::R) reader structure"]
impl crate::Readable for MAC_15_4_0 {}
#[doc = "MAC IEEE 802.15.4 Address 0"]
pub mod mac_15_4_0;
#[doc = "MAC IEEE 802.15.4 Address 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_15_4_1](mac_15_4_1) module"]
pub type MAC_15_4_1 = crate::Reg<u32, _MAC_15_4_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC_15_4_1;
#[doc = "`read()` method returns [mac_15_4_1::R](mac_15_4_1::R) reader structure"]
impl crate::Readable for MAC_15_4_1 {}
#[doc = "MAC IEEE 802.15.4 Address 1"]
pub mod mac_15_4_1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_otp_data4](flash_otp_data4) module"]
pub type FLASH_OTP_DATA4 = crate::Reg<u32, _FLASH_OTP_DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_OTP_DATA4;
#[doc = "`read()` method returns [flash_otp_data4::R](flash_otp_data4::R) reader structure"]
impl crate::Readable for FLASH_OTP_DATA4 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_otp_data4;
#[doc = "Miscellaneous Trim Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_trim](misc_trim) module"]
pub type MISC_TRIM = crate::Reg<u32, _MISC_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_TRIM;
#[doc = "`read()` method returns [misc_trim::R](misc_trim::R) reader structure"]
impl crate::Readable for MISC_TRIM {}
#[doc = "Miscellaneous Trim Parameters"]
pub mod misc_trim;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcosc_hf_tempcomp](rcosc_hf_tempcomp) module"]
pub type RCOSC_HF_TEMPCOMP = crate::Reg<u32, _RCOSC_HF_TEMPCOMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCOSC_HF_TEMPCOMP;
#[doc = "`read()` method returns [rcosc_hf_tempcomp::R](rcosc_hf_tempcomp::R) reader structure"]
impl crate::Readable for RCOSC_HF_TEMPCOMP {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod rcosc_hf_tempcomp;
#[doc = "IcePick Device Identification Reading this register or the USER_ID register is the only support way of identifying a device.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icepick_device_id](icepick_device_id) module"]
pub type ICEPICK_DEVICE_ID = crate::Reg<u32, _ICEPICK_DEVICE_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICEPICK_DEVICE_ID;
#[doc = "`read()` method returns [icepick_device_id::R](icepick_device_id::R) reader structure"]
impl crate::Readable for ICEPICK_DEVICE_ID {}
#[doc = "IcePick Device Identification Reading this register or the USER_ID register is the only support way of identifying a device."]
pub mod icepick_device_id;
#[doc = "Factory Configuration (FCFG1) Revision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg1_revision](fcfg1_revision) module"]
pub type FCFG1_REVISION = crate::Reg<u32, _FCFG1_REVISION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG1_REVISION;
#[doc = "`read()` method returns [fcfg1_revision::R](fcfg1_revision::R) reader structure"]
impl crate::Readable for FCFG1_REVISION {}
#[doc = "Factory Configuration (FCFG1) Revision"]
pub mod fcfg1_revision;
#[doc = "Misc OTP Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_otp_data](misc_otp_data) module"]
pub type MISC_OTP_DATA = crate::Reg<u32, _MISC_OTP_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_OTP_DATA;
#[doc = "`read()` method returns [misc_otp_data::R](misc_otp_data::R) reader structure"]
impl crate::Readable for MISC_OTP_DATA {}
#[doc = "Misc OTP Data"]
pub mod misc_otp_data;
#[doc = "IO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioconf](ioconf) module"]
pub type IOCONF = crate::Reg<u32, _IOCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCONF;
#[doc = "`read()` method returns [ioconf::R](ioconf::R) reader structure"]
impl crate::Readable for IOCONF {}
#[doc = "IO Configuration"]
pub mod ioconf;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_if_adc](config_if_adc) module"]
pub type CONFIG_IF_ADC = crate::Reg<u32, _CONFIG_IF_ADC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_IF_ADC;
#[doc = "`read()` method returns [config_if_adc::R](config_if_adc::R) reader structure"]
impl crate::Readable for CONFIG_IF_ADC {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_if_adc;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_osc_top](config_osc_top) module"]
pub type CONFIG_OSC_TOP = crate::Reg<u32, _CONFIG_OSC_TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_OSC_TOP;
#[doc = "`read()` method returns [config_osc_top::R](config_osc_top::R) reader structure"]
impl crate::Readable for CONFIG_OSC_TOP {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_osc_top;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_rf_frontend](config_rf_frontend) module"]
pub type CONFIG_RF_FRONTEND = crate::Reg<u32, _CONFIG_RF_FRONTEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_RF_FRONTEND;
#[doc = "`read()` method returns [config_rf_frontend::R](config_rf_frontend::R) reader structure"]
impl crate::Readable for CONFIG_RF_FRONTEND {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_rf_frontend;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth](config_synth) module"]
pub type CONFIG_SYNTH = crate::Reg<u32, _CONFIG_SYNTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_SYNTH;
#[doc = "`read()` method returns [config_synth::R](config_synth::R) reader structure"]
impl crate::Readable for CONFIG_SYNTH {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_synth;
#[doc = "AUX_ADC Gain in Absolute Reference Mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_adc_abs_gain](soc_adc_abs_gain) module"]
pub type SOC_ADC_ABS_GAIN = crate::Reg<u32, _SOC_ADC_ABS_GAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOC_ADC_ABS_GAIN;
#[doc = "`read()` method returns [soc_adc_abs_gain::R](soc_adc_abs_gain::R) reader structure"]
impl crate::Readable for SOC_ADC_ABS_GAIN {}
#[doc = "AUX_ADC Gain in Absolute Reference Mode"]
pub mod soc_adc_abs_gain;
#[doc = "AUX_ADC Gain in Relative Reference Mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_adc_rel_gain](soc_adc_rel_gain) module"]
pub type SOC_ADC_REL_GAIN = crate::Reg<u32, _SOC_ADC_REL_GAIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOC_ADC_REL_GAIN;
#[doc = "`read()` method returns [soc_adc_rel_gain::R](soc_adc_rel_gain::R) reader structure"]
impl crate::Readable for SOC_ADC_REL_GAIN {}
#[doc = "AUX_ADC Gain in Relative Reference Mode"]
pub mod soc_adc_rel_gain;
#[doc = "AUX_ADC Temperature Offsets in Absolute Reference Mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_adc_offset_int](soc_adc_offset_int) module"]
pub type SOC_ADC_OFFSET_INT = crate::Reg<u32, _SOC_ADC_OFFSET_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOC_ADC_OFFSET_INT;
#[doc = "`read()` method returns [soc_adc_offset_int::R](soc_adc_offset_int::R) reader structure"]
impl crate::Readable for SOC_ADC_OFFSET_INT {}
#[doc = "AUX_ADC Temperature Offsets in Absolute Reference Mode"]
pub mod soc_adc_offset_int;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_adc_ref_trim_and_offset_ext](soc_adc_ref_trim_and_offset_ext) module"]
pub type SOC_ADC_REF_TRIM_AND_OFFSET_EXT = crate::Reg<u32, _SOC_ADC_REF_TRIM_AND_OFFSET_EXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOC_ADC_REF_TRIM_AND_OFFSET_EXT;
#[doc = "`read()` method returns [soc_adc_ref_trim_and_offset_ext::R](soc_adc_ref_trim_and_offset_ext::R) reader structure"]
impl crate::Readable for SOC_ADC_REF_TRIM_AND_OFFSET_EXT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod soc_adc_ref_trim_and_offset_ext;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcomp_th1](ampcomp_th1) module"]
pub type AMPCOMP_TH1 = crate::Reg<u32, _AMPCOMP_TH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMPCOMP_TH1;
#[doc = "`read()` method returns [ampcomp_th1::R](ampcomp_th1::R) reader structure"]
impl crate::Readable for AMPCOMP_TH1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_th1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcomp_th2](ampcomp_th2) module"]
pub type AMPCOMP_TH2 = crate::Reg<u32, _AMPCOMP_TH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMPCOMP_TH2;
#[doc = "`read()` method returns [ampcomp_th2::R](ampcomp_th2::R) reader structure"]
impl crate::Readable for AMPCOMP_TH2 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_th2;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcomp_ctrl1](ampcomp_ctrl1) module"]
pub type AMPCOMP_CTRL1 = crate::Reg<u32, _AMPCOMP_CTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMPCOMP_CTRL1;
#[doc = "`read()` method returns [ampcomp_ctrl1::R](ampcomp_ctrl1::R) reader structure"]
impl crate::Readable for AMPCOMP_CTRL1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ampcomp_ctrl1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anabypass_value2](anabypass_value2) module"]
pub type ANABYPASS_VALUE2 = crate::Reg<u32, _ANABYPASS_VALUE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANABYPASS_VALUE2;
#[doc = "`read()` method returns [anabypass_value2::R](anabypass_value2::R) reader structure"]
impl crate::Readable for ANABYPASS_VALUE2 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod anabypass_value2;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_misc_adc](config_misc_adc) module"]
pub type CONFIG_MISC_ADC = crate::Reg<u32, _CONFIG_MISC_ADC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG_MISC_ADC;
#[doc = "`read()` method returns [config_misc_adc::R](config_misc_adc::R) reader structure"]
impl crate::Readable for CONFIG_MISC_ADC {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod config_misc_adc;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [volt_trim](volt_trim) module"]
pub type VOLT_TRIM = crate::Reg<u32, _VOLT_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VOLT_TRIM;
#[doc = "`read()` method returns [volt_trim::R](volt_trim::R) reader structure"]
impl crate::Readable for VOLT_TRIM {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod volt_trim;
#[doc = "OSC Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_conf](osc_conf) module"]
pub type OSC_CONF = crate::Reg<u32, _OSC_CONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSC_CONF;
#[doc = "`read()` method returns [osc_conf::R](osc_conf::R) reader structure"]
impl crate::Readable for OSC_CONF {}
#[doc = "OSC Configuration"]
pub mod osc_conf;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_trim](cap_trim) module"]
pub type CAP_TRIM = crate::Reg<u32, _CAP_TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP_TRIM;
#[doc = "`read()` method returns [cap_trim::R](cap_trim::R) reader structure"]
impl crate::Readable for CAP_TRIM {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cap_trim;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_otp_data_1](misc_otp_data_1) module"]
pub type MISC_OTP_DATA_1 = crate::Reg<u32, _MISC_OTP_DATA_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC_OTP_DATA_1;
#[doc = "`read()` method returns [misc_otp_data_1::R](misc_otp_data_1::R) reader structure"]
impl crate::Readable for MISC_OTP_DATA_1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod misc_otp_data_1;
#[doc = "Power Down Current Control 20C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwd_curr_20c](pwd_curr_20c) module"]
pub type PWD_CURR_20C = crate::Reg<u32, _PWD_CURR_20C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWD_CURR_20C;
#[doc = "`read()` method returns [pwd_curr_20c::R](pwd_curr_20c::R) reader structure"]
impl crate::Readable for PWD_CURR_20C {}
#[doc = "Power Down Current Control 20C"]
pub mod pwd_curr_20c;
#[doc = "Power Down Current Control 35C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwd_curr_35c](pwd_curr_35c) module"]
pub type PWD_CURR_35C = crate::Reg<u32, _PWD_CURR_35C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWD_CURR_35C;
#[doc = "`read()` method returns [pwd_curr_35c::R](pwd_curr_35c::R) reader structure"]
impl crate::Readable for PWD_CURR_35C {}
#[doc = "Power Down Current Control 35C"]
pub mod pwd_curr_35c;
#[doc = "Power Down Current Control 50C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwd_curr_50c](pwd_curr_50c) module"]
pub type PWD_CURR_50C = crate::Reg<u32, _PWD_CURR_50C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWD_CURR_50C;
#[doc = "`read()` method returns [pwd_curr_50c::R](pwd_curr_50c::R) reader structure"]
impl crate::Readable for PWD_CURR_50C {}
#[doc = "Power Down Current Control 50C"]
pub mod pwd_curr_50c;
#[doc = "Power Down Current Control 65C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwd_curr_65c](pwd_curr_65c) module"]
pub type PWD_CURR_65C = crate::Reg<u32, _PWD_CURR_65C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWD_CURR_65C;
#[doc = "`read()` method returns [pwd_curr_65c::R](pwd_curr_65c::R) reader structure"]
impl crate::Readable for PWD_CURR_65C {}
#[doc = "Power Down Current Control 65C"]
pub mod pwd_curr_65c;
#[doc = "Power Down Current Control 80C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwd_curr_80c](pwd_curr_80c) module"]
pub type PWD_CURR_80C = crate::Reg<u32, _PWD_CURR_80C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWD_CURR_80C;
#[doc = "`read()` method returns [pwd_curr_80c::R](pwd_curr_80c::R) reader structure"]
impl crate::Readable for PWD_CURR_80C {}
#[doc = "Power Down Current Control 80C"]
pub mod pwd_curr_80c;
#[doc = "Power Down Current Control 95C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwd_curr_95c](pwd_curr_95c) module"]
pub type PWD_CURR_95C = crate::Reg<u32, _PWD_CURR_95C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWD_CURR_95C;
#[doc = "`read()` method returns [pwd_curr_95c::R](pwd_curr_95c::R) reader structure"]
impl crate::Readable for PWD_CURR_95C {}
#[doc = "Power Down Current Control 95C"]
pub mod pwd_curr_95c;
#[doc = "Power Down Current Control 110C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwd_curr_110c](pwd_curr_110c) module"]
pub type PWD_CURR_110C = crate::Reg<u32, _PWD_CURR_110C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWD_CURR_110C;
#[doc = "`read()` method returns [pwd_curr_110c::R](pwd_curr_110c::R) reader structure"]
impl crate::Readable for PWD_CURR_110C {}
#[doc = "Power Down Current Control 110C"]
pub mod pwd_curr_110c;
#[doc = "Power Down Current Control 125C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwd_curr_125c](pwd_curr_125c) module"]
pub type PWD_CURR_125C = crate::Reg<u32, _PWD_CURR_125C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWD_CURR_125C;
#[doc = "`read()` method returns [pwd_curr_125c::R](pwd_curr_125c::R) reader structure"]
impl crate::Readable for PWD_CURR_125C {}
#[doc = "Power Down Current Control 125C"]
pub mod pwd_curr_125c;

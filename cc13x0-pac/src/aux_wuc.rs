#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Clock Enable Clock enable for each module in the AUX domain For use by the system CPU The settings in this register are OR'ed with the corresponding settings in MODCLKEN1. This allows the system CPU and AUX_SCE to request clocks independently. Settings take effect immediately."]
    pub modclken0: MODCLKEN0,
    #[doc = "0x04 - Power Off Request Requests power off request for the AUX domain. When powered of the power supply and clock is disabled. This may only be used when taking the entire device into shutdown mode (i.e. with full device reset when resuming operation). Power off is prevented if AON_WUC:AUXCTL.AUX_FORCE_ON has been set, or if MCUBUSCTL.DISCONNECT_REQ has been cleared."]
    pub pwroffreq: PWROFFREQ,
    #[doc = "0x08 - Power Down Request Request from AUX for system to enter power down. When system is in power down there is limited current supply available and the clock source is set by AON_WUC:AUXCLK.PWR_DWN_SRC"]
    pub pwrdwnreq: PWRDWNREQ,
    #[doc = "0x0c - Power Down Acknowledgment"]
    pub pwrdwnack: PWRDWNACK,
    #[doc = "0x10 - Low Frequency Clock Request"]
    pub clklfreq: CLKLFREQ,
    #[doc = "0x14 - Low Frequency Clock Acknowledgment"]
    pub clklfack: CLKLFACK,
    _reserved6: [u8; 16usize],
    #[doc = "0x28 - Wake-up Event Flags Status of wake-up events from the AON domain The event flags are cleared by setting the corresponding bits in WUEVCLR"]
    pub wuevflags: WUEVFLAGS,
    #[doc = "0x2c - Wake-up Event Clear Clears wake-up events from the AON domain"]
    pub wuevclr: WUEVCLR,
    #[doc = "0x30 - ADC Clock Control Controls the ADC internal clock Note that the ADC command and data interface requires MODCLKEN0.ANAIF or MODCLKEN1.ANAIF also to be set"]
    pub adcclkctl: ADCCLKCTL,
    #[doc = "0x34 - TDC Clock Control Controls the TDC counter clock source, which steps the TDC counter value The source of this clock is controlled by OSC_DIG:CTL0.ACLK_TDC_SRC_SEL."]
    pub tdcclkctl: TDCCLKCTL,
    #[doc = "0x38 - Reference Clock Control Controls the TDC reference clock source, which is to be compared against the TDC counter clock. The source of this clock is controlled by OSC_DIG:CTL0.ACLK_REF_SRC_SEL."]
    pub refclkctl: REFCLKCTL,
    #[doc = "0x3c - Real Time Counter Sub Second Increment 0 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 15:0. After setting INC15_0 and RTCSUBSECINC1.INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
    pub rtcsubsecinc0: RTCSUBSECINC0,
    #[doc = "0x40 - Real Time Counter Sub Second Increment 1 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 23:16. After setting RTCSUBSECINC0.INC15_0 and INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
    pub rtcsubsecinc1: RTCSUBSECINC1,
    #[doc = "0x44 - Real Time Counter Sub Second Increment Control"]
    pub rtcsubsecincctl: RTCSUBSECINCCTL,
    #[doc = "0x48 - MCU Bus Control Controls the connection between the AUX domain bus and the MCU domain bus. The buses must be disconnected to allow power-down or power-off of the AUX domain."]
    pub mcubusctl: MCUBUSCTL,
    #[doc = "0x4c - MCU Bus Status Indicates the connection state of the AUX domain and MCU domain buses. Note that this register cannot be read from the MCU domain while disconnected, and is therefore only useful for the AUX_SCE."]
    pub mcubusstat: MCUBUSSTAT,
    #[doc = "0x50 - AON Domain Control Status Status of AUX domain control from AON_WUC."]
    pub aonctlstat: AONCTLSTAT,
    #[doc = "0x54 - AUX Input Output Latch Controls latching of signals between AUX_AIODIO0/AUX_AIODIO1 and AON_IOC."]
    pub auxiolatch: AUXIOLATCH,
    _reserved18: [u8; 4usize],
    #[doc = "0x5c - Module Clock Enable 1 Clock enable for each module in the AUX domain, for use by the AUX_SCE. Settings take effect immediately. The settings in this register are OR'ed with the corresponding settings in MODCLKEN0. This allows system CPU and AUX_SCE to request clocks independently."]
    pub modclken1: MODCLKEN1,
}
#[doc = "Module Clock Enable Clock enable for each module in the AUX domain For use by the system CPU The settings in this register are OR'ed with the corresponding settings in MODCLKEN1. This allows the system CPU and AUX_SCE to request clocks independently. Settings take effect immediately.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modclken0](modclken0) module"]
pub type MODCLKEN0 = crate::Reg<u32, _MODCLKEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODCLKEN0;
#[doc = "`read()` method returns [modclken0::R](modclken0::R) reader structure"]
impl crate::Readable for MODCLKEN0 {}
#[doc = "`write(|w| ..)` method takes [modclken0::W](modclken0::W) writer structure"]
impl crate::Writable for MODCLKEN0 {}
#[doc = "Module Clock Enable Clock enable for each module in the AUX domain For use by the system CPU The settings in this register are OR'ed with the corresponding settings in MODCLKEN1. This allows the system CPU and AUX_SCE to request clocks independently. Settings take effect immediately."]
pub mod modclken0;
#[doc = "Power Off Request Requests power off request for the AUX domain. When powered of the power supply and clock is disabled. This may only be used when taking the entire device into shutdown mode (i.e. with full device reset when resuming operation). Power off is prevented if AON_WUC:AUXCTL.AUX_FORCE_ON has been set, or if MCUBUSCTL.DISCONNECT_REQ has been cleared.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwroffreq](pwroffreq) module"]
pub type PWROFFREQ = crate::Reg<u32, _PWROFFREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWROFFREQ;
#[doc = "`read()` method returns [pwroffreq::R](pwroffreq::R) reader structure"]
impl crate::Readable for PWROFFREQ {}
#[doc = "`write(|w| ..)` method takes [pwroffreq::W](pwroffreq::W) writer structure"]
impl crate::Writable for PWROFFREQ {}
#[doc = "Power Off Request Requests power off request for the AUX domain. When powered of the power supply and clock is disabled. This may only be used when taking the entire device into shutdown mode (i.e. with full device reset when resuming operation). Power off is prevented if AON_WUC:AUXCTL.AUX_FORCE_ON has been set, or if MCUBUSCTL.DISCONNECT_REQ has been cleared."]
pub mod pwroffreq;
#[doc = "Power Down Request Request from AUX for system to enter power down. When system is in power down there is limited current supply available and the clock source is set by AON_WUC:AUXCLK.PWR_DWN_SRC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrdwnreq](pwrdwnreq) module"]
pub type PWRDWNREQ = crate::Reg<u32, _PWRDWNREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRDWNREQ;
#[doc = "`read()` method returns [pwrdwnreq::R](pwrdwnreq::R) reader structure"]
impl crate::Readable for PWRDWNREQ {}
#[doc = "`write(|w| ..)` method takes [pwrdwnreq::W](pwrdwnreq::W) writer structure"]
impl crate::Writable for PWRDWNREQ {}
#[doc = "Power Down Request Request from AUX for system to enter power down. When system is in power down there is limited current supply available and the clock source is set by AON_WUC:AUXCLK.PWR_DWN_SRC"]
pub mod pwrdwnreq;
#[doc = "Power Down Acknowledgment\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrdwnack](pwrdwnack) module"]
pub type PWRDWNACK = crate::Reg<u32, _PWRDWNACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRDWNACK;
#[doc = "`read()` method returns [pwrdwnack::R](pwrdwnack::R) reader structure"]
impl crate::Readable for PWRDWNACK {}
#[doc = "Power Down Acknowledgment"]
pub mod pwrdwnack;
#[doc = "Low Frequency Clock Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clklfreq](clklfreq) module"]
pub type CLKLFREQ = crate::Reg<u32, _CLKLFREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKLFREQ;
#[doc = "`read()` method returns [clklfreq::R](clklfreq::R) reader structure"]
impl crate::Readable for CLKLFREQ {}
#[doc = "`write(|w| ..)` method takes [clklfreq::W](clklfreq::W) writer structure"]
impl crate::Writable for CLKLFREQ {}
#[doc = "Low Frequency Clock Request"]
pub mod clklfreq;
#[doc = "Low Frequency Clock Acknowledgment\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clklfack](clklfack) module"]
pub type CLKLFACK = crate::Reg<u32, _CLKLFACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKLFACK;
#[doc = "`read()` method returns [clklfack::R](clklfack::R) reader structure"]
impl crate::Readable for CLKLFACK {}
#[doc = "Low Frequency Clock Acknowledgment"]
pub mod clklfack;
#[doc = "Wake-up Event Flags Status of wake-up events from the AON domain The event flags are cleared by setting the corresponding bits in WUEVCLR\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wuevflags](wuevflags) module"]
pub type WUEVFLAGS = crate::Reg<u32, _WUEVFLAGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WUEVFLAGS;
#[doc = "`read()` method returns [wuevflags::R](wuevflags::R) reader structure"]
impl crate::Readable for WUEVFLAGS {}
#[doc = "Wake-up Event Flags Status of wake-up events from the AON domain The event flags are cleared by setting the corresponding bits in WUEVCLR"]
pub mod wuevflags;
#[doc = "Wake-up Event Clear Clears wake-up events from the AON domain\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wuevclr](wuevclr) module"]
pub type WUEVCLR = crate::Reg<u32, _WUEVCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WUEVCLR;
#[doc = "`read()` method returns [wuevclr::R](wuevclr::R) reader structure"]
impl crate::Readable for WUEVCLR {}
#[doc = "`write(|w| ..)` method takes [wuevclr::W](wuevclr::W) writer structure"]
impl crate::Writable for WUEVCLR {}
#[doc = "Wake-up Event Clear Clears wake-up events from the AON domain"]
pub mod wuevclr;
#[doc = "ADC Clock Control Controls the ADC internal clock Note that the ADC command and data interface requires MODCLKEN0.ANAIF or MODCLKEN1.ANAIF also to be set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcclkctl](adcclkctl) module"]
pub type ADCCLKCTL = crate::Reg<u32, _ADCCLKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCLKCTL;
#[doc = "`read()` method returns [adcclkctl::R](adcclkctl::R) reader structure"]
impl crate::Readable for ADCCLKCTL {}
#[doc = "`write(|w| ..)` method takes [adcclkctl::W](adcclkctl::W) writer structure"]
impl crate::Writable for ADCCLKCTL {}
#[doc = "ADC Clock Control Controls the ADC internal clock Note that the ADC command and data interface requires MODCLKEN0.ANAIF or MODCLKEN1.ANAIF also to be set"]
pub mod adcclkctl;
#[doc = "TDC Clock Control Controls the TDC counter clock source, which steps the TDC counter value The source of this clock is controlled by OSC_DIG:CTL0.ACLK_TDC_SRC_SEL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdcclkctl](tdcclkctl) module"]
pub type TDCCLKCTL = crate::Reg<u32, _TDCCLKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDCCLKCTL;
#[doc = "`read()` method returns [tdcclkctl::R](tdcclkctl::R) reader structure"]
impl crate::Readable for TDCCLKCTL {}
#[doc = "`write(|w| ..)` method takes [tdcclkctl::W](tdcclkctl::W) writer structure"]
impl crate::Writable for TDCCLKCTL {}
#[doc = "TDC Clock Control Controls the TDC counter clock source, which steps the TDC counter value The source of this clock is controlled by OSC_DIG:CTL0.ACLK_TDC_SRC_SEL."]
pub mod tdcclkctl;
#[doc = "Reference Clock Control Controls the TDC reference clock source, which is to be compared against the TDC counter clock. The source of this clock is controlled by OSC_DIG:CTL0.ACLK_REF_SRC_SEL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refclkctl](refclkctl) module"]
pub type REFCLKCTL = crate::Reg<u32, _REFCLKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFCLKCTL;
#[doc = "`read()` method returns [refclkctl::R](refclkctl::R) reader structure"]
impl crate::Readable for REFCLKCTL {}
#[doc = "`write(|w| ..)` method takes [refclkctl::W](refclkctl::W) writer structure"]
impl crate::Writable for REFCLKCTL {}
#[doc = "Reference Clock Control Controls the TDC reference clock source, which is to be compared against the TDC counter clock. The source of this clock is controlled by OSC_DIG:CTL0.ACLK_REF_SRC_SEL."]
pub mod refclkctl;
#[doc = "Real Time Counter Sub Second Increment 0 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 15:0. After setting INC15_0 and RTCSUBSECINC1.INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsubsecinc0](rtcsubsecinc0) module"]
pub type RTCSUBSECINC0 = crate::Reg<u32, _RTCSUBSECINC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCSUBSECINC0;
#[doc = "`read()` method returns [rtcsubsecinc0::R](rtcsubsecinc0::R) reader structure"]
impl crate::Readable for RTCSUBSECINC0 {}
#[doc = "`write(|w| ..)` method takes [rtcsubsecinc0::W](rtcsubsecinc0::W) writer structure"]
impl crate::Writable for RTCSUBSECINC0 {}
#[doc = "Real Time Counter Sub Second Increment 0 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 15:0. After setting INC15_0 and RTCSUBSECINC1.INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
pub mod rtcsubsecinc0;
#[doc = "Real Time Counter Sub Second Increment 1 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 23:16. After setting RTCSUBSECINC0.INC15_0 and INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsubsecinc1](rtcsubsecinc1) module"]
pub type RTCSUBSECINC1 = crate::Reg<u32, _RTCSUBSECINC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCSUBSECINC1;
#[doc = "`read()` method returns [rtcsubsecinc1::R](rtcsubsecinc1::R) reader structure"]
impl crate::Readable for RTCSUBSECINC1 {}
#[doc = "`write(|w| ..)` method takes [rtcsubsecinc1::W](rtcsubsecinc1::W) writer structure"]
impl crate::Writable for RTCSUBSECINC1 {}
#[doc = "Real Time Counter Sub Second Increment 1 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 23:16. After setting RTCSUBSECINC0.INC15_0 and INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
pub mod rtcsubsecinc1;
#[doc = "Real Time Counter Sub Second Increment Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsubsecincctl](rtcsubsecincctl) module"]
pub type RTCSUBSECINCCTL = crate::Reg<u32, _RTCSUBSECINCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCSUBSECINCCTL;
#[doc = "`read()` method returns [rtcsubsecincctl::R](rtcsubsecincctl::R) reader structure"]
impl crate::Readable for RTCSUBSECINCCTL {}
#[doc = "`write(|w| ..)` method takes [rtcsubsecincctl::W](rtcsubsecincctl::W) writer structure"]
impl crate::Writable for RTCSUBSECINCCTL {}
#[doc = "Real Time Counter Sub Second Increment Control"]
pub mod rtcsubsecincctl;
#[doc = "MCU Bus Control Controls the connection between the AUX domain bus and the MCU domain bus. The buses must be disconnected to allow power-down or power-off of the AUX domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcubusctl](mcubusctl) module"]
pub type MCUBUSCTL = crate::Reg<u32, _MCUBUSCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCUBUSCTL;
#[doc = "`read()` method returns [mcubusctl::R](mcubusctl::R) reader structure"]
impl crate::Readable for MCUBUSCTL {}
#[doc = "`write(|w| ..)` method takes [mcubusctl::W](mcubusctl::W) writer structure"]
impl crate::Writable for MCUBUSCTL {}
#[doc = "MCU Bus Control Controls the connection between the AUX domain bus and the MCU domain bus. The buses must be disconnected to allow power-down or power-off of the AUX domain."]
pub mod mcubusctl;
#[doc = "MCU Bus Status Indicates the connection state of the AUX domain and MCU domain buses. Note that this register cannot be read from the MCU domain while disconnected, and is therefore only useful for the AUX_SCE.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcubusstat](mcubusstat) module"]
pub type MCUBUSSTAT = crate::Reg<u32, _MCUBUSSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCUBUSSTAT;
#[doc = "`read()` method returns [mcubusstat::R](mcubusstat::R) reader structure"]
impl crate::Readable for MCUBUSSTAT {}
#[doc = "MCU Bus Status Indicates the connection state of the AUX domain and MCU domain buses. Note that this register cannot be read from the MCU domain while disconnected, and is therefore only useful for the AUX_SCE."]
pub mod mcubusstat;
#[doc = "AON Domain Control Status Status of AUX domain control from AON_WUC.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aonctlstat](aonctlstat) module"]
pub type AONCTLSTAT = crate::Reg<u32, _AONCTLSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AONCTLSTAT;
#[doc = "`read()` method returns [aonctlstat::R](aonctlstat::R) reader structure"]
impl crate::Readable for AONCTLSTAT {}
#[doc = "AON Domain Control Status Status of AUX domain control from AON_WUC."]
pub mod aonctlstat;
#[doc = "AUX Input Output Latch Controls latching of signals between AUX_AIODIO0/AUX_AIODIO1 and AON_IOC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxiolatch](auxiolatch) module"]
pub type AUXIOLATCH = crate::Reg<u32, _AUXIOLATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUXIOLATCH;
#[doc = "`read()` method returns [auxiolatch::R](auxiolatch::R) reader structure"]
impl crate::Readable for AUXIOLATCH {}
#[doc = "`write(|w| ..)` method takes [auxiolatch::W](auxiolatch::W) writer structure"]
impl crate::Writable for AUXIOLATCH {}
#[doc = "AUX Input Output Latch Controls latching of signals between AUX_AIODIO0/AUX_AIODIO1 and AON_IOC."]
pub mod auxiolatch;
#[doc = "Module Clock Enable 1 Clock enable for each module in the AUX domain, for use by the AUX_SCE. Settings take effect immediately. The settings in this register are OR'ed with the corresponding settings in MODCLKEN0. This allows system CPU and AUX_SCE to request clocks independently.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modclken1](modclken1) module"]
pub type MODCLKEN1 = crate::Reg<u32, _MODCLKEN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODCLKEN1;
#[doc = "`read()` method returns [modclken1::R](modclken1::R) reader structure"]
impl crate::Readable for MODCLKEN1 {}
#[doc = "`write(|w| ..)` method takes [modclken1::W](modclken1::W) writer structure"]
impl crate::Writable for MODCLKEN1 {}
#[doc = "Module Clock Enable 1 Clock enable for each module in the AUX domain, for use by the AUX_SCE. Settings take effect immediately. The settings in this register are OR'ed with the corresponding settings in MODCLKEN0. This allows system CPU and AUX_SCE to request clocks independently."]
pub mod modclken1;

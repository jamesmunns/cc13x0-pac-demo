#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Clock Management This register contains bitfields related to the MCU clock."]
    pub mcuclk: MCUCLK,
    #[doc = "0x04 - AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
    pub auxclk: AUXCLK,
    #[doc = "0x08 - MCU Configuration This register contains power management related bitfields for the MCU domain."]
    pub mcucfg: MCUCFG,
    #[doc = "0x0c - AUX Configuration This register contains power management related signals for the AUX domain."]
    pub auxcfg: AUXCFG,
    #[doc = "0x10 - AUX Control This register contains events and control signals for the AUX domain."]
    pub auxctl: AUXCTL,
    #[doc = "0x14 - Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up."]
    pub pwrstat: PWRSTAT,
    #[doc = "0x18 - Shutdown Control This register contains bitfields required for entering shutdown mode"]
    pub shutdown: SHUTDOWN,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Control 0 This register contains various chip level control and debug bitfields."]
    pub ctl0: CTL0,
    #[doc = "0x24 - Control 1 This register contains various chip level control and debug bitfields."]
    pub ctl1: CTL1,
    _reserved9: [u8; 8usize],
    #[doc = "0x30 - Recharge Controller Configuration This register sets all relevant patameters for controlling the recharge algorithm."]
    pub rechargecfg: RECHARGECFG,
    #[doc = "0x34 - Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
    pub rechargestat: RECHARGESTAT,
    #[doc = "0x38 - Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
    pub osccfg: OSCCFG,
    _reserved12: [u8; 4usize],
    #[doc = "0x40 - JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP."]
    pub jtagcfg: JTAGCFG,
    #[doc = "0x44 - JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
    pub jtagusercode: JTAGUSERCODE,
}
#[doc = "MCU Clock Management This register contains bitfields related to the MCU clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcuclk](mcuclk) module"]
pub type MCUCLK = crate::Reg<u32, _MCUCLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCUCLK;
#[doc = "`read()` method returns [mcuclk::R](mcuclk::R) reader structure"]
impl crate::Readable for MCUCLK {}
#[doc = "`write(|w| ..)` method takes [mcuclk::W](mcuclk::W) writer structure"]
impl crate::Writable for MCUCLK {}
#[doc = "MCU Clock Management This register contains bitfields related to the MCU clock."]
pub mod mcuclk;
#[doc = "AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxclk](auxclk) module"]
pub type AUXCLK = crate::Reg<u32, _AUXCLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUXCLK;
#[doc = "`read()` method returns [auxclk::R](auxclk::R) reader structure"]
impl crate::Readable for AUXCLK {}
#[doc = "`write(|w| ..)` method takes [auxclk::W](auxclk::W) writer structure"]
impl crate::Writable for AUXCLK {}
#[doc = "AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
pub mod auxclk;
#[doc = "MCU Configuration This register contains power management related bitfields for the MCU domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcucfg](mcucfg) module"]
pub type MCUCFG = crate::Reg<u32, _MCUCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCUCFG;
#[doc = "`read()` method returns [mcucfg::R](mcucfg::R) reader structure"]
impl crate::Readable for MCUCFG {}
#[doc = "`write(|w| ..)` method takes [mcucfg::W](mcucfg::W) writer structure"]
impl crate::Writable for MCUCFG {}
#[doc = "MCU Configuration This register contains power management related bitfields for the MCU domain."]
pub mod mcucfg;
#[doc = "AUX Configuration This register contains power management related signals for the AUX domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxcfg](auxcfg) module"]
pub type AUXCFG = crate::Reg<u32, _AUXCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUXCFG;
#[doc = "`read()` method returns [auxcfg::R](auxcfg::R) reader structure"]
impl crate::Readable for AUXCFG {}
#[doc = "`write(|w| ..)` method takes [auxcfg::W](auxcfg::W) writer structure"]
impl crate::Writable for AUXCFG {}
#[doc = "AUX Configuration This register contains power management related signals for the AUX domain."]
pub mod auxcfg;
#[doc = "AUX Control This register contains events and control signals for the AUX domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxctl](auxctl) module"]
pub type AUXCTL = crate::Reg<u32, _AUXCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUXCTL;
#[doc = "`read()` method returns [auxctl::R](auxctl::R) reader structure"]
impl crate::Readable for AUXCTL {}
#[doc = "`write(|w| ..)` method takes [auxctl::W](auxctl::W) writer structure"]
impl crate::Writable for AUXCTL {}
#[doc = "AUX Control This register contains events and control signals for the AUX domain."]
pub mod auxctl;
#[doc = "Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrstat](pwrstat) module"]
pub type PWRSTAT = crate::Reg<u32, _PWRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRSTAT;
#[doc = "`read()` method returns [pwrstat::R](pwrstat::R) reader structure"]
impl crate::Readable for PWRSTAT {}
#[doc = "`write(|w| ..)` method takes [pwrstat::W](pwrstat::W) writer structure"]
impl crate::Writable for PWRSTAT {}
#[doc = "Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up."]
pub mod pwrstat;
#[doc = "Shutdown Control This register contains bitfields required for entering shutdown mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shutdown](shutdown) module"]
pub type SHUTDOWN = crate::Reg<u32, _SHUTDOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHUTDOWN;
#[doc = "`read()` method returns [shutdown::R](shutdown::R) reader structure"]
impl crate::Readable for SHUTDOWN {}
#[doc = "`write(|w| ..)` method takes [shutdown::W](shutdown::W) writer structure"]
impl crate::Writable for SHUTDOWN {}
#[doc = "Shutdown Control This register contains bitfields required for entering shutdown mode"]
pub mod shutdown;
#[doc = "Control 0 This register contains various chip level control and debug bitfields.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](ctl0) module"]
pub type CTL0 = crate::Reg<u32, _CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL0;
#[doc = "`read()` method returns [ctl0::R](ctl0::R) reader structure"]
impl crate::Readable for CTL0 {}
#[doc = "`write(|w| ..)` method takes [ctl0::W](ctl0::W) writer structure"]
impl crate::Writable for CTL0 {}
#[doc = "Control 0 This register contains various chip level control and debug bitfields."]
pub mod ctl0;
#[doc = "Control 1 This register contains various chip level control and debug bitfields.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](ctl1) module"]
pub type CTL1 = crate::Reg<u32, _CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL1;
#[doc = "`read()` method returns [ctl1::R](ctl1::R) reader structure"]
impl crate::Readable for CTL1 {}
#[doc = "`write(|w| ..)` method takes [ctl1::W](ctl1::W) writer structure"]
impl crate::Writable for CTL1 {}
#[doc = "Control 1 This register contains various chip level control and debug bitfields."]
pub mod ctl1;
#[doc = "Recharge Controller Configuration This register sets all relevant patameters for controlling the recharge algorithm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rechargecfg](rechargecfg) module"]
pub type RECHARGECFG = crate::Reg<u32, _RECHARGECFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RECHARGECFG;
#[doc = "`read()` method returns [rechargecfg::R](rechargecfg::R) reader structure"]
impl crate::Readable for RECHARGECFG {}
#[doc = "`write(|w| ..)` method takes [rechargecfg::W](rechargecfg::W) writer structure"]
impl crate::Writable for RECHARGECFG {}
#[doc = "Recharge Controller Configuration This register sets all relevant patameters for controlling the recharge algorithm."]
pub mod rechargecfg;
#[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rechargestat](rechargestat) module"]
pub type RECHARGESTAT = crate::Reg<u32, _RECHARGESTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RECHARGESTAT;
#[doc = "`read()` method returns [rechargestat::R](rechargestat::R) reader structure"]
impl crate::Readable for RECHARGESTAT {}
#[doc = "`write(|w| ..)` method takes [rechargestat::W](rechargestat::W) writer structure"]
impl crate::Writable for RECHARGESTAT {}
#[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
pub mod rechargestat;
#[doc = "Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osccfg](osccfg) module"]
pub type OSCCFG = crate::Reg<u32, _OSCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCCFG;
#[doc = "`read()` method returns [osccfg::R](osccfg::R) reader structure"]
impl crate::Readable for OSCCFG {}
#[doc = "`write(|w| ..)` method takes [osccfg::W](osccfg::W) writer structure"]
impl crate::Writable for OSCCFG {}
#[doc = "Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
pub mod osccfg;
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtagcfg](jtagcfg) module"]
pub type JTAGCFG = crate::Reg<u32, _JTAGCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JTAGCFG;
#[doc = "`read()` method returns [jtagcfg::R](jtagcfg::R) reader structure"]
impl crate::Readable for JTAGCFG {}
#[doc = "`write(|w| ..)` method takes [jtagcfg::W](jtagcfg::W) writer structure"]
impl crate::Writable for JTAGCFG {}
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP."]
pub mod jtagcfg;
#[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtagusercode](jtagusercode) module"]
pub type JTAGUSERCODE = crate::Reg<u32, _JTAGUSERCODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JTAGUSERCODE;
#[doc = "`read()` method returns [jtagusercode::R](jtagusercode::R) reader structure"]
impl crate::Readable for JTAGUSERCODE {}
#[doc = "`write(|w| ..)` method takes [jtagusercode::W](jtagusercode::W) writer structure"]
impl crate::Writable for JTAGUSERCODE {}
#[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
pub mod jtagusercode;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Random Number Lower Word Readout Value"]
    pub out0: OUT0,
    #[doc = "0x04 - Random Number Upper Word Readout Value"]
    pub out1: OUT1,
    #[doc = "0x08 - Interrupt Status"]
    pub irqflagstat: IRQFLAGSTAT,
    #[doc = "0x0c - Interrupt Mask"]
    pub irqflagmask: IRQFLAGMASK,
    #[doc = "0x10 - Interrupt Flag Clear"]
    pub irqflagclr: IRQFLAGCLR,
    #[doc = "0x14 - Control"]
    pub ctl: CTL,
    #[doc = "0x18 - Configuration 0"]
    pub cfg0: CFG0,
    #[doc = "0x1c - Alarm Control"]
    pub alarmcnt: ALARMCNT,
    #[doc = "0x20 - FRO Enable"]
    pub froen: FROEN,
    #[doc = "0x24 - FRO De-tune Bit"]
    pub frodetune: FRODETUNE,
    #[doc = "0x28 - Alarm Event"]
    pub alarmmask: ALARMMASK,
    #[doc = "0x2c - Alarm Shutdown"]
    pub alarmstop: ALARMSTOP,
    #[doc = "0x30 - LFSR Readout Value"]
    pub lfsr0: LFSR0,
    #[doc = "0x34 - LFSR Readout Value"]
    pub lfsr1: LFSR1,
    #[doc = "0x38 - LFSR Readout Value"]
    pub lfsr2: LFSR2,
    _reserved15: [u8; 60usize],
    #[doc = "0x78 - TRNG Engine Options Information"]
    pub hwopt: HWOPT,
    #[doc = "0x7c - HW Version 0 EIP Number And Core Revision"]
    pub hwver0: HWVER0,
    _reserved17: [u8; 8024usize],
    #[doc = "0x1fd8 - Interrupt Status After Masking"]
    pub irqstatmask: IRQSTATMASK,
    _reserved18: [u8; 4usize],
    #[doc = "0x1fe0 - HW Version 1 TRNG Revision Number"]
    pub hwver1: HWVER1,
    _reserved19: [u8; 8usize],
    #[doc = "0x1fec - Interrupt Set"]
    pub irqset: IRQSET,
    #[doc = "0x1ff0 - SW Reset Control"]
    pub swreset: SWRESET,
    _reserved21: [u8; 4usize],
    #[doc = "0x1ff8 - Interrupt Status"]
    pub irqstat: IRQSTAT,
}
#[doc = "Random Number Lower Word Readout Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out0](out0) module"]
pub type OUT0 = crate::Reg<u32, _OUT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT0;
#[doc = "`read()` method returns [out0::R](out0::R) reader structure"]
impl crate::Readable for OUT0 {}
#[doc = "Random Number Lower Word Readout Value"]
pub mod out0;
#[doc = "Random Number Upper Word Readout Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out1](out1) module"]
pub type OUT1 = crate::Reg<u32, _OUT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUT1;
#[doc = "`read()` method returns [out1::R](out1::R) reader structure"]
impl crate::Readable for OUT1 {}
#[doc = "Random Number Upper Word Readout Value"]
pub mod out1;
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqflagstat](irqflagstat) module"]
pub type IRQFLAGSTAT = crate::Reg<u32, _IRQFLAGSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQFLAGSTAT;
#[doc = "`read()` method returns [irqflagstat::R](irqflagstat::R) reader structure"]
impl crate::Readable for IRQFLAGSTAT {}
#[doc = "Interrupt Status"]
pub mod irqflagstat;
#[doc = "Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqflagmask](irqflagmask) module"]
pub type IRQFLAGMASK = crate::Reg<u32, _IRQFLAGMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQFLAGMASK;
#[doc = "`read()` method returns [irqflagmask::R](irqflagmask::R) reader structure"]
impl crate::Readable for IRQFLAGMASK {}
#[doc = "`write(|w| ..)` method takes [irqflagmask::W](irqflagmask::W) writer structure"]
impl crate::Writable for IRQFLAGMASK {}
#[doc = "Interrupt Mask"]
pub mod irqflagmask;
#[doc = "Interrupt Flag Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqflagclr](irqflagclr) module"]
pub type IRQFLAGCLR = crate::Reg<u32, _IRQFLAGCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQFLAGCLR;
#[doc = "`write(|w| ..)` method takes [irqflagclr::W](irqflagclr::W) writer structure"]
impl crate::Writable for IRQFLAGCLR {}
#[doc = "Interrupt Flag Clear"]
pub mod irqflagclr;
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control"]
pub mod ctl;
#[doc = "Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg0](cfg0) module"]
pub type CFG0 = crate::Reg<u32, _CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG0;
#[doc = "`read()` method returns [cfg0::R](cfg0::R) reader structure"]
impl crate::Readable for CFG0 {}
#[doc = "`write(|w| ..)` method takes [cfg0::W](cfg0::W) writer structure"]
impl crate::Writable for CFG0 {}
#[doc = "Configuration 0"]
pub mod cfg0;
#[doc = "Alarm Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarmcnt](alarmcnt) module"]
pub type ALARMCNT = crate::Reg<u32, _ALARMCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALARMCNT;
#[doc = "`read()` method returns [alarmcnt::R](alarmcnt::R) reader structure"]
impl crate::Readable for ALARMCNT {}
#[doc = "`write(|w| ..)` method takes [alarmcnt::W](alarmcnt::W) writer structure"]
impl crate::Writable for ALARMCNT {}
#[doc = "Alarm Control"]
pub mod alarmcnt;
#[doc = "FRO Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [froen](froen) module"]
pub type FROEN = crate::Reg<u32, _FROEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FROEN;
#[doc = "`read()` method returns [froen::R](froen::R) reader structure"]
impl crate::Readable for FROEN {}
#[doc = "`write(|w| ..)` method takes [froen::W](froen::W) writer structure"]
impl crate::Writable for FROEN {}
#[doc = "FRO Enable"]
pub mod froen;
#[doc = "FRO De-tune Bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frodetune](frodetune) module"]
pub type FRODETUNE = crate::Reg<u32, _FRODETUNE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRODETUNE;
#[doc = "`read()` method returns [frodetune::R](frodetune::R) reader structure"]
impl crate::Readable for FRODETUNE {}
#[doc = "`write(|w| ..)` method takes [frodetune::W](frodetune::W) writer structure"]
impl crate::Writable for FRODETUNE {}
#[doc = "FRO De-tune Bit"]
pub mod frodetune;
#[doc = "Alarm Event\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarmmask](alarmmask) module"]
pub type ALARMMASK = crate::Reg<u32, _ALARMMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALARMMASK;
#[doc = "`read()` method returns [alarmmask::R](alarmmask::R) reader structure"]
impl crate::Readable for ALARMMASK {}
#[doc = "`write(|w| ..)` method takes [alarmmask::W](alarmmask::W) writer structure"]
impl crate::Writable for ALARMMASK {}
#[doc = "Alarm Event"]
pub mod alarmmask;
#[doc = "Alarm Shutdown\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarmstop](alarmstop) module"]
pub type ALARMSTOP = crate::Reg<u32, _ALARMSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALARMSTOP;
#[doc = "`read()` method returns [alarmstop::R](alarmstop::R) reader structure"]
impl crate::Readable for ALARMSTOP {}
#[doc = "`write(|w| ..)` method takes [alarmstop::W](alarmstop::W) writer structure"]
impl crate::Writable for ALARMSTOP {}
#[doc = "Alarm Shutdown"]
pub mod alarmstop;
#[doc = "LFSR Readout Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfsr0](lfsr0) module"]
pub type LFSR0 = crate::Reg<u32, _LFSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFSR0;
#[doc = "`read()` method returns [lfsr0::R](lfsr0::R) reader structure"]
impl crate::Readable for LFSR0 {}
#[doc = "`write(|w| ..)` method takes [lfsr0::W](lfsr0::W) writer structure"]
impl crate::Writable for LFSR0 {}
#[doc = "LFSR Readout Value"]
pub mod lfsr0;
#[doc = "LFSR Readout Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfsr1](lfsr1) module"]
pub type LFSR1 = crate::Reg<u32, _LFSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFSR1;
#[doc = "`read()` method returns [lfsr1::R](lfsr1::R) reader structure"]
impl crate::Readable for LFSR1 {}
#[doc = "`write(|w| ..)` method takes [lfsr1::W](lfsr1::W) writer structure"]
impl crate::Writable for LFSR1 {}
#[doc = "LFSR Readout Value"]
pub mod lfsr1;
#[doc = "LFSR Readout Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfsr2](lfsr2) module"]
pub type LFSR2 = crate::Reg<u32, _LFSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFSR2;
#[doc = "`read()` method returns [lfsr2::R](lfsr2::R) reader structure"]
impl crate::Readable for LFSR2 {}
#[doc = "`write(|w| ..)` method takes [lfsr2::W](lfsr2::W) writer structure"]
impl crate::Writable for LFSR2 {}
#[doc = "LFSR Readout Value"]
pub mod lfsr2;
#[doc = "TRNG Engine Options Information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwopt](hwopt) module"]
pub type HWOPT = crate::Reg<u32, _HWOPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWOPT;
#[doc = "`read()` method returns [hwopt::R](hwopt::R) reader structure"]
impl crate::Readable for HWOPT {}
#[doc = "TRNG Engine Options Information"]
pub mod hwopt;
#[doc = "HW Version 0 EIP Number And Core Revision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwver0](hwver0) module"]
pub type HWVER0 = crate::Reg<u32, _HWVER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWVER0;
#[doc = "`read()` method returns [hwver0::R](hwver0::R) reader structure"]
impl crate::Readable for HWVER0 {}
#[doc = "HW Version 0 EIP Number And Core Revision"]
pub mod hwver0;
#[doc = "Interrupt Status After Masking\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqstatmask](irqstatmask) module"]
pub type IRQSTATMASK = crate::Reg<u32, _IRQSTATMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSTATMASK;
#[doc = "`read()` method returns [irqstatmask::R](irqstatmask::R) reader structure"]
impl crate::Readable for IRQSTATMASK {}
#[doc = "Interrupt Status After Masking"]
pub mod irqstatmask;
#[doc = "HW Version 1 TRNG Revision Number\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwver1](hwver1) module"]
pub type HWVER1 = crate::Reg<u32, _HWVER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWVER1;
#[doc = "`read()` method returns [hwver1::R](hwver1::R) reader structure"]
impl crate::Readable for HWVER1 {}
#[doc = "HW Version 1 TRNG Revision Number"]
pub mod hwver1;
#[doc = "Interrupt Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqset](irqset) module"]
pub type IRQSET = crate::Reg<u32, _IRQSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSET;
#[doc = "`read()` method returns [irqset::R](irqset::R) reader structure"]
impl crate::Readable for IRQSET {}
#[doc = "`write(|w| ..)` method takes [irqset::W](irqset::W) writer structure"]
impl crate::Writable for IRQSET {}
#[doc = "Interrupt Set"]
pub mod irqset;
#[doc = "SW Reset Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swreset](swreset) module"]
pub type SWRESET = crate::Reg<u32, _SWRESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWRESET;
#[doc = "`read()` method returns [swreset::R](swreset::R) reader structure"]
impl crate::Readable for SWRESET {}
#[doc = "`write(|w| ..)` method takes [swreset::W](swreset::W) writer structure"]
impl crate::Writable for SWRESET {}
#[doc = "SW Reset Control"]
pub mod swreset;
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqstat](irqstat) module"]
pub type IRQSTAT = crate::Reg<u32, _IRQSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSTAT;
#[doc = "`read()` method returns [irqstat::R](irqstat::R) reader structure"]
impl crate::Readable for IRQSTAT {}
#[doc = "Interrupt Status"]
pub mod irqstat;

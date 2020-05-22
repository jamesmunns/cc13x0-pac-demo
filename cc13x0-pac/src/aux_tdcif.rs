#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Status"]
    pub stat: STAT,
    #[doc = "0x08 - Result Result of last TDC conversion"]
    pub result: RESULT,
    #[doc = "0x0c - Saturation Configuration"]
    pub satcfg: SATCFG,
    #[doc = "0x10 - Trigger Source TDC start/stop trigger source selection"]
    pub trigsrc: TRIGSRC,
    #[doc = "0x14 - Trigger Counter Stop counter status/control of TDC"]
    pub trigcnt: TRIGCNT,
    #[doc = "0x18 - Trigger Counter Load Stop counter control of TDC"]
    pub trigcntload: TRIGCNTLOAD,
    #[doc = "0x1c - Trigger Counter Configuration"]
    pub trigcntcfg: TRIGCNTCFG,
    #[doc = "0x20 - Prescaler Control The prescaler can be used to count events that are faster than the AUX clock speed. It can be used standalone or as a start/stop source for the TDC by configuring TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to TDC_PRE. When counting fast signals with the TDC that are faster than 1/10th of the clock frequency of AUX it is recommended to use the prescaler."]
    pub prectl: PRECTL,
    #[doc = "0x24 - Prescaler Counter Value of prescaler counter"]
    pub precnt: PRECNT,
}
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
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "Status"]
pub mod stat;
#[doc = "Result Result of last TDC conversion\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](result) module"]
pub type RESULT = crate::Reg<u32, _RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT;
#[doc = "`read()` method returns [result::R](result::R) reader structure"]
impl crate::Readable for RESULT {}
#[doc = "Result Result of last TDC conversion"]
pub mod result;
#[doc = "Saturation Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [satcfg](satcfg) module"]
pub type SATCFG = crate::Reg<u32, _SATCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SATCFG;
#[doc = "`read()` method returns [satcfg::R](satcfg::R) reader structure"]
impl crate::Readable for SATCFG {}
#[doc = "`write(|w| ..)` method takes [satcfg::W](satcfg::W) writer structure"]
impl crate::Writable for SATCFG {}
#[doc = "Saturation Configuration"]
pub mod satcfg;
#[doc = "Trigger Source TDC start/stop trigger source selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigsrc](trigsrc) module"]
pub type TRIGSRC = crate::Reg<u32, _TRIGSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIGSRC;
#[doc = "`read()` method returns [trigsrc::R](trigsrc::R) reader structure"]
impl crate::Readable for TRIGSRC {}
#[doc = "`write(|w| ..)` method takes [trigsrc::W](trigsrc::W) writer structure"]
impl crate::Writable for TRIGSRC {}
#[doc = "Trigger Source TDC start/stop trigger source selection"]
pub mod trigsrc;
#[doc = "Trigger Counter Stop counter status/control of TDC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigcnt](trigcnt) module"]
pub type TRIGCNT = crate::Reg<u32, _TRIGCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIGCNT;
#[doc = "`read()` method returns [trigcnt::R](trigcnt::R) reader structure"]
impl crate::Readable for TRIGCNT {}
#[doc = "`write(|w| ..)` method takes [trigcnt::W](trigcnt::W) writer structure"]
impl crate::Writable for TRIGCNT {}
#[doc = "Trigger Counter Stop counter status/control of TDC"]
pub mod trigcnt;
#[doc = "Trigger Counter Load Stop counter control of TDC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigcntload](trigcntload) module"]
pub type TRIGCNTLOAD = crate::Reg<u32, _TRIGCNTLOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIGCNTLOAD;
#[doc = "`read()` method returns [trigcntload::R](trigcntload::R) reader structure"]
impl crate::Readable for TRIGCNTLOAD {}
#[doc = "`write(|w| ..)` method takes [trigcntload::W](trigcntload::W) writer structure"]
impl crate::Writable for TRIGCNTLOAD {}
#[doc = "Trigger Counter Load Stop counter control of TDC"]
pub mod trigcntload;
#[doc = "Trigger Counter Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigcntcfg](trigcntcfg) module"]
pub type TRIGCNTCFG = crate::Reg<u32, _TRIGCNTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIGCNTCFG;
#[doc = "`read()` method returns [trigcntcfg::R](trigcntcfg::R) reader structure"]
impl crate::Readable for TRIGCNTCFG {}
#[doc = "`write(|w| ..)` method takes [trigcntcfg::W](trigcntcfg::W) writer structure"]
impl crate::Writable for TRIGCNTCFG {}
#[doc = "Trigger Counter Configuration"]
pub mod trigcntcfg;
#[doc = "Prescaler Control The prescaler can be used to count events that are faster than the AUX clock speed. It can be used standalone or as a start/stop source for the TDC by configuring TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to TDC_PRE. When counting fast signals with the TDC that are faster than 1/10th of the clock frequency of AUX it is recommended to use the prescaler.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prectl](prectl) module"]
pub type PRECTL = crate::Reg<u32, _PRECTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRECTL;
#[doc = "`read()` method returns [prectl::R](prectl::R) reader structure"]
impl crate::Readable for PRECTL {}
#[doc = "`write(|w| ..)` method takes [prectl::W](prectl::W) writer structure"]
impl crate::Writable for PRECTL {}
#[doc = "Prescaler Control The prescaler can be used to count events that are faster than the AUX clock speed. It can be used standalone or as a start/stop source for the TDC by configuring TRIGSRC.START_SRC and TRIGSRC.STOP_SRC to TDC_PRE. When counting fast signals with the TDC that are faster than 1/10th of the clock frequency of AUX it is recommended to use the prescaler."]
pub mod prectl;
#[doc = "Prescaler Counter Value of prescaler counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [precnt](precnt) module"]
pub type PRECNT = crate::Reg<u32, _PRECNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRECNT;
#[doc = "`read()` method returns [precnt::R](precnt::R) reader structure"]
impl crate::Readable for PRECNT {}
#[doc = "`write(|w| ..)` method takes [precnt::W](precnt::W) writer structure"]
impl crate::Writable for PRECNT {}
#[doc = "Prescaler Counter Value of prescaler counter"]
pub mod precnt;

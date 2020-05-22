#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - ADC Control"]
    pub adcctl: ADCCTL,
    #[doc = "0x14 - ADC FIFO Status FIFO can hold up to four ADC samples"]
    pub adcfifostat: ADCFIFOSTAT,
    #[doc = "0x18 - ADC FIFO"]
    pub adcfifo: ADCFIFO,
    #[doc = "0x1c - ADC Trigger"]
    pub adctrig: ADCTRIG,
    #[doc = "0x20 - Current Source Control"]
    pub isrcctl: ISRCCTL,
}
#[doc = "ADC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcctl](adcctl) module"]
pub type ADCCTL = crate::Reg<u32, _ADCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCCTL;
#[doc = "`read()` method returns [adcctl::R](adcctl::R) reader structure"]
impl crate::Readable for ADCCTL {}
#[doc = "`write(|w| ..)` method takes [adcctl::W](adcctl::W) writer structure"]
impl crate::Writable for ADCCTL {}
#[doc = "ADC Control"]
pub mod adcctl;
#[doc = "ADC FIFO Status FIFO can hold up to four ADC samples\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcfifostat](adcfifostat) module"]
pub type ADCFIFOSTAT = crate::Reg<u32, _ADCFIFOSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCFIFOSTAT;
#[doc = "`read()` method returns [adcfifostat::R](adcfifostat::R) reader structure"]
impl crate::Readable for ADCFIFOSTAT {}
#[doc = "ADC FIFO Status FIFO can hold up to four ADC samples"]
pub mod adcfifostat;
#[doc = "ADC FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcfifo](adcfifo) module"]
pub type ADCFIFO = crate::Reg<u32, _ADCFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCFIFO;
#[doc = "`read()` method returns [adcfifo::R](adcfifo::R) reader structure"]
impl crate::Readable for ADCFIFO {}
#[doc = "`write(|w| ..)` method takes [adcfifo::W](adcfifo::W) writer structure"]
impl crate::Writable for ADCFIFO {}
#[doc = "ADC FIFO"]
pub mod adcfifo;
#[doc = "ADC Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adctrig](adctrig) module"]
pub type ADCTRIG = crate::Reg<u32, _ADCTRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCTRIG;
#[doc = "`read()` method returns [adctrig::R](adctrig::R) reader structure"]
impl crate::Readable for ADCTRIG {}
#[doc = "`write(|w| ..)` method takes [adctrig::W](adctrig::W) writer structure"]
impl crate::Writable for ADCTRIG {}
#[doc = "ADC Trigger"]
pub mod adctrig;
#[doc = "Current Source Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isrcctl](isrcctl) module"]
pub type ISRCCTL = crate::Reg<u32, _ISRCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISRCCTL;
#[doc = "`read()` method returns [isrcctl::R](isrcctl::R) reader structure"]
impl crate::Readable for ISRCCTL {}
#[doc = "`write(|w| ..)` method takes [isrcctl::W](isrcctl::W) writer structure"]
impl crate::Writable for ISRCCTL {}
#[doc = "Current Source Control"]
pub mod isrcctl;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Radio Timer Counter Value"]
    pub ratcnt: RATCNT,
    _reserved1: [u8; 120usize],
    #[doc = "0x80 - Timer Channel 0 Capture/Compare Register"]
    pub ratch0val: RATCH0VAL,
    #[doc = "0x84 - Timer Channel 1 Capture/Compare Register"]
    pub ratch1val: RATCH1VAL,
    #[doc = "0x88 - Timer Channel 2 Capture/Compare Register"]
    pub ratch2val: RATCH2VAL,
    #[doc = "0x8c - Timer Channel 3 Capture/Compare Register"]
    pub ratch3val: RATCH3VAL,
    #[doc = "0x90 - Timer Channel 4 Capture/Compare Register"]
    pub ratch4val: RATCH4VAL,
    #[doc = "0x94 - Timer Channel 5 Capture/Compare Register"]
    pub ratch5val: RATCH5VAL,
    #[doc = "0x98 - Timer Channel 6 Capture/Compare Register"]
    pub ratch6val: RATCH6VAL,
    #[doc = "0x9c - Timer Channel 7 Capture/Compare Register"]
    pub ratch7val: RATCH7VAL,
}
#[doc = "Radio Timer Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratcnt](ratcnt) module"]
pub type RATCNT = crate::Reg<u32, _RATCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RATCNT;
#[doc = "`read()` method returns [ratcnt::R](ratcnt::R) reader structure"]
impl crate::Readable for RATCNT {}
#[doc = "`write(|w| ..)` method takes [ratcnt::W](ratcnt::W) writer structure"]
impl crate::Writable for RATCNT {}
#[doc = "Radio Timer Counter Value"]
pub mod ratcnt;
#[doc = "Timer Channel 0 Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratch0val](ratch0val) module"]
pub type RATCH0VAL = crate::Reg<u32, _RATCH0VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RATCH0VAL;
#[doc = "`read()` method returns [ratch0val::R](ratch0val::R) reader structure"]
impl crate::Readable for RATCH0VAL {}
#[doc = "`write(|w| ..)` method takes [ratch0val::W](ratch0val::W) writer structure"]
impl crate::Writable for RATCH0VAL {}
#[doc = "Timer Channel 0 Capture/Compare Register"]
pub mod ratch0val;
#[doc = "Timer Channel 1 Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratch1val](ratch1val) module"]
pub type RATCH1VAL = crate::Reg<u32, _RATCH1VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RATCH1VAL;
#[doc = "`read()` method returns [ratch1val::R](ratch1val::R) reader structure"]
impl crate::Readable for RATCH1VAL {}
#[doc = "`write(|w| ..)` method takes [ratch1val::W](ratch1val::W) writer structure"]
impl crate::Writable for RATCH1VAL {}
#[doc = "Timer Channel 1 Capture/Compare Register"]
pub mod ratch1val;
#[doc = "Timer Channel 2 Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratch2val](ratch2val) module"]
pub type RATCH2VAL = crate::Reg<u32, _RATCH2VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RATCH2VAL;
#[doc = "`read()` method returns [ratch2val::R](ratch2val::R) reader structure"]
impl crate::Readable for RATCH2VAL {}
#[doc = "`write(|w| ..)` method takes [ratch2val::W](ratch2val::W) writer structure"]
impl crate::Writable for RATCH2VAL {}
#[doc = "Timer Channel 2 Capture/Compare Register"]
pub mod ratch2val;
#[doc = "Timer Channel 3 Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratch3val](ratch3val) module"]
pub type RATCH3VAL = crate::Reg<u32, _RATCH3VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RATCH3VAL;
#[doc = "`read()` method returns [ratch3val::R](ratch3val::R) reader structure"]
impl crate::Readable for RATCH3VAL {}
#[doc = "`write(|w| ..)` method takes [ratch3val::W](ratch3val::W) writer structure"]
impl crate::Writable for RATCH3VAL {}
#[doc = "Timer Channel 3 Capture/Compare Register"]
pub mod ratch3val;
#[doc = "Timer Channel 4 Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratch4val](ratch4val) module"]
pub type RATCH4VAL = crate::Reg<u32, _RATCH4VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RATCH4VAL;
#[doc = "`read()` method returns [ratch4val::R](ratch4val::R) reader structure"]
impl crate::Readable for RATCH4VAL {}
#[doc = "`write(|w| ..)` method takes [ratch4val::W](ratch4val::W) writer structure"]
impl crate::Writable for RATCH4VAL {}
#[doc = "Timer Channel 4 Capture/Compare Register"]
pub mod ratch4val;
#[doc = "Timer Channel 5 Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratch5val](ratch5val) module"]
pub type RATCH5VAL = crate::Reg<u32, _RATCH5VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RATCH5VAL;
#[doc = "`read()` method returns [ratch5val::R](ratch5val::R) reader structure"]
impl crate::Readable for RATCH5VAL {}
#[doc = "`write(|w| ..)` method takes [ratch5val::W](ratch5val::W) writer structure"]
impl crate::Writable for RATCH5VAL {}
#[doc = "Timer Channel 5 Capture/Compare Register"]
pub mod ratch5val;
#[doc = "Timer Channel 6 Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratch6val](ratch6val) module"]
pub type RATCH6VAL = crate::Reg<u32, _RATCH6VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RATCH6VAL;
#[doc = "`read()` method returns [ratch6val::R](ratch6val::R) reader structure"]
impl crate::Readable for RATCH6VAL {}
#[doc = "`write(|w| ..)` method takes [ratch6val::W](ratch6val::W) writer structure"]
impl crate::Writable for RATCH6VAL {}
#[doc = "Timer Channel 6 Capture/Compare Register"]
pub mod ratch6val;
#[doc = "Timer Channel 7 Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratch7val](ratch7val) module"]
pub type RATCH7VAL = crate::Reg<u32, _RATCH7VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RATCH7VAL;
#[doc = "`read()` method returns [ratch7val::R](ratch7val::R) reader structure"]
impl crate::Readable for RATCH7VAL {}
#[doc = "`write(|w| ..)` method takes [ratch7val::W](ratch7val::W) writer structure"]
impl crate::Writable for RATCH7VAL {}
#[doc = "Timer Channel 7 Capture/Compare Register"]
pub mod ratch7val;

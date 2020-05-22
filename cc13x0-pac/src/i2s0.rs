#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WCLK Source Selection"]
    pub aifwclksrc: AIFWCLKSRC,
    #[doc = "0x04 - DMA Buffer Size Configuration"]
    pub aifdmacfg: AIFDMACFG,
    #[doc = "0x08 - Pin Direction"]
    pub aifdircfg: AIFDIRCFG,
    #[doc = "0x0c - Serial Interface Format Configuration"]
    pub aiffmtcfg: AIFFMTCFG,
    #[doc = "0x10 - Word Selection Bit Mask for Pin 0"]
    pub aifwmask0: AIFWMASK0,
    #[doc = "0x14 - Word Selection Bit Mask for Pin 1"]
    pub aifwmask1: AIFWMASK1,
    #[doc = "0x18 - Word Selection Bit Mask for Pin 2"]
    pub aifwmask2: AIFWMASK2,
    #[doc = "0x1c - Audio Interface PWM Debug Value"]
    pub aifpwmvalue: AIFPWMVALUE,
    #[doc = "0x20 - DMA Input Buffer Next Pointer"]
    pub aifinptrnext: AIFINPTRNEXT,
    #[doc = "0x24 - DMA Input Buffer Current Pointer"]
    pub aifinptr: AIFINPTR,
    #[doc = "0x28 - DMA Output Buffer Next Pointer"]
    pub aifoutptrnext: AIFOUTPTRNEXT,
    #[doc = "0x2c - DMA Output Buffer Current Pointer"]
    pub aifoutptr: AIFOUTPTR,
    _reserved12: [u8; 4usize],
    #[doc = "0x34 - SampleStaMP Generator Control Register"]
    pub stmpctl: STMPCTL,
    #[doc = "0x38 - Captured XOSC Counter Value, Capture Channel 0"]
    pub stmpxcntcapt0: STMPXCNTCAPT0,
    #[doc = "0x3c - XOSC Period Value"]
    pub stmpxper: STMPXPER,
    #[doc = "0x40 - Captured WCLK Counter Value, Capture Channel 0"]
    pub stmpwcntcapt0: STMPWCNTCAPT0,
    #[doc = "0x44 - WCLK Counter Period Value"]
    pub stmpwper: STMPWPER,
    #[doc = "0x48 - WCLK Counter Trigger Value for Input Pins"]
    pub stmpintrig: STMPINTRIG,
    #[doc = "0x4c - WCLK Counter Trigger Value for Output Pins"]
    pub stmpouttrig: STMPOUTTRIG,
    #[doc = "0x50 - WCLK Counter Set Operation"]
    pub stmpwset: STMPWSET,
    #[doc = "0x54 - WCLK Counter Add Operation"]
    pub stmpwadd: STMPWADD,
    #[doc = "0x58 - XOSC Minimum Period Value Minimum Value of STMPXPER"]
    pub stmpxpermin: STMPXPERMIN,
    #[doc = "0x5c - Current Value of WCNT"]
    pub stmpwcnt: STMPWCNT,
    #[doc = "0x60 - Current Value of XCNT"]
    pub stmpxcnt: STMPXCNT,
    #[doc = "0x64 - Captured XOSC Counter Value, Capture Channel 1"]
    pub stmpxcntcapt1: STMPXCNTCAPT1,
    #[doc = "0x68 - Captured WCLK Counter Value, Capture Channel 1"]
    pub stmpwcntcapt1: STMPWCNTCAPT1,
    _reserved26: [u8; 4usize],
    #[doc = "0x70 - Masked Interrupt Status Register"]
    pub irqmask: IRQMASK,
    #[doc = "0x74 - Raw Interrupt Status Register"]
    pub irqflags: IRQFLAGS,
    #[doc = "0x78 - Interrupt Set Register"]
    pub irqset: IRQSET,
    #[doc = "0x7c - Interrupt Clear Register"]
    pub irqclr: IRQCLR,
}
#[doc = "WCLK Source Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifwclksrc](aifwclksrc) module"]
pub type AIFWCLKSRC = crate::Reg<u32, _AIFWCLKSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIFWCLKSRC;
#[doc = "`read()` method returns [aifwclksrc::R](aifwclksrc::R) reader structure"]
impl crate::Readable for AIFWCLKSRC {}
#[doc = "`write(|w| ..)` method takes [aifwclksrc::W](aifwclksrc::W) writer structure"]
impl crate::Writable for AIFWCLKSRC {}
#[doc = "WCLK Source Selection"]
pub mod aifwclksrc;
#[doc = "DMA Buffer Size Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifdmacfg](aifdmacfg) module"]
pub type AIFDMACFG = crate::Reg<u32, _AIFDMACFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIFDMACFG;
#[doc = "`read()` method returns [aifdmacfg::R](aifdmacfg::R) reader structure"]
impl crate::Readable for AIFDMACFG {}
#[doc = "`write(|w| ..)` method takes [aifdmacfg::W](aifdmacfg::W) writer structure"]
impl crate::Writable for AIFDMACFG {}
#[doc = "DMA Buffer Size Configuration"]
pub mod aifdmacfg;
#[doc = "Pin Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifdircfg](aifdircfg) module"]
pub type AIFDIRCFG = crate::Reg<u32, _AIFDIRCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIFDIRCFG;
#[doc = "`read()` method returns [aifdircfg::R](aifdircfg::R) reader structure"]
impl crate::Readable for AIFDIRCFG {}
#[doc = "`write(|w| ..)` method takes [aifdircfg::W](aifdircfg::W) writer structure"]
impl crate::Writable for AIFDIRCFG {}
#[doc = "Pin Direction"]
pub mod aifdircfg;
#[doc = "Serial Interface Format Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aiffmtcfg](aiffmtcfg) module"]
pub type AIFFMTCFG = crate::Reg<u32, _AIFFMTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIFFMTCFG;
#[doc = "`read()` method returns [aiffmtcfg::R](aiffmtcfg::R) reader structure"]
impl crate::Readable for AIFFMTCFG {}
#[doc = "`write(|w| ..)` method takes [aiffmtcfg::W](aiffmtcfg::W) writer structure"]
impl crate::Writable for AIFFMTCFG {}
#[doc = "Serial Interface Format Configuration"]
pub mod aiffmtcfg;
#[doc = "Word Selection Bit Mask for Pin 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifwmask0](aifwmask0) module"]
pub type AIFWMASK0 = crate::Reg<u32, _AIFWMASK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIFWMASK0;
#[doc = "`read()` method returns [aifwmask0::R](aifwmask0::R) reader structure"]
impl crate::Readable for AIFWMASK0 {}
#[doc = "`write(|w| ..)` method takes [aifwmask0::W](aifwmask0::W) writer structure"]
impl crate::Writable for AIFWMASK0 {}
#[doc = "Word Selection Bit Mask for Pin 0"]
pub mod aifwmask0;
#[doc = "Word Selection Bit Mask for Pin 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifwmask1](aifwmask1) module"]
pub type AIFWMASK1 = crate::Reg<u32, _AIFWMASK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIFWMASK1;
#[doc = "`read()` method returns [aifwmask1::R](aifwmask1::R) reader structure"]
impl crate::Readable for AIFWMASK1 {}
#[doc = "`write(|w| ..)` method takes [aifwmask1::W](aifwmask1::W) writer structure"]
impl crate::Writable for AIFWMASK1 {}
#[doc = "Word Selection Bit Mask for Pin 1"]
pub mod aifwmask1;
#[doc = "Word Selection Bit Mask for Pin 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifwmask2](aifwmask2) module"]
pub type AIFWMASK2 = crate::Reg<u32, _AIFWMASK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIFWMASK2;
#[doc = "`read()` method returns [aifwmask2::R](aifwmask2::R) reader structure"]
impl crate::Readable for AIFWMASK2 {}
#[doc = "`write(|w| ..)` method takes [aifwmask2::W](aifwmask2::W) writer structure"]
impl crate::Writable for AIFWMASK2 {}
#[doc = "Word Selection Bit Mask for Pin 2"]
pub mod aifwmask2;
#[doc = "Audio Interface PWM Debug Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifpwmvalue](aifpwmvalue) module"]
pub type AIFPWMVALUE = crate::Reg<u32, _AIFPWMVALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIFPWMVALUE;
#[doc = "`read()` method returns [aifpwmvalue::R](aifpwmvalue::R) reader structure"]
impl crate::Readable for AIFPWMVALUE {}
#[doc = "`write(|w| ..)` method takes [aifpwmvalue::W](aifpwmvalue::W) writer structure"]
impl crate::Writable for AIFPWMVALUE {}
#[doc = "Audio Interface PWM Debug Value"]
pub mod aifpwmvalue;
#[doc = "DMA Input Buffer Next Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifinptrnext](aifinptrnext) module"]
pub type AIFINPTRNEXT = crate::Reg<u32, _AIFINPTRNEXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIFINPTRNEXT;
#[doc = "`read()` method returns [aifinptrnext::R](aifinptrnext::R) reader structure"]
impl crate::Readable for AIFINPTRNEXT {}
#[doc = "`write(|w| ..)` method takes [aifinptrnext::W](aifinptrnext::W) writer structure"]
impl crate::Writable for AIFINPTRNEXT {}
#[doc = "DMA Input Buffer Next Pointer"]
pub mod aifinptrnext;
#[doc = "DMA Input Buffer Current Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifinptr](aifinptr) module"]
pub type AIFINPTR = crate::Reg<u32, _AIFINPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIFINPTR;
#[doc = "`read()` method returns [aifinptr::R](aifinptr::R) reader structure"]
impl crate::Readable for AIFINPTR {}
#[doc = "`write(|w| ..)` method takes [aifinptr::W](aifinptr::W) writer structure"]
impl crate::Writable for AIFINPTR {}
#[doc = "DMA Input Buffer Current Pointer"]
pub mod aifinptr;
#[doc = "DMA Output Buffer Next Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifoutptrnext](aifoutptrnext) module"]
pub type AIFOUTPTRNEXT = crate::Reg<u32, _AIFOUTPTRNEXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIFOUTPTRNEXT;
#[doc = "`read()` method returns [aifoutptrnext::R](aifoutptrnext::R) reader structure"]
impl crate::Readable for AIFOUTPTRNEXT {}
#[doc = "`write(|w| ..)` method takes [aifoutptrnext::W](aifoutptrnext::W) writer structure"]
impl crate::Writable for AIFOUTPTRNEXT {}
#[doc = "DMA Output Buffer Next Pointer"]
pub mod aifoutptrnext;
#[doc = "DMA Output Buffer Current Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifoutptr](aifoutptr) module"]
pub type AIFOUTPTR = crate::Reg<u32, _AIFOUTPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIFOUTPTR;
#[doc = "`read()` method returns [aifoutptr::R](aifoutptr::R) reader structure"]
impl crate::Readable for AIFOUTPTR {}
#[doc = "`write(|w| ..)` method takes [aifoutptr::W](aifoutptr::W) writer structure"]
impl crate::Writable for AIFOUTPTR {}
#[doc = "DMA Output Buffer Current Pointer"]
pub mod aifoutptr;
#[doc = "SampleStaMP Generator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpctl](stmpctl) module"]
pub type STMPCTL = crate::Reg<u32, _STMPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMPCTL;
#[doc = "`read()` method returns [stmpctl::R](stmpctl::R) reader structure"]
impl crate::Readable for STMPCTL {}
#[doc = "`write(|w| ..)` method takes [stmpctl::W](stmpctl::W) writer structure"]
impl crate::Writable for STMPCTL {}
#[doc = "SampleStaMP Generator Control Register"]
pub mod stmpctl;
#[doc = "Captured XOSC Counter Value, Capture Channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpxcntcapt0](stmpxcntcapt0) module"]
pub type STMPXCNTCAPT0 = crate::Reg<u32, _STMPXCNTCAPT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMPXCNTCAPT0;
#[doc = "`read()` method returns [stmpxcntcapt0::R](stmpxcntcapt0::R) reader structure"]
impl crate::Readable for STMPXCNTCAPT0 {}
#[doc = "Captured XOSC Counter Value, Capture Channel 0"]
pub mod stmpxcntcapt0;
#[doc = "XOSC Period Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpxper](stmpxper) module"]
pub type STMPXPER = crate::Reg<u32, _STMPXPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMPXPER;
#[doc = "`read()` method returns [stmpxper::R](stmpxper::R) reader structure"]
impl crate::Readable for STMPXPER {}
#[doc = "XOSC Period Value"]
pub mod stmpxper;
#[doc = "Captured WCLK Counter Value, Capture Channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpwcntcapt0](stmpwcntcapt0) module"]
pub type STMPWCNTCAPT0 = crate::Reg<u32, _STMPWCNTCAPT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMPWCNTCAPT0;
#[doc = "`read()` method returns [stmpwcntcapt0::R](stmpwcntcapt0::R) reader structure"]
impl crate::Readable for STMPWCNTCAPT0 {}
#[doc = "Captured WCLK Counter Value, Capture Channel 0"]
pub mod stmpwcntcapt0;
#[doc = "WCLK Counter Period Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpwper](stmpwper) module"]
pub type STMPWPER = crate::Reg<u32, _STMPWPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMPWPER;
#[doc = "`read()` method returns [stmpwper::R](stmpwper::R) reader structure"]
impl crate::Readable for STMPWPER {}
#[doc = "`write(|w| ..)` method takes [stmpwper::W](stmpwper::W) writer structure"]
impl crate::Writable for STMPWPER {}
#[doc = "WCLK Counter Period Value"]
pub mod stmpwper;
#[doc = "WCLK Counter Trigger Value for Input Pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpintrig](stmpintrig) module"]
pub type STMPINTRIG = crate::Reg<u32, _STMPINTRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMPINTRIG;
#[doc = "`read()` method returns [stmpintrig::R](stmpintrig::R) reader structure"]
impl crate::Readable for STMPINTRIG {}
#[doc = "`write(|w| ..)` method takes [stmpintrig::W](stmpintrig::W) writer structure"]
impl crate::Writable for STMPINTRIG {}
#[doc = "WCLK Counter Trigger Value for Input Pins"]
pub mod stmpintrig;
#[doc = "WCLK Counter Trigger Value for Output Pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpouttrig](stmpouttrig) module"]
pub type STMPOUTTRIG = crate::Reg<u32, _STMPOUTTRIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMPOUTTRIG;
#[doc = "`read()` method returns [stmpouttrig::R](stmpouttrig::R) reader structure"]
impl crate::Readable for STMPOUTTRIG {}
#[doc = "`write(|w| ..)` method takes [stmpouttrig::W](stmpouttrig::W) writer structure"]
impl crate::Writable for STMPOUTTRIG {}
#[doc = "WCLK Counter Trigger Value for Output Pins"]
pub mod stmpouttrig;
#[doc = "WCLK Counter Set Operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpwset](stmpwset) module"]
pub type STMPWSET = crate::Reg<u32, _STMPWSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMPWSET;
#[doc = "`read()` method returns [stmpwset::R](stmpwset::R) reader structure"]
impl crate::Readable for STMPWSET {}
#[doc = "`write(|w| ..)` method takes [stmpwset::W](stmpwset::W) writer structure"]
impl crate::Writable for STMPWSET {}
#[doc = "WCLK Counter Set Operation"]
pub mod stmpwset;
#[doc = "WCLK Counter Add Operation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpwadd](stmpwadd) module"]
pub type STMPWADD = crate::Reg<u32, _STMPWADD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMPWADD;
#[doc = "`read()` method returns [stmpwadd::R](stmpwadd::R) reader structure"]
impl crate::Readable for STMPWADD {}
#[doc = "`write(|w| ..)` method takes [stmpwadd::W](stmpwadd::W) writer structure"]
impl crate::Writable for STMPWADD {}
#[doc = "WCLK Counter Add Operation"]
pub mod stmpwadd;
#[doc = "XOSC Minimum Period Value Minimum Value of STMPXPER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpxpermin](stmpxpermin) module"]
pub type STMPXPERMIN = crate::Reg<u32, _STMPXPERMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMPXPERMIN;
#[doc = "`read()` method returns [stmpxpermin::R](stmpxpermin::R) reader structure"]
impl crate::Readable for STMPXPERMIN {}
#[doc = "`write(|w| ..)` method takes [stmpxpermin::W](stmpxpermin::W) writer structure"]
impl crate::Writable for STMPXPERMIN {}
#[doc = "XOSC Minimum Period Value Minimum Value of STMPXPER"]
pub mod stmpxpermin;
#[doc = "Current Value of WCNT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpwcnt](stmpwcnt) module"]
pub type STMPWCNT = crate::Reg<u32, _STMPWCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMPWCNT;
#[doc = "`read()` method returns [stmpwcnt::R](stmpwcnt::R) reader structure"]
impl crate::Readable for STMPWCNT {}
#[doc = "Current Value of WCNT"]
pub mod stmpwcnt;
#[doc = "Current Value of XCNT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpxcnt](stmpxcnt) module"]
pub type STMPXCNT = crate::Reg<u32, _STMPXCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMPXCNT;
#[doc = "`read()` method returns [stmpxcnt::R](stmpxcnt::R) reader structure"]
impl crate::Readable for STMPXCNT {}
#[doc = "Current Value of XCNT"]
pub mod stmpxcnt;
#[doc = "Captured XOSC Counter Value, Capture Channel 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpxcntcapt1](stmpxcntcapt1) module"]
pub type STMPXCNTCAPT1 = crate::Reg<u32, _STMPXCNTCAPT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMPXCNTCAPT1;
#[doc = "`read()` method returns [stmpxcntcapt1::R](stmpxcntcapt1::R) reader structure"]
impl crate::Readable for STMPXCNTCAPT1 {}
#[doc = "Captured XOSC Counter Value, Capture Channel 1"]
pub mod stmpxcntcapt1;
#[doc = "Captured WCLK Counter Value, Capture Channel 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpwcntcapt1](stmpwcntcapt1) module"]
pub type STMPWCNTCAPT1 = crate::Reg<u32, _STMPWCNTCAPT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STMPWCNTCAPT1;
#[doc = "`read()` method returns [stmpwcntcapt1::R](stmpwcntcapt1::R) reader structure"]
impl crate::Readable for STMPWCNTCAPT1 {}
#[doc = "Captured WCLK Counter Value, Capture Channel 1"]
pub mod stmpwcntcapt1;
#[doc = "Masked Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqmask](irqmask) module"]
pub type IRQMASK = crate::Reg<u32, _IRQMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQMASK;
#[doc = "`read()` method returns [irqmask::R](irqmask::R) reader structure"]
impl crate::Readable for IRQMASK {}
#[doc = "`write(|w| ..)` method takes [irqmask::W](irqmask::W) writer structure"]
impl crate::Writable for IRQMASK {}
#[doc = "Masked Interrupt Status Register"]
pub mod irqmask;
#[doc = "Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqflags](irqflags) module"]
pub type IRQFLAGS = crate::Reg<u32, _IRQFLAGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQFLAGS;
#[doc = "`read()` method returns [irqflags::R](irqflags::R) reader structure"]
impl crate::Readable for IRQFLAGS {}
#[doc = "Raw Interrupt Status Register"]
pub mod irqflags;
#[doc = "Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqset](irqset) module"]
pub type IRQSET = crate::Reg<u32, _IRQSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSET;
#[doc = "`write(|w| ..)` method takes [irqset::W](irqset::W) writer structure"]
impl crate::Writable for IRQSET {}
#[doc = "Interrupt Set Register"]
pub mod irqset;
#[doc = "Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqclr](irqclr) module"]
pub type IRQCLR = crate::Reg<u32, _IRQCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQCLR;
#[doc = "`write(|w| ..)` method takes [irqclr::W](irqclr::W) writer structure"]
impl crate::Writable for IRQCLR {}
#[doc = "Interrupt Clear Register"]
pub mod irqclr;

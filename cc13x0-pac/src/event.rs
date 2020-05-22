#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Output Selection for CPU Interrupt 0"]
    pub cpuirqsel0: CPUIRQSEL0,
    #[doc = "0x04 - Output Selection for CPU Interrupt 1"]
    pub cpuirqsel1: CPUIRQSEL1,
    #[doc = "0x08 - Output Selection for CPU Interrupt 2"]
    pub cpuirqsel2: CPUIRQSEL2,
    #[doc = "0x0c - Output Selection for CPU Interrupt 3"]
    pub cpuirqsel3: CPUIRQSEL3,
    #[doc = "0x10 - Output Selection for CPU Interrupt 4"]
    pub cpuirqsel4: CPUIRQSEL4,
    #[doc = "0x14 - Output Selection for CPU Interrupt 5"]
    pub cpuirqsel5: CPUIRQSEL5,
    #[doc = "0x18 - Output Selection for CPU Interrupt 6"]
    pub cpuirqsel6: CPUIRQSEL6,
    #[doc = "0x1c - Output Selection for CPU Interrupt 7"]
    pub cpuirqsel7: CPUIRQSEL7,
    #[doc = "0x20 - Output Selection for CPU Interrupt 8"]
    pub cpuirqsel8: CPUIRQSEL8,
    #[doc = "0x24 - Output Selection for CPU Interrupt 9"]
    pub cpuirqsel9: CPUIRQSEL9,
    #[doc = "0x28 - Output Selection for CPU Interrupt 10"]
    pub cpuirqsel10: CPUIRQSEL10,
    #[doc = "0x2c - Output Selection for CPU Interrupt 11"]
    pub cpuirqsel11: CPUIRQSEL11,
    #[doc = "0x30 - Output Selection for CPU Interrupt 12"]
    pub cpuirqsel12: CPUIRQSEL12,
    #[doc = "0x34 - Output Selection for CPU Interrupt 13"]
    pub cpuirqsel13: CPUIRQSEL13,
    #[doc = "0x38 - Output Selection for CPU Interrupt 14"]
    pub cpuirqsel14: CPUIRQSEL14,
    #[doc = "0x3c - Output Selection for CPU Interrupt 15"]
    pub cpuirqsel15: CPUIRQSEL15,
    #[doc = "0x40 - Output Selection for CPU Interrupt 16"]
    pub cpuirqsel16: CPUIRQSEL16,
    #[doc = "0x44 - Output Selection for CPU Interrupt 17"]
    pub cpuirqsel17: CPUIRQSEL17,
    #[doc = "0x48 - Output Selection for CPU Interrupt 18"]
    pub cpuirqsel18: CPUIRQSEL18,
    #[doc = "0x4c - Output Selection for CPU Interrupt 19"]
    pub cpuirqsel19: CPUIRQSEL19,
    #[doc = "0x50 - Output Selection for CPU Interrupt 20"]
    pub cpuirqsel20: CPUIRQSEL20,
    #[doc = "0x54 - Output Selection for CPU Interrupt 21"]
    pub cpuirqsel21: CPUIRQSEL21,
    #[doc = "0x58 - Output Selection for CPU Interrupt 22"]
    pub cpuirqsel22: CPUIRQSEL22,
    #[doc = "0x5c - Output Selection for CPU Interrupt 23"]
    pub cpuirqsel23: CPUIRQSEL23,
    #[doc = "0x60 - Output Selection for CPU Interrupt 24"]
    pub cpuirqsel24: CPUIRQSEL24,
    #[doc = "0x64 - Output Selection for CPU Interrupt 25"]
    pub cpuirqsel25: CPUIRQSEL25,
    #[doc = "0x68 - Output Selection for CPU Interrupt 26"]
    pub cpuirqsel26: CPUIRQSEL26,
    #[doc = "0x6c - Output Selection for CPU Interrupt 27"]
    pub cpuirqsel27: CPUIRQSEL27,
    #[doc = "0x70 - Output Selection for CPU Interrupt 28"]
    pub cpuirqsel28: CPUIRQSEL28,
    #[doc = "0x74 - Output Selection for CPU Interrupt 29"]
    pub cpuirqsel29: CPUIRQSEL29,
    #[doc = "0x78 - Output Selection for CPU Interrupt 30"]
    pub cpuirqsel30: CPUIRQSEL30,
    #[doc = "0x7c - Output Selection for CPU Interrupt 31"]
    pub cpuirqsel31: CPUIRQSEL31,
    #[doc = "0x80 - Output Selection for CPU Interrupt 32"]
    pub cpuirqsel32: CPUIRQSEL32,
    #[doc = "0x84 - Output Selection for CPU Interrupt 33"]
    pub cpuirqsel33: CPUIRQSEL33,
    _reserved34: [u8; 120usize],
    #[doc = "0x100 - Output Selection for RFC Event 0"]
    pub rfcsel0: RFCSEL0,
    #[doc = "0x104 - Output Selection for RFC Event 1"]
    pub rfcsel1: RFCSEL1,
    #[doc = "0x108 - Output Selection for RFC Event 2"]
    pub rfcsel2: RFCSEL2,
    #[doc = "0x10c - Output Selection for RFC Event 3"]
    pub rfcsel3: RFCSEL3,
    #[doc = "0x110 - Output Selection for RFC Event 4"]
    pub rfcsel4: RFCSEL4,
    #[doc = "0x114 - Output Selection for RFC Event 5"]
    pub rfcsel5: RFCSEL5,
    #[doc = "0x118 - Output Selection for RFC Event 6"]
    pub rfcsel6: RFCSEL6,
    #[doc = "0x11c - Output Selection for RFC Event 7"]
    pub rfcsel7: RFCSEL7,
    #[doc = "0x120 - Output Selection for RFC Event 8"]
    pub rfcsel8: RFCSEL8,
    #[doc = "0x124 - Output Selection for RFC Event 9"]
    pub rfcsel9: RFCSEL9,
    _reserved44: [u8; 216usize],
    #[doc = "0x200 - Output Selection for GPT0 0"]
    pub gpt0acaptsel: GPT0ACAPTSEL,
    #[doc = "0x204 - Output Selection for GPT0 1"]
    pub gpt0bcaptsel: GPT0BCAPTSEL,
    _reserved46: [u8; 248usize],
    #[doc = "0x300 - Output Selection for GPT1 0"]
    pub gpt1acaptsel: GPT1ACAPTSEL,
    #[doc = "0x304 - Output Selection for GPT1 1"]
    pub gpt1bcaptsel: GPT1BCAPTSEL,
    _reserved48: [u8; 248usize],
    #[doc = "0x400 - Output Selection for GPT2 0"]
    pub gpt2acaptsel: GPT2ACAPTSEL,
    #[doc = "0x404 - Output Selection for GPT2 1"]
    pub gpt2bcaptsel: GPT2BCAPTSEL,
    _reserved50: [u8; 256usize],
    #[doc = "0x508 - Output Selection for DMA Channel 1 SREQ"]
    pub udmach1ssel: UDMACH1SSEL,
    #[doc = "0x50c - Output Selection for DMA Channel 1 REQ"]
    pub udmach1bsel: UDMACH1BSEL,
    #[doc = "0x510 - Output Selection for DMA Channel 2 SREQ"]
    pub udmach2ssel: UDMACH2SSEL,
    #[doc = "0x514 - Output Selection for DMA Channel 2 REQ"]
    pub udmach2bsel: UDMACH2BSEL,
    #[doc = "0x518 - Output Selection for DMA Channel 3 SREQ"]
    pub udmach3ssel: UDMACH3SSEL,
    #[doc = "0x51c - Output Selection for DMA Channel 3 REQ"]
    pub udmach3bsel: UDMACH3BSEL,
    #[doc = "0x520 - Output Selection for DMA Channel 4 SREQ"]
    pub udmach4ssel: UDMACH4SSEL,
    #[doc = "0x524 - Output Selection for DMA Channel 4 REQ"]
    pub udmach4bsel: UDMACH4BSEL,
    #[doc = "0x528 - Output Selection for DMA Channel 5 SREQ"]
    pub udmach5ssel: UDMACH5SSEL,
    #[doc = "0x52c - Output Selection for DMA Channel 5 REQ"]
    pub udmach5bsel: UDMACH5BSEL,
    #[doc = "0x530 - Output Selection for DMA Channel 6 SREQ"]
    pub udmach6ssel: UDMACH6SSEL,
    #[doc = "0x534 - Output Selection for DMA Channel 6 REQ"]
    pub udmach6bsel: UDMACH6BSEL,
    #[doc = "0x538 - Output Selection for DMA Channel 7 SREQ"]
    pub udmach7ssel: UDMACH7SSEL,
    #[doc = "0x53c - Output Selection for DMA Channel 7 REQ"]
    pub udmach7bsel: UDMACH7BSEL,
    #[doc = "0x540 - Output Selection for DMA Channel 8 SREQ Single request is ignored for this channel"]
    pub udmach8ssel: UDMACH8SSEL,
    #[doc = "0x544 - Output Selection for DMA Channel 8 REQ"]
    pub udmach8bsel: UDMACH8BSEL,
    #[doc = "0x548 - Output Selection for DMA Channel 9 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
    pub udmach9ssel: UDMACH9SSEL,
    #[doc = "0x54c - Output Selection for DMA Channel 9 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
    pub udmach9bsel: UDMACH9BSEL,
    #[doc = "0x550 - Output Selection for DMA Channel 10 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
    pub udmach10ssel: UDMACH10SSEL,
    #[doc = "0x554 - Output Selection for DMA Channel 10 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
    pub udmach10bsel: UDMACH10BSEL,
    #[doc = "0x558 - Output Selection for DMA Channel 11 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
    pub udmach11ssel: UDMACH11SSEL,
    #[doc = "0x55c - Output Selection for DMA Channel 11 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
    pub udmach11bsel: UDMACH11BSEL,
    #[doc = "0x560 - Output Selection for DMA Channel 12 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
    pub udmach12ssel: UDMACH12SSEL,
    #[doc = "0x564 - Output Selection for DMA Channel 12 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
    pub udmach12bsel: UDMACH12BSEL,
    _reserved74: [u8; 4usize],
    #[doc = "0x56c - Output Selection for DMA Channel 13 REQ"]
    pub udmach13bsel: UDMACH13BSEL,
    _reserved75: [u8; 4usize],
    #[doc = "0x574 - Output Selection for DMA Channel 14 REQ"]
    pub udmach14bsel: UDMACH14BSEL,
    _reserved76: [u8; 4usize],
    #[doc = "0x57c - Output Selection for DMA Channel 15 REQ"]
    pub udmach15bsel: UDMACH15BSEL,
    #[doc = "0x580 - Output Selection for DMA Channel 16 SREQ"]
    pub udmach16ssel: UDMACH16SSEL,
    #[doc = "0x584 - Output Selection for DMA Channel 16 REQ"]
    pub udmach16bsel: UDMACH16BSEL,
    #[doc = "0x588 - Output Selection for DMA Channel 17 SREQ"]
    pub udmach17ssel: UDMACH17SSEL,
    #[doc = "0x58c - Output Selection for DMA Channel 17 REQ"]
    pub udmach17bsel: UDMACH17BSEL,
    _reserved81: [u8; 24usize],
    #[doc = "0x5a8 - Output Selection for DMA Channel 21 SREQ"]
    pub udmach21ssel: UDMACH21SSEL,
    #[doc = "0x5ac - Output Selection for DMA Channel 21 REQ"]
    pub udmach21bsel: UDMACH21BSEL,
    #[doc = "0x5b0 - Output Selection for DMA Channel 22 SREQ"]
    pub udmach22ssel: UDMACH22SSEL,
    #[doc = "0x5b4 - Output Selection for DMA Channel 22 REQ"]
    pub udmach22bsel: UDMACH22BSEL,
    #[doc = "0x5b8 - Output Selection for DMA Channel 23 SREQ"]
    pub udmach23ssel: UDMACH23SSEL,
    #[doc = "0x5bc - Output Selection for DMA Channel 23 REQ"]
    pub udmach23bsel: UDMACH23BSEL,
    #[doc = "0x5c0 - Output Selection for DMA Channel 24 SREQ"]
    pub udmach24ssel: UDMACH24SSEL,
    #[doc = "0x5c4 - Output Selection for DMA Channel 24 REQ"]
    pub udmach24bsel: UDMACH24BSEL,
    _reserved89: [u8; 56usize],
    #[doc = "0x600 - Output Selection for GPT3 0"]
    pub gpt3acaptsel: GPT3ACAPTSEL,
    #[doc = "0x604 - Output Selection for GPT3 1"]
    pub gpt3bcaptsel: GPT3BCAPTSEL,
    _reserved91: [u8; 248usize],
    #[doc = "0x700 - Output Selection for AUX Subscriber 0"]
    pub auxsel0: AUXSEL0,
    _reserved92: [u8; 252usize],
    #[doc = "0x800 - Output Selection for NMI Subscriber 0"]
    pub cm3nmisel0: CM3NMISEL0,
    _reserved93: [u8; 252usize],
    #[doc = "0x900 - Output Selection for I2S Subscriber 0"]
    pub i2sstmpsel0: I2SSTMPSEL0,
    _reserved94: [u8; 252usize],
    #[doc = "0xa00 - Output Selection for FRZ Subscriber 0"]
    pub frzsel0: FRZSEL0,
    _reserved95: [u8; 1276usize],
    #[doc = "0xf00 - Set or Clear Software Events"]
    pub swev: SWEV,
}
#[doc = "Output Selection for CPU Interrupt 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel0](cpuirqsel0) module"]
pub type CPUIRQSEL0 = crate::Reg<u32, _CPUIRQSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL0;
#[doc = "`read()` method returns [cpuirqsel0::R](cpuirqsel0::R) reader structure"]
impl crate::Readable for CPUIRQSEL0 {}
#[doc = "Output Selection for CPU Interrupt 0"]
pub mod cpuirqsel0;
#[doc = "Output Selection for CPU Interrupt 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel1](cpuirqsel1) module"]
pub type CPUIRQSEL1 = crate::Reg<u32, _CPUIRQSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL1;
#[doc = "`read()` method returns [cpuirqsel1::R](cpuirqsel1::R) reader structure"]
impl crate::Readable for CPUIRQSEL1 {}
#[doc = "Output Selection for CPU Interrupt 1"]
pub mod cpuirqsel1;
#[doc = "Output Selection for CPU Interrupt 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel2](cpuirqsel2) module"]
pub type CPUIRQSEL2 = crate::Reg<u32, _CPUIRQSEL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL2;
#[doc = "`read()` method returns [cpuirqsel2::R](cpuirqsel2::R) reader structure"]
impl crate::Readable for CPUIRQSEL2 {}
#[doc = "Output Selection for CPU Interrupt 2"]
pub mod cpuirqsel2;
#[doc = "Output Selection for CPU Interrupt 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel3](cpuirqsel3) module"]
pub type CPUIRQSEL3 = crate::Reg<u32, _CPUIRQSEL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL3;
#[doc = "`read()` method returns [cpuirqsel3::R](cpuirqsel3::R) reader structure"]
impl crate::Readable for CPUIRQSEL3 {}
#[doc = "Output Selection for CPU Interrupt 3"]
pub mod cpuirqsel3;
#[doc = "Output Selection for CPU Interrupt 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel4](cpuirqsel4) module"]
pub type CPUIRQSEL4 = crate::Reg<u32, _CPUIRQSEL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL4;
#[doc = "`read()` method returns [cpuirqsel4::R](cpuirqsel4::R) reader structure"]
impl crate::Readable for CPUIRQSEL4 {}
#[doc = "Output Selection for CPU Interrupt 4"]
pub mod cpuirqsel4;
#[doc = "Output Selection for CPU Interrupt 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel5](cpuirqsel5) module"]
pub type CPUIRQSEL5 = crate::Reg<u32, _CPUIRQSEL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL5;
#[doc = "`read()` method returns [cpuirqsel5::R](cpuirqsel5::R) reader structure"]
impl crate::Readable for CPUIRQSEL5 {}
#[doc = "Output Selection for CPU Interrupt 5"]
pub mod cpuirqsel5;
#[doc = "Output Selection for CPU Interrupt 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel6](cpuirqsel6) module"]
pub type CPUIRQSEL6 = crate::Reg<u32, _CPUIRQSEL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL6;
#[doc = "`read()` method returns [cpuirqsel6::R](cpuirqsel6::R) reader structure"]
impl crate::Readable for CPUIRQSEL6 {}
#[doc = "Output Selection for CPU Interrupt 6"]
pub mod cpuirqsel6;
#[doc = "Output Selection for CPU Interrupt 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel7](cpuirqsel7) module"]
pub type CPUIRQSEL7 = crate::Reg<u32, _CPUIRQSEL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL7;
#[doc = "`read()` method returns [cpuirqsel7::R](cpuirqsel7::R) reader structure"]
impl crate::Readable for CPUIRQSEL7 {}
#[doc = "Output Selection for CPU Interrupt 7"]
pub mod cpuirqsel7;
#[doc = "Output Selection for CPU Interrupt 8\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel8](cpuirqsel8) module"]
pub type CPUIRQSEL8 = crate::Reg<u32, _CPUIRQSEL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL8;
#[doc = "`read()` method returns [cpuirqsel8::R](cpuirqsel8::R) reader structure"]
impl crate::Readable for CPUIRQSEL8 {}
#[doc = "Output Selection for CPU Interrupt 8"]
pub mod cpuirqsel8;
#[doc = "Output Selection for CPU Interrupt 9\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel9](cpuirqsel9) module"]
pub type CPUIRQSEL9 = crate::Reg<u32, _CPUIRQSEL9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL9;
#[doc = "`read()` method returns [cpuirqsel9::R](cpuirqsel9::R) reader structure"]
impl crate::Readable for CPUIRQSEL9 {}
#[doc = "Output Selection for CPU Interrupt 9"]
pub mod cpuirqsel9;
#[doc = "Output Selection for CPU Interrupt 10\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel10](cpuirqsel10) module"]
pub type CPUIRQSEL10 = crate::Reg<u32, _CPUIRQSEL10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL10;
#[doc = "`read()` method returns [cpuirqsel10::R](cpuirqsel10::R) reader structure"]
impl crate::Readable for CPUIRQSEL10 {}
#[doc = "Output Selection for CPU Interrupt 10"]
pub mod cpuirqsel10;
#[doc = "Output Selection for CPU Interrupt 11\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel11](cpuirqsel11) module"]
pub type CPUIRQSEL11 = crate::Reg<u32, _CPUIRQSEL11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL11;
#[doc = "`read()` method returns [cpuirqsel11::R](cpuirqsel11::R) reader structure"]
impl crate::Readable for CPUIRQSEL11 {}
#[doc = "Output Selection for CPU Interrupt 11"]
pub mod cpuirqsel11;
#[doc = "Output Selection for CPU Interrupt 12\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel12](cpuirqsel12) module"]
pub type CPUIRQSEL12 = crate::Reg<u32, _CPUIRQSEL12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL12;
#[doc = "`read()` method returns [cpuirqsel12::R](cpuirqsel12::R) reader structure"]
impl crate::Readable for CPUIRQSEL12 {}
#[doc = "Output Selection for CPU Interrupt 12"]
pub mod cpuirqsel12;
#[doc = "Output Selection for CPU Interrupt 13\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel13](cpuirqsel13) module"]
pub type CPUIRQSEL13 = crate::Reg<u32, _CPUIRQSEL13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL13;
#[doc = "`read()` method returns [cpuirqsel13::R](cpuirqsel13::R) reader structure"]
impl crate::Readable for CPUIRQSEL13 {}
#[doc = "Output Selection for CPU Interrupt 13"]
pub mod cpuirqsel13;
#[doc = "Output Selection for CPU Interrupt 14\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel14](cpuirqsel14) module"]
pub type CPUIRQSEL14 = crate::Reg<u32, _CPUIRQSEL14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL14;
#[doc = "`read()` method returns [cpuirqsel14::R](cpuirqsel14::R) reader structure"]
impl crate::Readable for CPUIRQSEL14 {}
#[doc = "Output Selection for CPU Interrupt 14"]
pub mod cpuirqsel14;
#[doc = "Output Selection for CPU Interrupt 15\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel15](cpuirqsel15) module"]
pub type CPUIRQSEL15 = crate::Reg<u32, _CPUIRQSEL15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL15;
#[doc = "`read()` method returns [cpuirqsel15::R](cpuirqsel15::R) reader structure"]
impl crate::Readable for CPUIRQSEL15 {}
#[doc = "Output Selection for CPU Interrupt 15"]
pub mod cpuirqsel15;
#[doc = "Output Selection for CPU Interrupt 16\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel16](cpuirqsel16) module"]
pub type CPUIRQSEL16 = crate::Reg<u32, _CPUIRQSEL16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL16;
#[doc = "`read()` method returns [cpuirqsel16::R](cpuirqsel16::R) reader structure"]
impl crate::Readable for CPUIRQSEL16 {}
#[doc = "Output Selection for CPU Interrupt 16"]
pub mod cpuirqsel16;
#[doc = "Output Selection for CPU Interrupt 17\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel17](cpuirqsel17) module"]
pub type CPUIRQSEL17 = crate::Reg<u32, _CPUIRQSEL17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL17;
#[doc = "`read()` method returns [cpuirqsel17::R](cpuirqsel17::R) reader structure"]
impl crate::Readable for CPUIRQSEL17 {}
#[doc = "Output Selection for CPU Interrupt 17"]
pub mod cpuirqsel17;
#[doc = "Output Selection for CPU Interrupt 18\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel18](cpuirqsel18) module"]
pub type CPUIRQSEL18 = crate::Reg<u32, _CPUIRQSEL18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL18;
#[doc = "`read()` method returns [cpuirqsel18::R](cpuirqsel18::R) reader structure"]
impl crate::Readable for CPUIRQSEL18 {}
#[doc = "Output Selection for CPU Interrupt 18"]
pub mod cpuirqsel18;
#[doc = "Output Selection for CPU Interrupt 19\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel19](cpuirqsel19) module"]
pub type CPUIRQSEL19 = crate::Reg<u32, _CPUIRQSEL19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL19;
#[doc = "`read()` method returns [cpuirqsel19::R](cpuirqsel19::R) reader structure"]
impl crate::Readable for CPUIRQSEL19 {}
#[doc = "Output Selection for CPU Interrupt 19"]
pub mod cpuirqsel19;
#[doc = "Output Selection for CPU Interrupt 20\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel20](cpuirqsel20) module"]
pub type CPUIRQSEL20 = crate::Reg<u32, _CPUIRQSEL20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL20;
#[doc = "`read()` method returns [cpuirqsel20::R](cpuirqsel20::R) reader structure"]
impl crate::Readable for CPUIRQSEL20 {}
#[doc = "Output Selection for CPU Interrupt 20"]
pub mod cpuirqsel20;
#[doc = "Output Selection for CPU Interrupt 21\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel21](cpuirqsel21) module"]
pub type CPUIRQSEL21 = crate::Reg<u32, _CPUIRQSEL21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL21;
#[doc = "`read()` method returns [cpuirqsel21::R](cpuirqsel21::R) reader structure"]
impl crate::Readable for CPUIRQSEL21 {}
#[doc = "Output Selection for CPU Interrupt 21"]
pub mod cpuirqsel21;
#[doc = "Output Selection for CPU Interrupt 22\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel22](cpuirqsel22) module"]
pub type CPUIRQSEL22 = crate::Reg<u32, _CPUIRQSEL22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL22;
#[doc = "`read()` method returns [cpuirqsel22::R](cpuirqsel22::R) reader structure"]
impl crate::Readable for CPUIRQSEL22 {}
#[doc = "Output Selection for CPU Interrupt 22"]
pub mod cpuirqsel22;
#[doc = "Output Selection for CPU Interrupt 23\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel23](cpuirqsel23) module"]
pub type CPUIRQSEL23 = crate::Reg<u32, _CPUIRQSEL23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL23;
#[doc = "`read()` method returns [cpuirqsel23::R](cpuirqsel23::R) reader structure"]
impl crate::Readable for CPUIRQSEL23 {}
#[doc = "Output Selection for CPU Interrupt 23"]
pub mod cpuirqsel23;
#[doc = "Output Selection for CPU Interrupt 24\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel24](cpuirqsel24) module"]
pub type CPUIRQSEL24 = crate::Reg<u32, _CPUIRQSEL24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL24;
#[doc = "`read()` method returns [cpuirqsel24::R](cpuirqsel24::R) reader structure"]
impl crate::Readable for CPUIRQSEL24 {}
#[doc = "Output Selection for CPU Interrupt 24"]
pub mod cpuirqsel24;
#[doc = "Output Selection for CPU Interrupt 25\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel25](cpuirqsel25) module"]
pub type CPUIRQSEL25 = crate::Reg<u32, _CPUIRQSEL25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL25;
#[doc = "`read()` method returns [cpuirqsel25::R](cpuirqsel25::R) reader structure"]
impl crate::Readable for CPUIRQSEL25 {}
#[doc = "Output Selection for CPU Interrupt 25"]
pub mod cpuirqsel25;
#[doc = "Output Selection for CPU Interrupt 26\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel26](cpuirqsel26) module"]
pub type CPUIRQSEL26 = crate::Reg<u32, _CPUIRQSEL26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL26;
#[doc = "`read()` method returns [cpuirqsel26::R](cpuirqsel26::R) reader structure"]
impl crate::Readable for CPUIRQSEL26 {}
#[doc = "Output Selection for CPU Interrupt 26"]
pub mod cpuirqsel26;
#[doc = "Output Selection for CPU Interrupt 27\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel27](cpuirqsel27) module"]
pub type CPUIRQSEL27 = crate::Reg<u32, _CPUIRQSEL27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL27;
#[doc = "`read()` method returns [cpuirqsel27::R](cpuirqsel27::R) reader structure"]
impl crate::Readable for CPUIRQSEL27 {}
#[doc = "Output Selection for CPU Interrupt 27"]
pub mod cpuirqsel27;
#[doc = "Output Selection for CPU Interrupt 28\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel28](cpuirqsel28) module"]
pub type CPUIRQSEL28 = crate::Reg<u32, _CPUIRQSEL28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL28;
#[doc = "`read()` method returns [cpuirqsel28::R](cpuirqsel28::R) reader structure"]
impl crate::Readable for CPUIRQSEL28 {}
#[doc = "Output Selection for CPU Interrupt 28"]
pub mod cpuirqsel28;
#[doc = "Output Selection for CPU Interrupt 29\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel29](cpuirqsel29) module"]
pub type CPUIRQSEL29 = crate::Reg<u32, _CPUIRQSEL29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL29;
#[doc = "`read()` method returns [cpuirqsel29::R](cpuirqsel29::R) reader structure"]
impl crate::Readable for CPUIRQSEL29 {}
#[doc = "Output Selection for CPU Interrupt 29"]
pub mod cpuirqsel29;
#[doc = "Output Selection for CPU Interrupt 30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel30](cpuirqsel30) module"]
pub type CPUIRQSEL30 = crate::Reg<u32, _CPUIRQSEL30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL30;
#[doc = "`read()` method returns [cpuirqsel30::R](cpuirqsel30::R) reader structure"]
impl crate::Readable for CPUIRQSEL30 {}
#[doc = "`write(|w| ..)` method takes [cpuirqsel30::W](cpuirqsel30::W) writer structure"]
impl crate::Writable for CPUIRQSEL30 {}
#[doc = "Output Selection for CPU Interrupt 30"]
pub mod cpuirqsel30;
#[doc = "Output Selection for CPU Interrupt 31\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel31](cpuirqsel31) module"]
pub type CPUIRQSEL31 = crate::Reg<u32, _CPUIRQSEL31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL31;
#[doc = "`read()` method returns [cpuirqsel31::R](cpuirqsel31::R) reader structure"]
impl crate::Readable for CPUIRQSEL31 {}
#[doc = "Output Selection for CPU Interrupt 31"]
pub mod cpuirqsel31;
#[doc = "Output Selection for CPU Interrupt 32\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel32](cpuirqsel32) module"]
pub type CPUIRQSEL32 = crate::Reg<u32, _CPUIRQSEL32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL32;
#[doc = "`read()` method returns [cpuirqsel32::R](cpuirqsel32::R) reader structure"]
impl crate::Readable for CPUIRQSEL32 {}
#[doc = "Output Selection for CPU Interrupt 32"]
pub mod cpuirqsel32;
#[doc = "Output Selection for CPU Interrupt 33\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuirqsel33](cpuirqsel33) module"]
pub type CPUIRQSEL33 = crate::Reg<u32, _CPUIRQSEL33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUIRQSEL33;
#[doc = "`read()` method returns [cpuirqsel33::R](cpuirqsel33::R) reader structure"]
impl crate::Readable for CPUIRQSEL33 {}
#[doc = "Output Selection for CPU Interrupt 33"]
pub mod cpuirqsel33;
#[doc = "Output Selection for RFC Event 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcsel0](rfcsel0) module"]
pub type RFCSEL0 = crate::Reg<u32, _RFCSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCSEL0;
#[doc = "`read()` method returns [rfcsel0::R](rfcsel0::R) reader structure"]
impl crate::Readable for RFCSEL0 {}
#[doc = "Output Selection for RFC Event 0"]
pub mod rfcsel0;
#[doc = "Output Selection for RFC Event 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcsel1](rfcsel1) module"]
pub type RFCSEL1 = crate::Reg<u32, _RFCSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCSEL1;
#[doc = "`read()` method returns [rfcsel1::R](rfcsel1::R) reader structure"]
impl crate::Readable for RFCSEL1 {}
#[doc = "Output Selection for RFC Event 1"]
pub mod rfcsel1;
#[doc = "Output Selection for RFC Event 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcsel2](rfcsel2) module"]
pub type RFCSEL2 = crate::Reg<u32, _RFCSEL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCSEL2;
#[doc = "`read()` method returns [rfcsel2::R](rfcsel2::R) reader structure"]
impl crate::Readable for RFCSEL2 {}
#[doc = "Output Selection for RFC Event 2"]
pub mod rfcsel2;
#[doc = "Output Selection for RFC Event 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcsel3](rfcsel3) module"]
pub type RFCSEL3 = crate::Reg<u32, _RFCSEL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCSEL3;
#[doc = "`read()` method returns [rfcsel3::R](rfcsel3::R) reader structure"]
impl crate::Readable for RFCSEL3 {}
#[doc = "Output Selection for RFC Event 3"]
pub mod rfcsel3;
#[doc = "Output Selection for RFC Event 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcsel4](rfcsel4) module"]
pub type RFCSEL4 = crate::Reg<u32, _RFCSEL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCSEL4;
#[doc = "`read()` method returns [rfcsel4::R](rfcsel4::R) reader structure"]
impl crate::Readable for RFCSEL4 {}
#[doc = "Output Selection for RFC Event 4"]
pub mod rfcsel4;
#[doc = "Output Selection for RFC Event 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcsel5](rfcsel5) module"]
pub type RFCSEL5 = crate::Reg<u32, _RFCSEL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCSEL5;
#[doc = "`read()` method returns [rfcsel5::R](rfcsel5::R) reader structure"]
impl crate::Readable for RFCSEL5 {}
#[doc = "Output Selection for RFC Event 5"]
pub mod rfcsel5;
#[doc = "Output Selection for RFC Event 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcsel6](rfcsel6) module"]
pub type RFCSEL6 = crate::Reg<u32, _RFCSEL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCSEL6;
#[doc = "`read()` method returns [rfcsel6::R](rfcsel6::R) reader structure"]
impl crate::Readable for RFCSEL6 {}
#[doc = "Output Selection for RFC Event 6"]
pub mod rfcsel6;
#[doc = "Output Selection for RFC Event 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcsel7](rfcsel7) module"]
pub type RFCSEL7 = crate::Reg<u32, _RFCSEL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCSEL7;
#[doc = "`read()` method returns [rfcsel7::R](rfcsel7::R) reader structure"]
impl crate::Readable for RFCSEL7 {}
#[doc = "Output Selection for RFC Event 7"]
pub mod rfcsel7;
#[doc = "Output Selection for RFC Event 8\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcsel8](rfcsel8) module"]
pub type RFCSEL8 = crate::Reg<u32, _RFCSEL8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCSEL8;
#[doc = "`read()` method returns [rfcsel8::R](rfcsel8::R) reader structure"]
impl crate::Readable for RFCSEL8 {}
#[doc = "Output Selection for RFC Event 8"]
pub mod rfcsel8;
#[doc = "Output Selection for RFC Event 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcsel9](rfcsel9) module"]
pub type RFCSEL9 = crate::Reg<u32, _RFCSEL9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCSEL9;
#[doc = "`read()` method returns [rfcsel9::R](rfcsel9::R) reader structure"]
impl crate::Readable for RFCSEL9 {}
#[doc = "`write(|w| ..)` method takes [rfcsel9::W](rfcsel9::W) writer structure"]
impl crate::Writable for RFCSEL9 {}
#[doc = "Output Selection for RFC Event 9"]
pub mod rfcsel9;
#[doc = "Output Selection for GPT0 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt0acaptsel](gpt0acaptsel) module"]
pub type GPT0ACAPTSEL = crate::Reg<u32, _GPT0ACAPTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT0ACAPTSEL;
#[doc = "`read()` method returns [gpt0acaptsel::R](gpt0acaptsel::R) reader structure"]
impl crate::Readable for GPT0ACAPTSEL {}
#[doc = "`write(|w| ..)` method takes [gpt0acaptsel::W](gpt0acaptsel::W) writer structure"]
impl crate::Writable for GPT0ACAPTSEL {}
#[doc = "Output Selection for GPT0 0"]
pub mod gpt0acaptsel;
#[doc = "Output Selection for GPT0 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt0bcaptsel](gpt0bcaptsel) module"]
pub type GPT0BCAPTSEL = crate::Reg<u32, _GPT0BCAPTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT0BCAPTSEL;
#[doc = "`read()` method returns [gpt0bcaptsel::R](gpt0bcaptsel::R) reader structure"]
impl crate::Readable for GPT0BCAPTSEL {}
#[doc = "`write(|w| ..)` method takes [gpt0bcaptsel::W](gpt0bcaptsel::W) writer structure"]
impl crate::Writable for GPT0BCAPTSEL {}
#[doc = "Output Selection for GPT0 1"]
pub mod gpt0bcaptsel;
#[doc = "Output Selection for GPT1 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt1acaptsel](gpt1acaptsel) module"]
pub type GPT1ACAPTSEL = crate::Reg<u32, _GPT1ACAPTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT1ACAPTSEL;
#[doc = "`read()` method returns [gpt1acaptsel::R](gpt1acaptsel::R) reader structure"]
impl crate::Readable for GPT1ACAPTSEL {}
#[doc = "`write(|w| ..)` method takes [gpt1acaptsel::W](gpt1acaptsel::W) writer structure"]
impl crate::Writable for GPT1ACAPTSEL {}
#[doc = "Output Selection for GPT1 0"]
pub mod gpt1acaptsel;
#[doc = "Output Selection for GPT1 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt1bcaptsel](gpt1bcaptsel) module"]
pub type GPT1BCAPTSEL = crate::Reg<u32, _GPT1BCAPTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT1BCAPTSEL;
#[doc = "`read()` method returns [gpt1bcaptsel::R](gpt1bcaptsel::R) reader structure"]
impl crate::Readable for GPT1BCAPTSEL {}
#[doc = "`write(|w| ..)` method takes [gpt1bcaptsel::W](gpt1bcaptsel::W) writer structure"]
impl crate::Writable for GPT1BCAPTSEL {}
#[doc = "Output Selection for GPT1 1"]
pub mod gpt1bcaptsel;
#[doc = "Output Selection for GPT2 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt2acaptsel](gpt2acaptsel) module"]
pub type GPT2ACAPTSEL = crate::Reg<u32, _GPT2ACAPTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT2ACAPTSEL;
#[doc = "`read()` method returns [gpt2acaptsel::R](gpt2acaptsel::R) reader structure"]
impl crate::Readable for GPT2ACAPTSEL {}
#[doc = "`write(|w| ..)` method takes [gpt2acaptsel::W](gpt2acaptsel::W) writer structure"]
impl crate::Writable for GPT2ACAPTSEL {}
#[doc = "Output Selection for GPT2 0"]
pub mod gpt2acaptsel;
#[doc = "Output Selection for GPT2 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt2bcaptsel](gpt2bcaptsel) module"]
pub type GPT2BCAPTSEL = crate::Reg<u32, _GPT2BCAPTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT2BCAPTSEL;
#[doc = "`read()` method returns [gpt2bcaptsel::R](gpt2bcaptsel::R) reader structure"]
impl crate::Readable for GPT2BCAPTSEL {}
#[doc = "`write(|w| ..)` method takes [gpt2bcaptsel::W](gpt2bcaptsel::W) writer structure"]
impl crate::Writable for GPT2BCAPTSEL {}
#[doc = "Output Selection for GPT2 1"]
pub mod gpt2bcaptsel;
#[doc = "Output Selection for DMA Channel 1 SREQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach1ssel](udmach1ssel) module"]
pub type UDMACH1SSEL = crate::Reg<u32, _UDMACH1SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH1SSEL;
#[doc = "`read()` method returns [udmach1ssel::R](udmach1ssel::R) reader structure"]
impl crate::Readable for UDMACH1SSEL {}
#[doc = "Output Selection for DMA Channel 1 SREQ"]
pub mod udmach1ssel;
#[doc = "Output Selection for DMA Channel 1 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach1bsel](udmach1bsel) module"]
pub type UDMACH1BSEL = crate::Reg<u32, _UDMACH1BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH1BSEL;
#[doc = "`read()` method returns [udmach1bsel::R](udmach1bsel::R) reader structure"]
impl crate::Readable for UDMACH1BSEL {}
#[doc = "Output Selection for DMA Channel 1 REQ"]
pub mod udmach1bsel;
#[doc = "Output Selection for DMA Channel 2 SREQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach2ssel](udmach2ssel) module"]
pub type UDMACH2SSEL = crate::Reg<u32, _UDMACH2SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH2SSEL;
#[doc = "`read()` method returns [udmach2ssel::R](udmach2ssel::R) reader structure"]
impl crate::Readable for UDMACH2SSEL {}
#[doc = "Output Selection for DMA Channel 2 SREQ"]
pub mod udmach2ssel;
#[doc = "Output Selection for DMA Channel 2 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach2bsel](udmach2bsel) module"]
pub type UDMACH2BSEL = crate::Reg<u32, _UDMACH2BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH2BSEL;
#[doc = "`read()` method returns [udmach2bsel::R](udmach2bsel::R) reader structure"]
impl crate::Readable for UDMACH2BSEL {}
#[doc = "Output Selection for DMA Channel 2 REQ"]
pub mod udmach2bsel;
#[doc = "Output Selection for DMA Channel 3 SREQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach3ssel](udmach3ssel) module"]
pub type UDMACH3SSEL = crate::Reg<u32, _UDMACH3SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH3SSEL;
#[doc = "`read()` method returns [udmach3ssel::R](udmach3ssel::R) reader structure"]
impl crate::Readable for UDMACH3SSEL {}
#[doc = "Output Selection for DMA Channel 3 SREQ"]
pub mod udmach3ssel;
#[doc = "Output Selection for DMA Channel 3 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach3bsel](udmach3bsel) module"]
pub type UDMACH3BSEL = crate::Reg<u32, _UDMACH3BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH3BSEL;
#[doc = "`read()` method returns [udmach3bsel::R](udmach3bsel::R) reader structure"]
impl crate::Readable for UDMACH3BSEL {}
#[doc = "Output Selection for DMA Channel 3 REQ"]
pub mod udmach3bsel;
#[doc = "Output Selection for DMA Channel 4 SREQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach4ssel](udmach4ssel) module"]
pub type UDMACH4SSEL = crate::Reg<u32, _UDMACH4SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH4SSEL;
#[doc = "`read()` method returns [udmach4ssel::R](udmach4ssel::R) reader structure"]
impl crate::Readable for UDMACH4SSEL {}
#[doc = "Output Selection for DMA Channel 4 SREQ"]
pub mod udmach4ssel;
#[doc = "Output Selection for DMA Channel 4 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach4bsel](udmach4bsel) module"]
pub type UDMACH4BSEL = crate::Reg<u32, _UDMACH4BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH4BSEL;
#[doc = "`read()` method returns [udmach4bsel::R](udmach4bsel::R) reader structure"]
impl crate::Readable for UDMACH4BSEL {}
#[doc = "Output Selection for DMA Channel 4 REQ"]
pub mod udmach4bsel;
#[doc = "Output Selection for DMA Channel 5 SREQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach5ssel](udmach5ssel) module"]
pub type UDMACH5SSEL = crate::Reg<u32, _UDMACH5SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH5SSEL;
#[doc = "`read()` method returns [udmach5ssel::R](udmach5ssel::R) reader structure"]
impl crate::Readable for UDMACH5SSEL {}
#[doc = "Output Selection for DMA Channel 5 SREQ"]
pub mod udmach5ssel;
#[doc = "Output Selection for DMA Channel 5 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach5bsel](udmach5bsel) module"]
pub type UDMACH5BSEL = crate::Reg<u32, _UDMACH5BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH5BSEL;
#[doc = "`read()` method returns [udmach5bsel::R](udmach5bsel::R) reader structure"]
impl crate::Readable for UDMACH5BSEL {}
#[doc = "Output Selection for DMA Channel 5 REQ"]
pub mod udmach5bsel;
#[doc = "Output Selection for DMA Channel 6 SREQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach6ssel](udmach6ssel) module"]
pub type UDMACH6SSEL = crate::Reg<u32, _UDMACH6SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH6SSEL;
#[doc = "`read()` method returns [udmach6ssel::R](udmach6ssel::R) reader structure"]
impl crate::Readable for UDMACH6SSEL {}
#[doc = "Output Selection for DMA Channel 6 SREQ"]
pub mod udmach6ssel;
#[doc = "Output Selection for DMA Channel 6 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach6bsel](udmach6bsel) module"]
pub type UDMACH6BSEL = crate::Reg<u32, _UDMACH6BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH6BSEL;
#[doc = "`read()` method returns [udmach6bsel::R](udmach6bsel::R) reader structure"]
impl crate::Readable for UDMACH6BSEL {}
#[doc = "Output Selection for DMA Channel 6 REQ"]
pub mod udmach6bsel;
#[doc = "Output Selection for DMA Channel 7 SREQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach7ssel](udmach7ssel) module"]
pub type UDMACH7SSEL = crate::Reg<u32, _UDMACH7SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH7SSEL;
#[doc = "`read()` method returns [udmach7ssel::R](udmach7ssel::R) reader structure"]
impl crate::Readable for UDMACH7SSEL {}
#[doc = "Output Selection for DMA Channel 7 SREQ"]
pub mod udmach7ssel;
#[doc = "Output Selection for DMA Channel 7 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach7bsel](udmach7bsel) module"]
pub type UDMACH7BSEL = crate::Reg<u32, _UDMACH7BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH7BSEL;
#[doc = "`read()` method returns [udmach7bsel::R](udmach7bsel::R) reader structure"]
impl crate::Readable for UDMACH7BSEL {}
#[doc = "Output Selection for DMA Channel 7 REQ"]
pub mod udmach7bsel;
#[doc = "Output Selection for DMA Channel 8 SREQ Single request is ignored for this channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach8ssel](udmach8ssel) module"]
pub type UDMACH8SSEL = crate::Reg<u32, _UDMACH8SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH8SSEL;
#[doc = "`read()` method returns [udmach8ssel::R](udmach8ssel::R) reader structure"]
impl crate::Readable for UDMACH8SSEL {}
#[doc = "Output Selection for DMA Channel 8 SREQ Single request is ignored for this channel"]
pub mod udmach8ssel;
#[doc = "Output Selection for DMA Channel 8 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach8bsel](udmach8bsel) module"]
pub type UDMACH8BSEL = crate::Reg<u32, _UDMACH8BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH8BSEL;
#[doc = "`read()` method returns [udmach8bsel::R](udmach8bsel::R) reader structure"]
impl crate::Readable for UDMACH8BSEL {}
#[doc = "Output Selection for DMA Channel 8 REQ"]
pub mod udmach8bsel;
#[doc = "Output Selection for DMA Channel 9 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach9ssel](udmach9ssel) module"]
pub type UDMACH9SSEL = crate::Reg<u32, _UDMACH9SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH9SSEL;
#[doc = "`read()` method returns [udmach9ssel::R](udmach9ssel::R) reader structure"]
impl crate::Readable for UDMACH9SSEL {}
#[doc = "`write(|w| ..)` method takes [udmach9ssel::W](udmach9ssel::W) writer structure"]
impl crate::Writable for UDMACH9SSEL {}
#[doc = "Output Selection for DMA Channel 9 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
pub mod udmach9ssel;
#[doc = "Output Selection for DMA Channel 9 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach9bsel](udmach9bsel) module"]
pub type UDMACH9BSEL = crate::Reg<u32, _UDMACH9BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH9BSEL;
#[doc = "`read()` method returns [udmach9bsel::R](udmach9bsel::R) reader structure"]
impl crate::Readable for UDMACH9BSEL {}
#[doc = "`write(|w| ..)` method takes [udmach9bsel::W](udmach9bsel::W) writer structure"]
impl crate::Writable for UDMACH9BSEL {}
#[doc = "Output Selection for DMA Channel 9 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
pub mod udmach9bsel;
#[doc = "Output Selection for DMA Channel 10 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach10ssel](udmach10ssel) module"]
pub type UDMACH10SSEL = crate::Reg<u32, _UDMACH10SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH10SSEL;
#[doc = "`read()` method returns [udmach10ssel::R](udmach10ssel::R) reader structure"]
impl crate::Readable for UDMACH10SSEL {}
#[doc = "`write(|w| ..)` method takes [udmach10ssel::W](udmach10ssel::W) writer structure"]
impl crate::Writable for UDMACH10SSEL {}
#[doc = "Output Selection for DMA Channel 10 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
pub mod udmach10ssel;
#[doc = "Output Selection for DMA Channel 10 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach10bsel](udmach10bsel) module"]
pub type UDMACH10BSEL = crate::Reg<u32, _UDMACH10BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH10BSEL;
#[doc = "`read()` method returns [udmach10bsel::R](udmach10bsel::R) reader structure"]
impl crate::Readable for UDMACH10BSEL {}
#[doc = "`write(|w| ..)` method takes [udmach10bsel::W](udmach10bsel::W) writer structure"]
impl crate::Writable for UDMACH10BSEL {}
#[doc = "Output Selection for DMA Channel 10 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
pub mod udmach10bsel;
#[doc = "Output Selection for DMA Channel 11 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach11ssel](udmach11ssel) module"]
pub type UDMACH11SSEL = crate::Reg<u32, _UDMACH11SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH11SSEL;
#[doc = "`read()` method returns [udmach11ssel::R](udmach11ssel::R) reader structure"]
impl crate::Readable for UDMACH11SSEL {}
#[doc = "`write(|w| ..)` method takes [udmach11ssel::W](udmach11ssel::W) writer structure"]
impl crate::Writable for UDMACH11SSEL {}
#[doc = "Output Selection for DMA Channel 11 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
pub mod udmach11ssel;
#[doc = "Output Selection for DMA Channel 11 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach11bsel](udmach11bsel) module"]
pub type UDMACH11BSEL = crate::Reg<u32, _UDMACH11BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH11BSEL;
#[doc = "`read()` method returns [udmach11bsel::R](udmach11bsel::R) reader structure"]
impl crate::Readable for UDMACH11BSEL {}
#[doc = "`write(|w| ..)` method takes [udmach11bsel::W](udmach11bsel::W) writer structure"]
impl crate::Writable for UDMACH11BSEL {}
#[doc = "Output Selection for DMA Channel 11 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
pub mod udmach11bsel;
#[doc = "Output Selection for DMA Channel 12 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach12ssel](udmach12ssel) module"]
pub type UDMACH12SSEL = crate::Reg<u32, _UDMACH12SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH12SSEL;
#[doc = "`read()` method returns [udmach12ssel::R](udmach12ssel::R) reader structure"]
impl crate::Readable for UDMACH12SSEL {}
#[doc = "`write(|w| ..)` method takes [udmach12ssel::W](udmach12ssel::W) writer structure"]
impl crate::Writable for UDMACH12SSEL {}
#[doc = "Output Selection for DMA Channel 12 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
pub mod udmach12ssel;
#[doc = "Output Selection for DMA Channel 12 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach12bsel](udmach12bsel) module"]
pub type UDMACH12BSEL = crate::Reg<u32, _UDMACH12BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH12BSEL;
#[doc = "`read()` method returns [udmach12bsel::R](udmach12bsel::R) reader structure"]
impl crate::Readable for UDMACH12BSEL {}
#[doc = "`write(|w| ..)` method takes [udmach12bsel::W](udmach12bsel::W) writer structure"]
impl crate::Writable for UDMACH12BSEL {}
#[doc = "Output Selection for DMA Channel 12 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
pub mod udmach12bsel;
#[doc = "Output Selection for DMA Channel 13 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach13bsel](udmach13bsel) module"]
pub type UDMACH13BSEL = crate::Reg<u32, _UDMACH13BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH13BSEL;
#[doc = "`read()` method returns [udmach13bsel::R](udmach13bsel::R) reader structure"]
impl crate::Readable for UDMACH13BSEL {}
#[doc = "Output Selection for DMA Channel 13 REQ"]
pub mod udmach13bsel;
#[doc = "Output Selection for DMA Channel 14 REQ\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach14bsel](udmach14bsel) module"]
pub type UDMACH14BSEL = crate::Reg<u32, _UDMACH14BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH14BSEL;
#[doc = "`read()` method returns [udmach14bsel::R](udmach14bsel::R) reader structure"]
impl crate::Readable for UDMACH14BSEL {}
#[doc = "`write(|w| ..)` method takes [udmach14bsel::W](udmach14bsel::W) writer structure"]
impl crate::Writable for UDMACH14BSEL {}
#[doc = "Output Selection for DMA Channel 14 REQ"]
pub mod udmach14bsel;
#[doc = "Output Selection for DMA Channel 15 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach15bsel](udmach15bsel) module"]
pub type UDMACH15BSEL = crate::Reg<u32, _UDMACH15BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH15BSEL;
#[doc = "`read()` method returns [udmach15bsel::R](udmach15bsel::R) reader structure"]
impl crate::Readable for UDMACH15BSEL {}
#[doc = "Output Selection for DMA Channel 15 REQ"]
pub mod udmach15bsel;
#[doc = "Output Selection for DMA Channel 16 SREQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach16ssel](udmach16ssel) module"]
pub type UDMACH16SSEL = crate::Reg<u32, _UDMACH16SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH16SSEL;
#[doc = "`read()` method returns [udmach16ssel::R](udmach16ssel::R) reader structure"]
impl crate::Readable for UDMACH16SSEL {}
#[doc = "Output Selection for DMA Channel 16 SREQ"]
pub mod udmach16ssel;
#[doc = "Output Selection for DMA Channel 16 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach16bsel](udmach16bsel) module"]
pub type UDMACH16BSEL = crate::Reg<u32, _UDMACH16BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH16BSEL;
#[doc = "`read()` method returns [udmach16bsel::R](udmach16bsel::R) reader structure"]
impl crate::Readable for UDMACH16BSEL {}
#[doc = "Output Selection for DMA Channel 16 REQ"]
pub mod udmach16bsel;
#[doc = "Output Selection for DMA Channel 17 SREQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach17ssel](udmach17ssel) module"]
pub type UDMACH17SSEL = crate::Reg<u32, _UDMACH17SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH17SSEL;
#[doc = "`read()` method returns [udmach17ssel::R](udmach17ssel::R) reader structure"]
impl crate::Readable for UDMACH17SSEL {}
#[doc = "Output Selection for DMA Channel 17 SREQ"]
pub mod udmach17ssel;
#[doc = "Output Selection for DMA Channel 17 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach17bsel](udmach17bsel) module"]
pub type UDMACH17BSEL = crate::Reg<u32, _UDMACH17BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH17BSEL;
#[doc = "`read()` method returns [udmach17bsel::R](udmach17bsel::R) reader structure"]
impl crate::Readable for UDMACH17BSEL {}
#[doc = "Output Selection for DMA Channel 17 REQ"]
pub mod udmach17bsel;
#[doc = "Output Selection for DMA Channel 21 SREQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach21ssel](udmach21ssel) module"]
pub type UDMACH21SSEL = crate::Reg<u32, _UDMACH21SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH21SSEL;
#[doc = "`read()` method returns [udmach21ssel::R](udmach21ssel::R) reader structure"]
impl crate::Readable for UDMACH21SSEL {}
#[doc = "Output Selection for DMA Channel 21 SREQ"]
pub mod udmach21ssel;
#[doc = "Output Selection for DMA Channel 21 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach21bsel](udmach21bsel) module"]
pub type UDMACH21BSEL = crate::Reg<u32, _UDMACH21BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH21BSEL;
#[doc = "`read()` method returns [udmach21bsel::R](udmach21bsel::R) reader structure"]
impl crate::Readable for UDMACH21BSEL {}
#[doc = "Output Selection for DMA Channel 21 REQ"]
pub mod udmach21bsel;
#[doc = "Output Selection for DMA Channel 22 SREQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach22ssel](udmach22ssel) module"]
pub type UDMACH22SSEL = crate::Reg<u32, _UDMACH22SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH22SSEL;
#[doc = "`read()` method returns [udmach22ssel::R](udmach22ssel::R) reader structure"]
impl crate::Readable for UDMACH22SSEL {}
#[doc = "Output Selection for DMA Channel 22 SREQ"]
pub mod udmach22ssel;
#[doc = "Output Selection for DMA Channel 22 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach22bsel](udmach22bsel) module"]
pub type UDMACH22BSEL = crate::Reg<u32, _UDMACH22BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH22BSEL;
#[doc = "`read()` method returns [udmach22bsel::R](udmach22bsel::R) reader structure"]
impl crate::Readable for UDMACH22BSEL {}
#[doc = "Output Selection for DMA Channel 22 REQ"]
pub mod udmach22bsel;
#[doc = "Output Selection for DMA Channel 23 SREQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach23ssel](udmach23ssel) module"]
pub type UDMACH23SSEL = crate::Reg<u32, _UDMACH23SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH23SSEL;
#[doc = "`read()` method returns [udmach23ssel::R](udmach23ssel::R) reader structure"]
impl crate::Readable for UDMACH23SSEL {}
#[doc = "Output Selection for DMA Channel 23 SREQ"]
pub mod udmach23ssel;
#[doc = "Output Selection for DMA Channel 23 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach23bsel](udmach23bsel) module"]
pub type UDMACH23BSEL = crate::Reg<u32, _UDMACH23BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH23BSEL;
#[doc = "`read()` method returns [udmach23bsel::R](udmach23bsel::R) reader structure"]
impl crate::Readable for UDMACH23BSEL {}
#[doc = "Output Selection for DMA Channel 23 REQ"]
pub mod udmach23bsel;
#[doc = "Output Selection for DMA Channel 24 SREQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach24ssel](udmach24ssel) module"]
pub type UDMACH24SSEL = crate::Reg<u32, _UDMACH24SSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH24SSEL;
#[doc = "`read()` method returns [udmach24ssel::R](udmach24ssel::R) reader structure"]
impl crate::Readable for UDMACH24SSEL {}
#[doc = "Output Selection for DMA Channel 24 SREQ"]
pub mod udmach24ssel;
#[doc = "Output Selection for DMA Channel 24 REQ\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach24bsel](udmach24bsel) module"]
pub type UDMACH24BSEL = crate::Reg<u32, _UDMACH24BSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDMACH24BSEL;
#[doc = "`read()` method returns [udmach24bsel::R](udmach24bsel::R) reader structure"]
impl crate::Readable for UDMACH24BSEL {}
#[doc = "Output Selection for DMA Channel 24 REQ"]
pub mod udmach24bsel;
#[doc = "Output Selection for GPT3 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt3acaptsel](gpt3acaptsel) module"]
pub type GPT3ACAPTSEL = crate::Reg<u32, _GPT3ACAPTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT3ACAPTSEL;
#[doc = "`read()` method returns [gpt3acaptsel::R](gpt3acaptsel::R) reader structure"]
impl crate::Readable for GPT3ACAPTSEL {}
#[doc = "`write(|w| ..)` method takes [gpt3acaptsel::W](gpt3acaptsel::W) writer structure"]
impl crate::Writable for GPT3ACAPTSEL {}
#[doc = "Output Selection for GPT3 0"]
pub mod gpt3acaptsel;
#[doc = "Output Selection for GPT3 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt3bcaptsel](gpt3bcaptsel) module"]
pub type GPT3BCAPTSEL = crate::Reg<u32, _GPT3BCAPTSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPT3BCAPTSEL;
#[doc = "`read()` method returns [gpt3bcaptsel::R](gpt3bcaptsel::R) reader structure"]
impl crate::Readable for GPT3BCAPTSEL {}
#[doc = "`write(|w| ..)` method takes [gpt3bcaptsel::W](gpt3bcaptsel::W) writer structure"]
impl crate::Writable for GPT3BCAPTSEL {}
#[doc = "Output Selection for GPT3 1"]
pub mod gpt3bcaptsel;
#[doc = "Output Selection for AUX Subscriber 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxsel0](auxsel0) module"]
pub type AUXSEL0 = crate::Reg<u32, _AUXSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUXSEL0;
#[doc = "`read()` method returns [auxsel0::R](auxsel0::R) reader structure"]
impl crate::Readable for AUXSEL0 {}
#[doc = "`write(|w| ..)` method takes [auxsel0::W](auxsel0::W) writer structure"]
impl crate::Writable for AUXSEL0 {}
#[doc = "Output Selection for AUX Subscriber 0"]
pub mod auxsel0;
#[doc = "Output Selection for NMI Subscriber 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm3nmisel0](cm3nmisel0) module"]
pub type CM3NMISEL0 = crate::Reg<u32, _CM3NMISEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM3NMISEL0;
#[doc = "`read()` method returns [cm3nmisel0::R](cm3nmisel0::R) reader structure"]
impl crate::Readable for CM3NMISEL0 {}
#[doc = "Output Selection for NMI Subscriber 0"]
pub mod cm3nmisel0;
#[doc = "Output Selection for I2S Subscriber 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sstmpsel0](i2sstmpsel0) module"]
pub type I2SSTMPSEL0 = crate::Reg<u32, _I2SSTMPSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SSTMPSEL0;
#[doc = "`read()` method returns [i2sstmpsel0::R](i2sstmpsel0::R) reader structure"]
impl crate::Readable for I2SSTMPSEL0 {}
#[doc = "`write(|w| ..)` method takes [i2sstmpsel0::W](i2sstmpsel0::W) writer structure"]
impl crate::Writable for I2SSTMPSEL0 {}
#[doc = "Output Selection for I2S Subscriber 0"]
pub mod i2sstmpsel0;
#[doc = "Output Selection for FRZ Subscriber 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frzsel0](frzsel0) module"]
pub type FRZSEL0 = crate::Reg<u32, _FRZSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRZSEL0;
#[doc = "`read()` method returns [frzsel0::R](frzsel0::R) reader structure"]
impl crate::Readable for FRZSEL0 {}
#[doc = "`write(|w| ..)` method takes [frzsel0::W](frzsel0::W) writer structure"]
impl crate::Writable for FRZSEL0 {}
#[doc = "Output Selection for FRZ Subscriber 0"]
pub mod frzsel0;
#[doc = "Set or Clear Software Events\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swev](swev) module"]
pub type SWEV = crate::Reg<u32, _SWEV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWEV;
#[doc = "`read()` method returns [swev::R](swev::R) reader structure"]
impl crate::Readable for SWEV {}
#[doc = "`write(|w| ..)` method takes [swev::W](swev::W) writer structure"]
impl crate::Writable for SWEV {}
#[doc = "Set or Clear Software Events"]
pub mod swev;

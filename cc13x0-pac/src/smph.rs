#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU SEMAPHORE 0"]
    pub smph0: SMPH0,
    #[doc = "0x04 - MCU SEMAPHORE 1"]
    pub smph1: SMPH1,
    #[doc = "0x08 - MCU SEMAPHORE 2"]
    pub smph2: SMPH2,
    #[doc = "0x0c - MCU SEMAPHORE 3"]
    pub smph3: SMPH3,
    #[doc = "0x10 - MCU SEMAPHORE 4"]
    pub smph4: SMPH4,
    #[doc = "0x14 - MCU SEMAPHORE 5"]
    pub smph5: SMPH5,
    #[doc = "0x18 - MCU SEMAPHORE 6"]
    pub smph6: SMPH6,
    #[doc = "0x1c - MCU SEMAPHORE 7"]
    pub smph7: SMPH7,
    #[doc = "0x20 - MCU SEMAPHORE 8"]
    pub smph8: SMPH8,
    #[doc = "0x24 - MCU SEMAPHORE 9"]
    pub smph9: SMPH9,
    #[doc = "0x28 - MCU SEMAPHORE 10"]
    pub smph10: SMPH10,
    #[doc = "0x2c - MCU SEMAPHORE 11"]
    pub smph11: SMPH11,
    #[doc = "0x30 - MCU SEMAPHORE 12"]
    pub smph12: SMPH12,
    #[doc = "0x34 - MCU SEMAPHORE 13"]
    pub smph13: SMPH13,
    #[doc = "0x38 - MCU SEMAPHORE 14"]
    pub smph14: SMPH14,
    #[doc = "0x3c - MCU SEMAPHORE 15"]
    pub smph15: SMPH15,
    #[doc = "0x40 - MCU SEMAPHORE 16"]
    pub smph16: SMPH16,
    #[doc = "0x44 - MCU SEMAPHORE 17"]
    pub smph17: SMPH17,
    #[doc = "0x48 - MCU SEMAPHORE 18"]
    pub smph18: SMPH18,
    #[doc = "0x4c - MCU SEMAPHORE 19"]
    pub smph19: SMPH19,
    #[doc = "0x50 - MCU SEMAPHORE 20"]
    pub smph20: SMPH20,
    #[doc = "0x54 - MCU SEMAPHORE 21"]
    pub smph21: SMPH21,
    #[doc = "0x58 - MCU SEMAPHORE 22"]
    pub smph22: SMPH22,
    #[doc = "0x5c - MCU SEMAPHORE 23"]
    pub smph23: SMPH23,
    #[doc = "0x60 - MCU SEMAPHORE 24"]
    pub smph24: SMPH24,
    #[doc = "0x64 - MCU SEMAPHORE 25"]
    pub smph25: SMPH25,
    #[doc = "0x68 - MCU SEMAPHORE 26"]
    pub smph26: SMPH26,
    #[doc = "0x6c - MCU SEMAPHORE 27"]
    pub smph27: SMPH27,
    #[doc = "0x70 - MCU SEMAPHORE 28"]
    pub smph28: SMPH28,
    #[doc = "0x74 - MCU SEMAPHORE 29"]
    pub smph29: SMPH29,
    #[doc = "0x78 - MCU SEMAPHORE 30"]
    pub smph30: SMPH30,
    #[doc = "0x7c - MCU SEMAPHORE 31"]
    pub smph31: SMPH31,
    _reserved32: [u8; 1920usize],
    #[doc = "0x800 - MCU SEMAPHORE 0 ALIAS"]
    pub peek0: PEEK0,
    #[doc = "0x804 - MCU SEMAPHORE 1 ALIAS"]
    pub peek1: PEEK1,
    #[doc = "0x808 - MCU SEMAPHORE 2 ALIAS"]
    pub peek2: PEEK2,
    #[doc = "0x80c - MCU SEMAPHORE 3 ALIAS"]
    pub peek3: PEEK3,
    #[doc = "0x810 - MCU SEMAPHORE 4 ALIAS"]
    pub peek4: PEEK4,
    #[doc = "0x814 - MCU SEMAPHORE 5 ALIAS"]
    pub peek5: PEEK5,
    #[doc = "0x818 - MCU SEMAPHORE 6 ALIAS"]
    pub peek6: PEEK6,
    #[doc = "0x81c - MCU SEMAPHORE 7 ALIAS"]
    pub peek7: PEEK7,
    #[doc = "0x820 - MCU SEMAPHORE 8 ALIAS"]
    pub peek8: PEEK8,
    #[doc = "0x824 - MCU SEMAPHORE 9 ALIAS"]
    pub peek9: PEEK9,
    #[doc = "0x828 - MCU SEMAPHORE 10 ALIAS"]
    pub peek10: PEEK10,
    #[doc = "0x82c - MCU SEMAPHORE 11 ALIAS"]
    pub peek11: PEEK11,
    #[doc = "0x830 - MCU SEMAPHORE 12 ALIAS"]
    pub peek12: PEEK12,
    #[doc = "0x834 - MCU SEMAPHORE 13 ALIAS"]
    pub peek13: PEEK13,
    #[doc = "0x838 - MCU SEMAPHORE 14 ALIAS"]
    pub peek14: PEEK14,
    #[doc = "0x83c - MCU SEMAPHORE 15 ALIAS"]
    pub peek15: PEEK15,
    #[doc = "0x840 - MCU SEMAPHORE 16 ALIAS"]
    pub peek16: PEEK16,
    #[doc = "0x844 - MCU SEMAPHORE 17 ALIAS"]
    pub peek17: PEEK17,
    #[doc = "0x848 - MCU SEMAPHORE 18 ALIAS"]
    pub peek18: PEEK18,
    #[doc = "0x84c - MCU SEMAPHORE 19 ALIAS"]
    pub peek19: PEEK19,
    #[doc = "0x850 - MCU SEMAPHORE 20 ALIAS"]
    pub peek20: PEEK20,
    #[doc = "0x854 - MCU SEMAPHORE 21 ALIAS"]
    pub peek21: PEEK21,
    #[doc = "0x858 - MCU SEMAPHORE 22 ALIAS"]
    pub peek22: PEEK22,
    #[doc = "0x85c - MCU SEMAPHORE 23 ALIAS"]
    pub peek23: PEEK23,
    #[doc = "0x860 - MCU SEMAPHORE 24 ALIAS"]
    pub peek24: PEEK24,
    #[doc = "0x864 - MCU SEMAPHORE 25 ALIAS"]
    pub peek25: PEEK25,
    #[doc = "0x868 - MCU SEMAPHORE 26 ALIAS"]
    pub peek26: PEEK26,
    #[doc = "0x86c - MCU SEMAPHORE 27 ALIAS"]
    pub peek27: PEEK27,
    #[doc = "0x870 - MCU SEMAPHORE 28 ALIAS"]
    pub peek28: PEEK28,
    #[doc = "0x874 - MCU SEMAPHORE 29 ALIAS"]
    pub peek29: PEEK29,
    #[doc = "0x878 - MCU SEMAPHORE 30 ALIAS"]
    pub peek30: PEEK30,
    #[doc = "0x87c - MCU SEMAPHORE 31 ALIAS"]
    pub peek31: PEEK31,
}
#[doc = "MCU SEMAPHORE 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph0](smph0) module"]
pub type SMPH0 = crate::Reg<u32, _SMPH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH0;
#[doc = "`read()` method returns [smph0::R](smph0::R) reader structure"]
impl crate::Readable for SMPH0 {}
#[doc = "`write(|w| ..)` method takes [smph0::W](smph0::W) writer structure"]
impl crate::Writable for SMPH0 {}
#[doc = "MCU SEMAPHORE 0"]
pub mod smph0;
#[doc = "MCU SEMAPHORE 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph1](smph1) module"]
pub type SMPH1 = crate::Reg<u32, _SMPH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH1;
#[doc = "`read()` method returns [smph1::R](smph1::R) reader structure"]
impl crate::Readable for SMPH1 {}
#[doc = "`write(|w| ..)` method takes [smph1::W](smph1::W) writer structure"]
impl crate::Writable for SMPH1 {}
#[doc = "MCU SEMAPHORE 1"]
pub mod smph1;
#[doc = "MCU SEMAPHORE 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph2](smph2) module"]
pub type SMPH2 = crate::Reg<u32, _SMPH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH2;
#[doc = "`read()` method returns [smph2::R](smph2::R) reader structure"]
impl crate::Readable for SMPH2 {}
#[doc = "`write(|w| ..)` method takes [smph2::W](smph2::W) writer structure"]
impl crate::Writable for SMPH2 {}
#[doc = "MCU SEMAPHORE 2"]
pub mod smph2;
#[doc = "MCU SEMAPHORE 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph3](smph3) module"]
pub type SMPH3 = crate::Reg<u32, _SMPH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH3;
#[doc = "`read()` method returns [smph3::R](smph3::R) reader structure"]
impl crate::Readable for SMPH3 {}
#[doc = "`write(|w| ..)` method takes [smph3::W](smph3::W) writer structure"]
impl crate::Writable for SMPH3 {}
#[doc = "MCU SEMAPHORE 3"]
pub mod smph3;
#[doc = "MCU SEMAPHORE 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph4](smph4) module"]
pub type SMPH4 = crate::Reg<u32, _SMPH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH4;
#[doc = "`read()` method returns [smph4::R](smph4::R) reader structure"]
impl crate::Readable for SMPH4 {}
#[doc = "`write(|w| ..)` method takes [smph4::W](smph4::W) writer structure"]
impl crate::Writable for SMPH4 {}
#[doc = "MCU SEMAPHORE 4"]
pub mod smph4;
#[doc = "MCU SEMAPHORE 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph5](smph5) module"]
pub type SMPH5 = crate::Reg<u32, _SMPH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH5;
#[doc = "`read()` method returns [smph5::R](smph5::R) reader structure"]
impl crate::Readable for SMPH5 {}
#[doc = "`write(|w| ..)` method takes [smph5::W](smph5::W) writer structure"]
impl crate::Writable for SMPH5 {}
#[doc = "MCU SEMAPHORE 5"]
pub mod smph5;
#[doc = "MCU SEMAPHORE 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph6](smph6) module"]
pub type SMPH6 = crate::Reg<u32, _SMPH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH6;
#[doc = "`read()` method returns [smph6::R](smph6::R) reader structure"]
impl crate::Readable for SMPH6 {}
#[doc = "`write(|w| ..)` method takes [smph6::W](smph6::W) writer structure"]
impl crate::Writable for SMPH6 {}
#[doc = "MCU SEMAPHORE 6"]
pub mod smph6;
#[doc = "MCU SEMAPHORE 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph7](smph7) module"]
pub type SMPH7 = crate::Reg<u32, _SMPH7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH7;
#[doc = "`read()` method returns [smph7::R](smph7::R) reader structure"]
impl crate::Readable for SMPH7 {}
#[doc = "`write(|w| ..)` method takes [smph7::W](smph7::W) writer structure"]
impl crate::Writable for SMPH7 {}
#[doc = "MCU SEMAPHORE 7"]
pub mod smph7;
#[doc = "MCU SEMAPHORE 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph8](smph8) module"]
pub type SMPH8 = crate::Reg<u32, _SMPH8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH8;
#[doc = "`read()` method returns [smph8::R](smph8::R) reader structure"]
impl crate::Readable for SMPH8 {}
#[doc = "`write(|w| ..)` method takes [smph8::W](smph8::W) writer structure"]
impl crate::Writable for SMPH8 {}
#[doc = "MCU SEMAPHORE 8"]
pub mod smph8;
#[doc = "MCU SEMAPHORE 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph9](smph9) module"]
pub type SMPH9 = crate::Reg<u32, _SMPH9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH9;
#[doc = "`read()` method returns [smph9::R](smph9::R) reader structure"]
impl crate::Readable for SMPH9 {}
#[doc = "`write(|w| ..)` method takes [smph9::W](smph9::W) writer structure"]
impl crate::Writable for SMPH9 {}
#[doc = "MCU SEMAPHORE 9"]
pub mod smph9;
#[doc = "MCU SEMAPHORE 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph10](smph10) module"]
pub type SMPH10 = crate::Reg<u32, _SMPH10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH10;
#[doc = "`read()` method returns [smph10::R](smph10::R) reader structure"]
impl crate::Readable for SMPH10 {}
#[doc = "`write(|w| ..)` method takes [smph10::W](smph10::W) writer structure"]
impl crate::Writable for SMPH10 {}
#[doc = "MCU SEMAPHORE 10"]
pub mod smph10;
#[doc = "MCU SEMAPHORE 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph11](smph11) module"]
pub type SMPH11 = crate::Reg<u32, _SMPH11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH11;
#[doc = "`read()` method returns [smph11::R](smph11::R) reader structure"]
impl crate::Readable for SMPH11 {}
#[doc = "`write(|w| ..)` method takes [smph11::W](smph11::W) writer structure"]
impl crate::Writable for SMPH11 {}
#[doc = "MCU SEMAPHORE 11"]
pub mod smph11;
#[doc = "MCU SEMAPHORE 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph12](smph12) module"]
pub type SMPH12 = crate::Reg<u32, _SMPH12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH12;
#[doc = "`read()` method returns [smph12::R](smph12::R) reader structure"]
impl crate::Readable for SMPH12 {}
#[doc = "`write(|w| ..)` method takes [smph12::W](smph12::W) writer structure"]
impl crate::Writable for SMPH12 {}
#[doc = "MCU SEMAPHORE 12"]
pub mod smph12;
#[doc = "MCU SEMAPHORE 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph13](smph13) module"]
pub type SMPH13 = crate::Reg<u32, _SMPH13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH13;
#[doc = "`read()` method returns [smph13::R](smph13::R) reader structure"]
impl crate::Readable for SMPH13 {}
#[doc = "`write(|w| ..)` method takes [smph13::W](smph13::W) writer structure"]
impl crate::Writable for SMPH13 {}
#[doc = "MCU SEMAPHORE 13"]
pub mod smph13;
#[doc = "MCU SEMAPHORE 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph14](smph14) module"]
pub type SMPH14 = crate::Reg<u32, _SMPH14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH14;
#[doc = "`read()` method returns [smph14::R](smph14::R) reader structure"]
impl crate::Readable for SMPH14 {}
#[doc = "`write(|w| ..)` method takes [smph14::W](smph14::W) writer structure"]
impl crate::Writable for SMPH14 {}
#[doc = "MCU SEMAPHORE 14"]
pub mod smph14;
#[doc = "MCU SEMAPHORE 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph15](smph15) module"]
pub type SMPH15 = crate::Reg<u32, _SMPH15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH15;
#[doc = "`read()` method returns [smph15::R](smph15::R) reader structure"]
impl crate::Readable for SMPH15 {}
#[doc = "`write(|w| ..)` method takes [smph15::W](smph15::W) writer structure"]
impl crate::Writable for SMPH15 {}
#[doc = "MCU SEMAPHORE 15"]
pub mod smph15;
#[doc = "MCU SEMAPHORE 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph16](smph16) module"]
pub type SMPH16 = crate::Reg<u32, _SMPH16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH16;
#[doc = "`read()` method returns [smph16::R](smph16::R) reader structure"]
impl crate::Readable for SMPH16 {}
#[doc = "`write(|w| ..)` method takes [smph16::W](smph16::W) writer structure"]
impl crate::Writable for SMPH16 {}
#[doc = "MCU SEMAPHORE 16"]
pub mod smph16;
#[doc = "MCU SEMAPHORE 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph17](smph17) module"]
pub type SMPH17 = crate::Reg<u32, _SMPH17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH17;
#[doc = "`read()` method returns [smph17::R](smph17::R) reader structure"]
impl crate::Readable for SMPH17 {}
#[doc = "`write(|w| ..)` method takes [smph17::W](smph17::W) writer structure"]
impl crate::Writable for SMPH17 {}
#[doc = "MCU SEMAPHORE 17"]
pub mod smph17;
#[doc = "MCU SEMAPHORE 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph18](smph18) module"]
pub type SMPH18 = crate::Reg<u32, _SMPH18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH18;
#[doc = "`read()` method returns [smph18::R](smph18::R) reader structure"]
impl crate::Readable for SMPH18 {}
#[doc = "`write(|w| ..)` method takes [smph18::W](smph18::W) writer structure"]
impl crate::Writable for SMPH18 {}
#[doc = "MCU SEMAPHORE 18"]
pub mod smph18;
#[doc = "MCU SEMAPHORE 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph19](smph19) module"]
pub type SMPH19 = crate::Reg<u32, _SMPH19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH19;
#[doc = "`read()` method returns [smph19::R](smph19::R) reader structure"]
impl crate::Readable for SMPH19 {}
#[doc = "`write(|w| ..)` method takes [smph19::W](smph19::W) writer structure"]
impl crate::Writable for SMPH19 {}
#[doc = "MCU SEMAPHORE 19"]
pub mod smph19;
#[doc = "MCU SEMAPHORE 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph20](smph20) module"]
pub type SMPH20 = crate::Reg<u32, _SMPH20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH20;
#[doc = "`read()` method returns [smph20::R](smph20::R) reader structure"]
impl crate::Readable for SMPH20 {}
#[doc = "`write(|w| ..)` method takes [smph20::W](smph20::W) writer structure"]
impl crate::Writable for SMPH20 {}
#[doc = "MCU SEMAPHORE 20"]
pub mod smph20;
#[doc = "MCU SEMAPHORE 21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph21](smph21) module"]
pub type SMPH21 = crate::Reg<u32, _SMPH21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH21;
#[doc = "`read()` method returns [smph21::R](smph21::R) reader structure"]
impl crate::Readable for SMPH21 {}
#[doc = "`write(|w| ..)` method takes [smph21::W](smph21::W) writer structure"]
impl crate::Writable for SMPH21 {}
#[doc = "MCU SEMAPHORE 21"]
pub mod smph21;
#[doc = "MCU SEMAPHORE 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph22](smph22) module"]
pub type SMPH22 = crate::Reg<u32, _SMPH22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH22;
#[doc = "`read()` method returns [smph22::R](smph22::R) reader structure"]
impl crate::Readable for SMPH22 {}
#[doc = "`write(|w| ..)` method takes [smph22::W](smph22::W) writer structure"]
impl crate::Writable for SMPH22 {}
#[doc = "MCU SEMAPHORE 22"]
pub mod smph22;
#[doc = "MCU SEMAPHORE 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph23](smph23) module"]
pub type SMPH23 = crate::Reg<u32, _SMPH23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH23;
#[doc = "`read()` method returns [smph23::R](smph23::R) reader structure"]
impl crate::Readable for SMPH23 {}
#[doc = "`write(|w| ..)` method takes [smph23::W](smph23::W) writer structure"]
impl crate::Writable for SMPH23 {}
#[doc = "MCU SEMAPHORE 23"]
pub mod smph23;
#[doc = "MCU SEMAPHORE 24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph24](smph24) module"]
pub type SMPH24 = crate::Reg<u32, _SMPH24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH24;
#[doc = "`read()` method returns [smph24::R](smph24::R) reader structure"]
impl crate::Readable for SMPH24 {}
#[doc = "`write(|w| ..)` method takes [smph24::W](smph24::W) writer structure"]
impl crate::Writable for SMPH24 {}
#[doc = "MCU SEMAPHORE 24"]
pub mod smph24;
#[doc = "MCU SEMAPHORE 25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph25](smph25) module"]
pub type SMPH25 = crate::Reg<u32, _SMPH25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH25;
#[doc = "`read()` method returns [smph25::R](smph25::R) reader structure"]
impl crate::Readable for SMPH25 {}
#[doc = "`write(|w| ..)` method takes [smph25::W](smph25::W) writer structure"]
impl crate::Writable for SMPH25 {}
#[doc = "MCU SEMAPHORE 25"]
pub mod smph25;
#[doc = "MCU SEMAPHORE 26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph26](smph26) module"]
pub type SMPH26 = crate::Reg<u32, _SMPH26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH26;
#[doc = "`read()` method returns [smph26::R](smph26::R) reader structure"]
impl crate::Readable for SMPH26 {}
#[doc = "`write(|w| ..)` method takes [smph26::W](smph26::W) writer structure"]
impl crate::Writable for SMPH26 {}
#[doc = "MCU SEMAPHORE 26"]
pub mod smph26;
#[doc = "MCU SEMAPHORE 27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph27](smph27) module"]
pub type SMPH27 = crate::Reg<u32, _SMPH27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH27;
#[doc = "`read()` method returns [smph27::R](smph27::R) reader structure"]
impl crate::Readable for SMPH27 {}
#[doc = "`write(|w| ..)` method takes [smph27::W](smph27::W) writer structure"]
impl crate::Writable for SMPH27 {}
#[doc = "MCU SEMAPHORE 27"]
pub mod smph27;
#[doc = "MCU SEMAPHORE 28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph28](smph28) module"]
pub type SMPH28 = crate::Reg<u32, _SMPH28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH28;
#[doc = "`read()` method returns [smph28::R](smph28::R) reader structure"]
impl crate::Readable for SMPH28 {}
#[doc = "`write(|w| ..)` method takes [smph28::W](smph28::W) writer structure"]
impl crate::Writable for SMPH28 {}
#[doc = "MCU SEMAPHORE 28"]
pub mod smph28;
#[doc = "MCU SEMAPHORE 29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph29](smph29) module"]
pub type SMPH29 = crate::Reg<u32, _SMPH29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH29;
#[doc = "`read()` method returns [smph29::R](smph29::R) reader structure"]
impl crate::Readable for SMPH29 {}
#[doc = "`write(|w| ..)` method takes [smph29::W](smph29::W) writer structure"]
impl crate::Writable for SMPH29 {}
#[doc = "MCU SEMAPHORE 29"]
pub mod smph29;
#[doc = "MCU SEMAPHORE 30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph30](smph30) module"]
pub type SMPH30 = crate::Reg<u32, _SMPH30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH30;
#[doc = "`read()` method returns [smph30::R](smph30::R) reader structure"]
impl crate::Readable for SMPH30 {}
#[doc = "`write(|w| ..)` method takes [smph30::W](smph30::W) writer structure"]
impl crate::Writable for SMPH30 {}
#[doc = "MCU SEMAPHORE 30"]
pub mod smph30;
#[doc = "MCU SEMAPHORE 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph31](smph31) module"]
pub type SMPH31 = crate::Reg<u32, _SMPH31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH31;
#[doc = "`read()` method returns [smph31::R](smph31::R) reader structure"]
impl crate::Readable for SMPH31 {}
#[doc = "`write(|w| ..)` method takes [smph31::W](smph31::W) writer structure"]
impl crate::Writable for SMPH31 {}
#[doc = "MCU SEMAPHORE 31"]
pub mod smph31;
#[doc = "MCU SEMAPHORE 0 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek0](peek0) module"]
pub type PEEK0 = crate::Reg<u32, _PEEK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK0;
#[doc = "`read()` method returns [peek0::R](peek0::R) reader structure"]
impl crate::Readable for PEEK0 {}
#[doc = "MCU SEMAPHORE 0 ALIAS"]
pub mod peek0;
#[doc = "MCU SEMAPHORE 1 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek1](peek1) module"]
pub type PEEK1 = crate::Reg<u32, _PEEK1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK1;
#[doc = "`read()` method returns [peek1::R](peek1::R) reader structure"]
impl crate::Readable for PEEK1 {}
#[doc = "MCU SEMAPHORE 1 ALIAS"]
pub mod peek1;
#[doc = "MCU SEMAPHORE 2 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek2](peek2) module"]
pub type PEEK2 = crate::Reg<u32, _PEEK2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK2;
#[doc = "`read()` method returns [peek2::R](peek2::R) reader structure"]
impl crate::Readable for PEEK2 {}
#[doc = "MCU SEMAPHORE 2 ALIAS"]
pub mod peek2;
#[doc = "MCU SEMAPHORE 3 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek3](peek3) module"]
pub type PEEK3 = crate::Reg<u32, _PEEK3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK3;
#[doc = "`read()` method returns [peek3::R](peek3::R) reader structure"]
impl crate::Readable for PEEK3 {}
#[doc = "MCU SEMAPHORE 3 ALIAS"]
pub mod peek3;
#[doc = "MCU SEMAPHORE 4 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek4](peek4) module"]
pub type PEEK4 = crate::Reg<u32, _PEEK4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK4;
#[doc = "`read()` method returns [peek4::R](peek4::R) reader structure"]
impl crate::Readable for PEEK4 {}
#[doc = "MCU SEMAPHORE 4 ALIAS"]
pub mod peek4;
#[doc = "MCU SEMAPHORE 5 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek5](peek5) module"]
pub type PEEK5 = crate::Reg<u32, _PEEK5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK5;
#[doc = "`read()` method returns [peek5::R](peek5::R) reader structure"]
impl crate::Readable for PEEK5 {}
#[doc = "MCU SEMAPHORE 5 ALIAS"]
pub mod peek5;
#[doc = "MCU SEMAPHORE 6 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek6](peek6) module"]
pub type PEEK6 = crate::Reg<u32, _PEEK6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK6;
#[doc = "`read()` method returns [peek6::R](peek6::R) reader structure"]
impl crate::Readable for PEEK6 {}
#[doc = "MCU SEMAPHORE 6 ALIAS"]
pub mod peek6;
#[doc = "MCU SEMAPHORE 7 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek7](peek7) module"]
pub type PEEK7 = crate::Reg<u32, _PEEK7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK7;
#[doc = "`read()` method returns [peek7::R](peek7::R) reader structure"]
impl crate::Readable for PEEK7 {}
#[doc = "MCU SEMAPHORE 7 ALIAS"]
pub mod peek7;
#[doc = "MCU SEMAPHORE 8 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek8](peek8) module"]
pub type PEEK8 = crate::Reg<u32, _PEEK8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK8;
#[doc = "`read()` method returns [peek8::R](peek8::R) reader structure"]
impl crate::Readable for PEEK8 {}
#[doc = "MCU SEMAPHORE 8 ALIAS"]
pub mod peek8;
#[doc = "MCU SEMAPHORE 9 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek9](peek9) module"]
pub type PEEK9 = crate::Reg<u32, _PEEK9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK9;
#[doc = "`read()` method returns [peek9::R](peek9::R) reader structure"]
impl crate::Readable for PEEK9 {}
#[doc = "MCU SEMAPHORE 9 ALIAS"]
pub mod peek9;
#[doc = "MCU SEMAPHORE 10 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek10](peek10) module"]
pub type PEEK10 = crate::Reg<u32, _PEEK10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK10;
#[doc = "`read()` method returns [peek10::R](peek10::R) reader structure"]
impl crate::Readable for PEEK10 {}
#[doc = "MCU SEMAPHORE 10 ALIAS"]
pub mod peek10;
#[doc = "MCU SEMAPHORE 11 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek11](peek11) module"]
pub type PEEK11 = crate::Reg<u32, _PEEK11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK11;
#[doc = "`read()` method returns [peek11::R](peek11::R) reader structure"]
impl crate::Readable for PEEK11 {}
#[doc = "MCU SEMAPHORE 11 ALIAS"]
pub mod peek11;
#[doc = "MCU SEMAPHORE 12 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek12](peek12) module"]
pub type PEEK12 = crate::Reg<u32, _PEEK12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK12;
#[doc = "`read()` method returns [peek12::R](peek12::R) reader structure"]
impl crate::Readable for PEEK12 {}
#[doc = "MCU SEMAPHORE 12 ALIAS"]
pub mod peek12;
#[doc = "MCU SEMAPHORE 13 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek13](peek13) module"]
pub type PEEK13 = crate::Reg<u32, _PEEK13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK13;
#[doc = "`read()` method returns [peek13::R](peek13::R) reader structure"]
impl crate::Readable for PEEK13 {}
#[doc = "MCU SEMAPHORE 13 ALIAS"]
pub mod peek13;
#[doc = "MCU SEMAPHORE 14 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek14](peek14) module"]
pub type PEEK14 = crate::Reg<u32, _PEEK14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK14;
#[doc = "`read()` method returns [peek14::R](peek14::R) reader structure"]
impl crate::Readable for PEEK14 {}
#[doc = "MCU SEMAPHORE 14 ALIAS"]
pub mod peek14;
#[doc = "MCU SEMAPHORE 15 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek15](peek15) module"]
pub type PEEK15 = crate::Reg<u32, _PEEK15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK15;
#[doc = "`read()` method returns [peek15::R](peek15::R) reader structure"]
impl crate::Readable for PEEK15 {}
#[doc = "MCU SEMAPHORE 15 ALIAS"]
pub mod peek15;
#[doc = "MCU SEMAPHORE 16 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek16](peek16) module"]
pub type PEEK16 = crate::Reg<u32, _PEEK16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK16;
#[doc = "`read()` method returns [peek16::R](peek16::R) reader structure"]
impl crate::Readable for PEEK16 {}
#[doc = "MCU SEMAPHORE 16 ALIAS"]
pub mod peek16;
#[doc = "MCU SEMAPHORE 17 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek17](peek17) module"]
pub type PEEK17 = crate::Reg<u32, _PEEK17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK17;
#[doc = "`read()` method returns [peek17::R](peek17::R) reader structure"]
impl crate::Readable for PEEK17 {}
#[doc = "MCU SEMAPHORE 17 ALIAS"]
pub mod peek17;
#[doc = "MCU SEMAPHORE 18 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek18](peek18) module"]
pub type PEEK18 = crate::Reg<u32, _PEEK18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK18;
#[doc = "`read()` method returns [peek18::R](peek18::R) reader structure"]
impl crate::Readable for PEEK18 {}
#[doc = "MCU SEMAPHORE 18 ALIAS"]
pub mod peek18;
#[doc = "MCU SEMAPHORE 19 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek19](peek19) module"]
pub type PEEK19 = crate::Reg<u32, _PEEK19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK19;
#[doc = "`read()` method returns [peek19::R](peek19::R) reader structure"]
impl crate::Readable for PEEK19 {}
#[doc = "MCU SEMAPHORE 19 ALIAS"]
pub mod peek19;
#[doc = "MCU SEMAPHORE 20 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek20](peek20) module"]
pub type PEEK20 = crate::Reg<u32, _PEEK20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK20;
#[doc = "`read()` method returns [peek20::R](peek20::R) reader structure"]
impl crate::Readable for PEEK20 {}
#[doc = "MCU SEMAPHORE 20 ALIAS"]
pub mod peek20;
#[doc = "MCU SEMAPHORE 21 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek21](peek21) module"]
pub type PEEK21 = crate::Reg<u32, _PEEK21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK21;
#[doc = "`read()` method returns [peek21::R](peek21::R) reader structure"]
impl crate::Readable for PEEK21 {}
#[doc = "MCU SEMAPHORE 21 ALIAS"]
pub mod peek21;
#[doc = "MCU SEMAPHORE 22 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek22](peek22) module"]
pub type PEEK22 = crate::Reg<u32, _PEEK22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK22;
#[doc = "`read()` method returns [peek22::R](peek22::R) reader structure"]
impl crate::Readable for PEEK22 {}
#[doc = "MCU SEMAPHORE 22 ALIAS"]
pub mod peek22;
#[doc = "MCU SEMAPHORE 23 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek23](peek23) module"]
pub type PEEK23 = crate::Reg<u32, _PEEK23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK23;
#[doc = "`read()` method returns [peek23::R](peek23::R) reader structure"]
impl crate::Readable for PEEK23 {}
#[doc = "MCU SEMAPHORE 23 ALIAS"]
pub mod peek23;
#[doc = "MCU SEMAPHORE 24 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek24](peek24) module"]
pub type PEEK24 = crate::Reg<u32, _PEEK24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK24;
#[doc = "`read()` method returns [peek24::R](peek24::R) reader structure"]
impl crate::Readable for PEEK24 {}
#[doc = "MCU SEMAPHORE 24 ALIAS"]
pub mod peek24;
#[doc = "MCU SEMAPHORE 25 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek25](peek25) module"]
pub type PEEK25 = crate::Reg<u32, _PEEK25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK25;
#[doc = "`read()` method returns [peek25::R](peek25::R) reader structure"]
impl crate::Readable for PEEK25 {}
#[doc = "MCU SEMAPHORE 25 ALIAS"]
pub mod peek25;
#[doc = "MCU SEMAPHORE 26 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek26](peek26) module"]
pub type PEEK26 = crate::Reg<u32, _PEEK26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK26;
#[doc = "`read()` method returns [peek26::R](peek26::R) reader structure"]
impl crate::Readable for PEEK26 {}
#[doc = "MCU SEMAPHORE 26 ALIAS"]
pub mod peek26;
#[doc = "MCU SEMAPHORE 27 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek27](peek27) module"]
pub type PEEK27 = crate::Reg<u32, _PEEK27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK27;
#[doc = "`read()` method returns [peek27::R](peek27::R) reader structure"]
impl crate::Readable for PEEK27 {}
#[doc = "MCU SEMAPHORE 27 ALIAS"]
pub mod peek27;
#[doc = "MCU SEMAPHORE 28 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek28](peek28) module"]
pub type PEEK28 = crate::Reg<u32, _PEEK28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK28;
#[doc = "`read()` method returns [peek28::R](peek28::R) reader structure"]
impl crate::Readable for PEEK28 {}
#[doc = "MCU SEMAPHORE 28 ALIAS"]
pub mod peek28;
#[doc = "MCU SEMAPHORE 29 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek29](peek29) module"]
pub type PEEK29 = crate::Reg<u32, _PEEK29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK29;
#[doc = "`read()` method returns [peek29::R](peek29::R) reader structure"]
impl crate::Readable for PEEK29 {}
#[doc = "MCU SEMAPHORE 29 ALIAS"]
pub mod peek29;
#[doc = "MCU SEMAPHORE 30 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek30](peek30) module"]
pub type PEEK30 = crate::Reg<u32, _PEEK30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK30;
#[doc = "`read()` method returns [peek30::R](peek30::R) reader structure"]
impl crate::Readable for PEEK30 {}
#[doc = "MCU SEMAPHORE 30 ALIAS"]
pub mod peek30;
#[doc = "MCU SEMAPHORE 31 ALIAS\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek31](peek31) module"]
pub type PEEK31 = crate::Reg<u32, _PEEK31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEEK31;
#[doc = "`read()` method returns [peek31::R](peek31::R) reader structure"]
impl crate::Readable for PEEK31 {}
#[doc = "MCU SEMAPHORE 31 ALIAS"]
pub mod peek31;

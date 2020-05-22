#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration of DIO0"]
    pub iocfg0: IOCFG0,
    #[doc = "0x04 - Configuration of DIO1"]
    pub iocfg1: IOCFG1,
    #[doc = "0x08 - Configuration of DIO2"]
    pub iocfg2: IOCFG2,
    #[doc = "0x0c - Configuration of DIO3"]
    pub iocfg3: IOCFG3,
    #[doc = "0x10 - Configuration of DIO4"]
    pub iocfg4: IOCFG4,
    #[doc = "0x14 - Configuration of DIO5"]
    pub iocfg5: IOCFG5,
    #[doc = "0x18 - Configuration of DIO6"]
    pub iocfg6: IOCFG6,
    #[doc = "0x1c - Configuration of DIO7"]
    pub iocfg7: IOCFG7,
    #[doc = "0x20 - Configuration of DIO8"]
    pub iocfg8: IOCFG8,
    #[doc = "0x24 - Configuration of DIO9"]
    pub iocfg9: IOCFG9,
    #[doc = "0x28 - Configuration of DIO10"]
    pub iocfg10: IOCFG10,
    #[doc = "0x2c - Configuration of DIO11"]
    pub iocfg11: IOCFG11,
    #[doc = "0x30 - Configuration of DIO12"]
    pub iocfg12: IOCFG12,
    #[doc = "0x34 - Configuration of DIO13"]
    pub iocfg13: IOCFG13,
    #[doc = "0x38 - Configuration of DIO14"]
    pub iocfg14: IOCFG14,
    #[doc = "0x3c - Configuration of DIO15"]
    pub iocfg15: IOCFG15,
    #[doc = "0x40 - Configuration of DIO16"]
    pub iocfg16: IOCFG16,
    #[doc = "0x44 - Configuration of DIO17"]
    pub iocfg17: IOCFG17,
    #[doc = "0x48 - Configuration of DIO18"]
    pub iocfg18: IOCFG18,
    #[doc = "0x4c - Configuration of DIO19"]
    pub iocfg19: IOCFG19,
    #[doc = "0x50 - Configuration of DIO20"]
    pub iocfg20: IOCFG20,
    #[doc = "0x54 - Configuration of DIO21"]
    pub iocfg21: IOCFG21,
    #[doc = "0x58 - Configuration of DIO22"]
    pub iocfg22: IOCFG22,
    #[doc = "0x5c - Configuration of DIO23"]
    pub iocfg23: IOCFG23,
    #[doc = "0x60 - Configuration of DIO24"]
    pub iocfg24: IOCFG24,
    #[doc = "0x64 - Configuration of DIO25"]
    pub iocfg25: IOCFG25,
    #[doc = "0x68 - Configuration of DIO26"]
    pub iocfg26: IOCFG26,
    #[doc = "0x6c - Configuration of DIO27"]
    pub iocfg27: IOCFG27,
    #[doc = "0x70 - Configuration of DIO28"]
    pub iocfg28: IOCFG28,
    #[doc = "0x74 - Configuration of DIO29"]
    pub iocfg29: IOCFG29,
    #[doc = "0x78 - Configuration of DIO30"]
    pub iocfg30: IOCFG30,
    #[doc = "0x7c - Configuration of DIO31"]
    pub iocfg31: IOCFG31,
}
#[doc = "Configuration of DIO0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg0](iocfg0) module"]
pub type IOCFG0 = crate::Reg<u32, _IOCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG0;
#[doc = "`read()` method returns [iocfg0::R](iocfg0::R) reader structure"]
impl crate::Readable for IOCFG0 {}
#[doc = "`write(|w| ..)` method takes [iocfg0::W](iocfg0::W) writer structure"]
impl crate::Writable for IOCFG0 {}
#[doc = "Configuration of DIO0"]
pub mod iocfg0;
#[doc = "Configuration of DIO1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg1](iocfg1) module"]
pub type IOCFG1 = crate::Reg<u32, _IOCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG1;
#[doc = "`read()` method returns [iocfg1::R](iocfg1::R) reader structure"]
impl crate::Readable for IOCFG1 {}
#[doc = "`write(|w| ..)` method takes [iocfg1::W](iocfg1::W) writer structure"]
impl crate::Writable for IOCFG1 {}
#[doc = "Configuration of DIO1"]
pub mod iocfg1;
#[doc = "Configuration of DIO2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg2](iocfg2) module"]
pub type IOCFG2 = crate::Reg<u32, _IOCFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG2;
#[doc = "`read()` method returns [iocfg2::R](iocfg2::R) reader structure"]
impl crate::Readable for IOCFG2 {}
#[doc = "`write(|w| ..)` method takes [iocfg2::W](iocfg2::W) writer structure"]
impl crate::Writable for IOCFG2 {}
#[doc = "Configuration of DIO2"]
pub mod iocfg2;
#[doc = "Configuration of DIO3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg3](iocfg3) module"]
pub type IOCFG3 = crate::Reg<u32, _IOCFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG3;
#[doc = "`read()` method returns [iocfg3::R](iocfg3::R) reader structure"]
impl crate::Readable for IOCFG3 {}
#[doc = "`write(|w| ..)` method takes [iocfg3::W](iocfg3::W) writer structure"]
impl crate::Writable for IOCFG3 {}
#[doc = "Configuration of DIO3"]
pub mod iocfg3;
#[doc = "Configuration of DIO4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg4](iocfg4) module"]
pub type IOCFG4 = crate::Reg<u32, _IOCFG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG4;
#[doc = "`read()` method returns [iocfg4::R](iocfg4::R) reader structure"]
impl crate::Readable for IOCFG4 {}
#[doc = "`write(|w| ..)` method takes [iocfg4::W](iocfg4::W) writer structure"]
impl crate::Writable for IOCFG4 {}
#[doc = "Configuration of DIO4"]
pub mod iocfg4;
#[doc = "Configuration of DIO5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg5](iocfg5) module"]
pub type IOCFG5 = crate::Reg<u32, _IOCFG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG5;
#[doc = "`read()` method returns [iocfg5::R](iocfg5::R) reader structure"]
impl crate::Readable for IOCFG5 {}
#[doc = "`write(|w| ..)` method takes [iocfg5::W](iocfg5::W) writer structure"]
impl crate::Writable for IOCFG5 {}
#[doc = "Configuration of DIO5"]
pub mod iocfg5;
#[doc = "Configuration of DIO6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg6](iocfg6) module"]
pub type IOCFG6 = crate::Reg<u32, _IOCFG6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG6;
#[doc = "`read()` method returns [iocfg6::R](iocfg6::R) reader structure"]
impl crate::Readable for IOCFG6 {}
#[doc = "`write(|w| ..)` method takes [iocfg6::W](iocfg6::W) writer structure"]
impl crate::Writable for IOCFG6 {}
#[doc = "Configuration of DIO6"]
pub mod iocfg6;
#[doc = "Configuration of DIO7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg7](iocfg7) module"]
pub type IOCFG7 = crate::Reg<u32, _IOCFG7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG7;
#[doc = "`read()` method returns [iocfg7::R](iocfg7::R) reader structure"]
impl crate::Readable for IOCFG7 {}
#[doc = "`write(|w| ..)` method takes [iocfg7::W](iocfg7::W) writer structure"]
impl crate::Writable for IOCFG7 {}
#[doc = "Configuration of DIO7"]
pub mod iocfg7;
#[doc = "Configuration of DIO8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg8](iocfg8) module"]
pub type IOCFG8 = crate::Reg<u32, _IOCFG8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG8;
#[doc = "`read()` method returns [iocfg8::R](iocfg8::R) reader structure"]
impl crate::Readable for IOCFG8 {}
#[doc = "`write(|w| ..)` method takes [iocfg8::W](iocfg8::W) writer structure"]
impl crate::Writable for IOCFG8 {}
#[doc = "Configuration of DIO8"]
pub mod iocfg8;
#[doc = "Configuration of DIO9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg9](iocfg9) module"]
pub type IOCFG9 = crate::Reg<u32, _IOCFG9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG9;
#[doc = "`read()` method returns [iocfg9::R](iocfg9::R) reader structure"]
impl crate::Readable for IOCFG9 {}
#[doc = "`write(|w| ..)` method takes [iocfg9::W](iocfg9::W) writer structure"]
impl crate::Writable for IOCFG9 {}
#[doc = "Configuration of DIO9"]
pub mod iocfg9;
#[doc = "Configuration of DIO10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg10](iocfg10) module"]
pub type IOCFG10 = crate::Reg<u32, _IOCFG10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG10;
#[doc = "`read()` method returns [iocfg10::R](iocfg10::R) reader structure"]
impl crate::Readable for IOCFG10 {}
#[doc = "`write(|w| ..)` method takes [iocfg10::W](iocfg10::W) writer structure"]
impl crate::Writable for IOCFG10 {}
#[doc = "Configuration of DIO10"]
pub mod iocfg10;
#[doc = "Configuration of DIO11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg11](iocfg11) module"]
pub type IOCFG11 = crate::Reg<u32, _IOCFG11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG11;
#[doc = "`read()` method returns [iocfg11::R](iocfg11::R) reader structure"]
impl crate::Readable for IOCFG11 {}
#[doc = "`write(|w| ..)` method takes [iocfg11::W](iocfg11::W) writer structure"]
impl crate::Writable for IOCFG11 {}
#[doc = "Configuration of DIO11"]
pub mod iocfg11;
#[doc = "Configuration of DIO12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg12](iocfg12) module"]
pub type IOCFG12 = crate::Reg<u32, _IOCFG12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG12;
#[doc = "`read()` method returns [iocfg12::R](iocfg12::R) reader structure"]
impl crate::Readable for IOCFG12 {}
#[doc = "`write(|w| ..)` method takes [iocfg12::W](iocfg12::W) writer structure"]
impl crate::Writable for IOCFG12 {}
#[doc = "Configuration of DIO12"]
pub mod iocfg12;
#[doc = "Configuration of DIO13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg13](iocfg13) module"]
pub type IOCFG13 = crate::Reg<u32, _IOCFG13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG13;
#[doc = "`read()` method returns [iocfg13::R](iocfg13::R) reader structure"]
impl crate::Readable for IOCFG13 {}
#[doc = "`write(|w| ..)` method takes [iocfg13::W](iocfg13::W) writer structure"]
impl crate::Writable for IOCFG13 {}
#[doc = "Configuration of DIO13"]
pub mod iocfg13;
#[doc = "Configuration of DIO14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg14](iocfg14) module"]
pub type IOCFG14 = crate::Reg<u32, _IOCFG14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG14;
#[doc = "`read()` method returns [iocfg14::R](iocfg14::R) reader structure"]
impl crate::Readable for IOCFG14 {}
#[doc = "`write(|w| ..)` method takes [iocfg14::W](iocfg14::W) writer structure"]
impl crate::Writable for IOCFG14 {}
#[doc = "Configuration of DIO14"]
pub mod iocfg14;
#[doc = "Configuration of DIO15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg15](iocfg15) module"]
pub type IOCFG15 = crate::Reg<u32, _IOCFG15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG15;
#[doc = "`read()` method returns [iocfg15::R](iocfg15::R) reader structure"]
impl crate::Readable for IOCFG15 {}
#[doc = "`write(|w| ..)` method takes [iocfg15::W](iocfg15::W) writer structure"]
impl crate::Writable for IOCFG15 {}
#[doc = "Configuration of DIO15"]
pub mod iocfg15;
#[doc = "Configuration of DIO16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg16](iocfg16) module"]
pub type IOCFG16 = crate::Reg<u32, _IOCFG16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG16;
#[doc = "`read()` method returns [iocfg16::R](iocfg16::R) reader structure"]
impl crate::Readable for IOCFG16 {}
#[doc = "`write(|w| ..)` method takes [iocfg16::W](iocfg16::W) writer structure"]
impl crate::Writable for IOCFG16 {}
#[doc = "Configuration of DIO16"]
pub mod iocfg16;
#[doc = "Configuration of DIO17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg17](iocfg17) module"]
pub type IOCFG17 = crate::Reg<u32, _IOCFG17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG17;
#[doc = "`read()` method returns [iocfg17::R](iocfg17::R) reader structure"]
impl crate::Readable for IOCFG17 {}
#[doc = "`write(|w| ..)` method takes [iocfg17::W](iocfg17::W) writer structure"]
impl crate::Writable for IOCFG17 {}
#[doc = "Configuration of DIO17"]
pub mod iocfg17;
#[doc = "Configuration of DIO18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg18](iocfg18) module"]
pub type IOCFG18 = crate::Reg<u32, _IOCFG18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG18;
#[doc = "`read()` method returns [iocfg18::R](iocfg18::R) reader structure"]
impl crate::Readable for IOCFG18 {}
#[doc = "`write(|w| ..)` method takes [iocfg18::W](iocfg18::W) writer structure"]
impl crate::Writable for IOCFG18 {}
#[doc = "Configuration of DIO18"]
pub mod iocfg18;
#[doc = "Configuration of DIO19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg19](iocfg19) module"]
pub type IOCFG19 = crate::Reg<u32, _IOCFG19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG19;
#[doc = "`read()` method returns [iocfg19::R](iocfg19::R) reader structure"]
impl crate::Readable for IOCFG19 {}
#[doc = "`write(|w| ..)` method takes [iocfg19::W](iocfg19::W) writer structure"]
impl crate::Writable for IOCFG19 {}
#[doc = "Configuration of DIO19"]
pub mod iocfg19;
#[doc = "Configuration of DIO20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg20](iocfg20) module"]
pub type IOCFG20 = crate::Reg<u32, _IOCFG20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG20;
#[doc = "`read()` method returns [iocfg20::R](iocfg20::R) reader structure"]
impl crate::Readable for IOCFG20 {}
#[doc = "`write(|w| ..)` method takes [iocfg20::W](iocfg20::W) writer structure"]
impl crate::Writable for IOCFG20 {}
#[doc = "Configuration of DIO20"]
pub mod iocfg20;
#[doc = "Configuration of DIO21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg21](iocfg21) module"]
pub type IOCFG21 = crate::Reg<u32, _IOCFG21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG21;
#[doc = "`read()` method returns [iocfg21::R](iocfg21::R) reader structure"]
impl crate::Readable for IOCFG21 {}
#[doc = "`write(|w| ..)` method takes [iocfg21::W](iocfg21::W) writer structure"]
impl crate::Writable for IOCFG21 {}
#[doc = "Configuration of DIO21"]
pub mod iocfg21;
#[doc = "Configuration of DIO22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg22](iocfg22) module"]
pub type IOCFG22 = crate::Reg<u32, _IOCFG22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG22;
#[doc = "`read()` method returns [iocfg22::R](iocfg22::R) reader structure"]
impl crate::Readable for IOCFG22 {}
#[doc = "`write(|w| ..)` method takes [iocfg22::W](iocfg22::W) writer structure"]
impl crate::Writable for IOCFG22 {}
#[doc = "Configuration of DIO22"]
pub mod iocfg22;
#[doc = "Configuration of DIO23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg23](iocfg23) module"]
pub type IOCFG23 = crate::Reg<u32, _IOCFG23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG23;
#[doc = "`read()` method returns [iocfg23::R](iocfg23::R) reader structure"]
impl crate::Readable for IOCFG23 {}
#[doc = "`write(|w| ..)` method takes [iocfg23::W](iocfg23::W) writer structure"]
impl crate::Writable for IOCFG23 {}
#[doc = "Configuration of DIO23"]
pub mod iocfg23;
#[doc = "Configuration of DIO24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg24](iocfg24) module"]
pub type IOCFG24 = crate::Reg<u32, _IOCFG24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG24;
#[doc = "`read()` method returns [iocfg24::R](iocfg24::R) reader structure"]
impl crate::Readable for IOCFG24 {}
#[doc = "`write(|w| ..)` method takes [iocfg24::W](iocfg24::W) writer structure"]
impl crate::Writable for IOCFG24 {}
#[doc = "Configuration of DIO24"]
pub mod iocfg24;
#[doc = "Configuration of DIO25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg25](iocfg25) module"]
pub type IOCFG25 = crate::Reg<u32, _IOCFG25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG25;
#[doc = "`read()` method returns [iocfg25::R](iocfg25::R) reader structure"]
impl crate::Readable for IOCFG25 {}
#[doc = "`write(|w| ..)` method takes [iocfg25::W](iocfg25::W) writer structure"]
impl crate::Writable for IOCFG25 {}
#[doc = "Configuration of DIO25"]
pub mod iocfg25;
#[doc = "Configuration of DIO26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg26](iocfg26) module"]
pub type IOCFG26 = crate::Reg<u32, _IOCFG26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG26;
#[doc = "`read()` method returns [iocfg26::R](iocfg26::R) reader structure"]
impl crate::Readable for IOCFG26 {}
#[doc = "`write(|w| ..)` method takes [iocfg26::W](iocfg26::W) writer structure"]
impl crate::Writable for IOCFG26 {}
#[doc = "Configuration of DIO26"]
pub mod iocfg26;
#[doc = "Configuration of DIO27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg27](iocfg27) module"]
pub type IOCFG27 = crate::Reg<u32, _IOCFG27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG27;
#[doc = "`read()` method returns [iocfg27::R](iocfg27::R) reader structure"]
impl crate::Readable for IOCFG27 {}
#[doc = "`write(|w| ..)` method takes [iocfg27::W](iocfg27::W) writer structure"]
impl crate::Writable for IOCFG27 {}
#[doc = "Configuration of DIO27"]
pub mod iocfg27;
#[doc = "Configuration of DIO28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg28](iocfg28) module"]
pub type IOCFG28 = crate::Reg<u32, _IOCFG28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG28;
#[doc = "`read()` method returns [iocfg28::R](iocfg28::R) reader structure"]
impl crate::Readable for IOCFG28 {}
#[doc = "`write(|w| ..)` method takes [iocfg28::W](iocfg28::W) writer structure"]
impl crate::Writable for IOCFG28 {}
#[doc = "Configuration of DIO28"]
pub mod iocfg28;
#[doc = "Configuration of DIO29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg29](iocfg29) module"]
pub type IOCFG29 = crate::Reg<u32, _IOCFG29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG29;
#[doc = "`read()` method returns [iocfg29::R](iocfg29::R) reader structure"]
impl crate::Readable for IOCFG29 {}
#[doc = "`write(|w| ..)` method takes [iocfg29::W](iocfg29::W) writer structure"]
impl crate::Writable for IOCFG29 {}
#[doc = "Configuration of DIO29"]
pub mod iocfg29;
#[doc = "Configuration of DIO30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg30](iocfg30) module"]
pub type IOCFG30 = crate::Reg<u32, _IOCFG30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG30;
#[doc = "`read()` method returns [iocfg30::R](iocfg30::R) reader structure"]
impl crate::Readable for IOCFG30 {}
#[doc = "`write(|w| ..)` method takes [iocfg30::W](iocfg30::W) writer structure"]
impl crate::Writable for IOCFG30 {}
#[doc = "Configuration of DIO30"]
pub mod iocfg30;
#[doc = "Configuration of DIO31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iocfg31](iocfg31) module"]
pub type IOCFG31 = crate::Reg<u32, _IOCFG31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOCFG31;
#[doc = "`read()` method returns [iocfg31::R](iocfg31::R) reader structure"]
impl crate::Readable for IOCFG31 {}
#[doc = "`write(|w| ..)` method takes [iocfg31::W](iocfg31::W) writer structure"]
impl crate::Writable for IOCFG31 {}
#[doc = "Configuration of DIO31"]
pub mod iocfg31;

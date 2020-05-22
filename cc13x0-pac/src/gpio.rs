#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Out 0 to 3 Alias register for byte access to each bit in DOUT31_0"]
    pub dout3_0: DOUT3_0,
    #[doc = "0x04 - Data Out 4 to 7 Alias register for byte access to each bit in DOUT31_0"]
    pub dout7_4: DOUT7_4,
    #[doc = "0x08 - Data Out 8 to 11 Alias register for byte access to each bit in DOUT31_0"]
    pub dout11_8: DOUT11_8,
    #[doc = "0x0c - Data Out 12 to 15 Alias register for byte access to each bit in DOUT31_0"]
    pub dout15_12: DOUT15_12,
    #[doc = "0x10 - Data Out 16 to 19 Alias register for byte access to each bit in DOUT31_0"]
    pub dout19_16: DOUT19_16,
    #[doc = "0x14 - Data Out 20 to 23 Alias register for byte access to each bit in DOUT31_0"]
    pub dout23_20: DOUT23_20,
    #[doc = "0x18 - Data Out 24 to 27 Alias register for byte access to each bit in DOUT31_0"]
    pub dout27_24: DOUT27_24,
    #[doc = "0x1c - Data Out 28 to 31 Alias register for byte access to each bit in DOUT31_0"]
    pub dout31_28: DOUT31_28,
    _reserved8: [u8; 96usize],
    #[doc = "0x80 - Data Output for DIO 0 to 31"]
    pub dout31_0: DOUT31_0,
    _reserved9: [u8; 12usize],
    #[doc = "0x90 - Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT31_0 register"]
    pub doutset31_0: DOUTSET31_0,
    _reserved10: [u8; 12usize],
    #[doc = "0xa0 - Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT31_0 register"]
    pub doutclr31_0: DOUTCLR31_0,
    _reserved11: [u8; 12usize],
    #[doc = "0xb0 - Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
    pub douttgl31_0: DOUTTGL31_0,
    _reserved12: [u8; 12usize],
    #[doc = "0xc0 - Data Input from DIO 0 to 31"]
    pub din31_0: DIN31_0,
    _reserved13: [u8; 12usize],
    #[doc = "0xd0 - Data Output Enable for DIO 0 to 31"]
    pub doe31_0: DOE31_0,
    _reserved14: [u8; 12usize],
    #[doc = "0xe0 - Event Register for DIO 0 to 31 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
    pub evflags31_0: EVFLAGS31_0,
}
#[doc = "Data Out 0 to 3 Alias register for byte access to each bit in DOUT31_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout3_0](dout3_0) module"]
pub type DOUT3_0 = crate::Reg<u32, _DOUT3_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT3_0;
#[doc = "`read()` method returns [dout3_0::R](dout3_0::R) reader structure"]
impl crate::Readable for DOUT3_0 {}
#[doc = "`write(|w| ..)` method takes [dout3_0::W](dout3_0::W) writer structure"]
impl crate::Writable for DOUT3_0 {}
#[doc = "Data Out 0 to 3 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout3_0;
#[doc = "Data Out 4 to 7 Alias register for byte access to each bit in DOUT31_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout7_4](dout7_4) module"]
pub type DOUT7_4 = crate::Reg<u32, _DOUT7_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT7_4;
#[doc = "`read()` method returns [dout7_4::R](dout7_4::R) reader structure"]
impl crate::Readable for DOUT7_4 {}
#[doc = "`write(|w| ..)` method takes [dout7_4::W](dout7_4::W) writer structure"]
impl crate::Writable for DOUT7_4 {}
#[doc = "Data Out 4 to 7 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout7_4;
#[doc = "Data Out 8 to 11 Alias register for byte access to each bit in DOUT31_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout11_8](dout11_8) module"]
pub type DOUT11_8 = crate::Reg<u32, _DOUT11_8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT11_8;
#[doc = "`read()` method returns [dout11_8::R](dout11_8::R) reader structure"]
impl crate::Readable for DOUT11_8 {}
#[doc = "`write(|w| ..)` method takes [dout11_8::W](dout11_8::W) writer structure"]
impl crate::Writable for DOUT11_8 {}
#[doc = "Data Out 8 to 11 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout11_8;
#[doc = "Data Out 12 to 15 Alias register for byte access to each bit in DOUT31_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout15_12](dout15_12) module"]
pub type DOUT15_12 = crate::Reg<u32, _DOUT15_12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT15_12;
#[doc = "`read()` method returns [dout15_12::R](dout15_12::R) reader structure"]
impl crate::Readable for DOUT15_12 {}
#[doc = "`write(|w| ..)` method takes [dout15_12::W](dout15_12::W) writer structure"]
impl crate::Writable for DOUT15_12 {}
#[doc = "Data Out 12 to 15 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout15_12;
#[doc = "Data Out 16 to 19 Alias register for byte access to each bit in DOUT31_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout19_16](dout19_16) module"]
pub type DOUT19_16 = crate::Reg<u32, _DOUT19_16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT19_16;
#[doc = "`read()` method returns [dout19_16::R](dout19_16::R) reader structure"]
impl crate::Readable for DOUT19_16 {}
#[doc = "`write(|w| ..)` method takes [dout19_16::W](dout19_16::W) writer structure"]
impl crate::Writable for DOUT19_16 {}
#[doc = "Data Out 16 to 19 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout19_16;
#[doc = "Data Out 20 to 23 Alias register for byte access to each bit in DOUT31_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout23_20](dout23_20) module"]
pub type DOUT23_20 = crate::Reg<u32, _DOUT23_20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT23_20;
#[doc = "`read()` method returns [dout23_20::R](dout23_20::R) reader structure"]
impl crate::Readable for DOUT23_20 {}
#[doc = "`write(|w| ..)` method takes [dout23_20::W](dout23_20::W) writer structure"]
impl crate::Writable for DOUT23_20 {}
#[doc = "Data Out 20 to 23 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout23_20;
#[doc = "Data Out 24 to 27 Alias register for byte access to each bit in DOUT31_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout27_24](dout27_24) module"]
pub type DOUT27_24 = crate::Reg<u32, _DOUT27_24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT27_24;
#[doc = "`read()` method returns [dout27_24::R](dout27_24::R) reader structure"]
impl crate::Readable for DOUT27_24 {}
#[doc = "`write(|w| ..)` method takes [dout27_24::W](dout27_24::W) writer structure"]
impl crate::Writable for DOUT27_24 {}
#[doc = "Data Out 24 to 27 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout27_24;
#[doc = "Data Out 28 to 31 Alias register for byte access to each bit in DOUT31_0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout31_28](dout31_28) module"]
pub type DOUT31_28 = crate::Reg<u32, _DOUT31_28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT31_28;
#[doc = "`read()` method returns [dout31_28::R](dout31_28::R) reader structure"]
impl crate::Readable for DOUT31_28 {}
#[doc = "`write(|w| ..)` method takes [dout31_28::W](dout31_28::W) writer structure"]
impl crate::Writable for DOUT31_28 {}
#[doc = "Data Out 28 to 31 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout31_28;
#[doc = "Data Output for DIO 0 to 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dout31_0](dout31_0) module"]
pub type DOUT31_0 = crate::Reg<u32, _DOUT31_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUT31_0;
#[doc = "`read()` method returns [dout31_0::R](dout31_0::R) reader structure"]
impl crate::Readable for DOUT31_0 {}
#[doc = "`write(|w| ..)` method takes [dout31_0::W](dout31_0::W) writer structure"]
impl crate::Writable for DOUT31_0 {}
#[doc = "Data Output for DIO 0 to 31"]
pub mod dout31_0;
#[doc = "Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT31_0 register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutset31_0](doutset31_0) module"]
pub type DOUTSET31_0 = crate::Reg<u32, _DOUTSET31_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTSET31_0;
#[doc = "`write(|w| ..)` method takes [doutset31_0::W](doutset31_0::W) writer structure"]
impl crate::Writable for DOUTSET31_0 {}
#[doc = "Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT31_0 register"]
pub mod doutset31_0;
#[doc = "Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT31_0 register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutclr31_0](doutclr31_0) module"]
pub type DOUTCLR31_0 = crate::Reg<u32, _DOUTCLR31_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTCLR31_0;
#[doc = "`write(|w| ..)` method takes [doutclr31_0::W](doutclr31_0::W) writer structure"]
impl crate::Writable for DOUTCLR31_0 {}
#[doc = "Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT31_0 register"]
pub mod doutclr31_0;
#[doc = "Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [douttgl31_0](douttgl31_0) module"]
pub type DOUTTGL31_0 = crate::Reg<u32, _DOUTTGL31_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOUTTGL31_0;
#[doc = "`read()` method returns [douttgl31_0::R](douttgl31_0::R) reader structure"]
impl crate::Readable for DOUTTGL31_0 {}
#[doc = "`write(|w| ..)` method takes [douttgl31_0::W](douttgl31_0::W) writer structure"]
impl crate::Writable for DOUTTGL31_0 {}
#[doc = "Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
pub mod douttgl31_0;
#[doc = "Data Input from DIO 0 to 31\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [din31_0](din31_0) module"]
pub type DIN31_0 = crate::Reg<u32, _DIN31_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIN31_0;
#[doc = "`read()` method returns [din31_0::R](din31_0::R) reader structure"]
impl crate::Readable for DIN31_0 {}
#[doc = "Data Input from DIO 0 to 31"]
pub mod din31_0;
#[doc = "Data Output Enable for DIO 0 to 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doe31_0](doe31_0) module"]
pub type DOE31_0 = crate::Reg<u32, _DOE31_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOE31_0;
#[doc = "`read()` method returns [doe31_0::R](doe31_0::R) reader structure"]
impl crate::Readable for DOE31_0 {}
#[doc = "`write(|w| ..)` method takes [doe31_0::W](doe31_0::W) writer structure"]
impl crate::Writable for DOE31_0 {}
#[doc = "Data Output Enable for DIO 0 to 31"]
pub mod doe31_0;
#[doc = "Event Register for DIO 0 to 31 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evflags31_0](evflags31_0) module"]
pub type EVFLAGS31_0 = crate::Reg<u32, _EVFLAGS31_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVFLAGS31_0;
#[doc = "`read()` method returns [evflags31_0::R](evflags31_0::R) reader structure"]
impl crate::Readable for EVFLAGS31_0 {}
#[doc = "`write(|w| ..)` method takes [evflags31_0::W](evflags31_0::W) writer structure"]
impl crate::Writable for EVFLAGS31_0 {}
#[doc = "Event Register for DIO 0 to 31 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
pub mod evflags31_0;

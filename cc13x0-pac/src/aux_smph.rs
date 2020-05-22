#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Semaphore 0"]
    pub smph0: SMPH0,
    #[doc = "0x04 - Semaphore 1"]
    pub smph1: SMPH1,
    #[doc = "0x08 - Semaphore 2"]
    pub smph2: SMPH2,
    #[doc = "0x0c - Semaphore 3"]
    pub smph3: SMPH3,
    #[doc = "0x10 - Semaphore 4"]
    pub smph4: SMPH4,
    #[doc = "0x14 - Semaphore 5"]
    pub smph5: SMPH5,
    #[doc = "0x18 - Semaphore 6"]
    pub smph6: SMPH6,
    #[doc = "0x1c - Semaphore 7"]
    pub smph7: SMPH7,
    #[doc = "0x20 - Sticky Request For Single Semaphore"]
    pub autotake: AUTOTAKE,
}
#[doc = "Semaphore 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph0](smph0) module"]
pub type SMPH0 = crate::Reg<u32, _SMPH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH0;
#[doc = "`read()` method returns [smph0::R](smph0::R) reader structure"]
impl crate::Readable for SMPH0 {}
#[doc = "`write(|w| ..)` method takes [smph0::W](smph0::W) writer structure"]
impl crate::Writable for SMPH0 {}
#[doc = "Semaphore 0"]
pub mod smph0;
#[doc = "Semaphore 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph1](smph1) module"]
pub type SMPH1 = crate::Reg<u32, _SMPH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH1;
#[doc = "`read()` method returns [smph1::R](smph1::R) reader structure"]
impl crate::Readable for SMPH1 {}
#[doc = "`write(|w| ..)` method takes [smph1::W](smph1::W) writer structure"]
impl crate::Writable for SMPH1 {}
#[doc = "Semaphore 1"]
pub mod smph1;
#[doc = "Semaphore 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph2](smph2) module"]
pub type SMPH2 = crate::Reg<u32, _SMPH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH2;
#[doc = "`read()` method returns [smph2::R](smph2::R) reader structure"]
impl crate::Readable for SMPH2 {}
#[doc = "`write(|w| ..)` method takes [smph2::W](smph2::W) writer structure"]
impl crate::Writable for SMPH2 {}
#[doc = "Semaphore 2"]
pub mod smph2;
#[doc = "Semaphore 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph3](smph3) module"]
pub type SMPH3 = crate::Reg<u32, _SMPH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH3;
#[doc = "`read()` method returns [smph3::R](smph3::R) reader structure"]
impl crate::Readable for SMPH3 {}
#[doc = "`write(|w| ..)` method takes [smph3::W](smph3::W) writer structure"]
impl crate::Writable for SMPH3 {}
#[doc = "Semaphore 3"]
pub mod smph3;
#[doc = "Semaphore 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph4](smph4) module"]
pub type SMPH4 = crate::Reg<u32, _SMPH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH4;
#[doc = "`read()` method returns [smph4::R](smph4::R) reader structure"]
impl crate::Readable for SMPH4 {}
#[doc = "`write(|w| ..)` method takes [smph4::W](smph4::W) writer structure"]
impl crate::Writable for SMPH4 {}
#[doc = "Semaphore 4"]
pub mod smph4;
#[doc = "Semaphore 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph5](smph5) module"]
pub type SMPH5 = crate::Reg<u32, _SMPH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH5;
#[doc = "`read()` method returns [smph5::R](smph5::R) reader structure"]
impl crate::Readable for SMPH5 {}
#[doc = "`write(|w| ..)` method takes [smph5::W](smph5::W) writer structure"]
impl crate::Writable for SMPH5 {}
#[doc = "Semaphore 5"]
pub mod smph5;
#[doc = "Semaphore 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph6](smph6) module"]
pub type SMPH6 = crate::Reg<u32, _SMPH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH6;
#[doc = "`read()` method returns [smph6::R](smph6::R) reader structure"]
impl crate::Readable for SMPH6 {}
#[doc = "`write(|w| ..)` method takes [smph6::W](smph6::W) writer structure"]
impl crate::Writable for SMPH6 {}
#[doc = "Semaphore 6"]
pub mod smph6;
#[doc = "Semaphore 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smph7](smph7) module"]
pub type SMPH7 = crate::Reg<u32, _SMPH7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMPH7;
#[doc = "`read()` method returns [smph7::R](smph7::R) reader structure"]
impl crate::Readable for SMPH7 {}
#[doc = "`write(|w| ..)` method takes [smph7::W](smph7::W) writer structure"]
impl crate::Writable for SMPH7 {}
#[doc = "Semaphore 7"]
pub mod smph7;
#[doc = "Sticky Request For Single Semaphore\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autotake](autotake) module"]
pub type AUTOTAKE = crate::Reg<u32, _AUTOTAKE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUTOTAKE;
#[doc = "`read()` method returns [autotake::R](autotake::R) reader structure"]
impl crate::Readable for AUTOTAKE {}
#[doc = "`write(|w| ..)` method takes [autotake::W](autotake::W) writer structure"]
impl crate::Writable for AUTOTAKE {}
#[doc = "Sticky Request For Single Semaphore"]
pub mod autotake;

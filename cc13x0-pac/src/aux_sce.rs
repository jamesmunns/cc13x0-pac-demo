#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub ctl: CTL,
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    pub fetchstat: FETCHSTAT,
    #[doc = "0x08 - Internal. Only to be used through TI provided API."]
    pub cpustat: CPUSTAT,
    #[doc = "0x0c - Internal. Only to be used through TI provided API."]
    pub wustat: WUSTAT,
    #[doc = "0x10 - Internal. Only to be used through TI provided API."]
    pub reg1_0: REG1_0,
    #[doc = "0x14 - Internal. Only to be used through TI provided API."]
    pub reg3_2: REG3_2,
    #[doc = "0x18 - Internal. Only to be used through TI provided API."]
    pub reg5_4: REG5_4,
    #[doc = "0x1c - Internal. Only to be used through TI provided API."]
    pub reg7_6: REG7_6,
    #[doc = "0x20 - Internal. Only to be used through TI provided API."]
    pub loopaddr: LOOPADDR,
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    pub loopcnt: LOOPCNT,
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ctl;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fetchstat](fetchstat) module"]
pub type FETCHSTAT = crate::Reg<u32, _FETCHSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FETCHSTAT;
#[doc = "`read()` method returns [fetchstat::R](fetchstat::R) reader structure"]
impl crate::Readable for FETCHSTAT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fetchstat;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpustat](cpustat) module"]
pub type CPUSTAT = crate::Reg<u32, _CPUSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUSTAT;
#[doc = "`read()` method returns [cpustat::R](cpustat::R) reader structure"]
impl crate::Readable for CPUSTAT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cpustat;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wustat](wustat) module"]
pub type WUSTAT = crate::Reg<u32, _WUSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WUSTAT;
#[doc = "`read()` method returns [wustat::R](wustat::R) reader structure"]
impl crate::Readable for WUSTAT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod wustat;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg1_0](reg1_0) module"]
pub type REG1_0 = crate::Reg<u32, _REG1_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG1_0;
#[doc = "`read()` method returns [reg1_0::R](reg1_0::R) reader structure"]
impl crate::Readable for REG1_0 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg1_0;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg3_2](reg3_2) module"]
pub type REG3_2 = crate::Reg<u32, _REG3_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG3_2;
#[doc = "`read()` method returns [reg3_2::R](reg3_2::R) reader structure"]
impl crate::Readable for REG3_2 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg3_2;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg5_4](reg5_4) module"]
pub type REG5_4 = crate::Reg<u32, _REG5_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG5_4;
#[doc = "`read()` method returns [reg5_4::R](reg5_4::R) reader structure"]
impl crate::Readable for REG5_4 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg5_4;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg7_6](reg7_6) module"]
pub type REG7_6 = crate::Reg<u32, _REG7_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REG7_6;
#[doc = "`read()` method returns [reg7_6::R](reg7_6::R) reader structure"]
impl crate::Readable for REG7_6 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod reg7_6;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loopaddr](loopaddr) module"]
pub type LOOPADDR = crate::Reg<u32, _LOOPADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOOPADDR;
#[doc = "`read()` method returns [loopaddr::R](loopaddr::R) reader structure"]
impl crate::Readable for LOOPADDR {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod loopaddr;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loopcnt](loopcnt) module"]
pub type LOOPCNT = crate::Reg<u32, _LOOPCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOOPCNT;
#[doc = "`read()` method returns [loopcnt::R](loopcnt::R) reader structure"]
impl crate::Readable for LOOPCNT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod loopcnt;

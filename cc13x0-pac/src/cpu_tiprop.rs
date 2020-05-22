#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4088usize],
    #[doc = "0xff8 - Internal. Only to be used through TI provided API."]
    pub traceclkmux: TRACECLKMUX,
    #[doc = "0xffc - Internal. Only to be used through TI provided API."]
    pub dyn_cg: DYN_CG,
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [traceclkmux](traceclkmux) module"]
pub type TRACECLKMUX = crate::Reg<u32, _TRACECLKMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRACECLKMUX;
#[doc = "`read()` method returns [traceclkmux::R](traceclkmux::R) reader structure"]
impl crate::Readable for TRACECLKMUX {}
#[doc = "`write(|w| ..)` method takes [traceclkmux::W](traceclkmux::W) writer structure"]
impl crate::Writable for TRACECLKMUX {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod traceclkmux;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dyn_cg](dyn_cg) module"]
pub type DYN_CG = crate::Reg<u32, _DYN_CG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DYN_CG;
#[doc = "`read()` method returns [dyn_cg::R](dyn_cg::R) reader structure"]
impl crate::Readable for DYN_CG {}
#[doc = "`write(|w| ..)` method takes [dyn_cg::W](dyn_cg::W) writer structure"]
impl crate::Writable for DYN_CG {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dyn_cg;

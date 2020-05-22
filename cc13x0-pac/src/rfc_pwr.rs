#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RF Core Power Management and Clock Enable"]
    pub pwmclken: PWMCLKEN,
}
#[doc = "RF Core Power Management and Clock Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmclken](pwmclken) module"]
pub type PWMCLKEN = crate::Reg<u32, _PWMCLKEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWMCLKEN;
#[doc = "`read()` method returns [pwmclken::R](pwmclken::R) reader structure"]
impl crate::Readable for PWMCLKEN {}
#[doc = "`write(|w| ..)` method takes [pwmclken::W](pwmclken::W) writer structure"]
impl crate::Writable for PWMCLKEN {}
#[doc = "RF Core Power Management and Clock Enable"]
pub mod pwmclken;

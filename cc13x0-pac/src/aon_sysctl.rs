#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Management This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
    pub pwrctl: PWRCTL,
    #[doc = "0x04 - Reset Management This register contains bitfields releated to system reset such as reset source and reset request and control of brown out resets."]
    pub resetctl: RESETCTL,
    #[doc = "0x08 - Sleep Mode This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
    pub sleepctl: SLEEPCTL,
}
#[doc = "Power Management This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrctl](pwrctl) module"]
pub type PWRCTL = crate::Reg<u32, _PWRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCTL;
#[doc = "`read()` method returns [pwrctl::R](pwrctl::R) reader structure"]
impl crate::Readable for PWRCTL {}
#[doc = "`write(|w| ..)` method takes [pwrctl::W](pwrctl::W) writer structure"]
impl crate::Writable for PWRCTL {}
#[doc = "Power Management This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
pub mod pwrctl;
#[doc = "Reset Management This register contains bitfields releated to system reset such as reset source and reset request and control of brown out resets.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetctl](resetctl) module"]
pub type RESETCTL = crate::Reg<u32, _RESETCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETCTL;
#[doc = "`read()` method returns [resetctl::R](resetctl::R) reader structure"]
impl crate::Readable for RESETCTL {}
#[doc = "`write(|w| ..)` method takes [resetctl::W](resetctl::W) writer structure"]
impl crate::Writable for RESETCTL {}
#[doc = "Reset Management This register contains bitfields releated to system reset such as reset source and reset request and control of brown out resets."]
pub mod resetctl;
#[doc = "Sleep Mode This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sleepctl](sleepctl) module"]
pub type SLEEPCTL = crate::Reg<u32, _SLEEPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLEEPCTL;
#[doc = "`read()` method returns [sleepctl::R](sleepctl::R) reader structure"]
impl crate::Readable for SLEEPCTL {}
#[doc = "`write(|w| ..)` method takes [sleepctl::W](sleepctl::W) writer structure"]
impl crate::Writable for SLEEPCTL {}
#[doc = "Sleep Mode This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
pub mod sleepctl;

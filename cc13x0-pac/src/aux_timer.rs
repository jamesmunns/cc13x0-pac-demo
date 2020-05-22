#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer 0 Configuration"]
    pub t0cfg: T0CFG,
    #[doc = "0x04 - Timer 1 Configuration"]
    pub t1cfg: T1CFG,
    #[doc = "0x08 - Timer 0 Control Run control/status for timer 0"]
    pub t0ctl: T0CTL,
    #[doc = "0x0c - Timer 0 Target Target counter value for timer 0"]
    pub t0target: T0TARGET,
    #[doc = "0x10 - Timer 1 Target Target Counter Value Timer 1"]
    pub t1target: T1TARGET,
    #[doc = "0x14 - Timer 1 Control Run Control/Status For Timer 1"]
    pub t1ctl: T1CTL,
}
#[doc = "Timer 0 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0cfg](t0cfg) module"]
pub type T0CFG = crate::Reg<u32, _T0CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0CFG;
#[doc = "`read()` method returns [t0cfg::R](t0cfg::R) reader structure"]
impl crate::Readable for T0CFG {}
#[doc = "`write(|w| ..)` method takes [t0cfg::W](t0cfg::W) writer structure"]
impl crate::Writable for T0CFG {}
#[doc = "Timer 0 Configuration"]
pub mod t0cfg;
#[doc = "Timer 1 Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1cfg](t1cfg) module"]
pub type T1CFG = crate::Reg<u32, _T1CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1CFG;
#[doc = "`read()` method returns [t1cfg::R](t1cfg::R) reader structure"]
impl crate::Readable for T1CFG {}
#[doc = "`write(|w| ..)` method takes [t1cfg::W](t1cfg::W) writer structure"]
impl crate::Writable for T1CFG {}
#[doc = "Timer 1 Configuration"]
pub mod t1cfg;
#[doc = "Timer 0 Control Run control/status for timer 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0ctl](t0ctl) module"]
pub type T0CTL = crate::Reg<u32, _T0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0CTL;
#[doc = "`read()` method returns [t0ctl::R](t0ctl::R) reader structure"]
impl crate::Readable for T0CTL {}
#[doc = "`write(|w| ..)` method takes [t0ctl::W](t0ctl::W) writer structure"]
impl crate::Writable for T0CTL {}
#[doc = "Timer 0 Control Run control/status for timer 0"]
pub mod t0ctl;
#[doc = "Timer 0 Target Target counter value for timer 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0target](t0target) module"]
pub type T0TARGET = crate::Reg<u32, _T0TARGET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0TARGET;
#[doc = "`read()` method returns [t0target::R](t0target::R) reader structure"]
impl crate::Readable for T0TARGET {}
#[doc = "`write(|w| ..)` method takes [t0target::W](t0target::W) writer structure"]
impl crate::Writable for T0TARGET {}
#[doc = "Timer 0 Target Target counter value for timer 0"]
pub mod t0target;
#[doc = "Timer 1 Target Target Counter Value Timer 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1target](t1target) module"]
pub type T1TARGET = crate::Reg<u32, _T1TARGET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1TARGET;
#[doc = "`read()` method returns [t1target::R](t1target::R) reader structure"]
impl crate::Readable for T1TARGET {}
#[doc = "`write(|w| ..)` method takes [t1target::W](t1target::W) writer structure"]
impl crate::Writable for T1TARGET {}
#[doc = "Timer 1 Target Target Counter Value Timer 1"]
pub mod t1target;
#[doc = "Timer 1 Control Run Control/Status For Timer 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1ctl](t1ctl) module"]
pub type T1CTL = crate::Reg<u32, _T1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1CTL;
#[doc = "`read()` method returns [t1ctl::R](t1ctl::R) reader structure"]
impl crate::Readable for T1CTL {}
#[doc = "`write(|w| ..)` method takes [t1ctl::W](t1ctl::W) writer structure"]
impl crate::Writable for T1CTL {}
#[doc = "Timer 1 Control Run Control/Status For Timer 1"]
pub mod t1ctl;

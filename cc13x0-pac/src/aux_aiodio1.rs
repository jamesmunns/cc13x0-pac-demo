#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose Input/Output Data Out This register is used to set data on the pads assigned to AUX"]
    pub gpiodout: GPIODOUT,
    #[doc = "0x04 - Input Output Mode Controls pull-up pull-down and output mode for the IO pins assigned to AUX"]
    pub iomode: IOMODE,
    #[doc = "0x08 - General Purpose Input Output Data In"]
    pub gpiodin: GPIODIN,
    #[doc = "0x0c - General Purpose Input Output Data Out Set Strobes for setting output data register bits"]
    pub gpiodoutset: GPIODOUTSET,
    #[doc = "0x10 - General Purpose Input Output Data Out Clear Strobes for clearing output data register bits"]
    pub gpiodoutclr: GPIODOUTCLR,
    #[doc = "0x14 - General Purpose Input Output Data Out Toggle Strobes for toggling output data register bits"]
    pub gpiodouttgl: GPIODOUTTGL,
    #[doc = "0x18 - General Purpose Input Output Input Enable"]
    pub gpiodie: GPIODIE,
}
#[doc = "General Purpose Input/Output Data Out This register is used to set data on the pads assigned to AUX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodout](gpiodout) module"]
pub type GPIODOUT = crate::Reg<u32, _GPIODOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODOUT;
#[doc = "`read()` method returns [gpiodout::R](gpiodout::R) reader structure"]
impl crate::Readable for GPIODOUT {}
#[doc = "`write(|w| ..)` method takes [gpiodout::W](gpiodout::W) writer structure"]
impl crate::Writable for GPIODOUT {}
#[doc = "General Purpose Input/Output Data Out This register is used to set data on the pads assigned to AUX"]
pub mod gpiodout;
#[doc = "Input Output Mode Controls pull-up pull-down and output mode for the IO pins assigned to AUX\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iomode](iomode) module"]
pub type IOMODE = crate::Reg<u32, _IOMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOMODE;
#[doc = "`read()` method returns [iomode::R](iomode::R) reader structure"]
impl crate::Readable for IOMODE {}
#[doc = "`write(|w| ..)` method takes [iomode::W](iomode::W) writer structure"]
impl crate::Writable for IOMODE {}
#[doc = "Input Output Mode Controls pull-up pull-down and output mode for the IO pins assigned to AUX"]
pub mod iomode;
#[doc = "General Purpose Input Output Data In\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodin](gpiodin) module"]
pub type GPIODIN = crate::Reg<u32, _GPIODIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODIN;
#[doc = "`read()` method returns [gpiodin::R](gpiodin::R) reader structure"]
impl crate::Readable for GPIODIN {}
#[doc = "General Purpose Input Output Data In"]
pub mod gpiodin;
#[doc = "General Purpose Input Output Data Out Set Strobes for setting output data register bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodoutset](gpiodoutset) module"]
pub type GPIODOUTSET = crate::Reg<u32, _GPIODOUTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODOUTSET;
#[doc = "`read()` method returns [gpiodoutset::R](gpiodoutset::R) reader structure"]
impl crate::Readable for GPIODOUTSET {}
#[doc = "`write(|w| ..)` method takes [gpiodoutset::W](gpiodoutset::W) writer structure"]
impl crate::Writable for GPIODOUTSET {}
#[doc = "General Purpose Input Output Data Out Set Strobes for setting output data register bits"]
pub mod gpiodoutset;
#[doc = "General Purpose Input Output Data Out Clear Strobes for clearing output data register bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodoutclr](gpiodoutclr) module"]
pub type GPIODOUTCLR = crate::Reg<u32, _GPIODOUTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODOUTCLR;
#[doc = "`read()` method returns [gpiodoutclr::R](gpiodoutclr::R) reader structure"]
impl crate::Readable for GPIODOUTCLR {}
#[doc = "`write(|w| ..)` method takes [gpiodoutclr::W](gpiodoutclr::W) writer structure"]
impl crate::Writable for GPIODOUTCLR {}
#[doc = "General Purpose Input Output Data Out Clear Strobes for clearing output data register bits"]
pub mod gpiodoutclr;
#[doc = "General Purpose Input Output Data Out Toggle Strobes for toggling output data register bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodouttgl](gpiodouttgl) module"]
pub type GPIODOUTTGL = crate::Reg<u32, _GPIODOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODOUTTGL;
#[doc = "`read()` method returns [gpiodouttgl::R](gpiodouttgl::R) reader structure"]
impl crate::Readable for GPIODOUTTGL {}
#[doc = "`write(|w| ..)` method takes [gpiodouttgl::W](gpiodouttgl::W) writer structure"]
impl crate::Writable for GPIODOUTTGL {}
#[doc = "General Purpose Input Output Data Out Toggle Strobes for toggling output data register bits"]
pub mod gpiodouttgl;
#[doc = "General Purpose Input Output Input Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiodie](gpiodie) module"]
pub type GPIODIE = crate::Reg<u32, _GPIODIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIODIE;
#[doc = "`read()` method returns [gpiodie::R](gpiodie::R) reader structure"]
impl crate::Readable for GPIODIE {}
#[doc = "`write(|w| ..)` method takes [gpiodie::W](gpiodie::W) writer structure"]
impl crate::Writable for GPIODIE {}
#[doc = "General Purpose Input Output Input Enable"]
pub mod gpiodie;

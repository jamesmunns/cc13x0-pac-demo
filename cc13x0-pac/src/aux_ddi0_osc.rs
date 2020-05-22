#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control 0 Controls various clock source selects"]
    pub ctl0: CTL0,
    #[doc = "0x04 - Control 1 This register contains various OSC_DIG configuration"]
    pub ctl1: CTL1,
    #[doc = "0x08 - RADC External Configuration"]
    pub radcextcfg: RADCEXTCFG,
    #[doc = "0x0c - Amplitude Compensation Control"]
    pub ampcompctl: AMPCOMPCTL,
    #[doc = "0x10 - Amplitude Compensation Threashold 1 This register contains various threshhold values for amplitude compensation algorithm"]
    pub ampcompth1: AMPCOMPTH1,
    #[doc = "0x14 - Amplitude Compensation Threashold 2 This register contains various threshhold values for amplitude compensation algorithm."]
    pub ampcompth2: AMPCOMPTH2,
    #[doc = "0x18 - Analog Bypass Values 1"]
    pub anabypassval1: ANABYPASSVAL1,
    #[doc = "0x1c - Internal. Only to be used through TI provided API."]
    pub anabypassval2: ANABYPASSVAL2,
    #[doc = "0x20 - Analog Test Control"]
    pub atestctl: ATESTCTL,
    #[doc = "0x24 - ADC Doubler Nanoamp Control"]
    pub adcdoublernanoampctl: ADCDOUBLERNANOAMPCTL,
    #[doc = "0x28 - XOSCHF Control"]
    pub xoschfctl: XOSCHFCTL,
    #[doc = "0x2c - Low Frequency Oscillator Control"]
    pub lfoscctl: LFOSCCTL,
    #[doc = "0x30 - RCOSCHF Control"]
    pub rcoschfctl: RCOSCHFCTL,
    #[doc = "0x34 - Status 0 This register contains status signals from OSC_DIG"]
    pub stat0: STAT0,
    #[doc = "0x38 - Status 1 This register contains status signals from OSC_DIG"]
    pub stat1: STAT1,
    #[doc = "0x3c - Status 2 This register contains status signals from AMPCOMP FSM"]
    pub stat2: STAT2,
}
#[doc = "Control 0 Controls various clock source selects\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](ctl0) module"]
pub type CTL0 = crate::Reg<u32, _CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL0;
#[doc = "`read()` method returns [ctl0::R](ctl0::R) reader structure"]
impl crate::Readable for CTL0 {}
#[doc = "`write(|w| ..)` method takes [ctl0::W](ctl0::W) writer structure"]
impl crate::Writable for CTL0 {}
#[doc = "Control 0 Controls various clock source selects"]
pub mod ctl0;
#[doc = "Control 1 This register contains various OSC_DIG configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](ctl1) module"]
pub type CTL1 = crate::Reg<u32, _CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL1;
#[doc = "`read()` method returns [ctl1::R](ctl1::R) reader structure"]
impl crate::Readable for CTL1 {}
#[doc = "`write(|w| ..)` method takes [ctl1::W](ctl1::W) writer structure"]
impl crate::Writable for CTL1 {}
#[doc = "Control 1 This register contains various OSC_DIG configuration"]
pub mod ctl1;
#[doc = "RADC External Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [radcextcfg](radcextcfg) module"]
pub type RADCEXTCFG = crate::Reg<u32, _RADCEXTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RADCEXTCFG;
#[doc = "`read()` method returns [radcextcfg::R](radcextcfg::R) reader structure"]
impl crate::Readable for RADCEXTCFG {}
#[doc = "`write(|w| ..)` method takes [radcextcfg::W](radcextcfg::W) writer structure"]
impl crate::Writable for RADCEXTCFG {}
#[doc = "RADC External Configuration"]
pub mod radcextcfg;
#[doc = "Amplitude Compensation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcompctl](ampcompctl) module"]
pub type AMPCOMPCTL = crate::Reg<u32, _AMPCOMPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMPCOMPCTL;
#[doc = "`read()` method returns [ampcompctl::R](ampcompctl::R) reader structure"]
impl crate::Readable for AMPCOMPCTL {}
#[doc = "`write(|w| ..)` method takes [ampcompctl::W](ampcompctl::W) writer structure"]
impl crate::Writable for AMPCOMPCTL {}
#[doc = "Amplitude Compensation Control"]
pub mod ampcompctl;
#[doc = "Amplitude Compensation Threashold 1 This register contains various threshhold values for amplitude compensation algorithm\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcompth1](ampcompth1) module"]
pub type AMPCOMPTH1 = crate::Reg<u32, _AMPCOMPTH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMPCOMPTH1;
#[doc = "`read()` method returns [ampcompth1::R](ampcompth1::R) reader structure"]
impl crate::Readable for AMPCOMPTH1 {}
#[doc = "`write(|w| ..)` method takes [ampcompth1::W](ampcompth1::W) writer structure"]
impl crate::Writable for AMPCOMPTH1 {}
#[doc = "Amplitude Compensation Threashold 1 This register contains various threshhold values for amplitude compensation algorithm"]
pub mod ampcompth1;
#[doc = "Amplitude Compensation Threashold 2 This register contains various threshhold values for amplitude compensation algorithm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcompth2](ampcompth2) module"]
pub type AMPCOMPTH2 = crate::Reg<u32, _AMPCOMPTH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMPCOMPTH2;
#[doc = "`read()` method returns [ampcompth2::R](ampcompth2::R) reader structure"]
impl crate::Readable for AMPCOMPTH2 {}
#[doc = "`write(|w| ..)` method takes [ampcompth2::W](ampcompth2::W) writer structure"]
impl crate::Writable for AMPCOMPTH2 {}
#[doc = "Amplitude Compensation Threashold 2 This register contains various threshhold values for amplitude compensation algorithm."]
pub mod ampcompth2;
#[doc = "Analog Bypass Values 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anabypassval1](anabypassval1) module"]
pub type ANABYPASSVAL1 = crate::Reg<u32, _ANABYPASSVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANABYPASSVAL1;
#[doc = "`read()` method returns [anabypassval1::R](anabypassval1::R) reader structure"]
impl crate::Readable for ANABYPASSVAL1 {}
#[doc = "`write(|w| ..)` method takes [anabypassval1::W](anabypassval1::W) writer structure"]
impl crate::Writable for ANABYPASSVAL1 {}
#[doc = "Analog Bypass Values 1"]
pub mod anabypassval1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anabypassval2](anabypassval2) module"]
pub type ANABYPASSVAL2 = crate::Reg<u32, _ANABYPASSVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANABYPASSVAL2;
#[doc = "`read()` method returns [anabypassval2::R](anabypassval2::R) reader structure"]
impl crate::Readable for ANABYPASSVAL2 {}
#[doc = "`write(|w| ..)` method takes [anabypassval2::W](anabypassval2::W) writer structure"]
impl crate::Writable for ANABYPASSVAL2 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod anabypassval2;
#[doc = "Analog Test Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atestctl](atestctl) module"]
pub type ATESTCTL = crate::Reg<u32, _ATESTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ATESTCTL;
#[doc = "`read()` method returns [atestctl::R](atestctl::R) reader structure"]
impl crate::Readable for ATESTCTL {}
#[doc = "`write(|w| ..)` method takes [atestctl::W](atestctl::W) writer structure"]
impl crate::Writable for ATESTCTL {}
#[doc = "Analog Test Control"]
pub mod atestctl;
#[doc = "ADC Doubler Nanoamp Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcdoublernanoampctl](adcdoublernanoampctl) module"]
pub type ADCDOUBLERNANOAMPCTL = crate::Reg<u32, _ADCDOUBLERNANOAMPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCDOUBLERNANOAMPCTL;
#[doc = "`read()` method returns [adcdoublernanoampctl::R](adcdoublernanoampctl::R) reader structure"]
impl crate::Readable for ADCDOUBLERNANOAMPCTL {}
#[doc = "`write(|w| ..)` method takes [adcdoublernanoampctl::W](adcdoublernanoampctl::W) writer structure"]
impl crate::Writable for ADCDOUBLERNANOAMPCTL {}
#[doc = "ADC Doubler Nanoamp Control"]
pub mod adcdoublernanoampctl;
#[doc = "XOSCHF Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xoschfctl](xoschfctl) module"]
pub type XOSCHFCTL = crate::Reg<u32, _XOSCHFCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XOSCHFCTL;
#[doc = "`read()` method returns [xoschfctl::R](xoschfctl::R) reader structure"]
impl crate::Readable for XOSCHFCTL {}
#[doc = "`write(|w| ..)` method takes [xoschfctl::W](xoschfctl::W) writer structure"]
impl crate::Writable for XOSCHFCTL {}
#[doc = "XOSCHF Control"]
pub mod xoschfctl;
#[doc = "Low Frequency Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfoscctl](lfoscctl) module"]
pub type LFOSCCTL = crate::Reg<u32, _LFOSCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LFOSCCTL;
#[doc = "`read()` method returns [lfoscctl::R](lfoscctl::R) reader structure"]
impl crate::Readable for LFOSCCTL {}
#[doc = "`write(|w| ..)` method takes [lfoscctl::W](lfoscctl::W) writer structure"]
impl crate::Writable for LFOSCCTL {}
#[doc = "Low Frequency Oscillator Control"]
pub mod lfoscctl;
#[doc = "RCOSCHF Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcoschfctl](rcoschfctl) module"]
pub type RCOSCHFCTL = crate::Reg<u32, _RCOSCHFCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCOSCHFCTL;
#[doc = "`read()` method returns [rcoschfctl::R](rcoschfctl::R) reader structure"]
impl crate::Readable for RCOSCHFCTL {}
#[doc = "`write(|w| ..)` method takes [rcoschfctl::W](rcoschfctl::W) writer structure"]
impl crate::Writable for RCOSCHFCTL {}
#[doc = "RCOSCHF Control"]
pub mod rcoschfctl;
#[doc = "Status 0 This register contains status signals from OSC_DIG\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat0](stat0) module"]
pub type STAT0 = crate::Reg<u32, _STAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT0;
#[doc = "`read()` method returns [stat0::R](stat0::R) reader structure"]
impl crate::Readable for STAT0 {}
#[doc = "Status 0 This register contains status signals from OSC_DIG"]
pub mod stat0;
#[doc = "Status 1 This register contains status signals from OSC_DIG\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat1](stat1) module"]
pub type STAT1 = crate::Reg<u32, _STAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT1;
#[doc = "`read()` method returns [stat1::R](stat1::R) reader structure"]
impl crate::Readable for STAT1 {}
#[doc = "Status 1 This register contains status signals from OSC_DIG"]
pub mod stat1;
#[doc = "Status 2 This register contains status signals from AMPCOMP FSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat2](stat2) module"]
pub type STAT2 = crate::Reg<u32, _STAT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT2;
#[doc = "`read()` method returns [stat2::R](stat2::R) reader structure"]
impl crate::Readable for STAT2 {}
#[doc = "Status 2 This register contains status signals from AMPCOMP FSM"]
pub mod stat2;

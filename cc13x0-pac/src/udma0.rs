#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status"]
    pub status: STATUS,
    #[doc = "0x04 - Configuration"]
    pub cfg: CFG,
    #[doc = "0x08 - Channel Control Data Base Pointer"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Channel Alternate Control Data Base Pointer"]
    pub altctrl: ALTCTRL,
    #[doc = "0x10 - Channel Wait On Request Status"]
    pub waitonreq: WAITONREQ,
    #[doc = "0x14 - Channel Software Request"]
    pub softreq: SOFTREQ,
    #[doc = "0x18 - Channel Set UseBurst"]
    pub setburst: SETBURST,
    #[doc = "0x1c - Channel Clear UseBurst"]
    pub clearburst: CLEARBURST,
    #[doc = "0x20 - Channel Set Request Mask"]
    pub setreqmask: SETREQMASK,
    #[doc = "0x24 - Clear Channel Request Mask"]
    pub clearreqmask: CLEARREQMASK,
    #[doc = "0x28 - Set Channel Enable"]
    pub setchannelen: SETCHANNELEN,
    #[doc = "0x2c - Clear Channel Enable"]
    pub clearchannelen: CLEARCHANNELEN,
    #[doc = "0x30 - Channel Set Primary-Alternate"]
    pub setchnlprialt: SETCHNLPRIALT,
    #[doc = "0x34 - Channel Clear Primary-Alternate"]
    pub clearchnlprialt: CLEARCHNLPRIALT,
    #[doc = "0x38 - Set Channel Priority"]
    pub setchnlpriority: SETCHNLPRIORITY,
    #[doc = "0x3c - Clear Channel Priority"]
    pub clearchnlpriority: CLEARCHNLPRIORITY,
    _reserved16: [u8; 12usize],
    #[doc = "0x4c - Error Status and Clear"]
    pub error: ERROR,
    _reserved17: [u8; 1204usize],
    #[doc = "0x504 - Channel Request Done"]
    pub reqdone: REQDONE,
    _reserved18: [u8; 24usize],
    #[doc = "0x520 - Channel Request Done Mask"]
    pub donemask: DONEMASK,
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status"]
pub mod status;
#[doc = "Configuration\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configuration"]
pub mod cfg;
#[doc = "Channel Control Data Base Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Channel Control Data Base Pointer"]
pub mod ctrl;
#[doc = "Channel Alternate Control Data Base Pointer\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altctrl](altctrl) module"]
pub type ALTCTRL = crate::Reg<u32, _ALTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALTCTRL;
#[doc = "`read()` method returns [altctrl::R](altctrl::R) reader structure"]
impl crate::Readable for ALTCTRL {}
#[doc = "Channel Alternate Control Data Base Pointer"]
pub mod altctrl;
#[doc = "Channel Wait On Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [waitonreq](waitonreq) module"]
pub type WAITONREQ = crate::Reg<u32, _WAITONREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAITONREQ;
#[doc = "`read()` method returns [waitonreq::R](waitonreq::R) reader structure"]
impl crate::Readable for WAITONREQ {}
#[doc = "Channel Wait On Request Status"]
pub mod waitonreq;
#[doc = "Channel Software Request\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softreq](softreq) module"]
pub type SOFTREQ = crate::Reg<u32, _SOFTREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOFTREQ;
#[doc = "`write(|w| ..)` method takes [softreq::W](softreq::W) writer structure"]
impl crate::Writable for SOFTREQ {}
#[doc = "Channel Software Request"]
pub mod softreq;
#[doc = "Channel Set UseBurst\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setburst](setburst) module"]
pub type SETBURST = crate::Reg<u32, _SETBURST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETBURST;
#[doc = "`read()` method returns [setburst::R](setburst::R) reader structure"]
impl crate::Readable for SETBURST {}
#[doc = "`write(|w| ..)` method takes [setburst::W](setburst::W) writer structure"]
impl crate::Writable for SETBURST {}
#[doc = "Channel Set UseBurst"]
pub mod setburst;
#[doc = "Channel Clear UseBurst\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clearburst](clearburst) module"]
pub type CLEARBURST = crate::Reg<u32, _CLEARBURST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLEARBURST;
#[doc = "`write(|w| ..)` method takes [clearburst::W](clearburst::W) writer structure"]
impl crate::Writable for CLEARBURST {}
#[doc = "Channel Clear UseBurst"]
pub mod clearburst;
#[doc = "Channel Set Request Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setreqmask](setreqmask) module"]
pub type SETREQMASK = crate::Reg<u32, _SETREQMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETREQMASK;
#[doc = "`read()` method returns [setreqmask::R](setreqmask::R) reader structure"]
impl crate::Readable for SETREQMASK {}
#[doc = "`write(|w| ..)` method takes [setreqmask::W](setreqmask::W) writer structure"]
impl crate::Writable for SETREQMASK {}
#[doc = "Channel Set Request Mask"]
pub mod setreqmask;
#[doc = "Clear Channel Request Mask\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clearreqmask](clearreqmask) module"]
pub type CLEARREQMASK = crate::Reg<u32, _CLEARREQMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLEARREQMASK;
#[doc = "`write(|w| ..)` method takes [clearreqmask::W](clearreqmask::W) writer structure"]
impl crate::Writable for CLEARREQMASK {}
#[doc = "Clear Channel Request Mask"]
pub mod clearreqmask;
#[doc = "Set Channel Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setchannelen](setchannelen) module"]
pub type SETCHANNELEN = crate::Reg<u32, _SETCHANNELEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETCHANNELEN;
#[doc = "`read()` method returns [setchannelen::R](setchannelen::R) reader structure"]
impl crate::Readable for SETCHANNELEN {}
#[doc = "`write(|w| ..)` method takes [setchannelen::W](setchannelen::W) writer structure"]
impl crate::Writable for SETCHANNELEN {}
#[doc = "Set Channel Enable"]
pub mod setchannelen;
#[doc = "Clear Channel Enable\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clearchannelen](clearchannelen) module"]
pub type CLEARCHANNELEN = crate::Reg<u32, _CLEARCHANNELEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLEARCHANNELEN;
#[doc = "`write(|w| ..)` method takes [clearchannelen::W](clearchannelen::W) writer structure"]
impl crate::Writable for CLEARCHANNELEN {}
#[doc = "Clear Channel Enable"]
pub mod clearchannelen;
#[doc = "Channel Set Primary-Alternate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setchnlprialt](setchnlprialt) module"]
pub type SETCHNLPRIALT = crate::Reg<u32, _SETCHNLPRIALT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETCHNLPRIALT;
#[doc = "`read()` method returns [setchnlprialt::R](setchnlprialt::R) reader structure"]
impl crate::Readable for SETCHNLPRIALT {}
#[doc = "`write(|w| ..)` method takes [setchnlprialt::W](setchnlprialt::W) writer structure"]
impl crate::Writable for SETCHNLPRIALT {}
#[doc = "Channel Set Primary-Alternate"]
pub mod setchnlprialt;
#[doc = "Channel Clear Primary-Alternate\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clearchnlprialt](clearchnlprialt) module"]
pub type CLEARCHNLPRIALT = crate::Reg<u32, _CLEARCHNLPRIALT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLEARCHNLPRIALT;
#[doc = "`write(|w| ..)` method takes [clearchnlprialt::W](clearchnlprialt::W) writer structure"]
impl crate::Writable for CLEARCHNLPRIALT {}
#[doc = "Channel Clear Primary-Alternate"]
pub mod clearchnlprialt;
#[doc = "Set Channel Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setchnlpriority](setchnlpriority) module"]
pub type SETCHNLPRIORITY = crate::Reg<u32, _SETCHNLPRIORITY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETCHNLPRIORITY;
#[doc = "`read()` method returns [setchnlpriority::R](setchnlpriority::R) reader structure"]
impl crate::Readable for SETCHNLPRIORITY {}
#[doc = "`write(|w| ..)` method takes [setchnlpriority::W](setchnlpriority::W) writer structure"]
impl crate::Writable for SETCHNLPRIORITY {}
#[doc = "Set Channel Priority"]
pub mod setchnlpriority;
#[doc = "Clear Channel Priority\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clearchnlpriority](clearchnlpriority) module"]
pub type CLEARCHNLPRIORITY = crate::Reg<u32, _CLEARCHNLPRIORITY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLEARCHNLPRIORITY;
#[doc = "`write(|w| ..)` method takes [clearchnlpriority::W](clearchnlpriority::W) writer structure"]
impl crate::Writable for CLEARCHNLPRIORITY {}
#[doc = "Clear Channel Priority"]
pub mod clearchnlpriority;
#[doc = "Error Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error](error) module"]
pub type ERROR = crate::Reg<u32, _ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERROR;
#[doc = "`read()` method returns [error::R](error::R) reader structure"]
impl crate::Readable for ERROR {}
#[doc = "`write(|w| ..)` method takes [error::W](error::W) writer structure"]
impl crate::Writable for ERROR {}
#[doc = "Error Status and Clear"]
pub mod error;
#[doc = "Channel Request Done\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqdone](reqdone) module"]
pub type REQDONE = crate::Reg<u32, _REQDONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQDONE;
#[doc = "`read()` method returns [reqdone::R](reqdone::R) reader structure"]
impl crate::Readable for REQDONE {}
#[doc = "`write(|w| ..)` method takes [reqdone::W](reqdone::W) writer structure"]
impl crate::Writable for REQDONE {}
#[doc = "Channel Request Done"]
pub mod reqdone;
#[doc = "Channel Request Done Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [donemask](donemask) module"]
pub type DONEMASK = crate::Reg<u32, _DONEMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DONEMASK;
#[doc = "`read()` method returns [donemask::R](donemask::R) reader structure"]
impl crate::Readable for DONEMASK {}
#[doc = "`write(|w| ..)` method takes [donemask::W](donemask::W) writer structure"]
impl crate::Writable for DONEMASK {}
#[doc = "Channel Request Done Mask"]
pub mod donemask;

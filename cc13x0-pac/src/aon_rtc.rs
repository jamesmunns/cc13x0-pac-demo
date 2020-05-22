#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control This register contains various bitfields for configuration of RTC"]
    pub ctl: CTL,
    #[doc = "0x04 - Event Flags, RTC Status This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield."]
    pub evflags: EVFLAGS,
    #[doc = "0x08 - Second Counter Value, Integer Part"]
    pub sec: SEC,
    #[doc = "0x0c - Second Counter Value, Fractional Part"]
    pub subsec: SUBSEC,
    #[doc = "0x10 - Subseconds Increment Value added to SUBSEC.VALUE on every SCLK_LFclock cycle."]
    pub subsecinc: SUBSECINC,
    #[doc = "0x14 - Channel Configuration"]
    pub chctl: CHCTL,
    #[doc = "0x18 - Channel 0 Compare Value"]
    pub ch0cmp: CH0CMP,
    #[doc = "0x1c - Channel 1 Compare Value"]
    pub ch1cmp: CH1CMP,
    #[doc = "0x20 - Channel 2 Compare Value"]
    pub ch2cmp: CH2CMP,
    #[doc = "0x24 - Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\]
event."]
    pub ch2cmpinc: CH2CMPINC,
    #[doc = "0x28 - Channel 1 Capture Value If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL."]
    pub ch1capt: CH1CAPT,
    #[doc = "0x2c - AON Synchronization This register is used for synchronizing between MCU and entire AON domain."]
    pub sync: SYNC,
}
#[doc = "Control This register contains various bitfields for configuration of RTC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control This register contains various bitfields for configuration of RTC"]
pub mod ctl;
#[doc = "Event Flags, RTC Status This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evflags](evflags) module"]
pub type EVFLAGS = crate::Reg<u32, _EVFLAGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVFLAGS;
#[doc = "`read()` method returns [evflags::R](evflags::R) reader structure"]
impl crate::Readable for EVFLAGS {}
#[doc = "`write(|w| ..)` method takes [evflags::W](evflags::W) writer structure"]
impl crate::Writable for EVFLAGS {}
#[doc = "Event Flags, RTC Status This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield."]
pub mod evflags;
#[doc = "Second Counter Value, Integer Part\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec](sec) module"]
pub type SEC = crate::Reg<u32, _SEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC;
#[doc = "`read()` method returns [sec::R](sec::R) reader structure"]
impl crate::Readable for SEC {}
#[doc = "`write(|w| ..)` method takes [sec::W](sec::W) writer structure"]
impl crate::Writable for SEC {}
#[doc = "Second Counter Value, Integer Part"]
pub mod sec;
#[doc = "Second Counter Value, Fractional Part\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subsec](subsec) module"]
pub type SUBSEC = crate::Reg<u32, _SUBSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSEC;
#[doc = "`read()` method returns [subsec::R](subsec::R) reader structure"]
impl crate::Readable for SUBSEC {}
#[doc = "`write(|w| ..)` method takes [subsec::W](subsec::W) writer structure"]
impl crate::Writable for SUBSEC {}
#[doc = "Second Counter Value, Fractional Part"]
pub mod subsec;
#[doc = "Subseconds Increment Value added to SUBSEC.VALUE on every SCLK_LFclock cycle.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subsecinc](subsecinc) module"]
pub type SUBSECINC = crate::Reg<u32, _SUBSECINC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSECINC;
#[doc = "`read()` method returns [subsecinc::R](subsecinc::R) reader structure"]
impl crate::Readable for SUBSECINC {}
#[doc = "Subseconds Increment Value added to SUBSEC.VALUE on every SCLK_LFclock cycle."]
pub mod subsecinc;
#[doc = "Channel Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl](chctl) module"]
pub type CHCTL = crate::Reg<u32, _CHCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCTL;
#[doc = "`read()` method returns [chctl::R](chctl::R) reader structure"]
impl crate::Readable for CHCTL {}
#[doc = "`write(|w| ..)` method takes [chctl::W](chctl::W) writer structure"]
impl crate::Writable for CHCTL {}
#[doc = "Channel Configuration"]
pub mod chctl;
#[doc = "Channel 0 Compare Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0cmp](ch0cmp) module"]
pub type CH0CMP = crate::Reg<u32, _CH0CMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CMP;
#[doc = "`read()` method returns [ch0cmp::R](ch0cmp::R) reader structure"]
impl crate::Readable for CH0CMP {}
#[doc = "`write(|w| ..)` method takes [ch0cmp::W](ch0cmp::W) writer structure"]
impl crate::Writable for CH0CMP {}
#[doc = "Channel 0 Compare Value"]
pub mod ch0cmp;
#[doc = "Channel 1 Compare Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1cmp](ch1cmp) module"]
pub type CH1CMP = crate::Reg<u32, _CH1CMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CMP;
#[doc = "`read()` method returns [ch1cmp::R](ch1cmp::R) reader structure"]
impl crate::Readable for CH1CMP {}
#[doc = "`write(|w| ..)` method takes [ch1cmp::W](ch1cmp::W) writer structure"]
impl crate::Writable for CH1CMP {}
#[doc = "Channel 1 Compare Value"]
pub mod ch1cmp;
#[doc = "Channel 2 Compare Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cmp](ch2cmp) module"]
pub type CH2CMP = crate::Reg<u32, _CH2CMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CMP;
#[doc = "`read()` method returns [ch2cmp::R](ch2cmp::R) reader structure"]
impl crate::Readable for CH2CMP {}
#[doc = "`write(|w| ..)` method takes [ch2cmp::W](ch2cmp::W) writer structure"]
impl crate::Writable for CH2CMP {}
#[doc = "Channel 2 Compare Value"]
pub mod ch2cmp;
#[doc = "Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\]
event.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cmpinc](ch2cmpinc) module"]
pub type CH2CMPINC = crate::Reg<u32, _CH2CMPINC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CMPINC;
#[doc = "`read()` method returns [ch2cmpinc::R](ch2cmpinc::R) reader structure"]
impl crate::Readable for CH2CMPINC {}
#[doc = "`write(|w| ..)` method takes [ch2cmpinc::W](ch2cmpinc::W) writer structure"]
impl crate::Writable for CH2CMPINC {}
#[doc = "Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\]
event."]
pub mod ch2cmpinc;
#[doc = "Channel 1 Capture Value If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1capt](ch1capt) module"]
pub type CH1CAPT = crate::Reg<u32, _CH1CAPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CAPT;
#[doc = "`read()` method returns [ch1capt::R](ch1capt::R) reader structure"]
impl crate::Readable for CH1CAPT {}
#[doc = "Channel 1 Capture Value If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL."]
pub mod ch1capt;
#[doc = "AON Synchronization This register is used for synchronizing between MCU and entire AON domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync](sync) module"]
pub type SYNC = crate::Reg<u32, _SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNC;
#[doc = "`read()` method returns [sync::R](sync::R) reader structure"]
impl crate::Readable for SYNC {}
#[doc = "`write(|w| ..)` method takes [sync::W](sync::W) writer structure"]
impl crate::Writable for SYNC {}
#[doc = "AON Synchronization This register is used for synchronizing between MCU and entire AON domain."]
pub mod sync;

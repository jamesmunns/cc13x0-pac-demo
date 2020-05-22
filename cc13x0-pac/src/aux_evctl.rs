#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Vector Configuration 0 AUX_SCE event vectors 0 and 1 configuration"]
    pub veccfg0: VECCFG0,
    #[doc = "0x04 - Vector Configuration 1 AUX_SCE event vectors 2 and 3 configuration"]
    pub veccfg1: VECCFG1,
    #[doc = "0x08 - Sensor Controller Engine Wait Event Selection Event selection for the AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions"]
    pub scewevsel: SCEWEVSEL,
    #[doc = "0x0c - Events To AON Domain Flags AUX event flags going to/through the AON domain These events may be used to wake up the MCU domain. The flags may be cleared by writing 0 to these bits or writing 1 to the corresponding bits in EVTOAONFLAGSCLR."]
    pub evtoaonflags: EVTOAONFLAGS,
    #[doc = "0x10 - Events To AON Domain Polarity AUX event source polarity for the event flags going to/through the AON domain Note the inverse polarity (0 = high, 1 = low)."]
    pub evtoaonpol: EVTOAONPOL,
    #[doc = "0x14 - Direct Memory Access Control"]
    pub dmactl: DMACTL,
    #[doc = "0x18 - Software Event Set Strobes for setting software events from the AUX domain to the AON/MCU Domains The use of these events is software-defined."]
    pub swevset: SWEVSET,
    #[doc = "0x1c - Event Status 0 Current event source levels, 15:0"]
    pub evstat0: EVSTAT0,
    #[doc = "0x20 - Event Status 1 Current event source levels, 31:16"]
    pub evstat1: EVSTAT1,
    #[doc = "0x24 - Event To MCU Domain Polarity AUX event source polarity for the event flags to the MCU domain Note the inverse polarity (0 = high, 1 = low)."]
    pub evtomcupol: EVTOMCUPOL,
    #[doc = "0x28 - Events to MCU Domain Flags AUX event flags going to the MCU domain The flags may be cleared by writing 0 to these bits or writing 1 to the corresponding bits in EVTOMCUFLAGSCLR."]
    pub evtomcuflags: EVTOMCUFLAGS,
    #[doc = "0x2c - Combined Event To MCU Domain Mask Selects which of the flags In EVTOMCUFLAGS that contribute to the AUX_COMB event to the MCU domain The AUX_COMB event is asserted as long as one or more of the included event flags are set."]
    pub combevtomcumask: COMBEVTOMCUMASK,
    _reserved12: [u8; 4usize],
    #[doc = "0x34 - Vector Flags If a vector flag has been set and AUX_SCE is sleeping, it will wake up and execute the vector. If multiple vectors have been set, the one with the lowest index will execute first, and the next after returning to sleep. During execution of a vector, the flag must be cleared, by writing a 1 to the corresponding bit in VECFLAGSCLR."]
    pub vecflags: VECFLAGS,
    #[doc = "0x38 - Events To MCU Domain Flags Clear Strobes for clearing flags in EVTOMCUFLAGS."]
    pub evtomcuflagsclr: EVTOMCUFLAGSCLR,
    #[doc = "0x3c - Events To AON Domain Clear Strobes for clearing flags in EVTOAONFLAGS."]
    pub evtoaonflagsclr: EVTOAONFLAGSCLR,
    #[doc = "0x40 - Vector Flags Clear Strobes for clearing flags in VECFLAGS."]
    pub vecflagsclr: VECFLAGSCLR,
}
#[doc = "Vector Configuration 0 AUX_SCE event vectors 0 and 1 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [veccfg0](veccfg0) module"]
pub type VECCFG0 = crate::Reg<u32, _VECCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VECCFG0;
#[doc = "`read()` method returns [veccfg0::R](veccfg0::R) reader structure"]
impl crate::Readable for VECCFG0 {}
#[doc = "`write(|w| ..)` method takes [veccfg0::W](veccfg0::W) writer structure"]
impl crate::Writable for VECCFG0 {}
#[doc = "Vector Configuration 0 AUX_SCE event vectors 0 and 1 configuration"]
pub mod veccfg0;
#[doc = "Vector Configuration 1 AUX_SCE event vectors 2 and 3 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [veccfg1](veccfg1) module"]
pub type VECCFG1 = crate::Reg<u32, _VECCFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VECCFG1;
#[doc = "`read()` method returns [veccfg1::R](veccfg1::R) reader structure"]
impl crate::Readable for VECCFG1 {}
#[doc = "`write(|w| ..)` method takes [veccfg1::W](veccfg1::W) writer structure"]
impl crate::Writable for VECCFG1 {}
#[doc = "Vector Configuration 1 AUX_SCE event vectors 2 and 3 configuration"]
pub mod veccfg1;
#[doc = "Sensor Controller Engine Wait Event Selection Event selection for the AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scewevsel](scewevsel) module"]
pub type SCEWEVSEL = crate::Reg<u32, _SCEWEVSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCEWEVSEL;
#[doc = "`read()` method returns [scewevsel::R](scewevsel::R) reader structure"]
impl crate::Readable for SCEWEVSEL {}
#[doc = "`write(|w| ..)` method takes [scewevsel::W](scewevsel::W) writer structure"]
impl crate::Writable for SCEWEVSEL {}
#[doc = "Sensor Controller Engine Wait Event Selection Event selection for the AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions"]
pub mod scewevsel;
#[doc = "Events To AON Domain Flags AUX event flags going to/through the AON domain These events may be used to wake up the MCU domain. The flags may be cleared by writing 0 to these bits or writing 1 to the corresponding bits in EVTOAONFLAGSCLR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtoaonflags](evtoaonflags) module"]
pub type EVTOAONFLAGS = crate::Reg<u32, _EVTOAONFLAGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTOAONFLAGS;
#[doc = "`read()` method returns [evtoaonflags::R](evtoaonflags::R) reader structure"]
impl crate::Readable for EVTOAONFLAGS {}
#[doc = "`write(|w| ..)` method takes [evtoaonflags::W](evtoaonflags::W) writer structure"]
impl crate::Writable for EVTOAONFLAGS {}
#[doc = "Events To AON Domain Flags AUX event flags going to/through the AON domain These events may be used to wake up the MCU domain. The flags may be cleared by writing 0 to these bits or writing 1 to the corresponding bits in EVTOAONFLAGSCLR."]
pub mod evtoaonflags;
#[doc = "Events To AON Domain Polarity AUX event source polarity for the event flags going to/through the AON domain Note the inverse polarity (0 = high, 1 = low).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtoaonpol](evtoaonpol) module"]
pub type EVTOAONPOL = crate::Reg<u32, _EVTOAONPOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTOAONPOL;
#[doc = "`read()` method returns [evtoaonpol::R](evtoaonpol::R) reader structure"]
impl crate::Readable for EVTOAONPOL {}
#[doc = "`write(|w| ..)` method takes [evtoaonpol::W](evtoaonpol::W) writer structure"]
impl crate::Writable for EVTOAONPOL {}
#[doc = "Events To AON Domain Polarity AUX event source polarity for the event flags going to/through the AON domain Note the inverse polarity (0 = high, 1 = low)."]
pub mod evtoaonpol;
#[doc = "Direct Memory Access Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl](dmactl) module"]
pub type DMACTL = crate::Reg<u32, _DMACTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL;
#[doc = "`read()` method returns [dmactl::R](dmactl::R) reader structure"]
impl crate::Readable for DMACTL {}
#[doc = "`write(|w| ..)` method takes [dmactl::W](dmactl::W) writer structure"]
impl crate::Writable for DMACTL {}
#[doc = "Direct Memory Access Control"]
pub mod dmactl;
#[doc = "Software Event Set Strobes for setting software events from the AUX domain to the AON/MCU Domains The use of these events is software-defined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swevset](swevset) module"]
pub type SWEVSET = crate::Reg<u32, _SWEVSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWEVSET;
#[doc = "`read()` method returns [swevset::R](swevset::R) reader structure"]
impl crate::Readable for SWEVSET {}
#[doc = "`write(|w| ..)` method takes [swevset::W](swevset::W) writer structure"]
impl crate::Writable for SWEVSET {}
#[doc = "Software Event Set Strobes for setting software events from the AUX domain to the AON/MCU Domains The use of these events is software-defined."]
pub mod swevset;
#[doc = "Event Status 0 Current event source levels, 15:0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat0](evstat0) module"]
pub type EVSTAT0 = crate::Reg<u32, _EVSTAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVSTAT0;
#[doc = "`read()` method returns [evstat0::R](evstat0::R) reader structure"]
impl crate::Readable for EVSTAT0 {}
#[doc = "Event Status 0 Current event source levels, 15:0"]
pub mod evstat0;
#[doc = "Event Status 1 Current event source levels, 31:16\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat1](evstat1) module"]
pub type EVSTAT1 = crate::Reg<u32, _EVSTAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVSTAT1;
#[doc = "`read()` method returns [evstat1::R](evstat1::R) reader structure"]
impl crate::Readable for EVSTAT1 {}
#[doc = "Event Status 1 Current event source levels, 31:16"]
pub mod evstat1;
#[doc = "Event To MCU Domain Polarity AUX event source polarity for the event flags to the MCU domain Note the inverse polarity (0 = high, 1 = low).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtomcupol](evtomcupol) module"]
pub type EVTOMCUPOL = crate::Reg<u32, _EVTOMCUPOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTOMCUPOL;
#[doc = "`read()` method returns [evtomcupol::R](evtomcupol::R) reader structure"]
impl crate::Readable for EVTOMCUPOL {}
#[doc = "`write(|w| ..)` method takes [evtomcupol::W](evtomcupol::W) writer structure"]
impl crate::Writable for EVTOMCUPOL {}
#[doc = "Event To MCU Domain Polarity AUX event source polarity for the event flags to the MCU domain Note the inverse polarity (0 = high, 1 = low)."]
pub mod evtomcupol;
#[doc = "Events to MCU Domain Flags AUX event flags going to the MCU domain The flags may be cleared by writing 0 to these bits or writing 1 to the corresponding bits in EVTOMCUFLAGSCLR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtomcuflags](evtomcuflags) module"]
pub type EVTOMCUFLAGS = crate::Reg<u32, _EVTOMCUFLAGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTOMCUFLAGS;
#[doc = "`read()` method returns [evtomcuflags::R](evtomcuflags::R) reader structure"]
impl crate::Readable for EVTOMCUFLAGS {}
#[doc = "`write(|w| ..)` method takes [evtomcuflags::W](evtomcuflags::W) writer structure"]
impl crate::Writable for EVTOMCUFLAGS {}
#[doc = "Events to MCU Domain Flags AUX event flags going to the MCU domain The flags may be cleared by writing 0 to these bits or writing 1 to the corresponding bits in EVTOMCUFLAGSCLR."]
pub mod evtomcuflags;
#[doc = "Combined Event To MCU Domain Mask Selects which of the flags In EVTOMCUFLAGS that contribute to the AUX_COMB event to the MCU domain The AUX_COMB event is asserted as long as one or more of the included event flags are set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [combevtomcumask](combevtomcumask) module"]
pub type COMBEVTOMCUMASK = crate::Reg<u32, _COMBEVTOMCUMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMBEVTOMCUMASK;
#[doc = "`read()` method returns [combevtomcumask::R](combevtomcumask::R) reader structure"]
impl crate::Readable for COMBEVTOMCUMASK {}
#[doc = "`write(|w| ..)` method takes [combevtomcumask::W](combevtomcumask::W) writer structure"]
impl crate::Writable for COMBEVTOMCUMASK {}
#[doc = "Combined Event To MCU Domain Mask Selects which of the flags In EVTOMCUFLAGS that contribute to the AUX_COMB event to the MCU domain The AUX_COMB event is asserted as long as one or more of the included event flags are set."]
pub mod combevtomcumask;
#[doc = "Vector Flags If a vector flag has been set and AUX_SCE is sleeping, it will wake up and execute the vector. If multiple vectors have been set, the one with the lowest index will execute first, and the next after returning to sleep. During execution of a vector, the flag must be cleared, by writing a 1 to the corresponding bit in VECFLAGSCLR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vecflags](vecflags) module"]
pub type VECFLAGS = crate::Reg<u32, _VECFLAGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VECFLAGS;
#[doc = "`read()` method returns [vecflags::R](vecflags::R) reader structure"]
impl crate::Readable for VECFLAGS {}
#[doc = "`write(|w| ..)` method takes [vecflags::W](vecflags::W) writer structure"]
impl crate::Writable for VECFLAGS {}
#[doc = "Vector Flags If a vector flag has been set and AUX_SCE is sleeping, it will wake up and execute the vector. If multiple vectors have been set, the one with the lowest index will execute first, and the next after returning to sleep. During execution of a vector, the flag must be cleared, by writing a 1 to the corresponding bit in VECFLAGSCLR."]
pub mod vecflags;
#[doc = "Events To MCU Domain Flags Clear Strobes for clearing flags in EVTOMCUFLAGS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtomcuflagsclr](evtomcuflagsclr) module"]
pub type EVTOMCUFLAGSCLR = crate::Reg<u32, _EVTOMCUFLAGSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTOMCUFLAGSCLR;
#[doc = "`read()` method returns [evtomcuflagsclr::R](evtomcuflagsclr::R) reader structure"]
impl crate::Readable for EVTOMCUFLAGSCLR {}
#[doc = "`write(|w| ..)` method takes [evtomcuflagsclr::W](evtomcuflagsclr::W) writer structure"]
impl crate::Writable for EVTOMCUFLAGSCLR {}
#[doc = "Events To MCU Domain Flags Clear Strobes for clearing flags in EVTOMCUFLAGS."]
pub mod evtomcuflagsclr;
#[doc = "Events To AON Domain Clear Strobes for clearing flags in EVTOAONFLAGS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtoaonflagsclr](evtoaonflagsclr) module"]
pub type EVTOAONFLAGSCLR = crate::Reg<u32, _EVTOAONFLAGSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTOAONFLAGSCLR;
#[doc = "`read()` method returns [evtoaonflagsclr::R](evtoaonflagsclr::R) reader structure"]
impl crate::Readable for EVTOAONFLAGSCLR {}
#[doc = "`write(|w| ..)` method takes [evtoaonflagsclr::W](evtoaonflagsclr::W) writer structure"]
impl crate::Writable for EVTOAONFLAGSCLR {}
#[doc = "Events To AON Domain Clear Strobes for clearing flags in EVTOAONFLAGS."]
pub mod evtoaonflagsclr;
#[doc = "Vector Flags Clear Strobes for clearing flags in VECFLAGS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vecflagsclr](vecflagsclr) module"]
pub type VECFLAGSCLR = crate::Reg<u32, _VECFLAGSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VECFLAGSCLR;
#[doc = "`read()` method returns [vecflagsclr::R](vecflagsclr::R) reader structure"]
impl crate::Readable for VECFLAGSCLR {}
#[doc = "`write(|w| ..)` method takes [vecflagsclr::W](vecflagsclr::W) writer structure"]
impl crate::Writable for VECFLAGSCLR {}
#[doc = "Vector Flags Clear Strobes for clearing flags in VECFLAGS."]
pub mod vecflagsclr;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Doorbell Command Register"]
    pub cmdr: CMDR,
    #[doc = "0x04 - Doorbell Command Status Register"]
    pub cmdsta: CMDSTA,
    #[doc = "0x08 - Interrupt Flags From RF Hardware Modules"]
    pub rfhwifg: RFHWIFG,
    #[doc = "0x0c - Interrupt Enable For RF Hardware Modules"]
    pub rfhwien: RFHWIEN,
    #[doc = "0x10 - Interrupt Flags For Command and Packet Engine Generated Interrupts"]
    pub rfcpeifg: RFCPEIFG,
    #[doc = "0x14 - Interrupt Enable For Command and Packet Engine Generated Interrupts"]
    pub rfcpeien: RFCPEIEN,
    #[doc = "0x18 - Interrupt Vector Selection For Command and Packet Engine Generated Interrupts"]
    pub rfcpeisl: RFCPEISL,
    #[doc = "0x1c - Doorbell Command Acknowledgement Interrupt Flag"]
    pub rfackifg: RFACKIFG,
    #[doc = "0x20 - RF Core General Purpose Output Control"]
    pub sysgpoctl: SYSGPOCTL,
}
#[doc = "Doorbell Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdr](cmdr) module"]
pub type CMDR = crate::Reg<u32, _CMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDR;
#[doc = "`read()` method returns [cmdr::R](cmdr::R) reader structure"]
impl crate::Readable for CMDR {}
#[doc = "`write(|w| ..)` method takes [cmdr::W](cmdr::W) writer structure"]
impl crate::Writable for CMDR {}
#[doc = "Doorbell Command Register"]
pub mod cmdr;
#[doc = "Doorbell Command Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdsta](cmdsta) module"]
pub type CMDSTA = crate::Reg<u32, _CMDSTA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDSTA;
#[doc = "`read()` method returns [cmdsta::R](cmdsta::R) reader structure"]
impl crate::Readable for CMDSTA {}
#[doc = "Doorbell Command Status Register"]
pub mod cmdsta;
#[doc = "Interrupt Flags From RF Hardware Modules\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfhwifg](rfhwifg) module"]
pub type RFHWIFG = crate::Reg<u32, _RFHWIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFHWIFG;
#[doc = "`read()` method returns [rfhwifg::R](rfhwifg::R) reader structure"]
impl crate::Readable for RFHWIFG {}
#[doc = "`write(|w| ..)` method takes [rfhwifg::W](rfhwifg::W) writer structure"]
impl crate::Writable for RFHWIFG {}
#[doc = "Interrupt Flags From RF Hardware Modules"]
pub mod rfhwifg;
#[doc = "Interrupt Enable For RF Hardware Modules\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfhwien](rfhwien) module"]
pub type RFHWIEN = crate::Reg<u32, _RFHWIEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFHWIEN;
#[doc = "`read()` method returns [rfhwien::R](rfhwien::R) reader structure"]
impl crate::Readable for RFHWIEN {}
#[doc = "`write(|w| ..)` method takes [rfhwien::W](rfhwien::W) writer structure"]
impl crate::Writable for RFHWIEN {}
#[doc = "Interrupt Enable For RF Hardware Modules"]
pub mod rfhwien;
#[doc = "Interrupt Flags For Command and Packet Engine Generated Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcpeifg](rfcpeifg) module"]
pub type RFCPEIFG = crate::Reg<u32, _RFCPEIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCPEIFG;
#[doc = "`read()` method returns [rfcpeifg::R](rfcpeifg::R) reader structure"]
impl crate::Readable for RFCPEIFG {}
#[doc = "`write(|w| ..)` method takes [rfcpeifg::W](rfcpeifg::W) writer structure"]
impl crate::Writable for RFCPEIFG {}
#[doc = "Interrupt Flags For Command and Packet Engine Generated Interrupts"]
pub mod rfcpeifg;
#[doc = "Interrupt Enable For Command and Packet Engine Generated Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcpeien](rfcpeien) module"]
pub type RFCPEIEN = crate::Reg<u32, _RFCPEIEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCPEIEN;
#[doc = "`read()` method returns [rfcpeien::R](rfcpeien::R) reader structure"]
impl crate::Readable for RFCPEIEN {}
#[doc = "`write(|w| ..)` method takes [rfcpeien::W](rfcpeien::W) writer structure"]
impl crate::Writable for RFCPEIEN {}
#[doc = "Interrupt Enable For Command and Packet Engine Generated Interrupts"]
pub mod rfcpeien;
#[doc = "Interrupt Vector Selection For Command and Packet Engine Generated Interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfcpeisl](rfcpeisl) module"]
pub type RFCPEISL = crate::Reg<u32, _RFCPEISL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCPEISL;
#[doc = "`read()` method returns [rfcpeisl::R](rfcpeisl::R) reader structure"]
impl crate::Readable for RFCPEISL {}
#[doc = "`write(|w| ..)` method takes [rfcpeisl::W](rfcpeisl::W) writer structure"]
impl crate::Writable for RFCPEISL {}
#[doc = "Interrupt Vector Selection For Command and Packet Engine Generated Interrupts"]
pub mod rfcpeisl;
#[doc = "Doorbell Command Acknowledgement Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfackifg](rfackifg) module"]
pub type RFACKIFG = crate::Reg<u32, _RFACKIFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFACKIFG;
#[doc = "`read()` method returns [rfackifg::R](rfackifg::R) reader structure"]
impl crate::Readable for RFACKIFG {}
#[doc = "`write(|w| ..)` method takes [rfackifg::W](rfackifg::W) writer structure"]
impl crate::Writable for RFACKIFG {}
#[doc = "Doorbell Command Acknowledgement Interrupt Flag"]
pub mod rfackifg;
#[doc = "RF Core General Purpose Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysgpoctl](sysgpoctl) module"]
pub type SYSGPOCTL = crate::Reg<u32, _SYSGPOCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSGPOCTL;
#[doc = "`read()` method returns [sysgpoctl::R](sysgpoctl::R) reader structure"]
impl crate::Readable for SYSGPOCTL {}
#[doc = "`write(|w| ..)` method takes [sysgpoctl::W](sysgpoctl::W) writer structure"]
impl crate::Writable for SYSGPOCTL {}
#[doc = "RF Core General Purpose Output Control"]
pub mod sysgpoctl;

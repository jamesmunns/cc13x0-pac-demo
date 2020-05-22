#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control 0"]
    pub cr0: CR0,
    #[doc = "0x04 - Control 1"]
    pub cr1: CR1,
    #[doc = "0x08 - Data 16-bits wide data register: When read, the entry in the receive FIFO, pointed to by the current FIFO read pointer, is accessed. As data values are removed by the receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current FIFO write pointer. When written, the entry in the transmit FIFO, pointed to by the write pointer, is written to. Data values are removed from the transmit FIFO one value at a time by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the TXD output pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits. Received data less than 16 bits is automatically right-justified in the receive buffer."]
    pub dr: DR,
    #[doc = "0x0c - Status"]
    pub sr: SR,
    #[doc = "0x10 - Clock Prescale"]
    pub cpsr: CPSR,
    #[doc = "0x14 - Interrupt Mask Set and Clear"]
    pub imsc: IMSC,
    #[doc = "0x18 - Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x1c - Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x20 - Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
    pub icr: ICR,
    #[doc = "0x24 - DMA Control"]
    pub dmacr: DMACR,
}
#[doc = "Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](cr0) module"]
pub type CR0 = crate::Reg<u32, _CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR0;
#[doc = "`read()` method returns [cr0::R](cr0::R) reader structure"]
impl crate::Readable for CR0 {}
#[doc = "`write(|w| ..)` method takes [cr0::W](cr0::W) writer structure"]
impl crate::Writable for CR0 {}
#[doc = "Control 0"]
pub mod cr0;
#[doc = "Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "Control 1"]
pub mod cr1;
#[doc = "Data 16-bits wide data register: When read, the entry in the receive FIFO, pointed to by the current FIFO read pointer, is accessed. As data values are removed by the receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current FIFO write pointer. When written, the entry in the transmit FIFO, pointed to by the write pointer, is written to. Data values are removed from the transmit FIFO one value at a time by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the TXD output pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits. Received data less than 16 bits is automatically right-justified in the receive buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "Data 16-bits wide data register: When read, the entry in the receive FIFO, pointed to by the current FIFO read pointer, is accessed. As data values are removed by the receive logic from the incoming data frame, they are placed into the entry in the receive FIFO, pointed to by the current FIFO write pointer. When written, the entry in the transmit FIFO, pointed to by the write pointer, is written to. Data values are removed from the transmit FIFO one value at a time by the transmit logic. It is loaded into the transmit serial shifter, then serially shifted out onto the TXD output pin at the programmed bit rate. When a data size of less than 16 bits is selected, the user must right-justify data written to the transmit FIFO. The transmit logic ignores the unused bits. Received data less than 16 bits is automatically right-justified in the receive buffer."]
pub mod dr;
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status"]
pub mod sr;
#[doc = "Clock Prescale\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpsr](cpsr) module"]
pub type CPSR = crate::Reg<u32, _CPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPSR;
#[doc = "`read()` method returns [cpsr::R](cpsr::R) reader structure"]
impl crate::Readable for CPSR {}
#[doc = "`write(|w| ..)` method takes [cpsr::W](cpsr::W) writer structure"]
impl crate::Writable for CPSR {}
#[doc = "Clock Prescale"]
pub mod cpsr;
#[doc = "Interrupt Mask Set and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imsc](imsc) module"]
pub type IMSC = crate::Reg<u32, _IMSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMSC;
#[doc = "`read()` method returns [imsc::R](imsc::R) reader structure"]
impl crate::Readable for IMSC {}
#[doc = "`write(|w| ..)` method takes [imsc::W](imsc::W) writer structure"]
impl crate::Writable for IMSC {}
#[doc = "Interrupt Mask Set and Clear"]
pub mod imsc;
#[doc = "Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "Raw Interrupt Status"]
pub mod ris;
#[doc = "Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](mis) module"]
pub type MIS = crate::Reg<u32, _MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIS;
#[doc = "`read()` method returns [mis::R](mis::R) reader structure"]
impl crate::Readable for MIS {}
#[doc = "Masked Interrupt Status"]
pub mod mis;
#[doc = "Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
pub mod icr;
#[doc = "DMA Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacr](dmacr) module"]
pub type DMACR = crate::Reg<u32, _DMACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACR;
#[doc = "`read()` method returns [dmacr::R](dmacr::R) reader structure"]
impl crate::Readable for DMACR {}
#[doc = "`write(|w| ..)` method takes [dmacr::W](dmacr::W) writer structure"]
impl crate::Writable for DMACR {}
#[doc = "DMA Control"]
pub mod dmacr;

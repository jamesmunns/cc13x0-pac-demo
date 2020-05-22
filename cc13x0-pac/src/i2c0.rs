#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Slave Own Address This register consists of seven address bits that identify this I2C device on the I2C bus."]
    pub soar: SOAR,
    _reserved_1_sctl: [u8; 4usize],
    #[doc = "0x08 - Slave Data This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state."]
    pub sdr: SDR,
    #[doc = "0x0c - Slave Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
    pub simr: SIMR,
    #[doc = "0x10 - Slave Raw Interrupt Status This register shows the unmasked interrupt status."]
    pub sris: SRIS,
    #[doc = "0x14 - Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR)."]
    pub smis: SMIS,
    #[doc = "0x18 - Slave Interrupt Clear This register clears the raw interrupt SRIS."]
    pub sicr: SICR,
    _reserved7: [u8; 2020usize],
    #[doc = "0x800 - Master Salve Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit."]
    pub msa: MSA,
    _reserved_8_mctrl: [u8; 4usize],
    #[doc = "0x808 - Master Data This register contains the data to be transmitted when in the Master Transmit state and the data received when in the Master Receive state."]
    pub mdr: MDR,
    #[doc = "0x80c - I2C Master Timer Period This register specifies the period of the SCL clock."]
    pub mtpr: MTPR,
    #[doc = "0x810 - Master Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
    pub mimr: MIMR,
    #[doc = "0x814 - Master Raw Interrupt Status This register show the unmasked interrupt status."]
    pub mris: MRIS,
    #[doc = "0x818 - Master Masked Interrupt Status This register show which interrupt is active (based on result from MRIS and MIMR)."]
    pub mmis: MMIS,
    #[doc = "0x81c - Master Interrupt Clear This register clears the raw and masked interrupt."]
    pub micr: MICR,
    #[doc = "0x820 - Master Configuration This register configures the mode (Master or Slave) and sets the interface for test mode loopback."]
    pub mcr: MCR,
}
impl RegisterBlock {
    #[doc = "0x04 - Slave Control Note: This register shares address with SSTAT, meaning that this register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub fn sctl(&self) -> &SCTL {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const SCTL) }
    }
    #[doc = "0x04 - Slave Control Note: This register shares address with SSTAT, meaning that this register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub fn sctl_mut(&self) -> &mut SCTL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut SCTL) }
    }
    #[doc = "0x04 - Slave Status Note: This register shares address with SCTL, meaning that this register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub fn sstat(&self) -> &SSTAT {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const SSTAT) }
    }
    #[doc = "0x04 - Slave Status Note: This register shares address with SCTL, meaning that this register functions as a control register when written, and a status register when read."]
    #[inline(always)]
    pub fn sstat_mut(&self) -> &mut SSTAT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut SSTAT) }
    }
    #[doc = "0x804 - Master Control This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller as stated in MSTAT. When written, the control register configures the I2C controller operation. To generate a single transmit cycle, the I2C Master Slave Address (MSA) register is written with the desired address, the MSA.RS bit is cleared, and this register is written with * ACK=X (0 or 1), * STOP=1, * START=1, * RUN=1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the MDR register."]
    #[inline(always)]
    pub fn mctrl(&self) -> &MCTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(2052usize) as *const MCTRL) }
    }
    #[doc = "0x804 - Master Control This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller as stated in MSTAT. When written, the control register configures the I2C controller operation. To generate a single transmit cycle, the I2C Master Slave Address (MSA) register is written with the desired address, the MSA.RS bit is cleared, and this register is written with * ACK=X (0 or 1), * STOP=1, * START=1, * RUN=1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the MDR register."]
    #[inline(always)]
    pub fn mctrl_mut(&self) -> &mut MCTRL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2052usize) as *mut MCTRL) }
    }
    #[doc = "0x804 - Master Status"]
    #[inline(always)]
    pub fn mstat(&self) -> &MSTAT {
        unsafe { &*(((self as *const Self) as *const u8).add(2052usize) as *const MSTAT) }
    }
    #[doc = "0x804 - Master Status"]
    #[inline(always)]
    pub fn mstat_mut(&self) -> &mut MSTAT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2052usize) as *mut MSTAT) }
    }
}
#[doc = "Slave Own Address This register consists of seven address bits that identify this I2C device on the I2C bus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soar](soar) module"]
pub type SOAR = crate::Reg<u32, _SOAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOAR;
#[doc = "`read()` method returns [soar::R](soar::R) reader structure"]
impl crate::Readable for SOAR {}
#[doc = "`write(|w| ..)` method takes [soar::W](soar::W) writer structure"]
impl crate::Writable for SOAR {}
#[doc = "Slave Own Address This register consists of seven address bits that identify this I2C device on the I2C bus."]
pub mod soar;
#[doc = "Slave Status Note: This register shares address with SCTL, meaning that this register functions as a control register when written, and a status register when read.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sstat](sstat) module"]
pub type SSTAT = crate::Reg<u32, _SSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSTAT;
#[doc = "`read()` method returns [sstat::R](sstat::R) reader structure"]
impl crate::Readable for SSTAT {}
#[doc = "Slave Status Note: This register shares address with SCTL, meaning that this register functions as a control register when written, and a status register when read."]
pub mod sstat;
#[doc = "Slave Control Note: This register shares address with SSTAT, meaning that this register functions as a control register when written, and a status register when read.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctl](sctl) module"]
pub type SCTL = crate::Reg<u32, _SCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCTL;
#[doc = "`write(|w| ..)` method takes [sctl::W](sctl::W) writer structure"]
impl crate::Writable for SCTL {}
#[doc = "Slave Control Note: This register shares address with SSTAT, meaning that this register functions as a control register when written, and a status register when read."]
pub mod sctl;
#[doc = "Slave Data This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdr](sdr) module"]
pub type SDR = crate::Reg<u32, _SDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDR;
#[doc = "`read()` method returns [sdr::R](sdr::R) reader structure"]
impl crate::Readable for SDR {}
#[doc = "`write(|w| ..)` method takes [sdr::W](sdr::W) writer structure"]
impl crate::Writable for SDR {}
#[doc = "Slave Data This register contains the data to be transmitted when in the Slave Transmit state, and the data received when in the Slave Receive state."]
pub mod sdr;
#[doc = "Slave Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [simr](simr) module"]
pub type SIMR = crate::Reg<u32, _SIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIMR;
#[doc = "`read()` method returns [simr::R](simr::R) reader structure"]
impl crate::Readable for SIMR {}
#[doc = "`write(|w| ..)` method takes [simr::W](simr::W) writer structure"]
impl crate::Writable for SIMR {}
#[doc = "Slave Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
pub mod simr;
#[doc = "Slave Raw Interrupt Status This register shows the unmasked interrupt status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sris](sris) module"]
pub type SRIS = crate::Reg<u32, _SRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRIS;
#[doc = "`read()` method returns [sris::R](sris::R) reader structure"]
impl crate::Readable for SRIS {}
#[doc = "Slave Raw Interrupt Status This register shows the unmasked interrupt status."]
pub mod sris;
#[doc = "Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smis](smis) module"]
pub type SMIS = crate::Reg<u32, _SMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMIS;
#[doc = "`read()` method returns [smis::R](smis::R) reader structure"]
impl crate::Readable for SMIS {}
#[doc = "Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR)."]
pub mod smis;
#[doc = "Slave Interrupt Clear This register clears the raw interrupt SRIS.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sicr](sicr) module"]
pub type SICR = crate::Reg<u32, _SICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SICR;
#[doc = "`write(|w| ..)` method takes [sicr::W](sicr::W) writer structure"]
impl crate::Writable for SICR {}
#[doc = "Slave Interrupt Clear This register clears the raw interrupt SRIS."]
pub mod sicr;
#[doc = "Master Salve Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msa](msa) module"]
pub type MSA = crate::Reg<u32, _MSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSA;
#[doc = "`read()` method returns [msa::R](msa::R) reader structure"]
impl crate::Readable for MSA {}
#[doc = "`write(|w| ..)` method takes [msa::W](msa::W) writer structure"]
impl crate::Writable for MSA {}
#[doc = "Master Salve Address This register contains seven address bits of the slave to be accessed by the master (a6-a0), and an RS bit determining if the next operation is a receive or transmit."]
pub mod msa;
#[doc = "Master Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstat](mstat) module"]
pub type MSTAT = crate::Reg<u32, _MSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSTAT;
#[doc = "`read()` method returns [mstat::R](mstat::R) reader structure"]
impl crate::Readable for MSTAT {}
#[doc = "Master Status"]
pub mod mstat;
#[doc = "Master Control This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller as stated in MSTAT. When written, the control register configures the I2C controller operation. To generate a single transmit cycle, the I2C Master Slave Address (MSA) register is written with the desired address, the MSA.RS bit is cleared, and this register is written with * ACK=X (0 or 1), * STOP=1, * START=1, * RUN=1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the MDR register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrl](mctrl) module"]
pub type MCTRL = crate::Reg<u32, _MCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTRL;
#[doc = "`write(|w| ..)` method takes [mctrl::W](mctrl::W) writer structure"]
impl crate::Writable for MCTRL {}
#[doc = "Master Control This register accesses status bits when read and control bits when written. When read, the status register indicates the state of the I2C bus controller as stated in MSTAT. When written, the control register configures the I2C controller operation. To generate a single transmit cycle, the I2C Master Slave Address (MSA) register is written with the desired address, the MSA.RS bit is cleared, and this register is written with * ACK=X (0 or 1), * STOP=1, * START=1, * RUN=1 to perform the operation and stop. When the operation is completed (or aborted due an error), an interrupt becomes active and the data may be read from the MDR register."]
pub mod mctrl;
#[doc = "Master Data This register contains the data to be transmitted when in the Master Transmit state and the data received when in the Master Receive state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdr](mdr) module"]
pub type MDR = crate::Reg<u32, _MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDR;
#[doc = "`read()` method returns [mdr::R](mdr::R) reader structure"]
impl crate::Readable for MDR {}
#[doc = "`write(|w| ..)` method takes [mdr::W](mdr::W) writer structure"]
impl crate::Writable for MDR {}
#[doc = "Master Data This register contains the data to be transmitted when in the Master Transmit state and the data received when in the Master Receive state."]
pub mod mdr;
#[doc = "I2C Master Timer Period This register specifies the period of the SCL clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtpr](mtpr) module"]
pub type MTPR = crate::Reg<u32, _MTPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTPR;
#[doc = "`read()` method returns [mtpr::R](mtpr::R) reader structure"]
impl crate::Readable for MTPR {}
#[doc = "`write(|w| ..)` method takes [mtpr::W](mtpr::W) writer structure"]
impl crate::Writable for MTPR {}
#[doc = "I2C Master Timer Period This register specifies the period of the SCL clock."]
pub mod mtpr;
#[doc = "Master Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mimr](mimr) module"]
pub type MIMR = crate::Reg<u32, _MIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIMR;
#[doc = "`read()` method returns [mimr::R](mimr::R) reader structure"]
impl crate::Readable for MIMR {}
#[doc = "`write(|w| ..)` method takes [mimr::W](mimr::W) writer structure"]
impl crate::Writable for MIMR {}
#[doc = "Master Interrupt Mask This register controls whether a raw interrupt is promoted to a controller interrupt."]
pub mod mimr;
#[doc = "Master Raw Interrupt Status This register show the unmasked interrupt status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mris](mris) module"]
pub type MRIS = crate::Reg<u32, _MRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRIS;
#[doc = "`read()` method returns [mris::R](mris::R) reader structure"]
impl crate::Readable for MRIS {}
#[doc = "Master Raw Interrupt Status This register show the unmasked interrupt status."]
pub mod mris;
#[doc = "Master Masked Interrupt Status This register show which interrupt is active (based on result from MRIS and MIMR).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmis](mmis) module"]
pub type MMIS = crate::Reg<u32, _MMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMIS;
#[doc = "`read()` method returns [mmis::R](mmis::R) reader structure"]
impl crate::Readable for MMIS {}
#[doc = "Master Masked Interrupt Status This register show which interrupt is active (based on result from MRIS and MIMR)."]
pub mod mmis;
#[doc = "Master Interrupt Clear This register clears the raw and masked interrupt.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [micr](micr) module"]
pub type MICR = crate::Reg<u32, _MICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MICR;
#[doc = "`write(|w| ..)` method takes [micr::W](micr::W) writer structure"]
impl crate::Writable for MICR {}
#[doc = "Master Interrupt Clear This register clears the raw and masked interrupt."]
pub mod micr;
#[doc = "Master Configuration This register configures the mode (Master or Slave) and sets the interface for test mode loopback.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Master Configuration This register configures the mode (Master or Slave) and sets the interface for test mode loopback."]
pub mod mcr;

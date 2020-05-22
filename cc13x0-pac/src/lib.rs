#![doc = "Peripheral access API for CC13X0 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn AON_GPIO_EDGE();
    fn I2C_IRQ();
    fn RFC_CPE_1();
    fn AON_RTC_COMB();
    fn UART0_COMB();
    fn AUX_SWEV0();
    fn SSI0_COMB();
    fn SSI1_COMB();
    fn RFC_CPE_0();
    fn RFC_HW_COMB();
    fn RFC_CMD_ACK();
    fn I2S_IRQ();
    fn AUX_SWEV1();
    fn WDT_IRQ();
    fn GPT0A();
    fn GPT0B();
    fn GPT1A();
    fn GPT1B();
    fn GPT2A();
    fn GPT2B();
    fn GPT3A();
    fn GPT3B();
    fn CRYPTO_RESULT_AVAIL_IRQ();
    fn DMA_DONE_COMB();
    fn DMA_ERR();
    fn FLASH();
    fn SWEV0();
    fn AUX_COMB();
    fn AON_PROG0();
    fn PROG0();
    fn AUX_COMPA();
    fn AUX_ADC_IRQ();
    fn TRNG_IRQ();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 133] = [
    Vector {
        _handler: AON_GPIO_EDGE,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2C_IRQ },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: RFC_CPE_1,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: AON_RTC_COMB,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: UART0_COMB,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: AUX_SWEV0,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: SSI0_COMB,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: SSI1_COMB,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: RFC_CPE_0,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: RFC_HW_COMB,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: RFC_CMD_ACK,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: I2S_IRQ },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: AUX_SWEV1,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: WDT_IRQ },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPT0A },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPT0B },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPT1A },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPT1B },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPT2A },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPT2B },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPT3A },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPT3B },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: CRYPTO_RESULT_AVAIL_IRQ,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: DMA_DONE_COMB,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: DMA_ERR },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FLASH },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SWEV0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: AUX_COMB },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: AON_PROG0,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PROG0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: AUX_COMPA,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: AUX_ADC_IRQ,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TRNG_IRQ },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - AON_GPIO_EDGE"]
    AON_GPIO_EDGE = 0,
    #[doc = "4 - I2C_IRQ"]
    I2C_IRQ = 4,
    #[doc = "8 - RFC_CPE_1"]
    RFC_CPE_1 = 8,
    #[doc = "16 - AON_RTC_COMB"]
    AON_RTC_COMB = 16,
    #[doc = "20 - UART0_COMB"]
    UART0_COMB = 20,
    #[doc = "24 - AUX_SWEV0"]
    AUX_SWEV0 = 24,
    #[doc = "28 - SSI0_COMB"]
    SSI0_COMB = 28,
    #[doc = "32 - SSI1_COMB"]
    SSI1_COMB = 32,
    #[doc = "36 - RFC_CPE_0"]
    RFC_CPE_0 = 36,
    #[doc = "40 - RFC_HW_COMB"]
    RFC_HW_COMB = 40,
    #[doc = "44 - RFC_CMD_ACK"]
    RFC_CMD_ACK = 44,
    #[doc = "48 - I2S_IRQ"]
    I2S_IRQ = 48,
    #[doc = "52 - AUX_SWEV1"]
    AUX_SWEV1 = 52,
    #[doc = "56 - WDT_IRQ"]
    WDT_IRQ = 56,
    #[doc = "60 - GPT0A"]
    GPT0A = 60,
    #[doc = "64 - GPT0B"]
    GPT0B = 64,
    #[doc = "68 - GPT1A"]
    GPT1A = 68,
    #[doc = "72 - GPT1B"]
    GPT1B = 72,
    #[doc = "76 - GPT2A"]
    GPT2A = 76,
    #[doc = "80 - GPT2B"]
    GPT2B = 80,
    #[doc = "84 - GPT3A"]
    GPT3A = 84,
    #[doc = "88 - GPT3B"]
    GPT3B = 88,
    #[doc = "92 - CRYPTO_RESULT_AVAIL_IRQ"]
    CRYPTO_RESULT_AVAIL_IRQ = 92,
    #[doc = "96 - DMA_DONE_COMB"]
    DMA_DONE_COMB = 96,
    #[doc = "100 - DMA_ERR"]
    DMA_ERR = 100,
    #[doc = "104 - FLASH"]
    FLASH = 104,
    #[doc = "108 - SWEV0"]
    SWEV0 = 108,
    #[doc = "112 - AUX_COMB"]
    AUX_COMB = 112,
    #[doc = "116 - AON_PROG0"]
    AON_PROG0 = 116,
    #[doc = "120 - PROG0"]
    PROG0 = 120,
    #[doc = "124 - AUX_COMPA"]
    AUX_COMPA = 124,
    #[doc = "128 - AUX_ADC_IRQ"]
    AUX_ADC_IRQ = 128,
    #[doc = "132 - TRNG_IRQ"]
    TRNG_IRQ = 132,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Always On (AON) Battery And Temperature MONitor (BATMON) residing in the AON domain Note: This module only supports 32 bit Read/Write access from MCU."]
pub struct AON_BATMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_BATMON {}
impl AON_BATMON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_batmon::RegisterBlock {
        0x4009_5000 as *const _
    }
}
impl Deref for AON_BATMON {
    type Target = aon_batmon::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_BATMON::ptr() }
    }
}
#[doc = "Always On (AON) Battery And Temperature MONitor (BATMON) residing in the AON domain Note: This module only supports 32 bit Read/Write access from MCU."]
pub mod aon_batmon;
#[doc = "This module configures the event fabric located in the AON domain. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub struct AON_EVENT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_EVENT {}
impl AON_EVENT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_event::RegisterBlock {
        0x4009_3000 as *const _
    }
}
impl Deref for AON_EVENT {
    type Target = aon_event::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_EVENT::ptr() }
    }
}
#[doc = "This module configures the event fabric located in the AON domain. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub mod aon_event;
#[doc = "Always On (AON) IO Controller - controls IO operation when the MCU IO Controller (IOC) is powered off and resides in the AON domain. Note: This module only supports 32 bit Read/Write access from MCU."]
pub struct AON_IOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_IOC {}
impl AON_IOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_ioc::RegisterBlock {
        0x4009_4000 as *const _
    }
}
impl Deref for AON_IOC {
    type Target = aon_ioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_IOC::ptr() }
    }
}
#[doc = "Always On (AON) IO Controller - controls IO operation when the MCU IO Controller (IOC) is powered off and resides in the AON domain. Note: This module only supports 32 bit Read/Write access from MCU."]
pub mod aon_ioc;
#[doc = "This component control the Real Time Clock residing in AON Note: This module is only supporting 32 bit ReadWrite access."]
pub struct AON_RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_RTC {}
impl AON_RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_rtc::RegisterBlock {
        0x4009_2000 as *const _
    }
}
impl Deref for AON_RTC {
    type Target = aon_rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_RTC::ptr() }
    }
}
#[doc = "This component control the Real Time Clock residing in AON Note: This module is only supporting 32 bit ReadWrite access."]
pub mod aon_rtc;
#[doc = "This component controls AON_SYSCTL, which is the device's system controller. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub struct AON_SYSCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_SYSCTL {}
impl AON_SYSCTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_sysctl::RegisterBlock {
        0x4009_0000 as *const _
    }
}
impl Deref for AON_SYSCTL {
    type Target = aon_sysctl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_SYSCTL::ptr() }
    }
}
#[doc = "This component controls AON_SYSCTL, which is the device's system controller. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub mod aon_sysctl;
#[doc = "This component control the Wakeup controller residing in the AON domain. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub struct AON_WUC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AON_WUC {}
impl AON_WUC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aon_wuc::RegisterBlock {
        0x4009_1000 as *const _
    }
}
impl Deref for AON_WUC {
    type Target = aon_wuc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AON_WUC::ptr() }
    }
}
#[doc = "This component control the Wakeup controller residing in the AON domain. Note: This module is only supporting 32 bit ReadWrite access from MCU"]
pub mod aon_wuc;
#[doc = "Configuration registers controlling analog peripherals of AUX. Registers Fields should be considered static unless otherwise noted (as dynamic)"]
pub struct AUX_ADI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_ADI4 {}
impl AUX_ADI4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_adi4::RegisterBlock {
        0x400c_b000 as *const _
    }
}
impl Deref for AUX_ADI4 {
    type Target = aux_adi4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_ADI4::ptr() }
    }
}
#[doc = "Configuration registers controlling analog peripherals of AUX. Registers Fields should be considered static unless otherwise noted (as dynamic)"]
pub mod aux_adi4;
#[doc = "AUX Analog/Digital Input Output Controller"]
pub struct AUX_AIODIO0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_AIODIO0 {}
impl AUX_AIODIO0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_aiodio0::RegisterBlock {
        0x400c_1000 as *const _
    }
}
impl Deref for AUX_AIODIO0 {
    type Target = aux_aiodio0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_AIODIO0::ptr() }
    }
}
#[doc = "AUX Analog/Digital Input Output Controller"]
pub mod aux_aiodio0;
#[doc = "AUX Analog/Digital Input Output Controller"]
pub struct AUX_AIODIO1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_AIODIO1 {}
impl AUX_AIODIO1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_aiodio1::RegisterBlock {
        0x400c_2000 as *const _
    }
}
impl Deref for AUX_AIODIO1 {
    type Target = aux_aiodio1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_AIODIO1::ptr() }
    }
}
#[doc = "AUX Analog/Digital Input Output Controller"]
pub mod aux_aiodio1;
#[doc = "AUX Analog Peripheral Control Module"]
pub struct AUX_ANAIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_ANAIF {}
impl AUX_ANAIF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_anaif::RegisterBlock {
        0x400c_9000 as *const _
    }
}
impl Deref for AUX_ANAIF {
    type Target = aux_anaif::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_ANAIF::ptr() }
    }
}
#[doc = "AUX Analog Peripheral Control Module"]
pub mod aux_anaif;
#[doc = "This is the DDI for the digital block that controls all the analog clock oscillators (OSC_DIG) and performs qualification of the clocks generated."]
pub struct AUX_DDI0_OSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_DDI0_OSC {}
impl AUX_DDI0_OSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_ddi0_osc::RegisterBlock {
        0x400c_a000 as *const _
    }
}
impl Deref for AUX_DDI0_OSC {
    type Target = aux_ddi0_osc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_DDI0_OSC::ptr() }
    }
}
#[doc = "This is the DDI for the digital block that controls all the analog clock oscillators (OSC_DIG) and performs qualification of the clocks generated."]
pub mod aux_ddi0_osc;
#[doc = "AUX Event Controller"]
pub struct AUX_EVCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_EVCTL {}
impl AUX_EVCTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_evctl::RegisterBlock {
        0x400c_5000 as *const _
    }
}
impl Deref for AUX_EVCTL {
    type Target = aux_evctl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_EVCTL::ptr() }
    }
}
#[doc = "AUX Event Controller"]
pub mod aux_evctl;
#[doc = "AUX Sensor Control Engine Control Module"]
pub struct AUX_SCE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_SCE {}
impl AUX_SCE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_sce::RegisterBlock {
        0x400e_1000 as *const _
    }
}
impl Deref for AUX_SCE {
    type Target = aux_sce::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_SCE::ptr() }
    }
}
#[doc = "AUX Sensor Control Engine Control Module"]
pub mod aux_sce;
#[doc = "AUX Semaphore Controller"]
pub struct AUX_SMPH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_SMPH {}
impl AUX_SMPH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_smph::RegisterBlock {
        0x400c_8000 as *const _
    }
}
impl Deref for AUX_SMPH {
    type Target = aux_smph::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_SMPH::ptr() }
    }
}
#[doc = "AUX Semaphore Controller"]
pub mod aux_smph;
#[doc = "AUX Time To Digital Converter"]
pub struct AUX_TDCIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_TDCIF {}
impl AUX_TDCIF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_tdcif::RegisterBlock {
        0x400c_4000 as *const _
    }
}
impl Deref for AUX_TDCIF {
    type Target = aux_tdcif::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_TDCIF::ptr() }
    }
}
#[doc = "AUX Time To Digital Converter"]
pub mod aux_tdcif;
#[doc = "AUX Timer"]
pub struct AUX_TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_TIMER {}
impl AUX_TIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_timer::RegisterBlock {
        0x400c_7000 as *const _
    }
}
impl Deref for AUX_TIMER {
    type Target = aux_timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_TIMER::ptr() }
    }
}
#[doc = "AUX Timer"]
pub mod aux_timer;
#[doc = "AUX Wake-up controller"]
pub struct AUX_WUC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AUX_WUC {}
impl AUX_WUC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aux_wuc::RegisterBlock {
        0x400c_6000 as *const _
    }
}
impl Deref for AUX_WUC {
    type Target = aux_wuc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AUX_WUC::ptr() }
    }
}
#[doc = "AUX Wake-up controller"]
pub mod aux_wuc;
#[doc = "Customer configuration area (CCFG)"]
pub struct CCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCFG {}
impl CCFG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccfg::RegisterBlock {
        0x5000_3000 as *const _
    }
}
impl Deref for CCFG {
    type Target = ccfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCFG::ptr() }
    }
}
#[doc = "Customer configuration area (CCFG)"]
pub mod ccfg;
#[doc = "Cortex-M's TI proprietary registers"]
pub struct CPU_TIPROP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU_TIPROP {}
impl CPU_TIPROP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpu_tiprop::RegisterBlock {
        0xe00f_e000 as *const _
    }
}
impl Deref for CPU_TIPROP {
    type Target = cpu_tiprop::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CPU_TIPROP::ptr() }
    }
}
#[doc = "Cortex-M's TI proprietary registers"]
pub mod cpu_tiprop;
#[doc = "Crypto core with DMA capability and local key storage"]
pub struct CRYPTO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYPTO {}
impl CRYPTO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crypto::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for CRYPTO {
    type Target = crypto::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRYPTO::ptr() }
    }
}
#[doc = "Crypto core with DMA capability and local key storage"]
pub mod crypto;
#[doc = "Event Fabric Component Definition"]
pub struct EVENT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EVENT {}
impl EVENT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const event::RegisterBlock {
        0x4008_3000 as *const _
    }
}
impl Deref for EVENT {
    type Target = event::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EVENT::ptr() }
    }
}
#[doc = "Event Fabric Component Definition"]
pub mod event;
#[doc = "Factory configuration area (FCFG1)"]
pub struct FCFG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FCFG1 {}
impl FCFG1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fcfg1::RegisterBlock {
        0x5000_1000 as *const _
    }
}
impl Deref for FCFG1 {
    type Target = fcfg1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FCFG1::ptr() }
    }
}
#[doc = "Factory configuration area (FCFG1)"]
pub mod fcfg1;
#[doc = "Flash sub-system registers, includes the Flash Memory Controller (FMC), flash read path, and an integrated Efuse controller and EFUSEROM."]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "Flash sub-system registers, includes the Flash Memory Controller (FMC), flash read path, and an integrated Efuse controller and EFUSEROM."]
pub mod flash;
#[doc = "MCU GPIO - I/F for controlling and reading IO status and IO event status"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0x4002_2000 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "MCU GPIO - I/F for controlling and reading IO status and IO event status"]
pub mod gpio;
#[doc = "General Purpose Timer."]
pub struct GPT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT0 {}
impl GPT0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpt0::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for GPT0 {
    type Target = gpt0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPT0::ptr() }
    }
}
#[doc = "General Purpose Timer."]
pub mod gpt0;
#[doc = "General Purpose Timer."]
pub struct GPT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT1 {}
impl GPT1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpt1::RegisterBlock {
        0x4001_1000 as *const _
    }
}
impl Deref for GPT1 {
    type Target = gpt1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPT1::ptr() }
    }
}
#[doc = "General Purpose Timer."]
pub mod gpt1;
#[doc = "General Purpose Timer."]
pub struct GPT2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT2 {}
impl GPT2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpt2::RegisterBlock {
        0x4001_2000 as *const _
    }
}
impl Deref for GPT2 {
    type Target = gpt2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPT2::ptr() }
    }
}
#[doc = "General Purpose Timer."]
pub mod gpt2;
#[doc = "General Purpose Timer."]
pub struct GPT3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPT3 {}
impl GPT3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpt3::RegisterBlock {
        0x4001_3000 as *const _
    }
}
impl Deref for GPT3 {
    type Target = gpt3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPT3::ptr() }
    }
}
#[doc = "General Purpose Timer."]
pub mod gpt3;
#[doc = "I2CMaster/Slave Serial Controler"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2CMaster/Slave Serial Controler"]
pub mod i2c0;
#[doc = "I2S Audio DMA module supporting formats I2S, LJF, RJF and DSP"]
pub struct I2S0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S0 {}
impl I2S0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0::RegisterBlock {
        0x4002_1000 as *const _
    }
}
impl Deref for I2S0 {
    type Target = i2s0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S0::ptr() }
    }
}
#[doc = "I2S Audio DMA module supporting formats I2S, LJF, RJF and DSP"]
pub mod i2s0;
#[doc = "IO Controller (IOC) - configures all the DIOs and resides in the MCU domain."]
pub struct IOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOC {}
impl IOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ioc::RegisterBlock {
        0x4008_1000 as *const _
    }
}
impl Deref for IOC {
    type Target = ioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IOC::ptr() }
    }
}
#[doc = "IO Controller (IOC) - configures all the DIOs and resides in the MCU domain."]
pub mod ioc;
#[doc = "Power, Reset and Clock Management"]
pub struct PRCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PRCM {}
impl PRCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const prcm::RegisterBlock {
        0x4008_2000 as *const _
    }
}
impl Deref for PRCM {
    type Target = prcm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PRCM::ptr() }
    }
}
#[doc = "Power, Reset and Clock Management"]
pub mod prcm;
#[doc = "RF Core Doorbell"]
pub struct RFC_DBELL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFC_DBELL {}
impl RFC_DBELL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rfc_dbell::RegisterBlock {
        0x4004_1000 as *const _
    }
}
impl Deref for RFC_DBELL {
    type Target = rfc_dbell::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RFC_DBELL::ptr() }
    }
}
#[doc = "RF Core Doorbell"]
pub mod rfc_dbell;
#[doc = "RF Core Power Management"]
pub struct RFC_PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFC_PWR {}
impl RFC_PWR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rfc_pwr::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for RFC_PWR {
    type Target = rfc_pwr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RFC_PWR::ptr() }
    }
}
#[doc = "RF Core Power Management"]
pub mod rfc_pwr;
#[doc = "RF Core Radio Timer"]
pub struct RFC_RAT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFC_RAT {}
impl RFC_RAT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rfc_rat::RegisterBlock {
        0x4004_3000 as *const _
    }
}
impl Deref for RFC_RAT {
    type Target = rfc_rat::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RFC_RAT::ptr() }
    }
}
#[doc = "RF Core Radio Timer"]
pub mod rfc_rat;
#[doc = "MCU Semaphore Module This module provides 32 binary semaphores. The state of a binary semaphore is either taken or available. A semaphore does not implement any ownership attribute. Still, a semaphore can be used to handle mutual exclusion scenarios."]
pub struct SMPH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMPH {}
impl SMPH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smph::RegisterBlock {
        0x4008_4000 as *const _
    }
}
impl Deref for SMPH {
    type Target = smph::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMPH::ptr() }
    }
}
#[doc = "MCU Semaphore Module This module provides 32 binary semaphores. The state of a binary semaphore is either taken or available. A semaphore does not implement any ownership attribute. Still, a semaphore can be used to handle mutual exclusion scenarios."]
pub mod smph;
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub struct SSI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI0 {}
impl SSI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi0::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for SSI0 {
    type Target = ssi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI0::ptr() }
    }
}
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub mod ssi0;
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub struct SSI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI1 {}
impl SSI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi1::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for SSI1 {
    type Target = ssi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI1::ptr() }
    }
}
#[doc = "Synchronous Serial Interface with master and slave capabilities"]
pub mod ssi1;
#[doc = "True Random Number Generator"]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const trng::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TRNG::ptr() }
    }
}
#[doc = "True Random Number Generator"]
pub mod trng;
#[doc = "Universal Asynchronous Receiver/Transmitter (UART) interface"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter (UART) interface"]
pub mod uart0;
#[doc = "ARM Micro Direct Memory Access Controller"]
pub struct UDMA0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UDMA0 {}
impl UDMA0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const udma0::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for UDMA0 {
    type Target = udma0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UDMA0::ptr() }
    }
}
#[doc = "ARM Micro Direct Memory Access Controller"]
pub mod udma0;
#[doc = "Versatile Instruction Memory System Controls memory access to the Flash and encapsulates the following instruction memories: - Boot ROM - Cache / GPRAM"]
pub struct VIMS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VIMS {}
impl VIMS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vims::RegisterBlock {
        0x4003_4000 as *const _
    }
}
impl Deref for VIMS {
    type Target = vims::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VIMS::ptr() }
    }
}
#[doc = "Versatile Instruction Memory System Controls memory access to the Flash and encapsulates the following instruction memories: - Boot ROM - Cache / GPRAM"]
pub mod vims;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x4008_0000 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AON_BATMON"]
    pub AON_BATMON: AON_BATMON,
    #[doc = "AON_EVENT"]
    pub AON_EVENT: AON_EVENT,
    #[doc = "AON_IOC"]
    pub AON_IOC: AON_IOC,
    #[doc = "AON_RTC"]
    pub AON_RTC: AON_RTC,
    #[doc = "AON_SYSCTL"]
    pub AON_SYSCTL: AON_SYSCTL,
    #[doc = "AON_WUC"]
    pub AON_WUC: AON_WUC,
    #[doc = "AUX_ADI4"]
    pub AUX_ADI4: AUX_ADI4,
    #[doc = "AUX_AIODIO0"]
    pub AUX_AIODIO0: AUX_AIODIO0,
    #[doc = "AUX_AIODIO1"]
    pub AUX_AIODIO1: AUX_AIODIO1,
    #[doc = "AUX_ANAIF"]
    pub AUX_ANAIF: AUX_ANAIF,
    #[doc = "AUX_DDI0_OSC"]
    pub AUX_DDI0_OSC: AUX_DDI0_OSC,
    #[doc = "AUX_EVCTL"]
    pub AUX_EVCTL: AUX_EVCTL,
    #[doc = "AUX_SCE"]
    pub AUX_SCE: AUX_SCE,
    #[doc = "AUX_SMPH"]
    pub AUX_SMPH: AUX_SMPH,
    #[doc = "AUX_TDCIF"]
    pub AUX_TDCIF: AUX_TDCIF,
    #[doc = "AUX_TIMER"]
    pub AUX_TIMER: AUX_TIMER,
    #[doc = "AUX_WUC"]
    pub AUX_WUC: AUX_WUC,
    #[doc = "CCFG"]
    pub CCFG: CCFG,
    #[doc = "CPU_TIPROP"]
    pub CPU_TIPROP: CPU_TIPROP,
    #[doc = "CRYPTO"]
    pub CRYPTO: CRYPTO,
    #[doc = "EVENT"]
    pub EVENT: EVENT,
    #[doc = "FCFG1"]
    pub FCFG1: FCFG1,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "GPT0"]
    pub GPT0: GPT0,
    #[doc = "GPT1"]
    pub GPT1: GPT1,
    #[doc = "GPT2"]
    pub GPT2: GPT2,
    #[doc = "GPT3"]
    pub GPT3: GPT3,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2S0"]
    pub I2S0: I2S0,
    #[doc = "IOC"]
    pub IOC: IOC,
    #[doc = "PRCM"]
    pub PRCM: PRCM,
    #[doc = "RFC_DBELL"]
    pub RFC_DBELL: RFC_DBELL,
    #[doc = "RFC_PWR"]
    pub RFC_PWR: RFC_PWR,
    #[doc = "RFC_RAT"]
    pub RFC_RAT: RFC_RAT,
    #[doc = "SMPH"]
    pub SMPH: SMPH,
    #[doc = "SSI0"]
    pub SSI0: SSI0,
    #[doc = "SSI1"]
    pub SSI1: SSI1,
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UDMA0"]
    pub UDMA0: UDMA0,
    #[doc = "VIMS"]
    pub VIMS: VIMS,
    #[doc = "WDT"]
    pub WDT: WDT,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AON_BATMON: AON_BATMON {
                _marker: PhantomData,
            },
            AON_EVENT: AON_EVENT {
                _marker: PhantomData,
            },
            AON_IOC: AON_IOC {
                _marker: PhantomData,
            },
            AON_RTC: AON_RTC {
                _marker: PhantomData,
            },
            AON_SYSCTL: AON_SYSCTL {
                _marker: PhantomData,
            },
            AON_WUC: AON_WUC {
                _marker: PhantomData,
            },
            AUX_ADI4: AUX_ADI4 {
                _marker: PhantomData,
            },
            AUX_AIODIO0: AUX_AIODIO0 {
                _marker: PhantomData,
            },
            AUX_AIODIO1: AUX_AIODIO1 {
                _marker: PhantomData,
            },
            AUX_ANAIF: AUX_ANAIF {
                _marker: PhantomData,
            },
            AUX_DDI0_OSC: AUX_DDI0_OSC {
                _marker: PhantomData,
            },
            AUX_EVCTL: AUX_EVCTL {
                _marker: PhantomData,
            },
            AUX_SCE: AUX_SCE {
                _marker: PhantomData,
            },
            AUX_SMPH: AUX_SMPH {
                _marker: PhantomData,
            },
            AUX_TDCIF: AUX_TDCIF {
                _marker: PhantomData,
            },
            AUX_TIMER: AUX_TIMER {
                _marker: PhantomData,
            },
            AUX_WUC: AUX_WUC {
                _marker: PhantomData,
            },
            CCFG: CCFG {
                _marker: PhantomData,
            },
            CPU_TIPROP: CPU_TIPROP {
                _marker: PhantomData,
            },
            CRYPTO: CRYPTO {
                _marker: PhantomData,
            },
            EVENT: EVENT {
                _marker: PhantomData,
            },
            FCFG1: FCFG1 {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            GPT0: GPT0 {
                _marker: PhantomData,
            },
            GPT1: GPT1 {
                _marker: PhantomData,
            },
            GPT2: GPT2 {
                _marker: PhantomData,
            },
            GPT3: GPT3 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2S0: I2S0 {
                _marker: PhantomData,
            },
            IOC: IOC {
                _marker: PhantomData,
            },
            PRCM: PRCM {
                _marker: PhantomData,
            },
            RFC_DBELL: RFC_DBELL {
                _marker: PhantomData,
            },
            RFC_PWR: RFC_PWR {
                _marker: PhantomData,
            },
            RFC_RAT: RFC_RAT {
                _marker: PhantomData,
            },
            SMPH: SMPH {
                _marker: PhantomData,
            },
            SSI0: SSI0 {
                _marker: PhantomData,
            },
            SSI1: SSI1 {
                _marker: PhantomData,
            },
            TRNG: TRNG {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UDMA0: UDMA0 {
                _marker: PhantomData,
            },
            VIMS: VIMS {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
        }
    }
}

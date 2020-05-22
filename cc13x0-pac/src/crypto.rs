#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Channel 0 Control"]
    pub dmach0ctl: DMACH0CTL,
    #[doc = "0x04 - DMA Channel 0 External Address"]
    pub dmach0extaddr: DMACH0EXTADDR,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - DMA Channel 0 Length"]
    pub dmach0len: DMACH0LEN,
    _reserved3: [u8; 8usize],
    #[doc = "0x18 - DMA Controller Status"]
    pub dmastat: DMASTAT,
    #[doc = "0x1c - DMA Controller Software Reset"]
    pub dmaswreset: DMASWRESET,
    #[doc = "0x20 - DMA Channel 1 Control"]
    pub dmach1ctl: DMACH1CTL,
    #[doc = "0x24 - DMA Channel 1 External Address"]
    pub dmach1extaddr: DMACH1EXTADDR,
    _reserved7: [u8; 4usize],
    #[doc = "0x2c - DMA Channel 1 Length"]
    pub dmach1len: DMACH1LEN,
    _reserved8: [u8; 72usize],
    #[doc = "0x78 - DMA Controller Master Configuration"]
    pub dmabuscfg: DMABUSCFG,
    #[doc = "0x7c - DMA Controller Port Error"]
    pub dmaporterr: DMAPORTERR,
    _reserved10: [u8; 124usize],
    #[doc = "0xfc - DMA Controller Version"]
    pub dmahwver: DMAHWVER,
    _reserved11: [u8; 768usize],
    #[doc = "0x400 - Key Write Area"]
    pub keywritearea: KEYWRITEAREA,
    #[doc = "0x404 - Key Written Area Status This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and will result in an error."]
    pub keywrittenarea: KEYWRITTENAREA,
    #[doc = "0x408 - Key Size This register defines the size of the keys that are written with DMA."]
    pub keysize: KEYSIZE,
    #[doc = "0x40c - Key Read Area"]
    pub keyreadarea: KEYREADAREA,
    _reserved15: [u8; 240usize],
    #[doc = "0x500 - Clear AES_KEY2/GHASH Key"]
    pub aeskey2: [AESKEY2; 4],
    #[doc = "0x510 - Clear AES_KEY3"]
    pub aeskey3: [AESKEY3; 4],
    _reserved17: [u8; 32usize],
    #[doc = "0x540 - AES Initialization Vector"]
    pub aesiv: [AESIV; 4],
    #[doc = "0x550 - AES Input/Output Buffer Control"]
    pub aesctl: AESCTL,
    #[doc = "0x554 - Crypto Data Length LSW"]
    pub aesdatalen0: AESDATALEN0,
    #[doc = "0x558 - Crypto Data Length MSW"]
    pub aesdatalen1: AESDATALEN1,
    #[doc = "0x55c - AES Authentication Length"]
    pub aesauthlen: AESAUTHLEN,
    _reserved_22_aesdatain0: [u8; 4usize],
    _reserved_23_aesdatain1: [u8; 4usize],
    _reserved_24_aesdatain2: [u8; 4usize],
    _reserved_25_aesdatain3: [u8; 4usize],
    #[doc = "0x570 - AES Tag Output"]
    pub aestagout: [AESTAGOUT; 4],
    _reserved27: [u8; 384usize],
    #[doc = "0x700 - Master Algorithm Select This register configures the internal destination of the DMA controller."]
    pub algsel: ALGSEL,
    #[doc = "0x704 - Master Protection Control"]
    pub dmaprotctl: DMAPROTCTL,
    _reserved29: [u8; 56usize],
    #[doc = "0x740 - Software Reset"]
    pub swreset: SWRESET,
    _reserved30: [u8; 60usize],
    #[doc = "0x780 - Interrupt Configuration"]
    pub irqtype: IRQTYPE,
    #[doc = "0x784 - Interrupt Enable"]
    pub irqen: IRQEN,
    #[doc = "0x788 - Interrupt Clear"]
    pub irqclr: IRQCLR,
    #[doc = "0x78c - Interrupt Set"]
    pub irqset: IRQSET,
    #[doc = "0x790 - Interrupt Status"]
    pub irqstat: IRQSTAT,
    _reserved35: [u8; 104usize],
    #[doc = "0x7fc - CTRL Module Version"]
    pub hwver: HWVER,
}
impl RegisterBlock {
    #[doc = "0x560 - AES Data Input/Output 0"]
    #[inline(always)]
    pub fn aesdatain0(&self) -> &AESDATAIN0 {
        unsafe { &*(((self as *const Self) as *const u8).add(1376usize) as *const AESDATAIN0) }
    }
    #[doc = "0x560 - AES Data Input/Output 0"]
    #[inline(always)]
    pub fn aesdatain0_mut(&self) -> &mut AESDATAIN0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1376usize) as *mut AESDATAIN0) }
    }
    #[doc = "0x560 - Data Input/Output"]
    #[inline(always)]
    pub fn aesdataout0(&self) -> &AESDATAOUT0 {
        unsafe { &*(((self as *const Self) as *const u8).add(1376usize) as *const AESDATAOUT0) }
    }
    #[doc = "0x560 - Data Input/Output"]
    #[inline(always)]
    pub fn aesdataout0_mut(&self) -> &mut AESDATAOUT0 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1376usize) as *mut AESDATAOUT0) }
    }
    #[doc = "0x564 - AES Data Input/Output 1"]
    #[inline(always)]
    pub fn aesdatain1(&self) -> &AESDATAIN1 {
        unsafe { &*(((self as *const Self) as *const u8).add(1380usize) as *const AESDATAIN1) }
    }
    #[doc = "0x564 - AES Data Input/Output 1"]
    #[inline(always)]
    pub fn aesdatain1_mut(&self) -> &mut AESDATAIN1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1380usize) as *mut AESDATAIN1) }
    }
    #[doc = "0x564 - AES Data Input/Output 3"]
    #[inline(always)]
    pub fn aesdataout1(&self) -> &AESDATAOUT1 {
        unsafe { &*(((self as *const Self) as *const u8).add(1380usize) as *const AESDATAOUT1) }
    }
    #[doc = "0x564 - AES Data Input/Output 3"]
    #[inline(always)]
    pub fn aesdataout1_mut(&self) -> &mut AESDATAOUT1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1380usize) as *mut AESDATAOUT1) }
    }
    #[doc = "0x568 - AES Data Input/Output 2"]
    #[inline(always)]
    pub fn aesdatain2(&self) -> &AESDATAIN2 {
        unsafe { &*(((self as *const Self) as *const u8).add(1384usize) as *const AESDATAIN2) }
    }
    #[doc = "0x568 - AES Data Input/Output 2"]
    #[inline(always)]
    pub fn aesdatain2_mut(&self) -> &mut AESDATAIN2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1384usize) as *mut AESDATAIN2) }
    }
    #[doc = "0x568 - AES Data Input/Output 2"]
    #[inline(always)]
    pub fn aesdataout2(&self) -> &AESDATAOUT2 {
        unsafe { &*(((self as *const Self) as *const u8).add(1384usize) as *const AESDATAOUT2) }
    }
    #[doc = "0x568 - AES Data Input/Output 2"]
    #[inline(always)]
    pub fn aesdataout2_mut(&self) -> &mut AESDATAOUT2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1384usize) as *mut AESDATAOUT2) }
    }
    #[doc = "0x56c - Data Input/Output"]
    #[inline(always)]
    pub fn aesdatain3(&self) -> &AESDATAIN3 {
        unsafe { &*(((self as *const Self) as *const u8).add(1388usize) as *const AESDATAIN3) }
    }
    #[doc = "0x56c - Data Input/Output"]
    #[inline(always)]
    pub fn aesdatain3_mut(&self) -> &mut AESDATAIN3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1388usize) as *mut AESDATAIN3) }
    }
    #[doc = "0x56c - AES Data Input/Output 3"]
    #[inline(always)]
    pub fn aesdataout3(&self) -> &AESDATAOUT3 {
        unsafe { &*(((self as *const Self) as *const u8).add(1388usize) as *const AESDATAOUT3) }
    }
    #[doc = "0x56c - AES Data Input/Output 3"]
    #[inline(always)]
    pub fn aesdataout3_mut(&self) -> &mut AESDATAOUT3 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1388usize) as *mut AESDATAOUT3) }
    }
}
#[doc = "DMA Channel 0 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach0ctl](dmach0ctl) module"]
pub type DMACH0CTL = crate::Reg<u32, _DMACH0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACH0CTL;
#[doc = "`read()` method returns [dmach0ctl::R](dmach0ctl::R) reader structure"]
impl crate::Readable for DMACH0CTL {}
#[doc = "`write(|w| ..)` method takes [dmach0ctl::W](dmach0ctl::W) writer structure"]
impl crate::Writable for DMACH0CTL {}
#[doc = "DMA Channel 0 Control"]
pub mod dmach0ctl;
#[doc = "DMA Channel 0 External Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach0extaddr](dmach0extaddr) module"]
pub type DMACH0EXTADDR = crate::Reg<u32, _DMACH0EXTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACH0EXTADDR;
#[doc = "`read()` method returns [dmach0extaddr::R](dmach0extaddr::R) reader structure"]
impl crate::Readable for DMACH0EXTADDR {}
#[doc = "`write(|w| ..)` method takes [dmach0extaddr::W](dmach0extaddr::W) writer structure"]
impl crate::Writable for DMACH0EXTADDR {}
#[doc = "DMA Channel 0 External Address"]
pub mod dmach0extaddr;
#[doc = "DMA Channel 0 Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach0len](dmach0len) module"]
pub type DMACH0LEN = crate::Reg<u32, _DMACH0LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACH0LEN;
#[doc = "`read()` method returns [dmach0len::R](dmach0len::R) reader structure"]
impl crate::Readable for DMACH0LEN {}
#[doc = "`write(|w| ..)` method takes [dmach0len::W](dmach0len::W) writer structure"]
impl crate::Writable for DMACH0LEN {}
#[doc = "DMA Channel 0 Length"]
pub mod dmach0len;
#[doc = "DMA Controller Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmastat](dmastat) module"]
pub type DMASTAT = crate::Reg<u32, _DMASTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASTAT;
#[doc = "`read()` method returns [dmastat::R](dmastat::R) reader structure"]
impl crate::Readable for DMASTAT {}
#[doc = "DMA Controller Status"]
pub mod dmastat;
#[doc = "DMA Controller Software Reset\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaswreset](dmaswreset) module"]
pub type DMASWRESET = crate::Reg<u32, _DMASWRESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASWRESET;
#[doc = "`write(|w| ..)` method takes [dmaswreset::W](dmaswreset::W) writer structure"]
impl crate::Writable for DMASWRESET {}
#[doc = "DMA Controller Software Reset"]
pub mod dmaswreset;
#[doc = "DMA Channel 1 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach1ctl](dmach1ctl) module"]
pub type DMACH1CTL = crate::Reg<u32, _DMACH1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACH1CTL;
#[doc = "`read()` method returns [dmach1ctl::R](dmach1ctl::R) reader structure"]
impl crate::Readable for DMACH1CTL {}
#[doc = "`write(|w| ..)` method takes [dmach1ctl::W](dmach1ctl::W) writer structure"]
impl crate::Writable for DMACH1CTL {}
#[doc = "DMA Channel 1 Control"]
pub mod dmach1ctl;
#[doc = "DMA Channel 1 External Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach1extaddr](dmach1extaddr) module"]
pub type DMACH1EXTADDR = crate::Reg<u32, _DMACH1EXTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACH1EXTADDR;
#[doc = "`read()` method returns [dmach1extaddr::R](dmach1extaddr::R) reader structure"]
impl crate::Readable for DMACH1EXTADDR {}
#[doc = "`write(|w| ..)` method takes [dmach1extaddr::W](dmach1extaddr::W) writer structure"]
impl crate::Writable for DMACH1EXTADDR {}
#[doc = "DMA Channel 1 External Address"]
pub mod dmach1extaddr;
#[doc = "DMA Channel 1 Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach1len](dmach1len) module"]
pub type DMACH1LEN = crate::Reg<u32, _DMACH1LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACH1LEN;
#[doc = "`read()` method returns [dmach1len::R](dmach1len::R) reader structure"]
impl crate::Readable for DMACH1LEN {}
#[doc = "`write(|w| ..)` method takes [dmach1len::W](dmach1len::W) writer structure"]
impl crate::Writable for DMACH1LEN {}
#[doc = "DMA Channel 1 Length"]
pub mod dmach1len;
#[doc = "DMA Controller Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmabuscfg](dmabuscfg) module"]
pub type DMABUSCFG = crate::Reg<u32, _DMABUSCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMABUSCFG;
#[doc = "`read()` method returns [dmabuscfg::R](dmabuscfg::R) reader structure"]
impl crate::Readable for DMABUSCFG {}
#[doc = "`write(|w| ..)` method takes [dmabuscfg::W](dmabuscfg::W) writer structure"]
impl crate::Writable for DMABUSCFG {}
#[doc = "DMA Controller Master Configuration"]
pub mod dmabuscfg;
#[doc = "DMA Controller Port Error\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaporterr](dmaporterr) module"]
pub type DMAPORTERR = crate::Reg<u32, _DMAPORTERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAPORTERR;
#[doc = "`read()` method returns [dmaporterr::R](dmaporterr::R) reader structure"]
impl crate::Readable for DMAPORTERR {}
#[doc = "DMA Controller Port Error"]
pub mod dmaporterr;
#[doc = "DMA Controller Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmahwver](dmahwver) module"]
pub type DMAHWVER = crate::Reg<u32, _DMAHWVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAHWVER;
#[doc = "`read()` method returns [dmahwver::R](dmahwver::R) reader structure"]
impl crate::Readable for DMAHWVER {}
#[doc = "DMA Controller Version"]
pub mod dmahwver;
#[doc = "Key Write Area\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keywritearea](keywritearea) module"]
pub type KEYWRITEAREA = crate::Reg<u32, _KEYWRITEAREA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYWRITEAREA;
#[doc = "`read()` method returns [keywritearea::R](keywritearea::R) reader structure"]
impl crate::Readable for KEYWRITEAREA {}
#[doc = "`write(|w| ..)` method takes [keywritearea::W](keywritearea::W) writer structure"]
impl crate::Writable for KEYWRITEAREA {}
#[doc = "Key Write Area"]
pub mod keywritearea;
#[doc = "Key Written Area Status This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and will result in an error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keywrittenarea](keywrittenarea) module"]
pub type KEYWRITTENAREA = crate::Reg<u32, _KEYWRITTENAREA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYWRITTENAREA;
#[doc = "`read()` method returns [keywrittenarea::R](keywrittenarea::R) reader structure"]
impl crate::Readable for KEYWRITTENAREA {}
#[doc = "`write(|w| ..)` method takes [keywrittenarea::W](keywrittenarea::W) writer structure"]
impl crate::Writable for KEYWRITTENAREA {}
#[doc = "Key Written Area Status This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and will result in an error."]
pub mod keywrittenarea;
#[doc = "Key Size This register defines the size of the keys that are written with DMA.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keysize](keysize) module"]
pub type KEYSIZE = crate::Reg<u32, _KEYSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYSIZE;
#[doc = "`read()` method returns [keysize::R](keysize::R) reader structure"]
impl crate::Readable for KEYSIZE {}
#[doc = "`write(|w| ..)` method takes [keysize::W](keysize::W) writer structure"]
impl crate::Writable for KEYSIZE {}
#[doc = "Key Size This register defines the size of the keys that are written with DMA."]
pub mod keysize;
#[doc = "Key Read Area\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyreadarea](keyreadarea) module"]
pub type KEYREADAREA = crate::Reg<u32, _KEYREADAREA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYREADAREA;
#[doc = "`read()` method returns [keyreadarea::R](keyreadarea::R) reader structure"]
impl crate::Readable for KEYREADAREA {}
#[doc = "`write(|w| ..)` method takes [keyreadarea::W](keyreadarea::W) writer structure"]
impl crate::Writable for KEYREADAREA {}
#[doc = "Key Read Area"]
pub mod keyreadarea;
#[doc = "Clear AES_KEY2/GHASH Key\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aeskey2](aeskey2) module"]
pub type AESKEY2 = crate::Reg<u32, _AESKEY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESKEY2;
#[doc = "`write(|w| ..)` method takes [aeskey2::W](aeskey2::W) writer structure"]
impl crate::Writable for AESKEY2 {}
#[doc = "Clear AES_KEY2/GHASH Key"]
pub mod aeskey2;
#[doc = "Clear AES_KEY3\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aeskey3](aeskey3) module"]
pub type AESKEY3 = crate::Reg<u32, _AESKEY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESKEY3;
#[doc = "`write(|w| ..)` method takes [aeskey3::W](aeskey3::W) writer structure"]
impl crate::Writable for AESKEY3 {}
#[doc = "Clear AES_KEY3"]
pub mod aeskey3;
#[doc = "AES Initialization Vector\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesiv](aesiv) module"]
pub type AESIV = crate::Reg<u32, _AESIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESIV;
#[doc = "`read()` method returns [aesiv::R](aesiv::R) reader structure"]
impl crate::Readable for AESIV {}
#[doc = "`write(|w| ..)` method takes [aesiv::W](aesiv::W) writer structure"]
impl crate::Writable for AESIV {}
#[doc = "AES Initialization Vector"]
pub mod aesiv;
#[doc = "AES Input/Output Buffer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesctl](aesctl) module"]
pub type AESCTL = crate::Reg<u32, _AESCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESCTL;
#[doc = "`read()` method returns [aesctl::R](aesctl::R) reader structure"]
impl crate::Readable for AESCTL {}
#[doc = "`write(|w| ..)` method takes [aesctl::W](aesctl::W) writer structure"]
impl crate::Writable for AESCTL {}
#[doc = "AES Input/Output Buffer Control"]
pub mod aesctl;
#[doc = "Crypto Data Length LSW\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatalen0](aesdatalen0) module"]
pub type AESDATALEN0 = crate::Reg<u32, _AESDATALEN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATALEN0;
#[doc = "`write(|w| ..)` method takes [aesdatalen0::W](aesdatalen0::W) writer structure"]
impl crate::Writable for AESDATALEN0 {}
#[doc = "Crypto Data Length LSW"]
pub mod aesdatalen0;
#[doc = "Crypto Data Length MSW\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatalen1](aesdatalen1) module"]
pub type AESDATALEN1 = crate::Reg<u32, _AESDATALEN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATALEN1;
#[doc = "`write(|w| ..)` method takes [aesdatalen1::W](aesdatalen1::W) writer structure"]
impl crate::Writable for AESDATALEN1 {}
#[doc = "Crypto Data Length MSW"]
pub mod aesdatalen1;
#[doc = "AES Authentication Length\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesauthlen](aesauthlen) module"]
pub type AESAUTHLEN = crate::Reg<u32, _AESAUTHLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESAUTHLEN;
#[doc = "`write(|w| ..)` method takes [aesauthlen::W](aesauthlen::W) writer structure"]
impl crate::Writable for AESAUTHLEN {}
#[doc = "AES Authentication Length"]
pub mod aesauthlen;
#[doc = "Data Input/Output\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdataout0](aesdataout0) module"]
pub type AESDATAOUT0 = crate::Reg<u32, _AESDATAOUT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAOUT0;
#[doc = "`read()` method returns [aesdataout0::R](aesdataout0::R) reader structure"]
impl crate::Readable for AESDATAOUT0 {}
#[doc = "Data Input/Output"]
pub mod aesdataout0;
#[doc = "AES Data Input/Output 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatain0](aesdatain0) module"]
pub type AESDATAIN0 = crate::Reg<u32, _AESDATAIN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAIN0;
#[doc = "`write(|w| ..)` method takes [aesdatain0::W](aesdatain0::W) writer structure"]
impl crate::Writable for AESDATAIN0 {}
#[doc = "AES Data Input/Output 0"]
pub mod aesdatain0;
#[doc = "AES Data Input/Output 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdataout1](aesdataout1) module"]
pub type AESDATAOUT1 = crate::Reg<u32, _AESDATAOUT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAOUT1;
#[doc = "`read()` method returns [aesdataout1::R](aesdataout1::R) reader structure"]
impl crate::Readable for AESDATAOUT1 {}
#[doc = "AES Data Input/Output 3"]
pub mod aesdataout1;
#[doc = "AES Data Input/Output 1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatain1](aesdatain1) module"]
pub type AESDATAIN1 = crate::Reg<u32, _AESDATAIN1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAIN1;
#[doc = "`write(|w| ..)` method takes [aesdatain1::W](aesdatain1::W) writer structure"]
impl crate::Writable for AESDATAIN1 {}
#[doc = "AES Data Input/Output 1"]
pub mod aesdatain1;
#[doc = "AES Data Input/Output 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdataout2](aesdataout2) module"]
pub type AESDATAOUT2 = crate::Reg<u32, _AESDATAOUT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAOUT2;
#[doc = "`read()` method returns [aesdataout2::R](aesdataout2::R) reader structure"]
impl crate::Readable for AESDATAOUT2 {}
#[doc = "AES Data Input/Output 2"]
pub mod aesdataout2;
#[doc = "AES Data Input/Output 2\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatain2](aesdatain2) module"]
pub type AESDATAIN2 = crate::Reg<u32, _AESDATAIN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAIN2;
#[doc = "`write(|w| ..)` method takes [aesdatain2::W](aesdatain2::W) writer structure"]
impl crate::Writable for AESDATAIN2 {}
#[doc = "AES Data Input/Output 2"]
pub mod aesdatain2;
#[doc = "AES Data Input/Output 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdataout3](aesdataout3) module"]
pub type AESDATAOUT3 = crate::Reg<u32, _AESDATAOUT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAOUT3;
#[doc = "`read()` method returns [aesdataout3::R](aesdataout3::R) reader structure"]
impl crate::Readable for AESDATAOUT3 {}
#[doc = "AES Data Input/Output 3"]
pub mod aesdataout3;
#[doc = "Data Input/Output\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesdatain3](aesdatain3) module"]
pub type AESDATAIN3 = crate::Reg<u32, _AESDATAIN3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESDATAIN3;
#[doc = "`write(|w| ..)` method takes [aesdatain3::W](aesdatain3::W) writer structure"]
impl crate::Writable for AESDATAIN3 {}
#[doc = "Data Input/Output"]
pub mod aesdatain3;
#[doc = "AES Tag Output\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aestagout](aestagout) module"]
pub type AESTAGOUT = crate::Reg<u32, _AESTAGOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AESTAGOUT;
#[doc = "`read()` method returns [aestagout::R](aestagout::R) reader structure"]
impl crate::Readable for AESTAGOUT {}
#[doc = "AES Tag Output"]
pub mod aestagout;
#[doc = "Master Algorithm Select This register configures the internal destination of the DMA controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [algsel](algsel) module"]
pub type ALGSEL = crate::Reg<u32, _ALGSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALGSEL;
#[doc = "`read()` method returns [algsel::R](algsel::R) reader structure"]
impl crate::Readable for ALGSEL {}
#[doc = "`write(|w| ..)` method takes [algsel::W](algsel::W) writer structure"]
impl crate::Writable for ALGSEL {}
#[doc = "Master Algorithm Select This register configures the internal destination of the DMA controller."]
pub mod algsel;
#[doc = "Master Protection Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaprotctl](dmaprotctl) module"]
pub type DMAPROTCTL = crate::Reg<u32, _DMAPROTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAPROTCTL;
#[doc = "`read()` method returns [dmaprotctl::R](dmaprotctl::R) reader structure"]
impl crate::Readable for DMAPROTCTL {}
#[doc = "`write(|w| ..)` method takes [dmaprotctl::W](dmaprotctl::W) writer structure"]
impl crate::Writable for DMAPROTCTL {}
#[doc = "Master Protection Control"]
pub mod dmaprotctl;
#[doc = "Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swreset](swreset) module"]
pub type SWRESET = crate::Reg<u32, _SWRESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWRESET;
#[doc = "`read()` method returns [swreset::R](swreset::R) reader structure"]
impl crate::Readable for SWRESET {}
#[doc = "`write(|w| ..)` method takes [swreset::W](swreset::W) writer structure"]
impl crate::Writable for SWRESET {}
#[doc = "Software Reset"]
pub mod swreset;
#[doc = "Interrupt Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqtype](irqtype) module"]
pub type IRQTYPE = crate::Reg<u32, _IRQTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQTYPE;
#[doc = "`read()` method returns [irqtype::R](irqtype::R) reader structure"]
impl crate::Readable for IRQTYPE {}
#[doc = "`write(|w| ..)` method takes [irqtype::W](irqtype::W) writer structure"]
impl crate::Writable for IRQTYPE {}
#[doc = "Interrupt Configuration"]
pub mod irqtype;
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqen](irqen) module"]
pub type IRQEN = crate::Reg<u32, _IRQEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQEN;
#[doc = "`read()` method returns [irqen::R](irqen::R) reader structure"]
impl crate::Readable for IRQEN {}
#[doc = "`write(|w| ..)` method takes [irqen::W](irqen::W) writer structure"]
impl crate::Writable for IRQEN {}
#[doc = "Interrupt Enable"]
pub mod irqen;
#[doc = "Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqclr](irqclr) module"]
pub type IRQCLR = crate::Reg<u32, _IRQCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQCLR;
#[doc = "`write(|w| ..)` method takes [irqclr::W](irqclr::W) writer structure"]
impl crate::Writable for IRQCLR {}
#[doc = "Interrupt Clear"]
pub mod irqclr;
#[doc = "Interrupt Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqset](irqset) module"]
pub type IRQSET = crate::Reg<u32, _IRQSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSET;
#[doc = "`write(|w| ..)` method takes [irqset::W](irqset::W) writer structure"]
impl crate::Writable for IRQSET {}
#[doc = "Interrupt Set"]
pub mod irqset;
#[doc = "Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqstat](irqstat) module"]
pub type IRQSTAT = crate::Reg<u32, _IRQSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQSTAT;
#[doc = "`read()` method returns [irqstat::R](irqstat::R) reader structure"]
impl crate::Readable for IRQSTAT {}
#[doc = "Interrupt Status"]
pub mod irqstat;
#[doc = "CTRL Module Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwver](hwver) module"]
pub type HWVER = crate::Reg<u32, _HWVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWVER;
#[doc = "`read()` method returns [hwver::R](hwver::R) reader structure"]
impl crate::Readable for HWVER {}
#[doc = "CTRL Module Version"]
pub mod hwver;

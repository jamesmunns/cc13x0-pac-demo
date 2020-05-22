#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub ctl: CTL,
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    pub meascfg: MEASCFG,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Internal. Only to be used through TI provided API."]
    pub tempp0: TEMPP0,
    #[doc = "0x10 - Internal. Only to be used through TI provided API."]
    pub tempp1: TEMPP1,
    #[doc = "0x14 - Internal. Only to be used through TI provided API."]
    pub tempp2: TEMPP2,
    #[doc = "0x18 - Internal. Only to be used through TI provided API."]
    pub batmonp0: BATMONP0,
    #[doc = "0x1c - Internal. Only to be used through TI provided API."]
    pub batmonp1: BATMONP1,
    #[doc = "0x20 - Internal. Only to be used through TI provided API."]
    pub iostrp0: IOSTRP0,
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    pub flashpumpp0: FLASHPUMPP0,
    #[doc = "0x28 - Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1"]
    pub bat: BAT,
    #[doc = "0x2c - Battery Update Indicates BAT Updates"]
    pub batupd: BATUPD,
    #[doc = "0x30 - Temperature Last Measured Temperature in Degrees Celsius This register may be read while TEMPUPD.STAT = 1."]
    pub temp: TEMP,
    #[doc = "0x34 - Temperature Update Indicates TEMP Updates"]
    pub tempupd: TEMPUPD,
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ctl;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [meascfg](meascfg) module"]
pub type MEASCFG = crate::Reg<u32, _MEASCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MEASCFG;
#[doc = "`read()` method returns [meascfg::R](meascfg::R) reader structure"]
impl crate::Readable for MEASCFG {}
#[doc = "`write(|w| ..)` method takes [meascfg::W](meascfg::W) writer structure"]
impl crate::Writable for MEASCFG {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod meascfg;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempp0](tempp0) module"]
pub type TEMPP0 = crate::Reg<u32, _TEMPP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPP0;
#[doc = "`read()` method returns [tempp0::R](tempp0::R) reader structure"]
impl crate::Readable for TEMPP0 {}
#[doc = "`write(|w| ..)` method takes [tempp0::W](tempp0::W) writer structure"]
impl crate::Writable for TEMPP0 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tempp0;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempp1](tempp1) module"]
pub type TEMPP1 = crate::Reg<u32, _TEMPP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPP1;
#[doc = "`read()` method returns [tempp1::R](tempp1::R) reader structure"]
impl crate::Readable for TEMPP1 {}
#[doc = "`write(|w| ..)` method takes [tempp1::W](tempp1::W) writer structure"]
impl crate::Writable for TEMPP1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tempp1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempp2](tempp2) module"]
pub type TEMPP2 = crate::Reg<u32, _TEMPP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPP2;
#[doc = "`read()` method returns [tempp2::R](tempp2::R) reader structure"]
impl crate::Readable for TEMPP2 {}
#[doc = "`write(|w| ..)` method takes [tempp2::W](tempp2::W) writer structure"]
impl crate::Writable for TEMPP2 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tempp2;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [batmonp0](batmonp0) module"]
pub type BATMONP0 = crate::Reg<u32, _BATMONP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BATMONP0;
#[doc = "`read()` method returns [batmonp0::R](batmonp0::R) reader structure"]
impl crate::Readable for BATMONP0 {}
#[doc = "`write(|w| ..)` method takes [batmonp0::W](batmonp0::W) writer structure"]
impl crate::Writable for BATMONP0 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod batmonp0;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [batmonp1](batmonp1) module"]
pub type BATMONP1 = crate::Reg<u32, _BATMONP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BATMONP1;
#[doc = "`read()` method returns [batmonp1::R](batmonp1::R) reader structure"]
impl crate::Readable for BATMONP1 {}
#[doc = "`write(|w| ..)` method takes [batmonp1::W](batmonp1::W) writer structure"]
impl crate::Writable for BATMONP1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod batmonp1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iostrp0](iostrp0) module"]
pub type IOSTRP0 = crate::Reg<u32, _IOSTRP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOSTRP0;
#[doc = "`read()` method returns [iostrp0::R](iostrp0::R) reader structure"]
impl crate::Readable for IOSTRP0 {}
#[doc = "`write(|w| ..)` method takes [iostrp0::W](iostrp0::W) writer structure"]
impl crate::Writable for IOSTRP0 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrp0;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashpumpp0](flashpumpp0) module"]
pub type FLASHPUMPP0 = crate::Reg<u32, _FLASHPUMPP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHPUMPP0;
#[doc = "`read()` method returns [flashpumpp0::R](flashpumpp0::R) reader structure"]
impl crate::Readable for FLASHPUMPP0 {}
#[doc = "`write(|w| ..)` method takes [flashpumpp0::W](flashpumpp0::W) writer structure"]
impl crate::Writable for FLASHPUMPP0 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flashpumpp0;
#[doc = "Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bat](bat) module"]
pub type BAT = crate::Reg<u32, _BAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAT;
#[doc = "`read()` method returns [bat::R](bat::R) reader structure"]
impl crate::Readable for BAT {}
#[doc = "Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1"]
pub mod bat;
#[doc = "Battery Update Indicates BAT Updates\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [batupd](batupd) module"]
pub type BATUPD = crate::Reg<u32, _BATUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BATUPD;
#[doc = "`read()` method returns [batupd::R](batupd::R) reader structure"]
impl crate::Readable for BATUPD {}
#[doc = "`write(|w| ..)` method takes [batupd::W](batupd::W) writer structure"]
impl crate::Writable for BATUPD {}
#[doc = "Battery Update Indicates BAT Updates"]
pub mod batupd;
#[doc = "Temperature Last Measured Temperature in Degrees Celsius This register may be read while TEMPUPD.STAT = 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp](temp) module"]
pub type TEMP = crate::Reg<u32, _TEMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMP;
#[doc = "`read()` method returns [temp::R](temp::R) reader structure"]
impl crate::Readable for TEMP {}
#[doc = "Temperature Last Measured Temperature in Degrees Celsius This register may be read while TEMPUPD.STAT = 1."]
pub mod temp;
#[doc = "Temperature Update Indicates TEMP Updates\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempupd](tempupd) module"]
pub type TEMPUPD = crate::Reg<u32, _TEMPUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMPUPD;
#[doc = "`read()` method returns [tempupd::R](tempupd::R) reader structure"]
impl crate::Readable for TEMPUPD {}
#[doc = "`write(|w| ..)` method takes [tempupd::W](tempupd::W) writer structure"]
impl crate::Writable for TEMPUPD {}
#[doc = "Temperature Update Indicates TEMP Updates"]
pub mod tempupd;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub mux0: MUX0,
    #[doc = "0x01 - Internal. Only to be used through TI provided API."]
    pub mux1: MUX1,
    #[doc = "0x02 - Internal. Only to be used through TI provided API."]
    pub mux2: MUX2,
    #[doc = "0x03 - Internal. Only to be used through TI provided API."]
    pub mux3: MUX3,
    #[doc = "0x04 - Current Source Strength and trim control for current source"]
    pub isrc: ISRC,
    #[doc = "0x05 - Comparator Control COMPA and COMPB comparators"]
    pub comp: COMP,
    _reserved6: [u8; 1usize],
    #[doc = "0x07 - Internal. Only to be used through TI provided API."]
    pub mux4: MUX4,
    #[doc = "0x08 - ADC Control 0"]
    pub adc0: ADC0,
    #[doc = "0x09 - ADC Control 1"]
    pub adc1: ADC1,
    #[doc = "0x0a - ADC Reference 0 Control reference used by the ADC"]
    pub adcref0: ADCREF0,
    #[doc = "0x0b - ADC Reference 1 Control reference used by the ADC"]
    pub adcref1: ADCREF1,
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux0](mux0) module"]
pub type MUX0 = crate::Reg<u8, _MUX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MUX0;
#[doc = "`read()` method returns [mux0::R](mux0::R) reader structure"]
impl crate::Readable for MUX0 {}
#[doc = "`write(|w| ..)` method takes [mux0::W](mux0::W) writer structure"]
impl crate::Writable for MUX0 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux0;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux1](mux1) module"]
pub type MUX1 = crate::Reg<u8, _MUX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MUX1;
#[doc = "`read()` method returns [mux1::R](mux1::R) reader structure"]
impl crate::Readable for MUX1 {}
#[doc = "`write(|w| ..)` method takes [mux1::W](mux1::W) writer structure"]
impl crate::Writable for MUX1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux2](mux2) module"]
pub type MUX2 = crate::Reg<u8, _MUX2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MUX2;
#[doc = "`read()` method returns [mux2::R](mux2::R) reader structure"]
impl crate::Readable for MUX2 {}
#[doc = "`write(|w| ..)` method takes [mux2::W](mux2::W) writer structure"]
impl crate::Writable for MUX2 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux2;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux3](mux3) module"]
pub type MUX3 = crate::Reg<u8, _MUX3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MUX3;
#[doc = "`read()` method returns [mux3::R](mux3::R) reader structure"]
impl crate::Readable for MUX3 {}
#[doc = "`write(|w| ..)` method takes [mux3::W](mux3::W) writer structure"]
impl crate::Writable for MUX3 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux3;
#[doc = "Current Source Strength and trim control for current source\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isrc](isrc) module"]
pub type ISRC = crate::Reg<u8, _ISRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISRC;
#[doc = "`read()` method returns [isrc::R](isrc::R) reader structure"]
impl crate::Readable for ISRC {}
#[doc = "`write(|w| ..)` method takes [isrc::W](isrc::W) writer structure"]
impl crate::Writable for ISRC {}
#[doc = "Current Source Strength and trim control for current source"]
pub mod isrc;
#[doc = "Comparator Control COMPA and COMPB comparators\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp](comp) module"]
pub type COMP = crate::Reg<u8, _COMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP;
#[doc = "`read()` method returns [comp::R](comp::R) reader structure"]
impl crate::Readable for COMP {}
#[doc = "`write(|w| ..)` method takes [comp::W](comp::W) writer structure"]
impl crate::Writable for COMP {}
#[doc = "Comparator Control COMPA and COMPB comparators"]
pub mod comp;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux4](mux4) module"]
pub type MUX4 = crate::Reg<u8, _MUX4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MUX4;
#[doc = "`read()` method returns [mux4::R](mux4::R) reader structure"]
impl crate::Readable for MUX4 {}
#[doc = "`write(|w| ..)` method takes [mux4::W](mux4::W) writer structure"]
impl crate::Writable for MUX4 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux4;
#[doc = "ADC Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0](adc0) module"]
pub type ADC0 = crate::Reg<u8, _ADC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC0;
#[doc = "`read()` method returns [adc0::R](adc0::R) reader structure"]
impl crate::Readable for ADC0 {}
#[doc = "`write(|w| ..)` method takes [adc0::W](adc0::W) writer structure"]
impl crate::Writable for ADC0 {}
#[doc = "ADC Control 0"]
pub mod adc0;
#[doc = "ADC Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1](adc1) module"]
pub type ADC1 = crate::Reg<u8, _ADC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1;
#[doc = "`read()` method returns [adc1::R](adc1::R) reader structure"]
impl crate::Readable for ADC1 {}
#[doc = "`write(|w| ..)` method takes [adc1::W](adc1::W) writer structure"]
impl crate::Writable for ADC1 {}
#[doc = "ADC Control 1"]
pub mod adc1;
#[doc = "ADC Reference 0 Control reference used by the ADC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcref0](adcref0) module"]
pub type ADCREF0 = crate::Reg<u8, _ADCREF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCREF0;
#[doc = "`read()` method returns [adcref0::R](adcref0::R) reader structure"]
impl crate::Readable for ADCREF0 {}
#[doc = "`write(|w| ..)` method takes [adcref0::W](adcref0::W) writer structure"]
impl crate::Writable for ADCREF0 {}
#[doc = "ADC Reference 0 Control reference used by the ADC"]
pub mod adcref0;
#[doc = "ADC Reference 1 Control reference used by the ADC\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcref1](adcref1) module"]
pub type ADCREF1 = crate::Reg<u8, _ADCREF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADCREF1;
#[doc = "`read()` method returns [adcref1::R](adcref1::R) reader structure"]
impl crate::Readable for ADCREF1 {}
#[doc = "`write(|w| ..)` method takes [adcref1::W](adcref1::W) writer structure"]
impl crate::Writable for ADCREF1 {}
#[doc = "ADC Reference 1 Control reference used by the ADC"]
pub mod adcref1;

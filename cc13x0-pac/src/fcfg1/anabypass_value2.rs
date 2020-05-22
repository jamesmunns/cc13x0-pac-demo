#[doc = "Reader of register ANABYPASS_VALUE2"]
pub type R = crate::R<u32, super::ANABYPASS_VALUE2>;
#[doc = "Reader of field `XOSC_HF_IBIASTHERM`"]
pub type XOSC_HF_IBIASTHERM_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - XOSC_HF_IBIASTHERM"]
    #[inline(always)]
    pub fn xosc_hf_ibiastherm(&self) -> XOSC_HF_IBIASTHERM_R {
        XOSC_HF_IBIASTHERM_R::new((self.bits & 0x3fff) as u16)
    }
}

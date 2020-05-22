#[doc = "Reader of register JTAGCFG"]
pub type R = crate::R<u32, super::JTAGCFG>;
#[doc = "Writer for register JTAGCFG"]
pub type W = crate::W<u32, super::JTAGCFG>;
#[doc = "Register JTAGCFG `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::JTAGCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `JTAG_PD_FORCE_ON`"]
pub type JTAG_PD_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JTAG_PD_FORCE_ON`"]
pub struct JTAG_PD_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_PD_FORCE_ON_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - JTAG_PD_FORCE_ON"]
    #[inline(always)]
    pub fn jtag_pd_force_on(&self) -> JTAG_PD_FORCE_ON_R {
        JTAG_PD_FORCE_ON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - JTAG_PD_FORCE_ON"]
    #[inline(always)]
    pub fn jtag_pd_force_on(&mut self) -> JTAG_PD_FORCE_ON_W {
        JTAG_PD_FORCE_ON_W { w: self }
    }
}

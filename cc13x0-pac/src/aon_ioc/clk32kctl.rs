#[doc = "Reader of register CLK32KCTL"]
pub type R = crate::R<u32, super::CLK32KCTL>;
#[doc = "Writer for register CLK32KCTL"]
pub type W = crate::W<u32, super::CLK32KCTL>;
#[doc = "Register CLK32KCTL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CLK32KCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `OE_N`"]
pub type OE_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OE_N`"]
pub struct OE_N_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_N_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - OE_N"]
    #[inline(always)]
    pub fn oe_n(&self) -> OE_N_R {
        OE_N_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OE_N"]
    #[inline(always)]
    pub fn oe_n(&mut self) -> OE_N_W {
        OE_N_W { w: self }
    }
}

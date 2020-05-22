#[doc = "Writer for register EFUSESTAT"]
pub type W = crate::W<u32, super::EFUSESTAT>;
#[doc = "Register EFUSESTAT `reset()`'s with value 0x01"]
impl crate::ResetValue for super::EFUSESTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Write proxy for field `RESETDONE`"]
pub struct RESETDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETDONE_W<'a> {
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
impl W {
    #[doc = "Bit 0 - RESETDONE"]
    #[inline(always)]
    pub fn resetdone(&mut self) -> RESETDONE_W {
        RESETDONE_W { w: self }
    }
}

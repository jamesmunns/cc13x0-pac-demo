#[doc = "Reader of register PRECTL"]
pub type R = crate::R<u32, super::PRECTL>;
#[doc = "Writer for register PRECTL"]
pub type W = crate::W<u32, super::PRECTL>;
#[doc = "Register PRECTL `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::PRECTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `RESET_N`"]
pub type RESET_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_N`"]
pub struct RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RATIO`"]
pub type RATIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RATIO`"]
pub struct RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RATIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - RESET_N"]
    #[inline(always)]
    pub fn reset_n(&self) -> RESET_N_R {
        RESET_N_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RATIO"]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - SRC"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - RESET_N"]
    #[inline(always)]
    pub fn reset_n(&mut self) -> RESET_N_W {
        RESET_N_W { w: self }
    }
    #[doc = "Bit 6 - RATIO"]
    #[inline(always)]
    pub fn ratio(&mut self) -> RATIO_W {
        RATIO_W { w: self }
    }
    #[doc = "Bits 0:4 - SRC"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
}

#[doc = "Writer for register SINGLEBIT"]
pub type W = crate::W<u32, super::SINGLEBIT>;
#[doc = "Register SINGLEBIT `reset()`'s with value 0"]
impl crate::ResetValue for super::SINGLEBIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `FROMN`"]
pub struct FROMN_W<'a> {
    w: &'a mut W,
}
impl<'a> FROMN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `FROM0`"]
pub struct FROM0_W<'a> {
    w: &'a mut W,
}
impl<'a> FROM0_W<'a> {
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
    #[doc = "Bits 1:31 - FROMN"]
    #[inline(always)]
    pub fn fromn(&mut self) -> FROMN_W {
        FROMN_W { w: self }
    }
    #[doc = "Bit 0 - FROM0"]
    #[inline(always)]
    pub fn from0(&mut self) -> FROM0_W {
        FROM0_W { w: self }
    }
}

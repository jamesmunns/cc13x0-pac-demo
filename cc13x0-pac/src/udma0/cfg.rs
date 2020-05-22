#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PRTOCTRL`"]
pub struct PRTOCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTOCTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `MASTERENABLE`"]
pub struct MASTERENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTERENABLE_W<'a> {
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
    #[doc = "Bits 5:7 - PRTOCTRL"]
    #[inline(always)]
    pub fn prtoctrl(&mut self) -> PRTOCTRL_W {
        PRTOCTRL_W { w: self }
    }
    #[doc = "Bit 0 - MASTERENABLE"]
    #[inline(always)]
    pub fn masterenable(&mut self) -> MASTERENABLE_W {
        MASTERENABLE_W { w: self }
    }
}

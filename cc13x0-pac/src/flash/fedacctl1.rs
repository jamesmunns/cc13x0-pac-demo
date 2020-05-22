#[doc = "Reader of register FEDACCTL1"]
pub type R = crate::R<u32, super::FEDACCTL1>;
#[doc = "Writer for register FEDACCTL1"]
pub type W = crate::W<u32, super::FEDACCTL1>;
#[doc = "Register FEDACCTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FEDACCTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SUSP_IGNR`"]
pub type SUSP_IGNR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSP_IGNR`"]
pub struct SUSP_IGNR_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_IGNR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - SUSP_IGNR"]
    #[inline(always)]
    pub fn susp_ignr(&self) -> SUSP_IGNR_R {
        SUSP_IGNR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - SUSP_IGNR"]
    #[inline(always)]
    pub fn susp_ignr(&mut self) -> SUSP_IGNR_W {
        SUSP_IGNR_W { w: self }
    }
}

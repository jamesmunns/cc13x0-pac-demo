#[doc = "Reader of register MTPR"]
pub type R = crate::R<u32, super::MTPR>;
#[doc = "Writer for register MTPR"]
pub type W = crate::W<u32, super::MTPR>;
#[doc = "Register MTPR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::MTPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `TPR_7`"]
pub type TPR_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPR_7`"]
pub struct TPR_7_W<'a> {
    w: &'a mut W,
}
impl<'a> TPR_7_W<'a> {
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
#[doc = "Reader of field `TPR`"]
pub type TPR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TPR`"]
pub struct TPR_W<'a> {
    w: &'a mut W,
}
impl<'a> TPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - TPR_7"]
    #[inline(always)]
    pub fn tpr_7(&self) -> TPR_7_R {
        TPR_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:6 - TPR"]
    #[inline(always)]
    pub fn tpr(&self) -> TPR_R {
        TPR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - TPR_7"]
    #[inline(always)]
    pub fn tpr_7(&mut self) -> TPR_7_W {
        TPR_7_W { w: self }
    }
    #[doc = "Bits 0:6 - TPR"]
    #[inline(always)]
    pub fn tpr(&mut self) -> TPR_W {
        TPR_W { w: self }
    }
}

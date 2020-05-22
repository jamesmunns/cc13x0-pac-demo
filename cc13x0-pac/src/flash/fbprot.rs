#[doc = "Reader of register FBPROT"]
pub type R = crate::R<u32, super::FBPROT>;
#[doc = "Writer for register FBPROT"]
pub type W = crate::W<u32, super::FBPROT>;
#[doc = "Register FBPROT `reset()`'s with value 0"]
impl crate::ResetValue for super::FBPROT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PROTL1DIS`"]
pub type PROTL1DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROTL1DIS`"]
pub struct PROTL1DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTL1DIS_W<'a> {
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
    #[doc = "Bit 0 - PROTL1DIS"]
    #[inline(always)]
    pub fn protl1dis(&self) -> PROTL1DIS_R {
        PROTL1DIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PROTL1DIS"]
    #[inline(always)]
    pub fn protl1dis(&mut self) -> PROTL1DIS_W {
        PROTL1DIS_W { w: self }
    }
}

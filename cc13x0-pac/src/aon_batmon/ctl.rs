#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALC_EN`"]
pub type CALC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALC_EN`"]
pub struct CALC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CALC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `MEAS_EN`"]
pub type MEAS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEAS_EN`"]
pub struct MEAS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEAS_EN_W<'a> {
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
    #[doc = "Bit 1 - CALC_EN"]
    #[inline(always)]
    pub fn calc_en(&self) -> CALC_EN_R {
        CALC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - MEAS_EN"]
    #[inline(always)]
    pub fn meas_en(&self) -> MEAS_EN_R {
        MEAS_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - CALC_EN"]
    #[inline(always)]
    pub fn calc_en(&mut self) -> CALC_EN_W {
        CALC_EN_W { w: self }
    }
    #[doc = "Bit 0 - MEAS_EN"]
    #[inline(always)]
    pub fn meas_en(&mut self) -> MEAS_EN_W {
        MEAS_EN_W { w: self }
    }
}

#[doc = "Reader of register CHCTL"]
pub type R = crate::R<u32, super::CHCTL>;
#[doc = "Writer for register CHCTL"]
pub type W = crate::W<u32, super::CHCTL>;
#[doc = "Register CHCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CHCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH2_CONT_EN`"]
pub type CH2_CONT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2_CONT_EN`"]
pub struct CH2_CONT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_CONT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CH2_EN`"]
pub type CH2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2_EN`"]
pub struct CH2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CH1_CAPT_EN`"]
pub type CH1_CAPT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1_CAPT_EN`"]
pub struct CH1_CAPT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_CAPT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CH1_EN`"]
pub type CH1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1_EN`"]
pub struct CH1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_EN_W<'a> {
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
#[doc = "Reader of field `CH0_EN`"]
pub type CH0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0_EN`"]
pub struct CH0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_EN_W<'a> {
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
    #[doc = "Bit 18 - CH2_CONT_EN"]
    #[inline(always)]
    pub fn ch2_cont_en(&self) -> CH2_CONT_EN_R {
        CH2_CONT_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CH2_EN"]
    #[inline(always)]
    pub fn ch2_en(&self) -> CH2_EN_R {
        CH2_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CH1_CAPT_EN"]
    #[inline(always)]
    pub fn ch1_capt_en(&self) -> CH1_CAPT_EN_R {
        CH1_CAPT_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CH1_EN"]
    #[inline(always)]
    pub fn ch1_en(&self) -> CH1_EN_R {
        CH1_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CH0_EN"]
    #[inline(always)]
    pub fn ch0_en(&self) -> CH0_EN_R {
        CH0_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - CH2_CONT_EN"]
    #[inline(always)]
    pub fn ch2_cont_en(&mut self) -> CH2_CONT_EN_W {
        CH2_CONT_EN_W { w: self }
    }
    #[doc = "Bit 16 - CH2_EN"]
    #[inline(always)]
    pub fn ch2_en(&mut self) -> CH2_EN_W {
        CH2_EN_W { w: self }
    }
    #[doc = "Bit 9 - CH1_CAPT_EN"]
    #[inline(always)]
    pub fn ch1_capt_en(&mut self) -> CH1_CAPT_EN_W {
        CH1_CAPT_EN_W { w: self }
    }
    #[doc = "Bit 8 - CH1_EN"]
    #[inline(always)]
    pub fn ch1_en(&mut self) -> CH1_EN_W {
        CH1_EN_W { w: self }
    }
    #[doc = "Bit 0 - CH0_EN"]
    #[inline(always)]
    pub fn ch0_en(&mut self) -> CH0_EN_W {
        CH0_EN_W { w: self }
    }
}

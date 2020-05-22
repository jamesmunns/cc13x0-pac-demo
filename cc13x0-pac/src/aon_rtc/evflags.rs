#[doc = "Reader of register EVFLAGS"]
pub type R = crate::R<u32, super::EVFLAGS>;
#[doc = "Writer for register EVFLAGS"]
pub type W = crate::W<u32, super::EVFLAGS>;
#[doc = "Register EVFLAGS `reset()`'s with value 0"]
impl crate::ResetValue for super::EVFLAGS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH2`"]
pub type CH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2`"]
pub struct CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_W<'a> {
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
#[doc = "Reader of field `CH1`"]
pub type CH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1`"]
pub struct CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_W<'a> {
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
#[doc = "Reader of field `CH0`"]
pub type CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0`"]
pub struct CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_W<'a> {
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
    #[doc = "Bit 16 - CH2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CH1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CH0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - CH2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W {
        CH2_W { w: self }
    }
    #[doc = "Bit 8 - CH1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W { w: self }
    }
    #[doc = "Bit 0 - CH0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W { w: self }
    }
}

#[doc = "Reader of register ADC0"]
pub type R = crate::R<u8, super::ADC0>;
#[doc = "Writer for register ADC0"]
pub type W = crate::W<u8, super::ADC0>;
#[doc = "Register ADC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMPL_MODE`"]
pub type SMPL_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMPL_MODE`"]
pub struct SMPL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPL_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SMPL_CYCLE_EXP`"]
pub type SMPL_CYCLE_EXP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMPL_CYCLE_EXP`"]
pub struct SMPL_CYCLE_EXP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPL_CYCLE_EXP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u8) & 0x0f) << 3);
        self.w
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - SMPL_MODE"]
    #[inline(always)]
    pub fn smpl_mode(&self) -> SMPL_MODE_R {
        SMPL_MODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - SMPL_CYCLE_EXP"]
    #[inline(always)]
    pub fn smpl_cycle_exp(&self) -> SMPL_CYCLE_EXP_R {
        SMPL_CYCLE_EXP_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - RESET_N"]
    #[inline(always)]
    pub fn reset_n(&self) -> RESET_N_R {
        RESET_N_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - SMPL_MODE"]
    #[inline(always)]
    pub fn smpl_mode(&mut self) -> SMPL_MODE_W {
        SMPL_MODE_W { w: self }
    }
    #[doc = "Bits 3:6 - SMPL_CYCLE_EXP"]
    #[inline(always)]
    pub fn smpl_cycle_exp(&mut self) -> SMPL_CYCLE_EXP_W {
        SMPL_CYCLE_EXP_W { w: self }
    }
    #[doc = "Bit 1 - RESET_N"]
    #[inline(always)]
    pub fn reset_n(&mut self) -> RESET_N_W {
        RESET_N_W { w: self }
    }
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}

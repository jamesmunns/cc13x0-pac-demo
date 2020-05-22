#[doc = "Reader of register I2SCLKCTL"]
pub type R = crate::R<u32, super::I2SCLKCTL>;
#[doc = "Writer for register I2SCLKCTL"]
pub type W = crate::W<u32, super::I2SCLKCTL>;
#[doc = "Register I2SCLKCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::I2SCLKCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMPL_ON_POSEDGE`"]
pub type SMPL_ON_POSEDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMPL_ON_POSEDGE`"]
pub struct SMPL_ON_POSEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPL_ON_POSEDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `WCLK_PHASE`"]
pub type WCLK_PHASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WCLK_PHASE`"]
pub struct WCLK_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> WCLK_PHASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - SMPL_ON_POSEDGE"]
    #[inline(always)]
    pub fn smpl_on_posedge(&self) -> SMPL_ON_POSEDGE_R {
        SMPL_ON_POSEDGE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - WCLK_PHASE"]
    #[inline(always)]
    pub fn wclk_phase(&self) -> WCLK_PHASE_R {
        WCLK_PHASE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - SMPL_ON_POSEDGE"]
    #[inline(always)]
    pub fn smpl_on_posedge(&mut self) -> SMPL_ON_POSEDGE_W {
        SMPL_ON_POSEDGE_W { w: self }
    }
    #[doc = "Bits 1:2 - WCLK_PHASE"]
    #[inline(always)]
    pub fn wclk_phase(&mut self) -> WCLK_PHASE_W {
        WCLK_PHASE_W { w: self }
    }
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}

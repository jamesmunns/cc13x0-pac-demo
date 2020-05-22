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
#[doc = "Reader of field `FORCE_EV_LOW`"]
pub type FORCE_EV_LOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FORCE_EV_LOW`"]
pub struct FORCE_EV_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_EV_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `FORCE_EV_HIGH`"]
pub type FORCE_EV_HIGH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FORCE_EV_HIGH`"]
pub struct FORCE_EV_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_EV_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESET_VECTOR`"]
pub type RESET_VECTOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESET_VECTOR`"]
pub struct RESET_VECTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_VECTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DBG_FREEZE_EN`"]
pub type DBG_FREEZE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBG_FREEZE_EN`"]
pub struct DBG_FREEZE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_FREEZE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `FORCE_WU_LOW`"]
pub type FORCE_WU_LOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_WU_LOW`"]
pub struct FORCE_WU_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_WU_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `FORCE_WU_HIGH`"]
pub type FORCE_WU_HIGH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_WU_HIGH`"]
pub struct FORCE_WU_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_WU_HIGH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RESTART`"]
pub type RESTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESTART`"]
pub struct RESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTART_W<'a> {
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
#[doc = "Reader of field `SINGLE_STEP`"]
pub type SINGLE_STEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SINGLE_STEP`"]
pub struct SINGLE_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_STEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SUSPEND`"]
pub type SUSPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPEND`"]
pub struct SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_W<'a> {
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
#[doc = "Reader of field `CLK_EN`"]
pub type CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_EN`"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
    #[doc = "Bits 24:31 - FORCE_EV_LOW"]
    #[inline(always)]
    pub fn force_ev_low(&self) -> FORCE_EV_LOW_R {
        FORCE_EV_LOW_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - FORCE_EV_HIGH"]
    #[inline(always)]
    pub fn force_ev_high(&self) -> FORCE_EV_HIGH_R {
        FORCE_EV_HIGH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - RESET_VECTOR"]
    #[inline(always)]
    pub fn reset_vector(&self) -> RESET_VECTOR_R {
        RESET_VECTOR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - DBG_FREEZE_EN"]
    #[inline(always)]
    pub fn dbg_freeze_en(&self) -> DBG_FREEZE_EN_R {
        DBG_FREEZE_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FORCE_WU_LOW"]
    #[inline(always)]
    pub fn force_wu_low(&self) -> FORCE_WU_LOW_R {
        FORCE_WU_LOW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FORCE_WU_HIGH"]
    #[inline(always)]
    pub fn force_wu_high(&self) -> FORCE_WU_HIGH_R {
        FORCE_WU_HIGH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RESTART"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SINGLE_STEP"]
    #[inline(always)]
    pub fn single_step(&self) -> SINGLE_STEP_R {
        SINGLE_STEP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SUSPEND"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CLK_EN"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - FORCE_EV_LOW"]
    #[inline(always)]
    pub fn force_ev_low(&mut self) -> FORCE_EV_LOW_W {
        FORCE_EV_LOW_W { w: self }
    }
    #[doc = "Bits 16:23 - FORCE_EV_HIGH"]
    #[inline(always)]
    pub fn force_ev_high(&mut self) -> FORCE_EV_HIGH_W {
        FORCE_EV_HIGH_W { w: self }
    }
    #[doc = "Bits 8:11 - RESET_VECTOR"]
    #[inline(always)]
    pub fn reset_vector(&mut self) -> RESET_VECTOR_W {
        RESET_VECTOR_W { w: self }
    }
    #[doc = "Bit 6 - DBG_FREEZE_EN"]
    #[inline(always)]
    pub fn dbg_freeze_en(&mut self) -> DBG_FREEZE_EN_W {
        DBG_FREEZE_EN_W { w: self }
    }
    #[doc = "Bit 5 - FORCE_WU_LOW"]
    #[inline(always)]
    pub fn force_wu_low(&mut self) -> FORCE_WU_LOW_W {
        FORCE_WU_LOW_W { w: self }
    }
    #[doc = "Bit 4 - FORCE_WU_HIGH"]
    #[inline(always)]
    pub fn force_wu_high(&mut self) -> FORCE_WU_HIGH_W {
        FORCE_WU_HIGH_W { w: self }
    }
    #[doc = "Bit 3 - RESTART"]
    #[inline(always)]
    pub fn restart(&mut self) -> RESTART_W {
        RESTART_W { w: self }
    }
    #[doc = "Bit 2 - SINGLE_STEP"]
    #[inline(always)]
    pub fn single_step(&mut self) -> SINGLE_STEP_W {
        SINGLE_STEP_W { w: self }
    }
    #[doc = "Bit 1 - SUSPEND"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W {
        SUSPEND_W { w: self }
    }
    #[doc = "Bit 0 - CLK_EN"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
}

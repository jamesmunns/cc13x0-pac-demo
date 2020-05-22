#[doc = "Reader of register AMPCOMPCTL"]
pub type R = crate::R<u32, super::AMPCOMPCTL>;
#[doc = "Writer for register AMPCOMPCTL"]
pub type W = crate::W<u32, super::AMPCOMPCTL>;
#[doc = "Register AMPCOMPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::AMPCOMPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AMPCOMP_REQ_MODE`"]
pub type AMPCOMP_REQ_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMPCOMP_REQ_MODE`"]
pub struct AMPCOMP_REQ_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPCOMP_REQ_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `AMPCOMP_FSM_UPDATE_RATE`"]
pub type AMPCOMP_FSM_UPDATE_RATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMPCOMP_FSM_UPDATE_RATE`"]
pub struct AMPCOMP_FSM_UPDATE_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPCOMP_FSM_UPDATE_RATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `AMPCOMP_SW_CTRL`"]
pub type AMPCOMP_SW_CTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMPCOMP_SW_CTRL`"]
pub struct AMPCOMP_SW_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPCOMP_SW_CTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `AMPCOMP_SW_EN`"]
pub type AMPCOMP_SW_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMPCOMP_SW_EN`"]
pub struct AMPCOMP_SW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPCOMP_SW_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `IBIAS_OFFSET`"]
pub type IBIAS_OFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IBIAS_OFFSET`"]
pub struct IBIAS_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIAS_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `IBIAS_INIT`"]
pub type IBIAS_INIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IBIAS_INIT`"]
pub struct IBIAS_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIAS_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `LPM_IBIAS_WAIT_CNT_FINAL`"]
pub type LPM_IBIAS_WAIT_CNT_FINAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPM_IBIAS_WAIT_CNT_FINAL`"]
pub struct LPM_IBIAS_WAIT_CNT_FINAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_IBIAS_WAIT_CNT_FINAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CAP_STEP`"]
pub type CAP_STEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAP_STEP`"]
pub struct CAP_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `IBIASCAP_HPTOLP_OL_CNT`"]
pub type IBIASCAP_HPTOLP_OL_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IBIASCAP_HPTOLP_OL_CNT`"]
pub struct IBIASCAP_HPTOLP_OL_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIASCAP_HPTOLP_OL_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - AMPCOMP_REQ_MODE"]
    #[inline(always)]
    pub fn ampcomp_req_mode(&self) -> AMPCOMP_REQ_MODE_R {
        AMPCOMP_REQ_MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - AMPCOMP_FSM_UPDATE_RATE"]
    #[inline(always)]
    pub fn ampcomp_fsm_update_rate(&self) -> AMPCOMP_FSM_UPDATE_RATE_R {
        AMPCOMP_FSM_UPDATE_RATE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 27 - AMPCOMP_SW_CTRL"]
    #[inline(always)]
    pub fn ampcomp_sw_ctrl(&self) -> AMPCOMP_SW_CTRL_R {
        AMPCOMP_SW_CTRL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - AMPCOMP_SW_EN"]
    #[inline(always)]
    pub fn ampcomp_sw_en(&self) -> AMPCOMP_SW_EN_R {
        AMPCOMP_SW_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - IBIAS_OFFSET"]
    #[inline(always)]
    pub fn ibias_offset(&self) -> IBIAS_OFFSET_R {
        IBIAS_OFFSET_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - IBIAS_INIT"]
    #[inline(always)]
    pub fn ibias_init(&self) -> IBIAS_INIT_R {
        IBIAS_INIT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - LPM_IBIAS_WAIT_CNT_FINAL"]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt_final(&self) -> LPM_IBIAS_WAIT_CNT_FINAL_R {
        LPM_IBIAS_WAIT_CNT_FINAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - CAP_STEP"]
    #[inline(always)]
    pub fn cap_step(&self) -> CAP_STEP_R {
        CAP_STEP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - IBIASCAP_HPTOLP_OL_CNT"]
    #[inline(always)]
    pub fn ibiascap_hptolp_ol_cnt(&self) -> IBIASCAP_HPTOLP_OL_CNT_R {
        IBIASCAP_HPTOLP_OL_CNT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - AMPCOMP_REQ_MODE"]
    #[inline(always)]
    pub fn ampcomp_req_mode(&mut self) -> AMPCOMP_REQ_MODE_W {
        AMPCOMP_REQ_MODE_W { w: self }
    }
    #[doc = "Bits 28:29 - AMPCOMP_FSM_UPDATE_RATE"]
    #[inline(always)]
    pub fn ampcomp_fsm_update_rate(&mut self) -> AMPCOMP_FSM_UPDATE_RATE_W {
        AMPCOMP_FSM_UPDATE_RATE_W { w: self }
    }
    #[doc = "Bit 27 - AMPCOMP_SW_CTRL"]
    #[inline(always)]
    pub fn ampcomp_sw_ctrl(&mut self) -> AMPCOMP_SW_CTRL_W {
        AMPCOMP_SW_CTRL_W { w: self }
    }
    #[doc = "Bit 26 - AMPCOMP_SW_EN"]
    #[inline(always)]
    pub fn ampcomp_sw_en(&mut self) -> AMPCOMP_SW_EN_W {
        AMPCOMP_SW_EN_W { w: self }
    }
    #[doc = "Bits 20:23 - IBIAS_OFFSET"]
    #[inline(always)]
    pub fn ibias_offset(&mut self) -> IBIAS_OFFSET_W {
        IBIAS_OFFSET_W { w: self }
    }
    #[doc = "Bits 16:19 - IBIAS_INIT"]
    #[inline(always)]
    pub fn ibias_init(&mut self) -> IBIAS_INIT_W {
        IBIAS_INIT_W { w: self }
    }
    #[doc = "Bits 8:15 - LPM_IBIAS_WAIT_CNT_FINAL"]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt_final(&mut self) -> LPM_IBIAS_WAIT_CNT_FINAL_W {
        LPM_IBIAS_WAIT_CNT_FINAL_W { w: self }
    }
    #[doc = "Bits 4:7 - CAP_STEP"]
    #[inline(always)]
    pub fn cap_step(&mut self) -> CAP_STEP_W {
        CAP_STEP_W { w: self }
    }
    #[doc = "Bits 0:3 - IBIASCAP_HPTOLP_OL_CNT"]
    #[inline(always)]
    pub fn ibiascap_hptolp_ol_cnt(&mut self) -> IBIASCAP_HPTOLP_OL_CNT_W {
        IBIASCAP_HPTOLP_OL_CNT_W { w: self }
    }
}

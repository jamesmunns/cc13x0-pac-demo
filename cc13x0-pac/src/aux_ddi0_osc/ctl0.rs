#[doc = "Reader of register CTL0"]
pub type R = crate::R<u32, super::CTL0>;
#[doc = "Writer for register CTL0"]
pub type W = crate::W<u32, super::CTL0>;
#[doc = "Register CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XTAL_IS_24M`"]
pub type XTAL_IS_24M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTAL_IS_24M`"]
pub struct XTAL_IS_24M_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_IS_24M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `BYPASS_XOSC_LF_CLK_QUAL`"]
pub type BYPASS_XOSC_LF_CLK_QUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS_XOSC_LF_CLK_QUAL`"]
pub struct BYPASS_XOSC_LF_CLK_QUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_XOSC_LF_CLK_QUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `BYPASS_RCOSC_LF_CLK_QUAL`"]
pub type BYPASS_RCOSC_LF_CLK_QUAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS_RCOSC_LF_CLK_QUAL`"]
pub struct BYPASS_RCOSC_LF_CLK_QUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_RCOSC_LF_CLK_QUAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DOUBLER_START_DURATION`"]
pub type DOUBLER_START_DURATION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DOUBLER_START_DURATION`"]
pub struct DOUBLER_START_DURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUBLER_START_DURATION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `DOUBLER_RESET_DURATION`"]
pub type DOUBLER_RESET_DURATION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOUBLER_RESET_DURATION`"]
pub struct DOUBLER_RESET_DURATION_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUBLER_RESET_DURATION_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `FORCE_KICKSTART_EN`"]
pub type FORCE_KICKSTART_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_KICKSTART_EN`"]
pub struct FORCE_KICKSTART_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_KICKSTART_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ALLOW_SCLK_HF_SWITCHING`"]
pub type ALLOW_SCLK_HF_SWITCHING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALLOW_SCLK_HF_SWITCHING`"]
pub struct ALLOW_SCLK_HF_SWITCHING_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLOW_SCLK_HF_SWITCHING_W<'a> {
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
#[doc = "Reader of field `RCOSC_LF_TRIMMED`"]
pub type RCOSC_LF_TRIMMED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSC_LF_TRIMMED`"]
pub struct RCOSC_LF_TRIMMED_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_LF_TRIMMED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `XOSC_HF_POWER_MODE`"]
pub type XOSC_HF_POWER_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC_HF_POWER_MODE`"]
pub struct XOSC_HF_POWER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_POWER_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `XOSC_LF_DIG_BYPASS`"]
pub type XOSC_LF_DIG_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSC_LF_DIG_BYPASS`"]
pub struct XOSC_LF_DIG_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_LF_DIG_BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CLK_LOSS_EN`"]
pub type CLK_LOSS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_LOSS_EN`"]
pub struct CLK_LOSS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_LOSS_EN_W<'a> {
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
#[doc = "Reader of field `ACLK_TDC_SRC_SEL`"]
pub type ACLK_TDC_SRC_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACLK_TDC_SRC_SEL`"]
pub struct ACLK_TDC_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_TDC_SRC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Reader of field `ACLK_REF_SRC_SEL`"]
pub type ACLK_REF_SRC_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACLK_REF_SRC_SEL`"]
pub struct ACLK_REF_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_REF_SRC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `SCLK_LF_SRC_SEL`"]
pub type SCLK_LF_SRC_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCLK_LF_SRC_SEL`"]
pub struct SCLK_LF_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_LF_SRC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SCLK_MF_SRC_SEL`"]
pub type SCLK_MF_SRC_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_MF_SRC_SEL`"]
pub struct SCLK_MF_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_MF_SRC_SEL_W<'a> {
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
#[doc = "Reader of field `SCLK_HF_SRC_SEL`"]
pub type SCLK_HF_SRC_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_HF_SRC_SEL`"]
pub struct SCLK_HF_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_HF_SRC_SEL_W<'a> {
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
    #[doc = "Bit 31 - XTAL_IS_24M"]
    #[inline(always)]
    pub fn xtal_is_24m(&self) -> XTAL_IS_24M_R {
        XTAL_IS_24M_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 29 - BYPASS_XOSC_LF_CLK_QUAL"]
    #[inline(always)]
    pub fn bypass_xosc_lf_clk_qual(&self) -> BYPASS_XOSC_LF_CLK_QUAL_R {
        BYPASS_XOSC_LF_CLK_QUAL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - BYPASS_RCOSC_LF_CLK_QUAL"]
    #[inline(always)]
    pub fn bypass_rcosc_lf_clk_qual(&self) -> BYPASS_RCOSC_LF_CLK_QUAL_R {
        BYPASS_RCOSC_LF_CLK_QUAL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - DOUBLER_START_DURATION"]
    #[inline(always)]
    pub fn doubler_start_duration(&self) -> DOUBLER_START_DURATION_R {
        DOUBLER_START_DURATION_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 25 - DOUBLER_RESET_DURATION"]
    #[inline(always)]
    pub fn doubler_reset_duration(&self) -> DOUBLER_RESET_DURATION_R {
        DOUBLER_RESET_DURATION_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 22 - FORCE_KICKSTART_EN"]
    #[inline(always)]
    pub fn force_kickstart_en(&self) -> FORCE_KICKSTART_EN_R {
        FORCE_KICKSTART_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ALLOW_SCLK_HF_SWITCHING"]
    #[inline(always)]
    pub fn allow_sclk_hf_switching(&self) -> ALLOW_SCLK_HF_SWITCHING_R {
        ALLOW_SCLK_HF_SWITCHING_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RCOSC_LF_TRIMMED"]
    #[inline(always)]
    pub fn rcosc_lf_trimmed(&self) -> RCOSC_LF_TRIMMED_R {
        RCOSC_LF_TRIMMED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - XOSC_HF_POWER_MODE"]
    #[inline(always)]
    pub fn xosc_hf_power_mode(&self) -> XOSC_HF_POWER_MODE_R {
        XOSC_HF_POWER_MODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - XOSC_LF_DIG_BYPASS"]
    #[inline(always)]
    pub fn xosc_lf_dig_bypass(&self) -> XOSC_LF_DIG_BYPASS_R {
        XOSC_LF_DIG_BYPASS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CLK_LOSS_EN"]
    #[inline(always)]
    pub fn clk_loss_en(&self) -> CLK_LOSS_EN_R {
        CLK_LOSS_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - ACLK_TDC_SRC_SEL"]
    #[inline(always)]
    pub fn aclk_tdc_src_sel(&self) -> ACLK_TDC_SRC_SEL_R {
        ACLK_TDC_SRC_SEL_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - ACLK_REF_SRC_SEL"]
    #[inline(always)]
    pub fn aclk_ref_src_sel(&self) -> ACLK_REF_SRC_SEL_R {
        ACLK_REF_SRC_SEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - SCLK_LF_SRC_SEL"]
    #[inline(always)]
    pub fn sclk_lf_src_sel(&self) -> SCLK_LF_SRC_SEL_R {
        SCLK_LF_SRC_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - SCLK_MF_SRC_SEL"]
    #[inline(always)]
    pub fn sclk_mf_src_sel(&self) -> SCLK_MF_SRC_SEL_R {
        SCLK_MF_SRC_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SCLK_HF_SRC_SEL"]
    #[inline(always)]
    pub fn sclk_hf_src_sel(&self) -> SCLK_HF_SRC_SEL_R {
        SCLK_HF_SRC_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - XTAL_IS_24M"]
    #[inline(always)]
    pub fn xtal_is_24m(&mut self) -> XTAL_IS_24M_W {
        XTAL_IS_24M_W { w: self }
    }
    #[doc = "Bit 29 - BYPASS_XOSC_LF_CLK_QUAL"]
    #[inline(always)]
    pub fn bypass_xosc_lf_clk_qual(&mut self) -> BYPASS_XOSC_LF_CLK_QUAL_W {
        BYPASS_XOSC_LF_CLK_QUAL_W { w: self }
    }
    #[doc = "Bit 28 - BYPASS_RCOSC_LF_CLK_QUAL"]
    #[inline(always)]
    pub fn bypass_rcosc_lf_clk_qual(&mut self) -> BYPASS_RCOSC_LF_CLK_QUAL_W {
        BYPASS_RCOSC_LF_CLK_QUAL_W { w: self }
    }
    #[doc = "Bits 26:27 - DOUBLER_START_DURATION"]
    #[inline(always)]
    pub fn doubler_start_duration(&mut self) -> DOUBLER_START_DURATION_W {
        DOUBLER_START_DURATION_W { w: self }
    }
    #[doc = "Bit 25 - DOUBLER_RESET_DURATION"]
    #[inline(always)]
    pub fn doubler_reset_duration(&mut self) -> DOUBLER_RESET_DURATION_W {
        DOUBLER_RESET_DURATION_W { w: self }
    }
    #[doc = "Bit 22 - FORCE_KICKSTART_EN"]
    #[inline(always)]
    pub fn force_kickstart_en(&mut self) -> FORCE_KICKSTART_EN_W {
        FORCE_KICKSTART_EN_W { w: self }
    }
    #[doc = "Bit 16 - ALLOW_SCLK_HF_SWITCHING"]
    #[inline(always)]
    pub fn allow_sclk_hf_switching(&mut self) -> ALLOW_SCLK_HF_SWITCHING_W {
        ALLOW_SCLK_HF_SWITCHING_W { w: self }
    }
    #[doc = "Bit 12 - RCOSC_LF_TRIMMED"]
    #[inline(always)]
    pub fn rcosc_lf_trimmed(&mut self) -> RCOSC_LF_TRIMMED_W {
        RCOSC_LF_TRIMMED_W { w: self }
    }
    #[doc = "Bit 11 - XOSC_HF_POWER_MODE"]
    #[inline(always)]
    pub fn xosc_hf_power_mode(&mut self) -> XOSC_HF_POWER_MODE_W {
        XOSC_HF_POWER_MODE_W { w: self }
    }
    #[doc = "Bit 10 - XOSC_LF_DIG_BYPASS"]
    #[inline(always)]
    pub fn xosc_lf_dig_bypass(&mut self) -> XOSC_LF_DIG_BYPASS_W {
        XOSC_LF_DIG_BYPASS_W { w: self }
    }
    #[doc = "Bit 9 - CLK_LOSS_EN"]
    #[inline(always)]
    pub fn clk_loss_en(&mut self) -> CLK_LOSS_EN_W {
        CLK_LOSS_EN_W { w: self }
    }
    #[doc = "Bits 7:8 - ACLK_TDC_SRC_SEL"]
    #[inline(always)]
    pub fn aclk_tdc_src_sel(&mut self) -> ACLK_TDC_SRC_SEL_W {
        ACLK_TDC_SRC_SEL_W { w: self }
    }
    #[doc = "Bits 5:6 - ACLK_REF_SRC_SEL"]
    #[inline(always)]
    pub fn aclk_ref_src_sel(&mut self) -> ACLK_REF_SRC_SEL_W {
        ACLK_REF_SRC_SEL_W { w: self }
    }
    #[doc = "Bits 2:3 - SCLK_LF_SRC_SEL"]
    #[inline(always)]
    pub fn sclk_lf_src_sel(&mut self) -> SCLK_LF_SRC_SEL_W {
        SCLK_LF_SRC_SEL_W { w: self }
    }
    #[doc = "Bit 1 - SCLK_MF_SRC_SEL"]
    #[inline(always)]
    pub fn sclk_mf_src_sel(&mut self) -> SCLK_MF_SRC_SEL_W {
        SCLK_MF_SRC_SEL_W { w: self }
    }
    #[doc = "Bit 0 - SCLK_HF_SRC_SEL"]
    #[inline(always)]
    pub fn sclk_hf_src_sel(&mut self) -> SCLK_HF_SRC_SEL_W {
        SCLK_HF_SRC_SEL_W { w: self }
    }
}

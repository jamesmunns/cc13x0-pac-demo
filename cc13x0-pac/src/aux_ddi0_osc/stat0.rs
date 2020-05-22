#[doc = "Reader of register STAT0"]
pub type R = crate::R<u32, super::STAT0>;
#[doc = "Reader of field `SCLK_LF_SRC`"]
pub type SCLK_LF_SRC_R = crate::R<u8, u8>;
#[doc = "Reader of field `SCLK_HF_SRC`"]
pub type SCLK_HF_SRC_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCOSC_HF_EN`"]
pub type RCOSC_HF_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCOSC_LF_EN`"]
pub type RCOSC_LF_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSC_LF_EN`"]
pub type XOSC_LF_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLK_DCDC_RDY`"]
pub type CLK_DCDC_RDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLK_DCDC_RDY_ACK`"]
pub type CLK_DCDC_RDY_ACK_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCLK_HF_LOSS`"]
pub type SCLK_HF_LOSS_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCLK_LF_LOSS`"]
pub type SCLK_LF_LOSS_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSC_HF_EN`"]
pub type XOSC_HF_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `XB_48M_CLK_EN`"]
pub type XB_48M_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSC_HF_LP_BUF_EN`"]
pub type XOSC_HF_LP_BUF_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSC_HF_HP_BUF_EN`"]
pub type XOSC_HF_HP_BUF_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC_THMET`"]
pub type ADC_THMET_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC_DATA_READY`"]
pub type ADC_DATA_READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC_DATA`"]
pub type ADC_DATA_R = crate::R<u8, u8>;
#[doc = "Reader of field `PENDINGSCLKHFSWITCHING`"]
pub type PENDINGSCLKHFSWITCHING_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 29:30 - SCLK_LF_SRC"]
    #[inline(always)]
    pub fn sclk_lf_src(&self) -> SCLK_LF_SRC_R {
        SCLK_LF_SRC_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - SCLK_HF_SRC"]
    #[inline(always)]
    pub fn sclk_hf_src(&self) -> SCLK_HF_SRC_R {
        SCLK_HF_SRC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 22 - RCOSC_HF_EN"]
    #[inline(always)]
    pub fn rcosc_hf_en(&self) -> RCOSC_HF_EN_R {
        RCOSC_HF_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - RCOSC_LF_EN"]
    #[inline(always)]
    pub fn rcosc_lf_en(&self) -> RCOSC_LF_EN_R {
        RCOSC_LF_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - XOSC_LF_EN"]
    #[inline(always)]
    pub fn xosc_lf_en(&self) -> XOSC_LF_EN_R {
        XOSC_LF_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CLK_DCDC_RDY"]
    #[inline(always)]
    pub fn clk_dcdc_rdy(&self) -> CLK_DCDC_RDY_R {
        CLK_DCDC_RDY_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CLK_DCDC_RDY_ACK"]
    #[inline(always)]
    pub fn clk_dcdc_rdy_ack(&self) -> CLK_DCDC_RDY_ACK_R {
        CLK_DCDC_RDY_ACK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SCLK_HF_LOSS"]
    #[inline(always)]
    pub fn sclk_hf_loss(&self) -> SCLK_HF_LOSS_R {
        SCLK_HF_LOSS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SCLK_LF_LOSS"]
    #[inline(always)]
    pub fn sclk_lf_loss(&self) -> SCLK_LF_LOSS_R {
        SCLK_LF_LOSS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - XOSC_HF_EN"]
    #[inline(always)]
    pub fn xosc_hf_en(&self) -> XOSC_HF_EN_R {
        XOSC_HF_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13 - XB_48M_CLK_EN"]
    #[inline(always)]
    pub fn xb_48m_clk_en(&self) -> XB_48M_CLK_EN_R {
        XB_48M_CLK_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 11 - XOSC_HF_LP_BUF_EN"]
    #[inline(always)]
    pub fn xosc_hf_lp_buf_en(&self) -> XOSC_HF_LP_BUF_EN_R {
        XOSC_HF_LP_BUF_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - XOSC_HF_HP_BUF_EN"]
    #[inline(always)]
    pub fn xosc_hf_hp_buf_en(&self) -> XOSC_HF_HP_BUF_EN_R {
        XOSC_HF_HP_BUF_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC_THMET"]
    #[inline(always)]
    pub fn adc_thmet(&self) -> ADC_THMET_R {
        ADC_THMET_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC_DATA_READY"]
    #[inline(always)]
    pub fn adc_data_ready(&self) -> ADC_DATA_READY_R {
        ADC_DATA_READY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 1:6 - ADC_DATA"]
    #[inline(always)]
    pub fn adc_data(&self) -> ADC_DATA_R {
        ADC_DATA_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 0 - PENDINGSCLKHFSWITCHING"]
    #[inline(always)]
    pub fn pendingsclkhfswitching(&self) -> PENDINGSCLKHFSWITCHING_R {
        PENDINGSCLKHFSWITCHING_R::new((self.bits & 0x01) != 0)
    }
}

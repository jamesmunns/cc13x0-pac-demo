#[doc = "Reader of register ATESTCTL"]
pub type R = crate::R<u32, super::ATESTCTL>;
#[doc = "Writer for register ATESTCTL"]
pub type W = crate::W<u32, super::ATESTCTL>;
#[doc = "Register ATESTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ATESTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCLK_LF_AUX_EN`"]
pub type SCLK_LF_AUX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLK_LF_AUX_EN`"]
pub struct SCLK_LF_AUX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_LF_AUX_EN_W<'a> {
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
impl R {
    #[doc = "Bit 29 - SCLK_LF_AUX_EN"]
    #[inline(always)]
    pub fn sclk_lf_aux_en(&self) -> SCLK_LF_AUX_EN_R {
        SCLK_LF_AUX_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - SCLK_LF_AUX_EN"]
    #[inline(always)]
    pub fn sclk_lf_aux_en(&mut self) -> SCLK_LF_AUX_EN_W {
        SCLK_LF_AUX_EN_W { w: self }
    }
}

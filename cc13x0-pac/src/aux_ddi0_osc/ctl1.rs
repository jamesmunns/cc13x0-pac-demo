#[doc = "Reader of register CTL1"]
pub type R = crate::R<u32, super::CTL1>;
#[doc = "Writer for register CTL1"]
pub type W = crate::W<u32, super::CTL1>;
#[doc = "Register CTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RCOSCHFCTRIMFRACT`"]
pub type RCOSCHFCTRIMFRACT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSCHFCTRIMFRACT`"]
pub struct RCOSCHFCTRIMFRACT_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHFCTRIMFRACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | (((value as u32) & 0x1f) << 18);
        self.w
    }
}
#[doc = "Reader of field `RCOSCHFCTRIMFRACT_EN`"]
pub type RCOSCHFCTRIMFRACT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSCHFCTRIMFRACT_EN`"]
pub struct RCOSCHFCTRIMFRACT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHFCTRIMFRACT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `XOSC_HF_FAST_START`"]
pub type XOSC_HF_FAST_START_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOSC_HF_FAST_START`"]
pub struct XOSC_HF_FAST_START_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_FAST_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:22 - RCOSCHFCTRIMFRACT"]
    #[inline(always)]
    pub fn rcoschfctrimfract(&self) -> RCOSCHFCTRIMFRACT_R {
        RCOSCHFCTRIMFRACT_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - RCOSCHFCTRIMFRACT_EN"]
    #[inline(always)]
    pub fn rcoschfctrimfract_en(&self) -> RCOSCHFCTRIMFRACT_EN_R {
        RCOSCHFCTRIMFRACT_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - XOSC_HF_FAST_START"]
    #[inline(always)]
    pub fn xosc_hf_fast_start(&self) -> XOSC_HF_FAST_START_R {
        XOSC_HF_FAST_START_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 18:22 - RCOSCHFCTRIMFRACT"]
    #[inline(always)]
    pub fn rcoschfctrimfract(&mut self) -> RCOSCHFCTRIMFRACT_W {
        RCOSCHFCTRIMFRACT_W { w: self }
    }
    #[doc = "Bit 17 - RCOSCHFCTRIMFRACT_EN"]
    #[inline(always)]
    pub fn rcoschfctrimfract_en(&mut self) -> RCOSCHFCTRIMFRACT_EN_W {
        RCOSCHFCTRIMFRACT_EN_W { w: self }
    }
    #[doc = "Bits 0:1 - XOSC_HF_FAST_START"]
    #[inline(always)]
    pub fn xosc_hf_fast_start(&mut self) -> XOSC_HF_FAST_START_W {
        XOSC_HF_FAST_START_W { w: self }
    }
}

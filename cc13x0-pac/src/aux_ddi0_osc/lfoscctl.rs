#[doc = "Reader of register LFOSCCTL"]
pub type R = crate::R<u32, super::LFOSCCTL>;
#[doc = "Writer for register LFOSCCTL"]
pub type W = crate::W<u32, super::LFOSCCTL>;
#[doc = "Register LFOSCCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::LFOSCCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XOSCLF_REGULATOR_TRIM`"]
pub type XOSCLF_REGULATOR_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOSCLF_REGULATOR_TRIM`"]
pub struct XOSCLF_REGULATOR_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCLF_REGULATOR_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `XOSCLF_CMIRRWR_RATIO`"]
pub type XOSCLF_CMIRRWR_RATIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOSCLF_CMIRRWR_RATIO`"]
pub struct XOSCLF_CMIRRWR_RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCLF_CMIRRWR_RATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `RCOSCLF_RTUNE_TRIM`"]
pub type RCOSCLF_RTUNE_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSCLF_RTUNE_TRIM`"]
pub struct RCOSCLF_RTUNE_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCLF_RTUNE_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `RCOSCLF_CTUNE_TRIM`"]
pub type RCOSCLF_CTUNE_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSCLF_CTUNE_TRIM`"]
pub struct RCOSCLF_CTUNE_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCLF_CTUNE_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:23 - XOSCLF_REGULATOR_TRIM"]
    #[inline(always)]
    pub fn xosclf_regulator_trim(&self) -> XOSCLF_REGULATOR_TRIM_R {
        XOSCLF_REGULATOR_TRIM_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 18:21 - XOSCLF_CMIRRWR_RATIO"]
    #[inline(always)]
    pub fn xosclf_cmirrwr_ratio(&self) -> XOSCLF_CMIRRWR_RATIO_R {
        XOSCLF_CMIRRWR_RATIO_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - RCOSCLF_RTUNE_TRIM"]
    #[inline(always)]
    pub fn rcosclf_rtune_trim(&self) -> RCOSCLF_RTUNE_TRIM_R {
        RCOSCLF_RTUNE_TRIM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:7 - RCOSCLF_CTUNE_TRIM"]
    #[inline(always)]
    pub fn rcosclf_ctune_trim(&self) -> RCOSCLF_CTUNE_TRIM_R {
        RCOSCLF_CTUNE_TRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 22:23 - XOSCLF_REGULATOR_TRIM"]
    #[inline(always)]
    pub fn xosclf_regulator_trim(&mut self) -> XOSCLF_REGULATOR_TRIM_W {
        XOSCLF_REGULATOR_TRIM_W { w: self }
    }
    #[doc = "Bits 18:21 - XOSCLF_CMIRRWR_RATIO"]
    #[inline(always)]
    pub fn xosclf_cmirrwr_ratio(&mut self) -> XOSCLF_CMIRRWR_RATIO_W {
        XOSCLF_CMIRRWR_RATIO_W { w: self }
    }
    #[doc = "Bits 8:9 - RCOSCLF_RTUNE_TRIM"]
    #[inline(always)]
    pub fn rcosclf_rtune_trim(&mut self) -> RCOSCLF_RTUNE_TRIM_W {
        RCOSCLF_RTUNE_TRIM_W { w: self }
    }
    #[doc = "Bits 0:7 - RCOSCLF_CTUNE_TRIM"]
    #[inline(always)]
    pub fn rcosclf_ctune_trim(&mut self) -> RCOSCLF_CTUNE_TRIM_W {
        RCOSCLF_CTUNE_TRIM_W { w: self }
    }
}

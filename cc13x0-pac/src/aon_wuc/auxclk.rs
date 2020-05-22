#[doc = "Reader of register AUXCLK"]
pub type R = crate::R<u32, super::AUXCLK>;
#[doc = "Writer for register AUXCLK"]
pub type W = crate::W<u32, super::AUXCLK>;
#[doc = "Register AUXCLK `reset()`'s with value 0x01"]
impl crate::ResetValue for super::AUXCLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `PWR_DWN_SRC`"]
pub type PWR_DWN_SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWR_DWN_SRC`"]
pub struct PWR_DWN_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_DWN_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `SCLK_HF_DIV`"]
pub type SCLK_HF_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCLK_HF_DIV`"]
pub struct SCLK_HF_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLK_HF_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:12 - PWR_DWN_SRC"]
    #[inline(always)]
    pub fn pwr_dwn_src(&self) -> PWR_DWN_SRC_R {
        PWR_DWN_SRC_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - SCLK_HF_DIV"]
    #[inline(always)]
    pub fn sclk_hf_div(&self) -> SCLK_HF_DIV_R {
        SCLK_HF_DIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - SRC"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 11:12 - PWR_DWN_SRC"]
    #[inline(always)]
    pub fn pwr_dwn_src(&mut self) -> PWR_DWN_SRC_W {
        PWR_DWN_SRC_W { w: self }
    }
    #[doc = "Bits 8:10 - SCLK_HF_DIV"]
    #[inline(always)]
    pub fn sclk_hf_div(&mut self) -> SCLK_HF_DIV_W {
        SCLK_HF_DIV_W { w: self }
    }
    #[doc = "Bits 0:2 - SRC"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
}

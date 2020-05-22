#[doc = "Reader of register XOSCHFCTL"]
pub type R = crate::R<u32, super::XOSCHFCTL>;
#[doc = "Writer for register XOSCHFCTL"]
pub type W = crate::W<u32, super::XOSCHFCTL>;
#[doc = "Register XOSCHFCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::XOSCHFCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PEAK_DET_ITRIM`"]
pub type PEAK_DET_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PEAK_DET_ITRIM`"]
pub struct PEAK_DET_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAK_DET_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
#[doc = "Reader of field `HP_BUF_ITRIM`"]
pub type HP_BUF_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HP_BUF_ITRIM`"]
pub struct HP_BUF_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HP_BUF_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `LP_BUF_ITRIM`"]
pub type LP_BUF_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LP_BUF_ITRIM`"]
pub struct LP_BUF_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_BUF_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - PEAK_DET_ITRIM"]
    #[inline(always)]
    pub fn peak_det_itrim(&self) -> PEAK_DET_ITRIM_R {
        PEAK_DET_ITRIM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 6 - BYPASS"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - HP_BUF_ITRIM"]
    #[inline(always)]
    pub fn hp_buf_itrim(&self) -> HP_BUF_ITRIM_R {
        HP_BUF_ITRIM_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 0:1 - LP_BUF_ITRIM"]
    #[inline(always)]
    pub fn lp_buf_itrim(&self) -> LP_BUF_ITRIM_R {
        LP_BUF_ITRIM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - PEAK_DET_ITRIM"]
    #[inline(always)]
    pub fn peak_det_itrim(&mut self) -> PEAK_DET_ITRIM_W {
        PEAK_DET_ITRIM_W { w: self }
    }
    #[doc = "Bit 6 - BYPASS"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bits 2:4 - HP_BUF_ITRIM"]
    #[inline(always)]
    pub fn hp_buf_itrim(&mut self) -> HP_BUF_ITRIM_W {
        HP_BUF_ITRIM_W { w: self }
    }
    #[doc = "Bits 0:1 - LP_BUF_ITRIM"]
    #[inline(always)]
    pub fn lp_buf_itrim(&mut self) -> LP_BUF_ITRIM_W {
        LP_BUF_ITRIM_W { w: self }
    }
}

#[doc = "Reader of register ANABYPASSVAL1"]
pub type R = crate::R<u32, super::ANABYPASSVAL1>;
#[doc = "Writer for register ANABYPASSVAL1"]
pub type W = crate::W<u32, super::ANABYPASSVAL1>;
#[doc = "Register ANABYPASSVAL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ANABYPASSVAL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XOSC_HF_ROW_Q12`"]
pub type XOSC_HF_ROW_Q12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XOSC_HF_ROW_Q12`"]
pub struct XOSC_HF_ROW_Q12_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_ROW_Q12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `XOSC_HF_COLUMN_Q12`"]
pub type XOSC_HF_COLUMN_Q12_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XOSC_HF_COLUMN_Q12`"]
pub struct XOSC_HF_COLUMN_Q12_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_COLUMN_Q12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - XOSC_HF_ROW_Q12"]
    #[inline(always)]
    pub fn xosc_hf_row_q12(&self) -> XOSC_HF_ROW_Q12_R {
        XOSC_HF_ROW_Q12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - XOSC_HF_COLUMN_Q12"]
    #[inline(always)]
    pub fn xosc_hf_column_q12(&self) -> XOSC_HF_COLUMN_Q12_R {
        XOSC_HF_COLUMN_Q12_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:19 - XOSC_HF_ROW_Q12"]
    #[inline(always)]
    pub fn xosc_hf_row_q12(&mut self) -> XOSC_HF_ROW_Q12_W {
        XOSC_HF_ROW_Q12_W { w: self }
    }
    #[doc = "Bits 0:15 - XOSC_HF_COLUMN_Q12"]
    #[inline(always)]
    pub fn xosc_hf_column_q12(&mut self) -> XOSC_HF_COLUMN_Q12_W {
        XOSC_HF_COLUMN_Q12_W { w: self }
    }
}

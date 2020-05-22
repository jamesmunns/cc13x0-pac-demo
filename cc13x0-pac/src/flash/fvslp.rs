#[doc = "Reader of register FVSLP"]
pub type R = crate::R<u32, super::FVSLP>;
#[doc = "Writer for register FVSLP"]
pub type W = crate::W<u32, super::FVSLP>;
#[doc = "Register FVSLP `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::FVSLP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "Reader of field `VSL_P`"]
pub type VSL_P_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VSL_P`"]
pub struct VSL_P_W<'a> {
    w: &'a mut W,
}
impl<'a> VSL_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - VSL_P"]
    #[inline(always)]
    pub fn vsl_p(&self) -> VSL_P_R {
        VSL_P_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - VSL_P"]
    #[inline(always)]
    pub fn vsl_p(&mut self) -> VSL_P_W {
        VSL_P_W { w: self }
    }
}

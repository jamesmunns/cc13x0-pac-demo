#[doc = "Reader of register FVHVCT2"]
pub type R = crate::R<u32, super::FVHVCT2>;
#[doc = "Writer for register FVHVCT2"]
pub type W = crate::W<u32, super::FVHVCT2>;
#[doc = "Register FVHVCT2 `reset()`'s with value 0x00a2_0000"]
impl crate::ResetValue for super::FVHVCT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00a2_0000
    }
}
#[doc = "Reader of field `TRIM13_P`"]
pub type TRIM13_P_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM13_P`"]
pub struct TRIM13_P_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM13_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `VHVCT_P`"]
pub type VHVCT_P_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VHVCT_P`"]
pub struct VHVCT_P_W<'a> {
    w: &'a mut W,
}
impl<'a> VHVCT_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:23 - TRIM13_P"]
    #[inline(always)]
    pub fn trim13_p(&self) -> TRIM13_P_R {
        TRIM13_P_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - VHVCT_P"]
    #[inline(always)]
    pub fn vhvct_p(&self) -> VHVCT_P_R {
        VHVCT_P_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - TRIM13_P"]
    #[inline(always)]
    pub fn trim13_p(&mut self) -> TRIM13_P_W {
        TRIM13_P_W { w: self }
    }
    #[doc = "Bits 16:19 - VHVCT_P"]
    #[inline(always)]
    pub fn vhvct_p(&mut self) -> VHVCT_P_W {
        VHVCT_P_W { w: self }
    }
}

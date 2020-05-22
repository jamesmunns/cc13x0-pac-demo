#[doc = "Reader of register FVHVCT3"]
pub type R = crate::R<u32, super::FVHVCT3>;
#[doc = "Writer for register FVHVCT3"]
pub type W = crate::W<u32, super::FVHVCT3>;
#[doc = "Register FVHVCT3 `reset()`'s with value 0x000f_0000"]
impl crate::ResetValue for super::FVHVCT3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000f_0000
    }
}
#[doc = "Reader of field `WCT`"]
pub type WCT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WCT`"]
pub struct WCT_W<'a> {
    w: &'a mut W,
}
impl<'a> WCT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `VHVCT_READ`"]
pub type VHVCT_READ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VHVCT_READ`"]
pub struct VHVCT_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> VHVCT_READ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - WCT"]
    #[inline(always)]
    pub fn wct(&self) -> WCT_R {
        WCT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - VHVCT_READ"]
    #[inline(always)]
    pub fn vhvct_read(&self) -> VHVCT_READ_R {
        VHVCT_READ_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - WCT"]
    #[inline(always)]
    pub fn wct(&mut self) -> WCT_W {
        WCT_W { w: self }
    }
    #[doc = "Bits 0:3 - VHVCT_READ"]
    #[inline(always)]
    pub fn vhvct_read(&mut self) -> VHVCT_READ_W {
        VHVCT_READ_W { w: self }
    }
}

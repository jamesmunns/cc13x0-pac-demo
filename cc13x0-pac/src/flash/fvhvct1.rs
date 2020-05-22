#[doc = "Reader of register FVHVCT1"]
pub type R = crate::R<u32, super::FVHVCT1>;
#[doc = "Writer for register FVHVCT1"]
pub type W = crate::W<u32, super::FVHVCT1>;
#[doc = "Register FVHVCT1 `reset()`'s with value 0x0084_0088"]
impl crate::ResetValue for super::FVHVCT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0084_0088
    }
}
#[doc = "Reader of field `TRIM13_E`"]
pub type TRIM13_E_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM13_E`"]
pub struct TRIM13_E_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM13_E_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `VHVCT_E`"]
pub type VHVCT_E_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VHVCT_E`"]
pub struct VHVCT_E_W<'a> {
    w: &'a mut W,
}
impl<'a> VHVCT_E_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `TRIM13_PV`"]
pub type TRIM13_PV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM13_PV`"]
pub struct TRIM13_PV_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM13_PV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `VHVCT_PV`"]
pub type VHVCT_PV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VHVCT_PV`"]
pub struct VHVCT_PV_W<'a> {
    w: &'a mut W,
}
impl<'a> VHVCT_PV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:23 - TRIM13_E"]
    #[inline(always)]
    pub fn trim13_e(&self) -> TRIM13_E_R {
        TRIM13_E_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - VHVCT_E"]
    #[inline(always)]
    pub fn vhvct_e(&self) -> VHVCT_E_R {
        VHVCT_E_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - TRIM13_PV"]
    #[inline(always)]
    pub fn trim13_pv(&self) -> TRIM13_PV_R {
        TRIM13_PV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - VHVCT_PV"]
    #[inline(always)]
    pub fn vhvct_pv(&self) -> VHVCT_PV_R {
        VHVCT_PV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - TRIM13_E"]
    #[inline(always)]
    pub fn trim13_e(&mut self) -> TRIM13_E_W {
        TRIM13_E_W { w: self }
    }
    #[doc = "Bits 16:19 - VHVCT_E"]
    #[inline(always)]
    pub fn vhvct_e(&mut self) -> VHVCT_E_W {
        VHVCT_E_W { w: self }
    }
    #[doc = "Bits 4:7 - TRIM13_PV"]
    #[inline(always)]
    pub fn trim13_pv(&mut self) -> TRIM13_PV_W {
        TRIM13_PV_W { w: self }
    }
    #[doc = "Bits 0:3 - VHVCT_PV"]
    #[inline(always)]
    pub fn vhvct_pv(&mut self) -> VHVCT_PV_W {
        VHVCT_PV_W { w: self }
    }
}

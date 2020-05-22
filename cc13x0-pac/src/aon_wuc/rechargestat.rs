#[doc = "Reader of register RECHARGESTAT"]
pub type R = crate::R<u32, super::RECHARGESTAT>;
#[doc = "Writer for register RECHARGESTAT"]
pub type W = crate::W<u32, super::RECHARGESTAT>;
#[doc = "Register RECHARGESTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::RECHARGESTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VDDR_SMPLS`"]
pub type VDDR_SMPLS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VDDR_SMPLS`"]
pub struct VDDR_SMPLS_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_SMPLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MAX_USED_PER`"]
pub type MAX_USED_PER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAX_USED_PER`"]
pub struct MAX_USED_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_USED_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - VDDR_SMPLS"]
    #[inline(always)]
    pub fn vddr_smpls(&self) -> VDDR_SMPLS_R {
        VDDR_SMPLS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - MAX_USED_PER"]
    #[inline(always)]
    pub fn max_used_per(&self) -> MAX_USED_PER_R {
        MAX_USED_PER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:19 - VDDR_SMPLS"]
    #[inline(always)]
    pub fn vddr_smpls(&mut self) -> VDDR_SMPLS_W {
        VDDR_SMPLS_W { w: self }
    }
    #[doc = "Bits 0:15 - MAX_USED_PER"]
    #[inline(always)]
    pub fn max_used_per(&mut self) -> MAX_USED_PER_W {
        MAX_USED_PER_W { w: self }
    }
}

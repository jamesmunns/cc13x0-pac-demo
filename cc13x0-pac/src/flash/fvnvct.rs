#[doc = "Reader of register FVNVCT"]
pub type R = crate::R<u32, super::FVNVCT>;
#[doc = "Writer for register FVNVCT"]
pub type W = crate::W<u32, super::FVNVCT>;
#[doc = "Register FVNVCT `reset()`'s with value 0x0800"]
impl crate::ResetValue for super::FVNVCT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0800
    }
}
#[doc = "Reader of field `VCG2P5CT`"]
pub type VCG2P5CT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VCG2P5CT`"]
pub struct VCG2P5CT_W<'a> {
    w: &'a mut W,
}
impl<'a> VCG2P5CT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `VIN_CT`"]
pub type VIN_CT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VIN_CT`"]
pub struct VIN_CT_W<'a> {
    w: &'a mut W,
}
impl<'a> VIN_CT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:12 - VCG2P5CT"]
    #[inline(always)]
    pub fn vcg2p5ct(&self) -> VCG2P5CT_R {
        VCG2P5CT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - VIN_CT"]
    #[inline(always)]
    pub fn vin_ct(&self) -> VIN_CT_R {
        VIN_CT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - VCG2P5CT"]
    #[inline(always)]
    pub fn vcg2p5ct(&mut self) -> VCG2P5CT_W {
        VCG2P5CT_W { w: self }
    }
    #[doc = "Bits 0:4 - VIN_CT"]
    #[inline(always)]
    pub fn vin_ct(&mut self) -> VIN_CT_W {
        VIN_CT_W { w: self }
    }
}

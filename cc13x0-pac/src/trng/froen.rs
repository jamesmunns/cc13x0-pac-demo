#[doc = "Reader of register FROEN"]
pub type R = crate::R<u32, super::FROEN>;
#[doc = "Writer for register FROEN"]
pub type W = crate::W<u32, super::FROEN>;
#[doc = "Register FROEN `reset()`'s with value 0x00ff_ffff"]
impl crate::ResetValue for super::FROEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00ff_ffff
    }
}
#[doc = "Reader of field `FRO_MASK`"]
pub type FRO_MASK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FRO_MASK`"]
pub struct FRO_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - FRO_MASK"]
    #[inline(always)]
    pub fn fro_mask(&self) -> FRO_MASK_R {
        FRO_MASK_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - FRO_MASK"]
    #[inline(always)]
    pub fn fro_mask(&mut self) -> FRO_MASK_W {
        FRO_MASK_W { w: self }
    }
}

#[doc = "Reader of register ALARMMASK"]
pub type R = crate::R<u32, super::ALARMMASK>;
#[doc = "Writer for register ALARMMASK"]
pub type W = crate::W<u32, super::ALARMMASK>;
#[doc = "Register ALARMMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::ALARMMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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

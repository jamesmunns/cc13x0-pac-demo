#[doc = "Reader of register DYN_CG"]
pub type R = crate::R<u32, super::DYN_CG>;
#[doc = "Writer for register DYN_CG"]
pub type W = crate::W<u32, super::DYN_CG>;
#[doc = "Register DYN_CG `reset()`'s with value 0"]
impl crate::ResetValue for super::DYN_CG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DYN_CG`"]
pub type DYN_CG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DYN_CG`"]
pub struct DYN_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> DYN_CG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - DYN_CG"]
    #[inline(always)]
    pub fn dyn_cg(&self) -> DYN_CG_R {
        DYN_CG_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DYN_CG"]
    #[inline(always)]
    pub fn dyn_cg(&mut self) -> DYN_CG_W {
        DYN_CG_W { w: self }
    }
}

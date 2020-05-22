#[doc = "Reader of register FBSE"]
pub type R = crate::R<u32, super::FBSE>;
#[doc = "Writer for register FBSE"]
pub type W = crate::W<u32, super::FBSE>;
#[doc = "Register FBSE `reset()`'s with value 0"]
impl crate::ResetValue for super::FBSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BSE`"]
pub type BSE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BSE`"]
pub struct BSE_W<'a> {
    w: &'a mut W,
}
impl<'a> BSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - BSE"]
    #[inline(always)]
    pub fn bse(&self) -> BSE_R {
        BSE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BSE"]
    #[inline(always)]
    pub fn bse(&mut self) -> BSE_W {
        BSE_W { w: self }
    }
}

#[doc = "Reader of register KEYSIZE"]
pub type R = crate::R<u32, super::KEYSIZE>;
#[doc = "Writer for register KEYSIZE"]
pub type W = crate::W<u32, super::KEYSIZE>;
#[doc = "Register KEYSIZE `reset()`'s with value 0x01"]
impl crate::ResetValue for super::KEYSIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SIZE"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SIZE"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
}

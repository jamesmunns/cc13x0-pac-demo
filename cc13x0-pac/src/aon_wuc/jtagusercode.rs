#[doc = "Reader of register JTAGUSERCODE"]
pub type R = crate::R<u32, super::JTAGUSERCODE>;
#[doc = "Writer for register JTAGUSERCODE"]
pub type W = crate::W<u32, super::JTAGUSERCODE>;
#[doc = "Register JTAGUSERCODE `reset()`'s with value 0x0b99_a02f"]
impl crate::ResetValue for super::JTAGUSERCODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0b99_a02f
    }
}
#[doc = "Reader of field `USER_CODE`"]
pub type USER_CODE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `USER_CODE`"]
pub struct USER_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> USER_CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - USER_CODE"]
    #[inline(always)]
    pub fn user_code(&self) -> USER_CODE_R {
        USER_CODE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - USER_CODE"]
    #[inline(always)]
    pub fn user_code(&mut self) -> USER_CODE_W {
        USER_CODE_W { w: self }
    }
}

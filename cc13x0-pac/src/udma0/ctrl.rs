#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BASEPTR`"]
pub type BASEPTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BASEPTR`"]
pub struct BASEPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> BASEPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - BASEPTR"]
    #[inline(always)]
    pub fn baseptr(&self) -> BASEPTR_R {
        BASEPTR_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 10:31 - BASEPTR"]
    #[inline(always)]
    pub fn baseptr(&mut self) -> BASEPTR_W {
        BASEPTR_W { w: self }
    }
}

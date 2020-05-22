#[doc = "Reader of register FRDCTL"]
pub type R = crate::R<u32, super::FRDCTL>;
#[doc = "Writer for register FRDCTL"]
pub type W = crate::W<u32, super::FRDCTL>;
#[doc = "Register FRDCTL `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::FRDCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "Reader of field `RWAIT`"]
pub type RWAIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RWAIT`"]
pub struct RWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - RWAIT"]
    #[inline(always)]
    pub fn rwait(&self) -> RWAIT_R {
        RWAIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - RWAIT"]
    #[inline(always)]
    pub fn rwait(&mut self) -> RWAIT_W {
        RWAIT_W { w: self }
    }
}

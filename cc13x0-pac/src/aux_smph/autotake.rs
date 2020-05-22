#[doc = "Reader of register AUTOTAKE"]
pub type R = crate::R<u32, super::AUTOTAKE>;
#[doc = "Writer for register AUTOTAKE"]
pub type W = crate::W<u32, super::AUTOTAKE>;
#[doc = "Register AUTOTAKE `reset()`'s with value 0"]
impl crate::ResetValue for super::AUTOTAKE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMPH_ID`"]
pub type SMPH_ID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMPH_ID`"]
pub struct SMPH_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPH_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SMPH_ID"]
    #[inline(always)]
    pub fn smph_id(&self) -> SMPH_ID_R {
        SMPH_ID_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMPH_ID"]
    #[inline(always)]
    pub fn smph_id(&mut self) -> SMPH_ID_W {
        SMPH_ID_W { w: self }
    }
}

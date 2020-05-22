#[doc = "Reader of register FSM_P_OH"]
pub type R = crate::R<u32, super::FSM_P_OH>;
#[doc = "Writer for register FSM_P_OH"]
pub type W = crate::W<u32, super::FSM_P_OH>;
#[doc = "Register FSM_P_OH `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::FSM_P_OH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `PGM_OH`"]
pub type PGM_OH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PGM_OH`"]
pub struct PGM_OH_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_OH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - PGM_OH"]
    #[inline(always)]
    pub fn pgm_oh(&self) -> PGM_OH_R {
        PGM_OH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - PGM_OH"]
    #[inline(always)]
    pub fn pgm_oh(&mut self) -> PGM_OH_W {
        PGM_OH_W { w: self }
    }
}

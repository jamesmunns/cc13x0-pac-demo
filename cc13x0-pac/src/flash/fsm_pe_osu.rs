#[doc = "Reader of register FSM_PE_OSU"]
pub type R = crate::R<u32, super::FSM_PE_OSU>;
#[doc = "Writer for register FSM_PE_OSU"]
pub type W = crate::W<u32, super::FSM_PE_OSU>;
#[doc = "Register FSM_PE_OSU `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_PE_OSU {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PGM_OSU`"]
pub type PGM_OSU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PGM_OSU`"]
pub struct PGM_OSU_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_OSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ERA_OSU`"]
pub type ERA_OSU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ERA_OSU`"]
pub struct ERA_OSU_W<'a> {
    w: &'a mut W,
}
impl<'a> ERA_OSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - PGM_OSU"]
    #[inline(always)]
    pub fn pgm_osu(&self) -> PGM_OSU_R {
        PGM_OSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - ERA_OSU"]
    #[inline(always)]
    pub fn era_osu(&self) -> ERA_OSU_R {
        ERA_OSU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - PGM_OSU"]
    #[inline(always)]
    pub fn pgm_osu(&mut self) -> PGM_OSU_W {
        PGM_OSU_W { w: self }
    }
    #[doc = "Bits 0:7 - ERA_OSU"]
    #[inline(always)]
    pub fn era_osu(&mut self) -> ERA_OSU_W {
        ERA_OSU_W { w: self }
    }
}

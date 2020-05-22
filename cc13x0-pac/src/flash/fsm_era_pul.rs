#[doc = "Reader of register FSM_ERA_PUL"]
pub type R = crate::R<u32, super::FSM_ERA_PUL>;
#[doc = "Writer for register FSM_ERA_PUL"]
pub type W = crate::W<u32, super::FSM_ERA_PUL>;
#[doc = "Register FSM_ERA_PUL `reset()`'s with value 0x0004_0bb8"]
impl crate::ResetValue for super::FSM_ERA_PUL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0004_0bb8
    }
}
#[doc = "Reader of field `MAX_EC_LEVEL`"]
pub type MAX_EC_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAX_EC_LEVEL`"]
pub struct MAX_EC_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_EC_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MAX_ERA_PUL`"]
pub type MAX_ERA_PUL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAX_ERA_PUL`"]
pub struct MAX_ERA_PUL_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_ERA_PUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - MAX_EC_LEVEL"]
    #[inline(always)]
    pub fn max_ec_level(&self) -> MAX_EC_LEVEL_R {
        MAX_EC_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11 - MAX_ERA_PUL"]
    #[inline(always)]
    pub fn max_era_pul(&self) -> MAX_ERA_PUL_R {
        MAX_ERA_PUL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:19 - MAX_EC_LEVEL"]
    #[inline(always)]
    pub fn max_ec_level(&mut self) -> MAX_EC_LEVEL_W {
        MAX_EC_LEVEL_W { w: self }
    }
    #[doc = "Bits 0:11 - MAX_ERA_PUL"]
    #[inline(always)]
    pub fn max_era_pul(&mut self) -> MAX_ERA_PUL_W {
        MAX_ERA_PUL_W { w: self }
    }
}

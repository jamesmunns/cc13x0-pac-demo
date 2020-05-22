#[doc = "Reader of register FSM_PRG_PUL"]
pub type R = crate::R<u32, super::FSM_PRG_PUL>;
#[doc = "Writer for register FSM_PRG_PUL"]
pub type W = crate::W<u32, super::FSM_PRG_PUL>;
#[doc = "Register FSM_PRG_PUL `reset()`'s with value 0x0004_0032"]
impl crate::ResetValue for super::FSM_PRG_PUL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0004_0032
    }
}
#[doc = "Reader of field `BEG_EC_LEVEL`"]
pub type BEG_EC_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BEG_EC_LEVEL`"]
pub struct BEG_EC_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BEG_EC_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `MAX_PRG_PUL`"]
pub type MAX_PRG_PUL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAX_PRG_PUL`"]
pub struct MAX_PRG_PUL_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_PRG_PUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - BEG_EC_LEVEL"]
    #[inline(always)]
    pub fn beg_ec_level(&self) -> BEG_EC_LEVEL_R {
        BEG_EC_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11 - MAX_PRG_PUL"]
    #[inline(always)]
    pub fn max_prg_pul(&self) -> MAX_PRG_PUL_R {
        MAX_PRG_PUL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:19 - BEG_EC_LEVEL"]
    #[inline(always)]
    pub fn beg_ec_level(&mut self) -> BEG_EC_LEVEL_W {
        BEG_EC_LEVEL_W { w: self }
    }
    #[doc = "Bits 0:11 - MAX_PRG_PUL"]
    #[inline(always)]
    pub fn max_prg_pul(&mut self) -> MAX_PRG_PUL_W {
        MAX_PRG_PUL_W { w: self }
    }
}

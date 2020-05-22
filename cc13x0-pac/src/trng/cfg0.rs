#[doc = "Reader of register CFG0"]
pub type R = crate::R<u32, super::CFG0>;
#[doc = "Writer for register CFG0"]
pub type W = crate::W<u32, super::CFG0>;
#[doc = "Register CFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAX_REFILL_CYCLES`"]
pub type MAX_REFILL_CYCLES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAX_REFILL_CYCLES`"]
pub struct MAX_REFILL_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_REFILL_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SMPL_DIV`"]
pub type SMPL_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMPL_DIV`"]
pub struct SMPL_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPL_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MIN_REFILL_CYCLES`"]
pub type MIN_REFILL_CYCLES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIN_REFILL_CYCLES`"]
pub struct MIN_REFILL_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_REFILL_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - MAX_REFILL_CYCLES"]
    #[inline(always)]
    pub fn max_refill_cycles(&self) -> MAX_REFILL_CYCLES_R {
        MAX_REFILL_CYCLES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:11 - SMPL_DIV"]
    #[inline(always)]
    pub fn smpl_div(&self) -> SMPL_DIV_R {
        SMPL_DIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - MIN_REFILL_CYCLES"]
    #[inline(always)]
    pub fn min_refill_cycles(&self) -> MIN_REFILL_CYCLES_R {
        MIN_REFILL_CYCLES_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - MAX_REFILL_CYCLES"]
    #[inline(always)]
    pub fn max_refill_cycles(&mut self) -> MAX_REFILL_CYCLES_W {
        MAX_REFILL_CYCLES_W { w: self }
    }
    #[doc = "Bits 8:11 - SMPL_DIV"]
    #[inline(always)]
    pub fn smpl_div(&mut self) -> SMPL_DIV_W {
        SMPL_DIV_W { w: self }
    }
    #[doc = "Bits 0:7 - MIN_REFILL_CYCLES"]
    #[inline(always)]
    pub fn min_refill_cycles(&mut self) -> MIN_REFILL_CYCLES_W {
        MIN_REFILL_CYCLES_W { w: self }
    }
}

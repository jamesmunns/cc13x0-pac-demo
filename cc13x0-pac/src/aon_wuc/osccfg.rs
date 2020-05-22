#[doc = "Reader of register OSCCFG"]
pub type R = crate::R<u32, super::OSCCFG>;
#[doc = "Writer for register OSCCFG"]
pub type W = crate::W<u32, super::OSCCFG>;
#[doc = "Register OSCCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::OSCCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PER_M`"]
pub type PER_M_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PER_M`"]
pub struct PER_M_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `PER_E`"]
pub type PER_E_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PER_E`"]
pub struct PER_E_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_E_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - PER_M"]
    #[inline(always)]
    pub fn per_m(&self) -> PER_M_R {
        PER_M_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 0:2 - PER_E"]
    #[inline(always)]
    pub fn per_e(&self) -> PER_E_R {
        PER_E_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - PER_M"]
    #[inline(always)]
    pub fn per_m(&mut self) -> PER_M_W {
        PER_M_W { w: self }
    }
    #[doc = "Bits 0:2 - PER_E"]
    #[inline(always)]
    pub fn per_e(&mut self) -> PER_E_W {
        PER_E_W { w: self }
    }
}

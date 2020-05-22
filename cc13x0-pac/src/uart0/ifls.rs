#[doc = "Reader of register IFLS"]
pub type R = crate::R<u32, super::IFLS>;
#[doc = "Writer for register IFLS"]
pub type W = crate::W<u32, super::IFLS>;
#[doc = "Register IFLS `reset()`'s with value 0x12"]
impl crate::ResetValue for super::IFLS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x12
    }
}
#[doc = "Reader of field `RXSEL`"]
pub type RXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXSEL`"]
pub struct RXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `TXSEL`"]
pub type TXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXSEL`"]
pub struct TXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:5 - RXSEL"]
    #[inline(always)]
    pub fn rxsel(&self) -> RXSEL_R {
        RXSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - TXSEL"]
    #[inline(always)]
    pub fn txsel(&self) -> TXSEL_R {
        TXSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - RXSEL"]
    #[inline(always)]
    pub fn rxsel(&mut self) -> RXSEL_W {
        RXSEL_W { w: self }
    }
    #[doc = "Bits 0:2 - TXSEL"]
    #[inline(always)]
    pub fn txsel(&mut self) -> TXSEL_W {
        TXSEL_W { w: self }
    }
}

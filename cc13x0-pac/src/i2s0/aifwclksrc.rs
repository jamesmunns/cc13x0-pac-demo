#[doc = "Reader of register AIFWCLKSRC"]
pub type R = crate::R<u32, super::AIFWCLKSRC>;
#[doc = "Writer for register AIFWCLKSRC"]
pub type W = crate::W<u32, super::AIFWCLKSRC>;
#[doc = "Register AIFWCLKSRC `reset()`'s with value 0"]
impl crate::ResetValue for super::AIFWCLKSRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WCLK_INV`"]
pub type WCLK_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WCLK_INV`"]
pub struct WCLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> WCLK_INV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `WCLK_SRC`"]
pub type WCLK_SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WCLK_SRC`"]
pub struct WCLK_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> WCLK_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - WCLK_INV"]
    #[inline(always)]
    pub fn wclk_inv(&self) -> WCLK_INV_R {
        WCLK_INV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - WCLK_SRC"]
    #[inline(always)]
    pub fn wclk_src(&self) -> WCLK_SRC_R {
        WCLK_SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - WCLK_INV"]
    #[inline(always)]
    pub fn wclk_inv(&mut self) -> WCLK_INV_W {
        WCLK_INV_W { w: self }
    }
    #[doc = "Bits 0:1 - WCLK_SRC"]
    #[inline(always)]
    pub fn wclk_src(&mut self) -> WCLK_SRC_W {
        WCLK_SRC_W { w: self }
    }
}

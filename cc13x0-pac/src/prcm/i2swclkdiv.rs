#[doc = "Reader of register I2SWCLKDIV"]
pub type R = crate::R<u32, super::I2SWCLKDIV>;
#[doc = "Writer for register I2SWCLKDIV"]
pub type W = crate::W<u32, super::I2SWCLKDIV>;
#[doc = "Register I2SWCLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::I2SWCLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDIV`"]
pub type WDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDIV`"]
pub struct WDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> WDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - WDIV"]
    #[inline(always)]
    pub fn wdiv(&self) -> WDIV_R {
        WDIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDIV"]
    #[inline(always)]
    pub fn wdiv(&mut self) -> WDIV_W {
        WDIV_W { w: self }
    }
}

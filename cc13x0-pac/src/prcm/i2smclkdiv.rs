#[doc = "Reader of register I2SMCLKDIV"]
pub type R = crate::R<u32, super::I2SMCLKDIV>;
#[doc = "Writer for register I2SMCLKDIV"]
pub type W = crate::W<u32, super::I2SMCLKDIV>;
#[doc = "Register I2SMCLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::I2SMCLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MDIV`"]
pub type MDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MDIV`"]
pub struct MDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - MDIV"]
    #[inline(always)]
    pub fn mdiv(&self) -> MDIV_R {
        MDIV_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - MDIV"]
    #[inline(always)]
    pub fn mdiv(&mut self) -> MDIV_W {
        MDIV_W { w: self }
    }
}

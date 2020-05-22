#[doc = "Reader of register I2SBCLKDIV"]
pub type R = crate::R<u32, super::I2SBCLKDIV>;
#[doc = "Writer for register I2SBCLKDIV"]
pub type W = crate::W<u32, super::I2SBCLKDIV>;
#[doc = "Register I2SBCLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::I2SBCLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BDIV`"]
pub type BDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BDIV`"]
pub struct BDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> BDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - BDIV"]
    #[inline(always)]
    pub fn bdiv(&self) -> BDIV_R {
        BDIV_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - BDIV"]
    #[inline(always)]
    pub fn bdiv(&mut self) -> BDIV_W {
        BDIV_W { w: self }
    }
}

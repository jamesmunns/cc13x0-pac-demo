#[doc = "Reader of register GPIODOUT"]
pub type R = crate::R<u32, super::GPIODOUT>;
#[doc = "Writer for register GPIODOUT"]
pub type W = crate::W<u32, super::GPIODOUT>;
#[doc = "Register GPIODOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIODOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IO7_0`"]
pub type IO7_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO7_0`"]
pub struct IO7_0_W<'a> {
    w: &'a mut W,
}
impl<'a> IO7_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - IO7_0"]
    #[inline(always)]
    pub fn io7_0(&self) -> IO7_0_R {
        IO7_0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IO7_0"]
    #[inline(always)]
    pub fn io7_0(&mut self) -> IO7_0_W {
        IO7_0_W { w: self }
    }
}

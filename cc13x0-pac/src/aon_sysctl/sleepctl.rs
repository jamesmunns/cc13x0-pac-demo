#[doc = "Reader of register SLEEPCTL"]
pub type R = crate::R<u32, super::SLEEPCTL>;
#[doc = "Writer for register SLEEPCTL"]
pub type W = crate::W<u32, super::SLEEPCTL>;
#[doc = "Register SLEEPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::SLEEPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IO_PAD_SLEEP_DIS`"]
pub type IO_PAD_SLEEP_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IO_PAD_SLEEP_DIS`"]
pub struct IO_PAD_SLEEP_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_PAD_SLEEP_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IO_PAD_SLEEP_DIS"]
    #[inline(always)]
    pub fn io_pad_sleep_dis(&self) -> IO_PAD_SLEEP_DIS_R {
        IO_PAD_SLEEP_DIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO_PAD_SLEEP_DIS"]
    #[inline(always)]
    pub fn io_pad_sleep_dis(&mut self) -> IO_PAD_SLEEP_DIS_W {
        IO_PAD_SLEEP_DIS_W { w: self }
    }
}

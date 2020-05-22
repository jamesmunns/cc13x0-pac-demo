#[doc = "Reader of register SWRESET"]
pub type R = crate::R<u32, super::SWRESET>;
#[doc = "Writer for register SWRESET"]
pub type W = crate::W<u32, super::SWRESET>;
#[doc = "Register SWRESET `reset()`'s with value 0"]
impl crate::ResetValue for super::SWRESET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCU`"]
pub type MCU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCU`"]
pub struct MCU_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_W<'a> {
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
impl R {
    #[doc = "Bit 2 - MCU"]
    #[inline(always)]
    pub fn mcu(&self) -> MCU_R {
        MCU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - MCU"]
    #[inline(always)]
    pub fn mcu(&mut self) -> MCU_W {
        MCU_W { w: self }
    }
}

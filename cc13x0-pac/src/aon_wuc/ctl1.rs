#[doc = "Reader of register CTL1"]
pub type R = crate::R<u32, super::CTL1>;
#[doc = "Writer for register CTL1"]
pub type W = crate::W<u32, super::CTL1>;
#[doc = "Register CTL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCU_RESET_SRC`"]
pub type MCU_RESET_SRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCU_RESET_SRC`"]
pub struct MCU_RESET_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_RESET_SRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `MCU_WARM_RESET`"]
pub type MCU_WARM_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCU_WARM_RESET`"]
pub struct MCU_WARM_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_WARM_RESET_W<'a> {
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
    #[doc = "Bit 1 - MCU_RESET_SRC"]
    #[inline(always)]
    pub fn mcu_reset_src(&self) -> MCU_RESET_SRC_R {
        MCU_RESET_SRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - MCU_WARM_RESET"]
    #[inline(always)]
    pub fn mcu_warm_reset(&self) -> MCU_WARM_RESET_R {
        MCU_WARM_RESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - MCU_RESET_SRC"]
    #[inline(always)]
    pub fn mcu_reset_src(&mut self) -> MCU_RESET_SRC_W {
        MCU_RESET_SRC_W { w: self }
    }
    #[doc = "Bit 0 - MCU_WARM_RESET"]
    #[inline(always)]
    pub fn mcu_warm_reset(&mut self) -> MCU_WARM_RESET_W {
        MCU_WARM_RESET_W { w: self }
    }
}

#[doc = "Reader of register CTL0"]
pub type R = crate::R<u32, super::CTL0>;
#[doc = "Writer for register CTL0"]
pub type W = crate::W<u32, super::CTL0>;
#[doc = "Register CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWR_DWN_DIS`"]
pub type PWR_DWN_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWR_DWN_DIS`"]
pub struct PWR_DWN_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_DWN_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `AUX_SRAM_ERASE`"]
pub type AUX_SRAM_ERASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_SRAM_ERASE`"]
pub struct AUX_SRAM_ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_SRAM_ERASE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `MCU_SRAM_ERASE`"]
pub type MCU_SRAM_ERASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCU_SRAM_ERASE`"]
pub struct MCU_SRAM_ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_SRAM_ERASE_W<'a> {
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
    #[doc = "Bit 8 - PWR_DWN_DIS"]
    #[inline(always)]
    pub fn pwr_dwn_dis(&self) -> PWR_DWN_DIS_R {
        PWR_DWN_DIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AUX_SRAM_ERASE"]
    #[inline(always)]
    pub fn aux_sram_erase(&self) -> AUX_SRAM_ERASE_R {
        AUX_SRAM_ERASE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MCU_SRAM_ERASE"]
    #[inline(always)]
    pub fn mcu_sram_erase(&self) -> MCU_SRAM_ERASE_R {
        MCU_SRAM_ERASE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - PWR_DWN_DIS"]
    #[inline(always)]
    pub fn pwr_dwn_dis(&mut self) -> PWR_DWN_DIS_W {
        PWR_DWN_DIS_W { w: self }
    }
    #[doc = "Bit 3 - AUX_SRAM_ERASE"]
    #[inline(always)]
    pub fn aux_sram_erase(&mut self) -> AUX_SRAM_ERASE_W {
        AUX_SRAM_ERASE_W { w: self }
    }
    #[doc = "Bit 2 - MCU_SRAM_ERASE"]
    #[inline(always)]
    pub fn mcu_sram_erase(&mut self) -> MCU_SRAM_ERASE_W {
        MCU_SRAM_ERASE_W { w: self }
    }
}

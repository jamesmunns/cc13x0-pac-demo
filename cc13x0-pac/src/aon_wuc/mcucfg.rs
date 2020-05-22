#[doc = "Reader of register MCUCFG"]
pub type R = crate::R<u32, super::MCUCFG>;
#[doc = "Writer for register MCUCFG"]
pub type W = crate::W<u32, super::MCUCFG>;
#[doc = "Register MCUCFG `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::MCUCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `VIRT_OFF`"]
pub type VIRT_OFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VIRT_OFF`"]
pub struct VIRT_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> VIRT_OFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `FIXED_WU_EN`"]
pub type FIXED_WU_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIXED_WU_EN`"]
pub struct FIXED_WU_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXED_WU_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SRAM_RET_EN`"]
pub type SRAM_RET_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SRAM_RET_EN`"]
pub struct SRAM_RET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_RET_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - VIRT_OFF"]
    #[inline(always)]
    pub fn virt_off(&self) -> VIRT_OFF_R {
        VIRT_OFF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FIXED_WU_EN"]
    #[inline(always)]
    pub fn fixed_wu_en(&self) -> FIXED_WU_EN_R {
        FIXED_WU_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - SRAM_RET_EN"]
    #[inline(always)]
    pub fn sram_ret_en(&self) -> SRAM_RET_EN_R {
        SRAM_RET_EN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 17 - VIRT_OFF"]
    #[inline(always)]
    pub fn virt_off(&mut self) -> VIRT_OFF_W {
        VIRT_OFF_W { w: self }
    }
    #[doc = "Bit 16 - FIXED_WU_EN"]
    #[inline(always)]
    pub fn fixed_wu_en(&mut self) -> FIXED_WU_EN_W {
        FIXED_WU_EN_W { w: self }
    }
    #[doc = "Bits 0:3 - SRAM_RET_EN"]
    #[inline(always)]
    pub fn sram_ret_en(&mut self) -> SRAM_RET_EN_W {
        SRAM_RET_EN_W { w: self }
    }
}

#[doc = "Reader of register VDCTL"]
pub type R = crate::R<u32, super::VDCTL>;
#[doc = "Writer for register VDCTL"]
pub type W = crate::W<u32, super::VDCTL>;
#[doc = "Register VDCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::VDCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCU_VD`"]
pub type MCU_VD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCU_VD`"]
pub struct MCU_VD_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_VD_W<'a> {
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
#[doc = "Reader of field `ULDO`"]
pub type ULDO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ULDO`"]
pub struct ULDO_W<'a> {
    w: &'a mut W,
}
impl<'a> ULDO_W<'a> {
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
    #[doc = "Bit 2 - MCU_VD"]
    #[inline(always)]
    pub fn mcu_vd(&self) -> MCU_VD_R {
        MCU_VD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ULDO"]
    #[inline(always)]
    pub fn uldo(&self) -> ULDO_R {
        ULDO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - MCU_VD"]
    #[inline(always)]
    pub fn mcu_vd(&mut self) -> MCU_VD_W {
        MCU_VD_W { w: self }
    }
    #[doc = "Bit 0 - ULDO"]
    #[inline(always)]
    pub fn uldo(&mut self) -> ULDO_W {
        ULDO_W { w: self }
    }
}

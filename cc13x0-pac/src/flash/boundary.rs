#[doc = "Writer for register BOUNDARY"]
pub type W = crate::W<u32, super::BOUNDARY>;
#[doc = "Register BOUNDARY `reset()`'s with value 0"]
impl crate::ResetValue for super::BOUNDARY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DISROW0`"]
pub struct DISROW0_W<'a> {
    w: &'a mut W,
}
impl<'a> DISROW0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Write proxy for field `SPARE`"]
pub struct SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Write proxy for field `EFC_SELF_TEST_ERROR`"]
pub struct EFC_SELF_TEST_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFC_SELF_TEST_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Write proxy for field `EFC_INSTRUCTION_INFO`"]
pub struct EFC_INSTRUCTION_INFO_W<'a> {
    w: &'a mut W,
}
impl<'a> EFC_INSTRUCTION_INFO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Write proxy for field `EFC_INSTRUCTION_ERROR`"]
pub struct EFC_INSTRUCTION_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFC_INSTRUCTION_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write proxy for field `EFC_AUTOLOAD_ERROR`"]
pub struct EFC_AUTOLOAD_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFC_AUTOLOAD_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `OUTPUTENABLE`"]
pub struct OUTPUTENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUTENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | (((value as u32) & 0x0f) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `SYS_ECC_SELF_TEST_EN`"]
pub struct SYS_ECC_SELF_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_ECC_SELF_TEST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `SYS_ECC_OVERRIDE_EN`"]
pub struct SYS_ECC_OVERRIDE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_ECC_OVERRIDE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `EFC_FDI`"]
pub struct EFC_FDI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFC_FDI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `SYS_DIEID_AUTOLOAD_EN`"]
pub struct SYS_DIEID_AUTOLOAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_DIEID_AUTOLOAD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `SYS_REPAIR_EN`"]
pub struct SYS_REPAIR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_REPAIR_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `SYS_WS_READ_STATES`"]
pub struct SYS_WS_READ_STATES_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_WS_READ_STATES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `INPUTENABLE`"]
pub struct INPUTENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl W {
    #[doc = "Bit 23 - DISROW0"]
    #[inline(always)]
    pub fn disrow0(&mut self) -> DISROW0_W {
        DISROW0_W { w: self }
    }
    #[doc = "Bit 22 - SPARE"]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W { w: self }
    }
    #[doc = "Bit 21 - EFC_SELF_TEST_ERROR"]
    #[inline(always)]
    pub fn efc_self_test_error(&mut self) -> EFC_SELF_TEST_ERROR_W {
        EFC_SELF_TEST_ERROR_W { w: self }
    }
    #[doc = "Bit 20 - EFC_INSTRUCTION_INFO"]
    #[inline(always)]
    pub fn efc_instruction_info(&mut self) -> EFC_INSTRUCTION_INFO_W {
        EFC_INSTRUCTION_INFO_W { w: self }
    }
    #[doc = "Bit 19 - EFC_INSTRUCTION_ERROR"]
    #[inline(always)]
    pub fn efc_instruction_error(&mut self) -> EFC_INSTRUCTION_ERROR_W {
        EFC_INSTRUCTION_ERROR_W { w: self }
    }
    #[doc = "Bit 18 - EFC_AUTOLOAD_ERROR"]
    #[inline(always)]
    pub fn efc_autoload_error(&mut self) -> EFC_AUTOLOAD_ERROR_W {
        EFC_AUTOLOAD_ERROR_W { w: self }
    }
    #[doc = "Bits 14:17 - OUTPUTENABLE"]
    #[inline(always)]
    pub fn outputenable(&mut self) -> OUTPUTENABLE_W {
        OUTPUTENABLE_W { w: self }
    }
    #[doc = "Bit 13 - SYS_ECC_SELF_TEST_EN"]
    #[inline(always)]
    pub fn sys_ecc_self_test_en(&mut self) -> SYS_ECC_SELF_TEST_EN_W {
        SYS_ECC_SELF_TEST_EN_W { w: self }
    }
    #[doc = "Bit 12 - SYS_ECC_OVERRIDE_EN"]
    #[inline(always)]
    pub fn sys_ecc_override_en(&mut self) -> SYS_ECC_OVERRIDE_EN_W {
        SYS_ECC_OVERRIDE_EN_W { w: self }
    }
    #[doc = "Bit 11 - EFC_FDI"]
    #[inline(always)]
    pub fn efc_fdi(&mut self) -> EFC_FDI_W {
        EFC_FDI_W { w: self }
    }
    #[doc = "Bit 10 - SYS_DIEID_AUTOLOAD_EN"]
    #[inline(always)]
    pub fn sys_dieid_autoload_en(&mut self) -> SYS_DIEID_AUTOLOAD_EN_W {
        SYS_DIEID_AUTOLOAD_EN_W { w: self }
    }
    #[doc = "Bits 8:9 - SYS_REPAIR_EN"]
    #[inline(always)]
    pub fn sys_repair_en(&mut self) -> SYS_REPAIR_EN_W {
        SYS_REPAIR_EN_W { w: self }
    }
    #[doc = "Bits 4:7 - SYS_WS_READ_STATES"]
    #[inline(always)]
    pub fn sys_ws_read_states(&mut self) -> SYS_WS_READ_STATES_W {
        SYS_WS_READ_STATES_W { w: self }
    }
    #[doc = "Bits 0:3 - INPUTENABLE"]
    #[inline(always)]
    pub fn inputenable(&mut self) -> INPUTENABLE_W {
        INPUTENABLE_W { w: self }
    }
}

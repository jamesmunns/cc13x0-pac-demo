#[doc = "Reader of register RESETCTL"]
pub type R = crate::R<u32, super::RESETCTL>;
#[doc = "Writer for register RESETCTL"]
pub type W = crate::W<u32, super::RESETCTL>;
#[doc = "Register RESETCTL `reset()`'s with value 0xe0"]
impl crate::ResetValue for super::RESETCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xe0
    }
}
#[doc = "Reader of field `SYSRESET`"]
pub type SYSRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSRESET`"]
pub struct SYSRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `BOOT_DET_1_CLR`"]
pub type BOOT_DET_1_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_DET_1_CLR`"]
pub struct BOOT_DET_1_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DET_1_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `BOOT_DET_0_CLR`"]
pub type BOOT_DET_0_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_DET_0_CLR`"]
pub struct BOOT_DET_0_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DET_0_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `BOOT_DET_1_SET`"]
pub type BOOT_DET_1_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_DET_1_SET`"]
pub struct BOOT_DET_1_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DET_1_SET_W<'a> {
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
#[doc = "Reader of field `BOOT_DET_0_SET`"]
pub type BOOT_DET_0_SET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_DET_0_SET`"]
pub struct BOOT_DET_0_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DET_0_SET_W<'a> {
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
#[doc = "Reader of field `WU_FROM_SD`"]
pub type WU_FROM_SD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WU_FROM_SD`"]
pub struct WU_FROM_SD_W<'a> {
    w: &'a mut W,
}
impl<'a> WU_FROM_SD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `GPIO_WU_FROM_SD`"]
pub type GPIO_WU_FROM_SD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_WU_FROM_SD`"]
pub struct GPIO_WU_FROM_SD_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_WU_FROM_SD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `BOOT_DET_1`"]
pub type BOOT_DET_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_DET_1`"]
pub struct BOOT_DET_1_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DET_1_W<'a> {
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
#[doc = "Reader of field `BOOT_DET_0`"]
pub type BOOT_DET_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_DET_0`"]
pub struct BOOT_DET_0_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DET_0_W<'a> {
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
#[doc = "Reader of field `VDDS_LOSS_EN_OVR`"]
pub type VDDS_LOSS_EN_OVR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDS_LOSS_EN_OVR`"]
pub struct VDDS_LOSS_EN_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDS_LOSS_EN_OVR_W<'a> {
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
#[doc = "Reader of field `VDDR_LOSS_EN_OVR`"]
pub type VDDR_LOSS_EN_OVR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDR_LOSS_EN_OVR`"]
pub struct VDDR_LOSS_EN_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_LOSS_EN_OVR_W<'a> {
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
#[doc = "Reader of field `VDD_LOSS_EN_OVR`"]
pub type VDD_LOSS_EN_OVR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDD_LOSS_EN_OVR`"]
pub struct VDD_LOSS_EN_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> VDD_LOSS_EN_OVR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `VDDS_LOSS_EN`"]
pub type VDDS_LOSS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDS_LOSS_EN`"]
pub struct VDDS_LOSS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDS_LOSS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `VDDR_LOSS_EN`"]
pub type VDDR_LOSS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDDR_LOSS_EN`"]
pub struct VDDR_LOSS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_LOSS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `VDD_LOSS_EN`"]
pub type VDD_LOSS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDD_LOSS_EN`"]
pub struct VDD_LOSS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VDD_LOSS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CLK_LOSS_EN`"]
pub type CLK_LOSS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_LOSS_EN`"]
pub struct CLK_LOSS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_LOSS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RESET_SRC`"]
pub type RESET_SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESET_SRC`"]
pub struct RESET_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - SYSRESET"]
    #[inline(always)]
    pub fn sysreset(&self) -> SYSRESET_R {
        SYSRESET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 25 - BOOT_DET_1_CLR"]
    #[inline(always)]
    pub fn boot_det_1_clr(&self) -> BOOT_DET_1_CLR_R {
        BOOT_DET_1_CLR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - BOOT_DET_0_CLR"]
    #[inline(always)]
    pub fn boot_det_0_clr(&self) -> BOOT_DET_0_CLR_R {
        BOOT_DET_0_CLR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 17 - BOOT_DET_1_SET"]
    #[inline(always)]
    pub fn boot_det_1_set(&self) -> BOOT_DET_1_SET_R {
        BOOT_DET_1_SET_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BOOT_DET_0_SET"]
    #[inline(always)]
    pub fn boot_det_0_set(&self) -> BOOT_DET_0_SET_R {
        BOOT_DET_0_SET_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - WU_FROM_SD"]
    #[inline(always)]
    pub fn wu_from_sd(&self) -> WU_FROM_SD_R {
        WU_FROM_SD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GPIO_WU_FROM_SD"]
    #[inline(always)]
    pub fn gpio_wu_from_sd(&self) -> GPIO_WU_FROM_SD_R {
        GPIO_WU_FROM_SD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - BOOT_DET_1"]
    #[inline(always)]
    pub fn boot_det_1(&self) -> BOOT_DET_1_R {
        BOOT_DET_1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - BOOT_DET_0"]
    #[inline(always)]
    pub fn boot_det_0(&self) -> BOOT_DET_0_R {
        BOOT_DET_0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - VDDS_LOSS_EN_OVR"]
    #[inline(always)]
    pub fn vdds_loss_en_ovr(&self) -> VDDS_LOSS_EN_OVR_R {
        VDDS_LOSS_EN_OVR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - VDDR_LOSS_EN_OVR"]
    #[inline(always)]
    pub fn vddr_loss_en_ovr(&self) -> VDDR_LOSS_EN_OVR_R {
        VDDR_LOSS_EN_OVR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VDD_LOSS_EN_OVR"]
    #[inline(always)]
    pub fn vdd_loss_en_ovr(&self) -> VDD_LOSS_EN_OVR_R {
        VDD_LOSS_EN_OVR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 7 - VDDS_LOSS_EN"]
    #[inline(always)]
    pub fn vdds_loss_en(&self) -> VDDS_LOSS_EN_R {
        VDDS_LOSS_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - VDDR_LOSS_EN"]
    #[inline(always)]
    pub fn vddr_loss_en(&self) -> VDDR_LOSS_EN_R {
        VDDR_LOSS_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - VDD_LOSS_EN"]
    #[inline(always)]
    pub fn vdd_loss_en(&self) -> VDD_LOSS_EN_R {
        VDD_LOSS_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CLK_LOSS_EN"]
    #[inline(always)]
    pub fn clk_loss_en(&self) -> CLK_LOSS_EN_R {
        CLK_LOSS_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - RESET_SRC"]
    #[inline(always)]
    pub fn reset_src(&self) -> RESET_SRC_R {
        RESET_SRC_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - SYSRESET"]
    #[inline(always)]
    pub fn sysreset(&mut self) -> SYSRESET_W {
        SYSRESET_W { w: self }
    }
    #[doc = "Bit 25 - BOOT_DET_1_CLR"]
    #[inline(always)]
    pub fn boot_det_1_clr(&mut self) -> BOOT_DET_1_CLR_W {
        BOOT_DET_1_CLR_W { w: self }
    }
    #[doc = "Bit 24 - BOOT_DET_0_CLR"]
    #[inline(always)]
    pub fn boot_det_0_clr(&mut self) -> BOOT_DET_0_CLR_W {
        BOOT_DET_0_CLR_W { w: self }
    }
    #[doc = "Bit 17 - BOOT_DET_1_SET"]
    #[inline(always)]
    pub fn boot_det_1_set(&mut self) -> BOOT_DET_1_SET_W {
        BOOT_DET_1_SET_W { w: self }
    }
    #[doc = "Bit 16 - BOOT_DET_0_SET"]
    #[inline(always)]
    pub fn boot_det_0_set(&mut self) -> BOOT_DET_0_SET_W {
        BOOT_DET_0_SET_W { w: self }
    }
    #[doc = "Bit 15 - WU_FROM_SD"]
    #[inline(always)]
    pub fn wu_from_sd(&mut self) -> WU_FROM_SD_W {
        WU_FROM_SD_W { w: self }
    }
    #[doc = "Bit 14 - GPIO_WU_FROM_SD"]
    #[inline(always)]
    pub fn gpio_wu_from_sd(&mut self) -> GPIO_WU_FROM_SD_W {
        GPIO_WU_FROM_SD_W { w: self }
    }
    #[doc = "Bit 13 - BOOT_DET_1"]
    #[inline(always)]
    pub fn boot_det_1(&mut self) -> BOOT_DET_1_W {
        BOOT_DET_1_W { w: self }
    }
    #[doc = "Bit 12 - BOOT_DET_0"]
    #[inline(always)]
    pub fn boot_det_0(&mut self) -> BOOT_DET_0_W {
        BOOT_DET_0_W { w: self }
    }
    #[doc = "Bit 11 - VDDS_LOSS_EN_OVR"]
    #[inline(always)]
    pub fn vdds_loss_en_ovr(&mut self) -> VDDS_LOSS_EN_OVR_W {
        VDDS_LOSS_EN_OVR_W { w: self }
    }
    #[doc = "Bit 10 - VDDR_LOSS_EN_OVR"]
    #[inline(always)]
    pub fn vddr_loss_en_ovr(&mut self) -> VDDR_LOSS_EN_OVR_W {
        VDDR_LOSS_EN_OVR_W { w: self }
    }
    #[doc = "Bit 9 - VDD_LOSS_EN_OVR"]
    #[inline(always)]
    pub fn vdd_loss_en_ovr(&mut self) -> VDD_LOSS_EN_OVR_W {
        VDD_LOSS_EN_OVR_W { w: self }
    }
    #[doc = "Bit 7 - VDDS_LOSS_EN"]
    #[inline(always)]
    pub fn vdds_loss_en(&mut self) -> VDDS_LOSS_EN_W {
        VDDS_LOSS_EN_W { w: self }
    }
    #[doc = "Bit 6 - VDDR_LOSS_EN"]
    #[inline(always)]
    pub fn vddr_loss_en(&mut self) -> VDDR_LOSS_EN_W {
        VDDR_LOSS_EN_W { w: self }
    }
    #[doc = "Bit 5 - VDD_LOSS_EN"]
    #[inline(always)]
    pub fn vdd_loss_en(&mut self) -> VDD_LOSS_EN_W {
        VDD_LOSS_EN_W { w: self }
    }
    #[doc = "Bit 4 - CLK_LOSS_EN"]
    #[inline(always)]
    pub fn clk_loss_en(&mut self) -> CLK_LOSS_EN_W {
        CLK_LOSS_EN_W { w: self }
    }
    #[doc = "Bits 1:3 - RESET_SRC"]
    #[inline(always)]
    pub fn reset_src(&mut self) -> RESET_SRC_W {
        RESET_SRC_W { w: self }
    }
}

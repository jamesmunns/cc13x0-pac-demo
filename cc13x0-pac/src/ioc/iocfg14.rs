#[doc = "Reader of register IOCFG14"]
pub type R = crate::R<u32, super::IOCFG14>;
#[doc = "Writer for register IOCFG14"]
pub type W = crate::W<u32, super::IOCFG14>;
#[doc = "Register IOCFG14 `reset()`'s with value 0x6000"]
impl crate::ResetValue for super::IOCFG14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6000
    }
}
#[doc = "Reader of field `HYST_EN`"]
pub type HYST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HYST_EN`"]
pub struct HYST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `IE`"]
pub type IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IE`"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `WU_CFG`"]
pub type WU_CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WU_CFG`"]
pub struct WU_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> WU_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `IOMODE`"]
pub type IOMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOMODE`"]
pub struct IOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `EDGE_IRQ_EN`"]
pub type EDGE_IRQ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGE_IRQ_EN`"]
pub struct EDGE_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_IRQ_EN_W<'a> {
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
#[doc = "Reader of field `EDGE_DET`"]
pub type EDGE_DET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EDGE_DET`"]
pub struct EDGE_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_DET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `PULL_CTL`"]
pub type PULL_CTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PULL_CTL`"]
pub struct PULL_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_CTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `SLEW_RED`"]
pub type SLEW_RED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLEW_RED`"]
pub struct SLEW_RED_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEW_RED_W<'a> {
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
#[doc = "Reader of field `IOCURR`"]
pub type IOCURR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOCURR`"]
pub struct IOCURR_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCURR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `IOSTR`"]
pub type IOSTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOSTR`"]
pub struct IOSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> IOSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `PORT_ID`"]
pub type PORT_ID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PORT_ID`"]
pub struct PORT_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - HYST_EN"]
    #[inline(always)]
    pub fn hyst_en(&self) -> HYST_EN_R {
        HYST_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - IE"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - WU_CFG"]
    #[inline(always)]
    pub fn wu_cfg(&self) -> WU_CFG_R {
        WU_CFG_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - IOMODE"]
    #[inline(always)]
    pub fn iomode(&self) -> IOMODE_R {
        IOMODE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 18 - EDGE_IRQ_EN"]
    #[inline(always)]
    pub fn edge_irq_en(&self) -> EDGE_IRQ_EN_R {
        EDGE_IRQ_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - EDGE_DET"]
    #[inline(always)]
    pub fn edge_det(&self) -> EDGE_DET_R {
        EDGE_DET_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 13:14 - PULL_CTL"]
    #[inline(always)]
    pub fn pull_ctl(&self) -> PULL_CTL_R {
        PULL_CTL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - SLEW_RED"]
    #[inline(always)]
    pub fn slew_red(&self) -> SLEW_RED_R {
        SLEW_RED_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - IOCURR"]
    #[inline(always)]
    pub fn iocurr(&self) -> IOCURR_R {
        IOCURR_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - IOSTR"]
    #[inline(always)]
    pub fn iostr(&self) -> IOSTR_R {
        IOSTR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:5 - PORT_ID"]
    #[inline(always)]
    pub fn port_id(&self) -> PORT_ID_R {
        PORT_ID_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - HYST_EN"]
    #[inline(always)]
    pub fn hyst_en(&mut self) -> HYST_EN_W {
        HYST_EN_W { w: self }
    }
    #[doc = "Bit 29 - IE"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bits 27:28 - WU_CFG"]
    #[inline(always)]
    pub fn wu_cfg(&mut self) -> WU_CFG_W {
        WU_CFG_W { w: self }
    }
    #[doc = "Bits 24:26 - IOMODE"]
    #[inline(always)]
    pub fn iomode(&mut self) -> IOMODE_W {
        IOMODE_W { w: self }
    }
    #[doc = "Bit 18 - EDGE_IRQ_EN"]
    #[inline(always)]
    pub fn edge_irq_en(&mut self) -> EDGE_IRQ_EN_W {
        EDGE_IRQ_EN_W { w: self }
    }
    #[doc = "Bits 16:17 - EDGE_DET"]
    #[inline(always)]
    pub fn edge_det(&mut self) -> EDGE_DET_W {
        EDGE_DET_W { w: self }
    }
    #[doc = "Bits 13:14 - PULL_CTL"]
    #[inline(always)]
    pub fn pull_ctl(&mut self) -> PULL_CTL_W {
        PULL_CTL_W { w: self }
    }
    #[doc = "Bit 12 - SLEW_RED"]
    #[inline(always)]
    pub fn slew_red(&mut self) -> SLEW_RED_W {
        SLEW_RED_W { w: self }
    }
    #[doc = "Bits 10:11 - IOCURR"]
    #[inline(always)]
    pub fn iocurr(&mut self) -> IOCURR_W {
        IOCURR_W { w: self }
    }
    #[doc = "Bits 8:9 - IOSTR"]
    #[inline(always)]
    pub fn iostr(&mut self) -> IOSTR_W {
        IOSTR_W { w: self }
    }
    #[doc = "Bits 0:5 - PORT_ID"]
    #[inline(always)]
    pub fn port_id(&mut self) -> PORT_ID_W {
        PORT_ID_W { w: self }
    }
}

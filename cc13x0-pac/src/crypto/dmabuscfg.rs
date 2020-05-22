#[doc = "Reader of register DMABUSCFG"]
pub type R = crate::R<u32, super::DMABUSCFG>;
#[doc = "Writer for register DMABUSCFG"]
pub type W = crate::W<u32, super::DMABUSCFG>;
#[doc = "Register DMABUSCFG `reset()`'s with value 0x2400"]
impl crate::ResetValue for super::DMABUSCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2400
    }
}
#[doc = "Reader of field `AHB_MST1_BURST_SIZE`"]
pub type AHB_MST1_BURST_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AHB_MST1_BURST_SIZE`"]
pub struct AHB_MST1_BURST_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_BURST_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `AHB_MST1_IDLE_EN`"]
pub type AHB_MST1_IDLE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB_MST1_IDLE_EN`"]
pub struct AHB_MST1_IDLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_IDLE_EN_W<'a> {
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
#[doc = "Reader of field `AHB_MST1_INCR_EN`"]
pub type AHB_MST1_INCR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB_MST1_INCR_EN`"]
pub struct AHB_MST1_INCR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_INCR_EN_W<'a> {
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
#[doc = "Reader of field `AHB_MST1_LOCK_EN`"]
pub type AHB_MST1_LOCK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB_MST1_LOCK_EN`"]
pub struct AHB_MST1_LOCK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_LOCK_EN_W<'a> {
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
#[doc = "Reader of field `AHB_MST1_BIGEND`"]
pub type AHB_MST1_BIGEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB_MST1_BIGEND`"]
pub struct AHB_MST1_BIGEND_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_MST1_BIGEND_W<'a> {
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
impl R {
    #[doc = "Bits 12:15 - AHB_MST1_BURST_SIZE"]
    #[inline(always)]
    pub fn ahb_mst1_burst_size(&self) -> AHB_MST1_BURST_SIZE_R {
        AHB_MST1_BURST_SIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - AHB_MST1_IDLE_EN"]
    #[inline(always)]
    pub fn ahb_mst1_idle_en(&self) -> AHB_MST1_IDLE_EN_R {
        AHB_MST1_IDLE_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AHB_MST1_INCR_EN"]
    #[inline(always)]
    pub fn ahb_mst1_incr_en(&self) -> AHB_MST1_INCR_EN_R {
        AHB_MST1_INCR_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AHB_MST1_LOCK_EN"]
    #[inline(always)]
    pub fn ahb_mst1_lock_en(&self) -> AHB_MST1_LOCK_EN_R {
        AHB_MST1_LOCK_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AHB_MST1_BIGEND"]
    #[inline(always)]
    pub fn ahb_mst1_bigend(&self) -> AHB_MST1_BIGEND_R {
        AHB_MST1_BIGEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 12:15 - AHB_MST1_BURST_SIZE"]
    #[inline(always)]
    pub fn ahb_mst1_burst_size(&mut self) -> AHB_MST1_BURST_SIZE_W {
        AHB_MST1_BURST_SIZE_W { w: self }
    }
    #[doc = "Bit 11 - AHB_MST1_IDLE_EN"]
    #[inline(always)]
    pub fn ahb_mst1_idle_en(&mut self) -> AHB_MST1_IDLE_EN_W {
        AHB_MST1_IDLE_EN_W { w: self }
    }
    #[doc = "Bit 10 - AHB_MST1_INCR_EN"]
    #[inline(always)]
    pub fn ahb_mst1_incr_en(&mut self) -> AHB_MST1_INCR_EN_W {
        AHB_MST1_INCR_EN_W { w: self }
    }
    #[doc = "Bit 9 - AHB_MST1_LOCK_EN"]
    #[inline(always)]
    pub fn ahb_mst1_lock_en(&mut self) -> AHB_MST1_LOCK_EN_W {
        AHB_MST1_LOCK_EN_W { w: self }
    }
    #[doc = "Bit 8 - AHB_MST1_BIGEND"]
    #[inline(always)]
    pub fn ahb_mst1_bigend(&mut self) -> AHB_MST1_BIGEND_W {
        AHB_MST1_BIGEND_W { w: self }
    }
}

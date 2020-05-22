#[doc = "Writer for register IRQCLR"]
pub type W = crate::W<u32, super::IRQCLR>;
#[doc = "Register IRQCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DMA_BUS_ERR`"]
pub struct DMA_BUS_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_BUS_ERR_W<'a> {
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
#[doc = "Write proxy for field `KEY_ST_WR_ERR`"]
pub struct KEY_ST_WR_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_ST_WR_ERR_W<'a> {
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
#[doc = "Write proxy for field `KEY_ST_RD_ERR`"]
pub struct KEY_ST_RD_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_ST_RD_ERR_W<'a> {
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
#[doc = "Write proxy for field `DMA_IN_DONE`"]
pub struct DMA_IN_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_DONE_W<'a> {
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
#[doc = "Write proxy for field `RESULT_AVAIL`"]
pub struct RESULT_AVAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> RESULT_AVAIL_W<'a> {
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
impl W {
    #[doc = "Bit 31 - DMA_BUS_ERR"]
    #[inline(always)]
    pub fn dma_bus_err(&mut self) -> DMA_BUS_ERR_W {
        DMA_BUS_ERR_W { w: self }
    }
    #[doc = "Bit 30 - KEY_ST_WR_ERR"]
    #[inline(always)]
    pub fn key_st_wr_err(&mut self) -> KEY_ST_WR_ERR_W {
        KEY_ST_WR_ERR_W { w: self }
    }
    #[doc = "Bit 29 - KEY_ST_RD_ERR"]
    #[inline(always)]
    pub fn key_st_rd_err(&mut self) -> KEY_ST_RD_ERR_W {
        KEY_ST_RD_ERR_W { w: self }
    }
    #[doc = "Bit 1 - DMA_IN_DONE"]
    #[inline(always)]
    pub fn dma_in_done(&mut self) -> DMA_IN_DONE_W {
        DMA_IN_DONE_W { w: self }
    }
    #[doc = "Bit 0 - RESULT_AVAIL"]
    #[inline(always)]
    pub fn result_avail(&mut self) -> RESULT_AVAIL_W {
        RESULT_AVAIL_W { w: self }
    }
}

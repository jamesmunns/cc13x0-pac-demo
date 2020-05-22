#[doc = "Reader of register IRQSTAT"]
pub type R = crate::R<u32, super::IRQSTAT>;
#[doc = "Reader of field `DMA_BUS_ERR`"]
pub type DMA_BUS_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `KEY_ST_WR_ERR`"]
pub type KEY_ST_WR_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `KEY_ST_RD_ERR`"]
pub type KEY_ST_RD_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_IN_DONE`"]
pub type DMA_IN_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RESULT_AVAIL`"]
pub type RESULT_AVAIL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31 - DMA_BUS_ERR"]
    #[inline(always)]
    pub fn dma_bus_err(&self) -> DMA_BUS_ERR_R {
        DMA_BUS_ERR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - KEY_ST_WR_ERR"]
    #[inline(always)]
    pub fn key_st_wr_err(&self) -> KEY_ST_WR_ERR_R {
        KEY_ST_WR_ERR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - KEY_ST_RD_ERR"]
    #[inline(always)]
    pub fn key_st_rd_err(&self) -> KEY_ST_RD_ERR_R {
        KEY_ST_RD_ERR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA_IN_DONE"]
    #[inline(always)]
    pub fn dma_in_done(&self) -> DMA_IN_DONE_R {
        DMA_IN_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RESULT_AVAIL"]
    #[inline(always)]
    pub fn result_avail(&self) -> RESULT_AVAIL_R {
        RESULT_AVAIL_R::new((self.bits & 0x01) != 0)
    }
}

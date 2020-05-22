#[doc = "Reader of register IRQFLAGS"]
pub type R = crate::R<u32, super::IRQFLAGS>;
#[doc = "Reader of field `AIF_DMA_IN`"]
pub type AIF_DMA_IN_R = crate::R<bool, bool>;
#[doc = "Reader of field `AIF_DMA_OUT`"]
pub type AIF_DMA_OUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `WCLK_TIMEOUT`"]
pub type WCLK_TIMEOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUS_ERR`"]
pub type BUS_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `WCLK_ERR`"]
pub type WCLK_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PTR_ERR`"]
pub type PTR_ERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 5 - AIF_DMA_IN"]
    #[inline(always)]
    pub fn aif_dma_in(&self) -> AIF_DMA_IN_R {
        AIF_DMA_IN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AIF_DMA_OUT"]
    #[inline(always)]
    pub fn aif_dma_out(&self) -> AIF_DMA_OUT_R {
        AIF_DMA_OUT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WCLK_TIMEOUT"]
    #[inline(always)]
    pub fn wclk_timeout(&self) -> WCLK_TIMEOUT_R {
        WCLK_TIMEOUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BUS_ERR"]
    #[inline(always)]
    pub fn bus_err(&self) -> BUS_ERR_R {
        BUS_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - WCLK_ERR"]
    #[inline(always)]
    pub fn wclk_err(&self) -> WCLK_ERR_R {
        WCLK_ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PTR_ERR"]
    #[inline(always)]
    pub fn ptr_err(&self) -> PTR_ERR_R {
        PTR_ERR_R::new((self.bits & 0x01) != 0)
    }
}

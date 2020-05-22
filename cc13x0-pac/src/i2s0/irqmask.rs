#[doc = "Reader of register IRQMASK"]
pub type R = crate::R<u32, super::IRQMASK>;
#[doc = "Writer for register IRQMASK"]
pub type W = crate::W<u32, super::IRQMASK>;
#[doc = "Register IRQMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AIF_DMA_IN`"]
pub type AIF_DMA_IN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AIF_DMA_IN`"]
pub struct AIF_DMA_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> AIF_DMA_IN_W<'a> {
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
#[doc = "Reader of field `AIF_DMA_OUT`"]
pub type AIF_DMA_OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AIF_DMA_OUT`"]
pub struct AIF_DMA_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> AIF_DMA_OUT_W<'a> {
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
#[doc = "Reader of field `WCLK_TIMEOUT`"]
pub type WCLK_TIMEOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WCLK_TIMEOUT`"]
pub struct WCLK_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> WCLK_TIMEOUT_W<'a> {
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
#[doc = "Reader of field `BUS_ERR`"]
pub type BUS_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUS_ERR`"]
pub struct BUS_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_ERR_W<'a> {
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
#[doc = "Reader of field `WCLK_ERR`"]
pub type WCLK_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WCLK_ERR`"]
pub struct WCLK_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WCLK_ERR_W<'a> {
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
#[doc = "Reader of field `PTR_ERR`"]
pub type PTR_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTR_ERR`"]
pub struct PTR_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PTR_ERR_W<'a> {
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
impl W {
    #[doc = "Bit 5 - AIF_DMA_IN"]
    #[inline(always)]
    pub fn aif_dma_in(&mut self) -> AIF_DMA_IN_W {
        AIF_DMA_IN_W { w: self }
    }
    #[doc = "Bit 4 - AIF_DMA_OUT"]
    #[inline(always)]
    pub fn aif_dma_out(&mut self) -> AIF_DMA_OUT_W {
        AIF_DMA_OUT_W { w: self }
    }
    #[doc = "Bit 3 - WCLK_TIMEOUT"]
    #[inline(always)]
    pub fn wclk_timeout(&mut self) -> WCLK_TIMEOUT_W {
        WCLK_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 2 - BUS_ERR"]
    #[inline(always)]
    pub fn bus_err(&mut self) -> BUS_ERR_W {
        BUS_ERR_W { w: self }
    }
    #[doc = "Bit 1 - WCLK_ERR"]
    #[inline(always)]
    pub fn wclk_err(&mut self) -> WCLK_ERR_W {
        WCLK_ERR_W { w: self }
    }
    #[doc = "Bit 0 - PTR_ERR"]
    #[inline(always)]
    pub fn ptr_err(&mut self) -> PTR_ERR_W {
        PTR_ERR_W { w: self }
    }
}

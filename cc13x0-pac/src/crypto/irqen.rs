#[doc = "Reader of register IRQEN"]
pub type R = crate::R<u32, super::IRQEN>;
#[doc = "Writer for register IRQEN"]
pub type W = crate::W<u32, super::IRQEN>;
#[doc = "Register IRQEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_IN_DONE`"]
pub type DMA_IN_DONE_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `RESULT_AVAIL`"]
pub type RESULT_AVAIL_R = crate::R<bool, bool>;
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
impl R {
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
impl W {
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

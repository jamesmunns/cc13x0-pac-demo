#[doc = "Reader of register SECDMACLKGDS"]
pub type R = crate::R<u32, super::SECDMACLKGDS>;
#[doc = "Writer for register SECDMACLKGDS"]
pub type W = crate::W<u32, super::SECDMACLKGDS>;
#[doc = "Register SECDMACLKGDS `reset()`'s with value 0"]
impl crate::ResetValue for super::SECDMACLKGDS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA_CLK_EN`"]
pub type DMA_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_CLK_EN`"]
pub struct DMA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_CLK_EN_W<'a> {
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
#[doc = "Reader of field `TRNG_CLK_EN`"]
pub type TRNG_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRNG_CLK_EN`"]
pub struct TRNG_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG_CLK_EN_W<'a> {
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
#[doc = "Reader of field `CRYPTO_CLK_EN`"]
pub type CRYPTO_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYPTO_CLK_EN`"]
pub struct CRYPTO_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_CLK_EN_W<'a> {
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
    #[doc = "Bit 8 - DMA_CLK_EN"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DMA_CLK_EN_R {
        DMA_CLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TRNG_CLK_EN"]
    #[inline(always)]
    pub fn trng_clk_en(&self) -> TRNG_CLK_EN_R {
        TRNG_CLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CRYPTO_CLK_EN"]
    #[inline(always)]
    pub fn crypto_clk_en(&self) -> CRYPTO_CLK_EN_R {
        CRYPTO_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - DMA_CLK_EN"]
    #[inline(always)]
    pub fn dma_clk_en(&mut self) -> DMA_CLK_EN_W {
        DMA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 1 - TRNG_CLK_EN"]
    #[inline(always)]
    pub fn trng_clk_en(&mut self) -> TRNG_CLK_EN_W {
        TRNG_CLK_EN_W { w: self }
    }
    #[doc = "Bit 0 - CRYPTO_CLK_EN"]
    #[inline(always)]
    pub fn crypto_clk_en(&mut self) -> CRYPTO_CLK_EN_W {
        CRYPTO_CLK_EN_W { w: self }
    }
}

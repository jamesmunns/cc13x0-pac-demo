#[doc = "Reader of register FTCTL"]
pub type R = crate::R<u32, super::FTCTL>;
#[doc = "Writer for register FTCTL"]
pub type W = crate::W<u32, super::FTCTL>;
#[doc = "Register FTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::FTCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDATA_BLK_CLR`"]
pub type WDATA_BLK_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDATA_BLK_CLR`"]
pub struct WDATA_BLK_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDATA_BLK_CLR_W<'a> {
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
#[doc = "Reader of field `TEST_EN`"]
pub type TEST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEST_EN`"]
pub struct TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_EN_W<'a> {
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
impl R {
    #[doc = "Bit 16 - WDATA_BLK_CLR"]
    #[inline(always)]
    pub fn wdata_blk_clr(&self) -> WDATA_BLK_CLR_R {
        WDATA_BLK_CLR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TEST_EN"]
    #[inline(always)]
    pub fn test_en(&self) -> TEST_EN_R {
        TEST_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - WDATA_BLK_CLR"]
    #[inline(always)]
    pub fn wdata_blk_clr(&mut self) -> WDATA_BLK_CLR_W {
        WDATA_BLK_CLR_W { w: self }
    }
    #[doc = "Bit 1 - TEST_EN"]
    #[inline(always)]
    pub fn test_en(&mut self) -> TEST_EN_W {
        TEST_EN_W { w: self }
    }
}

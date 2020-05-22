#[doc = "Reader of register WARMRESET"]
pub type R = crate::R<u32, super::WARMRESET>;
#[doc = "Writer for register WARMRESET"]
pub type W = crate::W<u32, super::WARMRESET>;
#[doc = "Register WARMRESET `reset()`'s with value 0"]
impl crate::ResetValue for super::WARMRESET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WR_TO_PINRESET`"]
pub type WR_TO_PINRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WR_TO_PINRESET`"]
pub struct WR_TO_PINRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_TO_PINRESET_W<'a> {
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
#[doc = "Reader of field `LOCKUP_STAT`"]
pub type LOCKUP_STAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKUP_STAT`"]
pub struct LOCKUP_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUP_STAT_W<'a> {
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
#[doc = "Reader of field `WDT_STAT`"]
pub type WDT_STAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_STAT`"]
pub struct WDT_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_STAT_W<'a> {
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
    #[doc = "Bit 2 - WR_TO_PINRESET"]
    #[inline(always)]
    pub fn wr_to_pinreset(&self) -> WR_TO_PINRESET_R {
        WR_TO_PINRESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LOCKUP_STAT"]
    #[inline(always)]
    pub fn lockup_stat(&self) -> LOCKUP_STAT_R {
        LOCKUP_STAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - WDT_STAT"]
    #[inline(always)]
    pub fn wdt_stat(&self) -> WDT_STAT_R {
        WDT_STAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - WR_TO_PINRESET"]
    #[inline(always)]
    pub fn wr_to_pinreset(&mut self) -> WR_TO_PINRESET_W {
        WR_TO_PINRESET_W { w: self }
    }
    #[doc = "Bit 1 - LOCKUP_STAT"]
    #[inline(always)]
    pub fn lockup_stat(&mut self) -> LOCKUP_STAT_W {
        LOCKUP_STAT_W { w: self }
    }
    #[doc = "Bit 0 - WDT_STAT"]
    #[inline(always)]
    pub fn wdt_stat(&mut self) -> WDT_STAT_W {
        WDT_STAT_W { w: self }
    }
}

#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STARTUP_CYCLES`"]
pub type STARTUP_CYCLES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STARTUP_CYCLES`"]
pub struct STARTUP_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUP_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TRNG_EN`"]
pub type TRNG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRNG_EN`"]
pub struct TRNG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG_EN_W<'a> {
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
#[doc = "Reader of field `NO_LFSR_FB`"]
pub type NO_LFSR_FB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NO_LFSR_FB`"]
pub struct NO_LFSR_FB_W<'a> {
    w: &'a mut W,
}
impl<'a> NO_LFSR_FB_W<'a> {
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
#[doc = "Reader of field `TEST_MODE`"]
pub type TEST_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEST_MODE`"]
pub struct TEST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_MODE_W<'a> {
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
    #[doc = "Bits 16:31 - STARTUP_CYCLES"]
    #[inline(always)]
    pub fn startup_cycles(&self) -> STARTUP_CYCLES_R {
        STARTUP_CYCLES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 10 - TRNG_EN"]
    #[inline(always)]
    pub fn trng_en(&self) -> TRNG_EN_R {
        TRNG_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NO_LFSR_FB"]
    #[inline(always)]
    pub fn no_lfsr_fb(&self) -> NO_LFSR_FB_R {
        NO_LFSR_FB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TEST_MODE"]
    #[inline(always)]
    pub fn test_mode(&self) -> TEST_MODE_R {
        TEST_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - STARTUP_CYCLES"]
    #[inline(always)]
    pub fn startup_cycles(&mut self) -> STARTUP_CYCLES_W {
        STARTUP_CYCLES_W { w: self }
    }
    #[doc = "Bit 10 - TRNG_EN"]
    #[inline(always)]
    pub fn trng_en(&mut self) -> TRNG_EN_W {
        TRNG_EN_W { w: self }
    }
    #[doc = "Bit 2 - NO_LFSR_FB"]
    #[inline(always)]
    pub fn no_lfsr_fb(&mut self) -> NO_LFSR_FB_W {
        NO_LFSR_FB_W { w: self }
    }
    #[doc = "Bit 1 - TEST_MODE"]
    #[inline(always)]
    pub fn test_mode(&mut self) -> TEST_MODE_W {
        TEST_MODE_W { w: self }
    }
}

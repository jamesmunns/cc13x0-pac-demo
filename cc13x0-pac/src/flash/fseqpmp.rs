#[doc = "Reader of register FSEQPMP"]
pub type R = crate::R<u32, super::FSEQPMP>;
#[doc = "Writer for register FSEQPMP"]
pub type W = crate::W<u32, super::FSEQPMP>;
#[doc = "Register FSEQPMP `reset()`'s with value 0x8508_0000"]
impl crate::ResetValue for super::FSEQPMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8508_0000
    }
}
#[doc = "Reader of field `TRIM_3P4`"]
pub type TRIM_3P4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM_3P4`"]
pub struct TRIM_3P4_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_3P4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `TRIM_1P7`"]
pub type TRIM_1P7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM_1P7`"]
pub struct TRIM_1P7_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_1P7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `TRIM_0P8`"]
pub type TRIM_0P8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIM_0P8`"]
pub struct TRIM_0P8_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_0P8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `VIN_AT_X`"]
pub type VIN_AT_X_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VIN_AT_X`"]
pub struct VIN_AT_X_W<'a> {
    w: &'a mut W,
}
impl<'a> VIN_AT_X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `VIN_BY_PASS`"]
pub type VIN_BY_PASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VIN_BY_PASS`"]
pub struct VIN_BY_PASS_W<'a> {
    w: &'a mut W,
}
impl<'a> VIN_BY_PASS_W<'a> {
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
    #[doc = "Bits 24:27 - TRIM_3P4"]
    #[inline(always)]
    pub fn trim_3p4(&self) -> TRIM_3P4_R {
        TRIM_3P4_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - TRIM_1P7"]
    #[inline(always)]
    pub fn trim_1p7(&self) -> TRIM_1P7_R {
        TRIM_1P7_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - TRIM_0P8"]
    #[inline(always)]
    pub fn trim_0p8(&self) -> TRIM_0P8_R {
        TRIM_0P8_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - VIN_AT_X"]
    #[inline(always)]
    pub fn vin_at_x(&self) -> VIN_AT_X_R {
        VIN_AT_X_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 8 - VIN_BY_PASS"]
    #[inline(always)]
    pub fn vin_by_pass(&self) -> VIN_BY_PASS_R {
        VIN_BY_PASS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:27 - TRIM_3P4"]
    #[inline(always)]
    pub fn trim_3p4(&mut self) -> TRIM_3P4_W {
        TRIM_3P4_W { w: self }
    }
    #[doc = "Bits 20:21 - TRIM_1P7"]
    #[inline(always)]
    pub fn trim_1p7(&mut self) -> TRIM_1P7_W {
        TRIM_1P7_W { w: self }
    }
    #[doc = "Bits 16:19 - TRIM_0P8"]
    #[inline(always)]
    pub fn trim_0p8(&mut self) -> TRIM_0P8_W {
        TRIM_0P8_W { w: self }
    }
    #[doc = "Bits 12:14 - VIN_AT_X"]
    #[inline(always)]
    pub fn vin_at_x(&mut self) -> VIN_AT_X_W {
        VIN_AT_X_W { w: self }
    }
    #[doc = "Bit 8 - VIN_BY_PASS"]
    #[inline(always)]
    pub fn vin_by_pass(&mut self) -> VIN_BY_PASS_W {
        VIN_BY_PASS_W { w: self }
    }
}

#[doc = "Writer for register EFUSEREAD"]
pub type W = crate::W<u32, super::EFUSEREAD>;
#[doc = "Register EFUSEREAD `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSEREAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DATABIT`"]
pub struct DATABIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATABIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `READCLOCK`"]
pub struct READCLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> READCLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `DEBUG`"]
pub struct DEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_W<'a> {
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
#[doc = "Write proxy for field `SPARE`"]
pub struct SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_W<'a> {
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
#[doc = "Write proxy for field `MARGIN`"]
pub struct MARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MARGIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl W {
    #[doc = "Bits 8:9 - DATABIT"]
    #[inline(always)]
    pub fn databit(&mut self) -> DATABIT_W {
        DATABIT_W { w: self }
    }
    #[doc = "Bits 4:7 - READCLOCK"]
    #[inline(always)]
    pub fn readclock(&mut self) -> READCLOCK_W {
        READCLOCK_W { w: self }
    }
    #[doc = "Bit 3 - DEBUG"]
    #[inline(always)]
    pub fn debug(&mut self) -> DEBUG_W {
        DEBUG_W { w: self }
    }
    #[doc = "Bit 2 - SPARE"]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W { w: self }
    }
    #[doc = "Bits 0:1 - MARGIN"]
    #[inline(always)]
    pub fn margin(&mut self) -> MARGIN_W {
        MARGIN_W { w: self }
    }
}

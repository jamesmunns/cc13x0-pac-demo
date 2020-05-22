#[doc = "Writer for register EFUSERELEASE"]
pub type W = crate::W<u32, super::EFUSERELEASE>;
#[doc = "Register EFUSERELEASE `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSERELEASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ODPYEAR`"]
pub struct ODPYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> ODPYEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Write proxy for field `ODPMONTH`"]
pub struct ODPMONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> ODPMONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 21)) | (((value as u32) & 0x0f) << 21);
        self.w
    }
}
#[doc = "Write proxy for field `ODPDAY`"]
pub struct ODPDAY_W<'a> {
    w: &'a mut W,
}
impl<'a> ODPDAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `EFUSEYEAR`"]
pub struct EFUSEYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSEYEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `EFUSEMONTH`"]
pub struct EFUSEMONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSEMONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `EFUSEDAY`"]
pub struct EFUSEDAY_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSEDAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl W {
    #[doc = "Bits 25:31 - ODPYEAR"]
    #[inline(always)]
    pub fn odpyear(&mut self) -> ODPYEAR_W {
        ODPYEAR_W { w: self }
    }
    #[doc = "Bits 21:24 - ODPMONTH"]
    #[inline(always)]
    pub fn odpmonth(&mut self) -> ODPMONTH_W {
        ODPMONTH_W { w: self }
    }
    #[doc = "Bits 16:20 - ODPDAY"]
    #[inline(always)]
    pub fn odpday(&mut self) -> ODPDAY_W {
        ODPDAY_W { w: self }
    }
    #[doc = "Bits 9:15 - EFUSEYEAR"]
    #[inline(always)]
    pub fn efuseyear(&mut self) -> EFUSEYEAR_W {
        EFUSEYEAR_W { w: self }
    }
    #[doc = "Bits 5:8 - EFUSEMONTH"]
    #[inline(always)]
    pub fn efusemonth(&mut self) -> EFUSEMONTH_W {
        EFUSEMONTH_W { w: self }
    }
    #[doc = "Bits 0:4 - EFUSEDAY"]
    #[inline(always)]
    pub fn efuseday(&mut self) -> EFUSEDAY_W {
        EFUSEDAY_W { w: self }
    }
}

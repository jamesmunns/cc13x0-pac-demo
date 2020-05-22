#[doc = "Reader of register SYSGPOCTL"]
pub type R = crate::R<u32, super::SYSGPOCTL>;
#[doc = "Writer for register SYSGPOCTL"]
pub type W = crate::W<u32, super::SYSGPOCTL>;
#[doc = "Register SYSGPOCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSGPOCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPOCTL3`"]
pub type GPOCTL3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPOCTL3`"]
pub struct GPOCTL3_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOCTL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `GPOCTL2`"]
pub type GPOCTL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPOCTL2`"]
pub struct GPOCTL2_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOCTL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `GPOCTL1`"]
pub type GPOCTL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPOCTL1`"]
pub struct GPOCTL1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOCTL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `GPOCTL0`"]
pub type GPOCTL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPOCTL0`"]
pub struct GPOCTL0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOCTL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - GPOCTL3"]
    #[inline(always)]
    pub fn gpoctl3(&self) -> GPOCTL3_R {
        GPOCTL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - GPOCTL2"]
    #[inline(always)]
    pub fn gpoctl2(&self) -> GPOCTL2_R {
        GPOCTL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - GPOCTL1"]
    #[inline(always)]
    pub fn gpoctl1(&self) -> GPOCTL1_R {
        GPOCTL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - GPOCTL0"]
    #[inline(always)]
    pub fn gpoctl0(&self) -> GPOCTL0_R {
        GPOCTL0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - GPOCTL3"]
    #[inline(always)]
    pub fn gpoctl3(&mut self) -> GPOCTL3_W {
        GPOCTL3_W { w: self }
    }
    #[doc = "Bits 8:11 - GPOCTL2"]
    #[inline(always)]
    pub fn gpoctl2(&mut self) -> GPOCTL2_W {
        GPOCTL2_W { w: self }
    }
    #[doc = "Bits 4:7 - GPOCTL1"]
    #[inline(always)]
    pub fn gpoctl1(&mut self) -> GPOCTL1_W {
        GPOCTL1_W { w: self }
    }
    #[doc = "Bits 0:3 - GPOCTL0"]
    #[inline(always)]
    pub fn gpoctl0(&mut self) -> GPOCTL0_W {
        GPOCTL0_W { w: self }
    }
}

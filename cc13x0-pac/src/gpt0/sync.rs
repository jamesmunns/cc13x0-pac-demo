#[doc = "Reader of register SYNC"]
pub type R = crate::R<u32, super::SYNC>;
#[doc = "Writer for register SYNC"]
pub type W = crate::W<u32, super::SYNC>;
#[doc = "Register SYNC `reset()`'s with value 0"]
impl crate::ResetValue for super::SYNC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYNC3`"]
pub type SYNC3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC3`"]
pub struct SYNC3_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `SYNC2`"]
pub type SYNC2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC2`"]
pub struct SYNC2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SYNC1`"]
pub type SYNC1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC1`"]
pub struct SYNC1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SYNC0`"]
pub type SYNC0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNC0`"]
pub struct SYNC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - SYNC3"]
    #[inline(always)]
    pub fn sync3(&self) -> SYNC3_R {
        SYNC3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - SYNC2"]
    #[inline(always)]
    pub fn sync2(&self) -> SYNC2_R {
        SYNC2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - SYNC1"]
    #[inline(always)]
    pub fn sync1(&self) -> SYNC1_R {
        SYNC1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - SYNC0"]
    #[inline(always)]
    pub fn sync0(&self) -> SYNC0_R {
        SYNC0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - SYNC3"]
    #[inline(always)]
    pub fn sync3(&mut self) -> SYNC3_W {
        SYNC3_W { w: self }
    }
    #[doc = "Bits 4:5 - SYNC2"]
    #[inline(always)]
    pub fn sync2(&mut self) -> SYNC2_W {
        SYNC2_W { w: self }
    }
    #[doc = "Bits 2:3 - SYNC1"]
    #[inline(always)]
    pub fn sync1(&mut self) -> SYNC1_W {
        SYNC1_W { w: self }
    }
    #[doc = "Bits 0:1 - SYNC0"]
    #[inline(always)]
    pub fn sync0(&mut self) -> SYNC0_W {
        SYNC0_W { w: self }
    }
}

#[doc = "Reader of register IOMODE"]
pub type R = crate::R<u32, super::IOMODE>;
#[doc = "Writer for register IOMODE"]
pub type W = crate::W<u32, super::IOMODE>;
#[doc = "Register IOMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::IOMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IO7`"]
pub type IO7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO7`"]
pub struct IO7_W<'a> {
    w: &'a mut W,
}
impl<'a> IO7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `IO6`"]
pub type IO6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO6`"]
pub struct IO6_W<'a> {
    w: &'a mut W,
}
impl<'a> IO6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `IO5`"]
pub type IO5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO5`"]
pub struct IO5_W<'a> {
    w: &'a mut W,
}
impl<'a> IO5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `IO4`"]
pub type IO4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO4`"]
pub struct IO4_W<'a> {
    w: &'a mut W,
}
impl<'a> IO4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `IO3`"]
pub type IO3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO3`"]
pub struct IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> IO3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `IO2`"]
pub type IO2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO2`"]
pub struct IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> IO2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `IO1`"]
pub type IO1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO1`"]
pub struct IO1_W<'a> {
    w: &'a mut W,
}
impl<'a> IO1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `IO0`"]
pub type IO0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO0`"]
pub struct IO0_W<'a> {
    w: &'a mut W,
}
impl<'a> IO0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:15 - IO7"]
    #[inline(always)]
    pub fn io7(&self) -> IO7_R {
        IO7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - IO6"]
    #[inline(always)]
    pub fn io6(&self) -> IO6_R {
        IO6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - IO5"]
    #[inline(always)]
    pub fn io5(&self) -> IO5_R {
        IO5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - IO4"]
    #[inline(always)]
    pub fn io4(&self) -> IO4_R {
        IO4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - IO3"]
    #[inline(always)]
    pub fn io3(&self) -> IO3_R {
        IO3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - IO2"]
    #[inline(always)]
    pub fn io2(&self) -> IO2_R {
        IO2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - IO1"]
    #[inline(always)]
    pub fn io1(&self) -> IO1_R {
        IO1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - IO0"]
    #[inline(always)]
    pub fn io0(&self) -> IO0_R {
        IO0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 14:15 - IO7"]
    #[inline(always)]
    pub fn io7(&mut self) -> IO7_W {
        IO7_W { w: self }
    }
    #[doc = "Bits 12:13 - IO6"]
    #[inline(always)]
    pub fn io6(&mut self) -> IO6_W {
        IO6_W { w: self }
    }
    #[doc = "Bits 10:11 - IO5"]
    #[inline(always)]
    pub fn io5(&mut self) -> IO5_W {
        IO5_W { w: self }
    }
    #[doc = "Bits 8:9 - IO4"]
    #[inline(always)]
    pub fn io4(&mut self) -> IO4_W {
        IO4_W { w: self }
    }
    #[doc = "Bits 6:7 - IO3"]
    #[inline(always)]
    pub fn io3(&mut self) -> IO3_W {
        IO3_W { w: self }
    }
    #[doc = "Bits 4:5 - IO2"]
    #[inline(always)]
    pub fn io2(&mut self) -> IO2_W {
        IO2_W { w: self }
    }
    #[doc = "Bits 2:3 - IO1"]
    #[inline(always)]
    pub fn io1(&mut self) -> IO1_W {
        IO1_W { w: self }
    }
    #[doc = "Bits 0:1 - IO0"]
    #[inline(always)]
    pub fn io0(&mut self) -> IO0_W {
        IO0_W { w: self }
    }
}

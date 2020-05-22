#[doc = "Reader of register MCUWUSEL"]
pub type R = crate::R<u32, super::MCUWUSEL>;
#[doc = "Writer for register MCUWUSEL"]
pub type W = crate::W<u32, super::MCUWUSEL>;
#[doc = "Register MCUWUSEL `reset()`'s with value 0x3f3f_3f3f"]
impl crate::ResetValue for super::MCUWUSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f3f_3f3f
    }
}
#[doc = "Reader of field `WU3_EV`"]
pub type WU3_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WU3_EV`"]
pub struct WU3_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> WU3_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `WU2_EV`"]
pub type WU2_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WU2_EV`"]
pub struct WU2_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> WU2_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `WU1_EV`"]
pub type WU1_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WU1_EV`"]
pub struct WU1_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> WU1_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `WU0_EV`"]
pub type WU0_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WU0_EV`"]
pub struct WU0_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> WU0_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29 - WU3_EV"]
    #[inline(always)]
    pub fn wu3_ev(&self) -> WU3_EV_R {
        WU3_EV_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - WU2_EV"]
    #[inline(always)]
    pub fn wu2_ev(&self) -> WU2_EV_R {
        WU2_EV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - WU1_EV"]
    #[inline(always)]
    pub fn wu1_ev(&self) -> WU1_EV_R {
        WU1_EV_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - WU0_EV"]
    #[inline(always)]
    pub fn wu0_ev(&self) -> WU0_EV_R {
        WU0_EV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - WU3_EV"]
    #[inline(always)]
    pub fn wu3_ev(&mut self) -> WU3_EV_W {
        WU3_EV_W { w: self }
    }
    #[doc = "Bits 16:21 - WU2_EV"]
    #[inline(always)]
    pub fn wu2_ev(&mut self) -> WU2_EV_W {
        WU2_EV_W { w: self }
    }
    #[doc = "Bits 8:13 - WU1_EV"]
    #[inline(always)]
    pub fn wu1_ev(&mut self) -> WU1_EV_W {
        WU1_EV_W { w: self }
    }
    #[doc = "Bits 0:5 - WU0_EV"]
    #[inline(always)]
    pub fn wu0_ev(&mut self) -> WU0_EV_W {
        WU0_EV_W { w: self }
    }
}

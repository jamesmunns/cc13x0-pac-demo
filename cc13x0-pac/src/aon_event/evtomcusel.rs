#[doc = "Reader of register EVTOMCUSEL"]
pub type R = crate::R<u32, super::EVTOMCUSEL>;
#[doc = "Writer for register EVTOMCUSEL"]
pub type W = crate::W<u32, super::EVTOMCUSEL>;
#[doc = "Register EVTOMCUSEL `reset()`'s with value 0x002b_2b2b"]
impl crate::ResetValue for super::EVTOMCUSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x002b_2b2b
    }
}
#[doc = "Reader of field `AON_PROG2_EV`"]
pub type AON_PROG2_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AON_PROG2_EV`"]
pub struct AON_PROG2_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_PROG2_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `AON_PROG1_EV`"]
pub type AON_PROG1_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AON_PROG1_EV`"]
pub struct AON_PROG1_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_PROG1_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `AON_PROG0_EV`"]
pub type AON_PROG0_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AON_PROG0_EV`"]
pub struct AON_PROG0_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_PROG0_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:21 - AON_PROG2_EV"]
    #[inline(always)]
    pub fn aon_prog2_ev(&self) -> AON_PROG2_EV_R {
        AON_PROG2_EV_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - AON_PROG1_EV"]
    #[inline(always)]
    pub fn aon_prog1_ev(&self) -> AON_PROG1_EV_R {
        AON_PROG1_EV_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - AON_PROG0_EV"]
    #[inline(always)]
    pub fn aon_prog0_ev(&self) -> AON_PROG0_EV_R {
        AON_PROG0_EV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:21 - AON_PROG2_EV"]
    #[inline(always)]
    pub fn aon_prog2_ev(&mut self) -> AON_PROG2_EV_W {
        AON_PROG2_EV_W { w: self }
    }
    #[doc = "Bits 8:13 - AON_PROG1_EV"]
    #[inline(always)]
    pub fn aon_prog1_ev(&mut self) -> AON_PROG1_EV_W {
        AON_PROG1_EV_W { w: self }
    }
    #[doc = "Bits 0:5 - AON_PROG0_EV"]
    #[inline(always)]
    pub fn aon_prog0_ev(&mut self) -> AON_PROG0_EV_W {
        AON_PROG0_EV_W { w: self }
    }
}

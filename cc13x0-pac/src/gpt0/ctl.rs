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
#[doc = "Reader of field `TBPWML`"]
pub type TBPWML_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBPWML`"]
pub struct TBPWML_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPWML_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TBEVENT`"]
pub type TBEVENT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TBEVENT`"]
pub struct TBEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBEVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `TBSTALL`"]
pub type TBSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBSTALL`"]
pub struct TBSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TBSTALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TBEN`"]
pub type TBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBEN`"]
pub struct TBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBEN_W<'a> {
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
#[doc = "Reader of field `TAPWML`"]
pub type TAPWML_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAPWML`"]
pub struct TAPWML_W<'a> {
    w: &'a mut W,
}
impl<'a> TAPWML_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RTCEN`"]
pub type RTCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCEN`"]
pub struct RTCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TAEVENT`"]
pub type TAEVENT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TAEVENT`"]
pub struct TAEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAEVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `TASTALL`"]
pub type TASTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TASTALL`"]
pub struct TASTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TASTALL_W<'a> {
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
#[doc = "Reader of field `TAEN`"]
pub type TAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAEN`"]
pub struct TAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TAEN_W<'a> {
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
    #[doc = "Bit 14 - TBPWML"]
    #[inline(always)]
    pub fn tbpwml(&self) -> TBPWML_R {
        TBPWML_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - TBEVENT"]
    #[inline(always)]
    pub fn tbevent(&self) -> TBEVENT_R {
        TBEVENT_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9 - TBSTALL"]
    #[inline(always)]
    pub fn tbstall(&self) -> TBSTALL_R {
        TBSTALL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TBEN"]
    #[inline(always)]
    pub fn tben(&self) -> TBEN_R {
        TBEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TAPWML"]
    #[inline(always)]
    pub fn tapwml(&self) -> TAPWML_R {
        TAPWML_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTCEN"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - TAEVENT"]
    #[inline(always)]
    pub fn taevent(&self) -> TAEVENT_R {
        TAEVENT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - TASTALL"]
    #[inline(always)]
    pub fn tastall(&self) -> TASTALL_R {
        TASTALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TAEN"]
    #[inline(always)]
    pub fn taen(&self) -> TAEN_R {
        TAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - TBPWML"]
    #[inline(always)]
    pub fn tbpwml(&mut self) -> TBPWML_W {
        TBPWML_W { w: self }
    }
    #[doc = "Bits 10:11 - TBEVENT"]
    #[inline(always)]
    pub fn tbevent(&mut self) -> TBEVENT_W {
        TBEVENT_W { w: self }
    }
    #[doc = "Bit 9 - TBSTALL"]
    #[inline(always)]
    pub fn tbstall(&mut self) -> TBSTALL_W {
        TBSTALL_W { w: self }
    }
    #[doc = "Bit 8 - TBEN"]
    #[inline(always)]
    pub fn tben(&mut self) -> TBEN_W {
        TBEN_W { w: self }
    }
    #[doc = "Bit 6 - TAPWML"]
    #[inline(always)]
    pub fn tapwml(&mut self) -> TAPWML_W {
        TAPWML_W { w: self }
    }
    #[doc = "Bit 4 - RTCEN"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W {
        RTCEN_W { w: self }
    }
    #[doc = "Bits 2:3 - TAEVENT"]
    #[inline(always)]
    pub fn taevent(&mut self) -> TAEVENT_W {
        TAEVENT_W { w: self }
    }
    #[doc = "Bit 1 - TASTALL"]
    #[inline(always)]
    pub fn tastall(&mut self) -> TASTALL_W {
        TASTALL_W { w: self }
    }
    #[doc = "Bit 0 - TAEN"]
    #[inline(always)]
    pub fn taen(&mut self) -> TAEN_W {
        TAEN_W { w: self }
    }
}

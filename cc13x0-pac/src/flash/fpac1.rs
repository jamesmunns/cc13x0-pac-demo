#[doc = "Reader of register FPAC1"]
pub type R = crate::R<u32, super::FPAC1>;
#[doc = "Writer for register FPAC1"]
pub type W = crate::W<u32, super::FPAC1>;
#[doc = "Register FPAC1 `reset()`'s with value 0x0208_2081"]
impl crate::ResetValue for super::FPAC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0208_2081
    }
}
#[doc = "Reader of field `PSLEEPTDIS`"]
pub type PSLEEPTDIS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PSLEEPTDIS`"]
pub struct PSLEEPTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PSLEEPTDIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PUMPRESET_PW`"]
pub type PUMPRESET_PW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PUMPRESET_PW`"]
pub struct PUMPRESET_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMPRESET_PW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Reader of field `PUMPPWR`"]
pub type PUMPPWR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUMPPWR`"]
pub struct PUMPPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMPPWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27 - PSLEEPTDIS"]
    #[inline(always)]
    pub fn psleeptdis(&self) -> PSLEEPTDIS_R {
        PSLEEPTDIS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 4:15 - PUMPRESET_PW"]
    #[inline(always)]
    pub fn pumpreset_pw(&self) -> PUMPRESET_PW_R {
        PUMPRESET_PW_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:1 - PUMPPWR"]
    #[inline(always)]
    pub fn pumppwr(&self) -> PUMPPWR_R {
        PUMPPWR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:27 - PSLEEPTDIS"]
    #[inline(always)]
    pub fn psleeptdis(&mut self) -> PSLEEPTDIS_W {
        PSLEEPTDIS_W { w: self }
    }
    #[doc = "Bits 4:15 - PUMPRESET_PW"]
    #[inline(always)]
    pub fn pumpreset_pw(&mut self) -> PUMPRESET_PW_W {
        PUMPRESET_PW_W { w: self }
    }
    #[doc = "Bits 0:1 - PUMPPWR"]
    #[inline(always)]
    pub fn pumppwr(&mut self) -> PUMPPWR_W {
        PUMPPWR_W { w: self }
    }
}

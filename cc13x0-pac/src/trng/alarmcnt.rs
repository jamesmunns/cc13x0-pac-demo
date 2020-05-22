#[doc = "Reader of register ALARMCNT"]
pub type R = crate::R<u32, super::ALARMCNT>;
#[doc = "Writer for register ALARMCNT"]
pub type W = crate::W<u32, super::ALARMCNT>;
#[doc = "Register ALARMCNT `reset()`'s with value 0xff"]
impl crate::ResetValue for super::ALARMCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `SHUTDOWN_CNT`"]
pub type SHUTDOWN_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHUTDOWN_CNT`"]
pub struct SHUTDOWN_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHUTDOWN_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `SHUTDOWN_THR`"]
pub type SHUTDOWN_THR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SHUTDOWN_THR`"]
pub struct SHUTDOWN_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> SHUTDOWN_THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ALARM_THR`"]
pub type ALARM_THR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALARM_THR`"]
pub struct ALARM_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM_THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29 - SHUTDOWN_CNT"]
    #[inline(always)]
    pub fn shutdown_cnt(&self) -> SHUTDOWN_CNT_R {
        SHUTDOWN_CNT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - SHUTDOWN_THR"]
    #[inline(always)]
    pub fn shutdown_thr(&self) -> SHUTDOWN_THR_R {
        SHUTDOWN_THR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 0:7 - ALARM_THR"]
    #[inline(always)]
    pub fn alarm_thr(&self) -> ALARM_THR_R {
        ALARM_THR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - SHUTDOWN_CNT"]
    #[inline(always)]
    pub fn shutdown_cnt(&mut self) -> SHUTDOWN_CNT_W {
        SHUTDOWN_CNT_W { w: self }
    }
    #[doc = "Bits 16:20 - SHUTDOWN_THR"]
    #[inline(always)]
    pub fn shutdown_thr(&mut self) -> SHUTDOWN_THR_W {
        SHUTDOWN_THR_W { w: self }
    }
    #[doc = "Bits 0:7 - ALARM_THR"]
    #[inline(always)]
    pub fn alarm_thr(&mut self) -> ALARM_THR_W {
        ALARM_THR_W { w: self }
    }
}

#[doc = "Reader of register FSM_FLES"]
pub type R = crate::R<u32, super::FSM_FLES>;
#[doc = "Writer for register FSM_FLES"]
pub type W = crate::W<u32, super::FSM_FLES>;
#[doc = "Register FSM_FLES `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_FLES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLK_TIOTP`"]
pub type BLK_TIOTP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLK_TIOTP`"]
pub struct BLK_TIOTP_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK_TIOTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `BLK_OTP`"]
pub type BLK_OTP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLK_OTP`"]
pub struct BLK_OTP_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK_OTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - BLK_TIOTP"]
    #[inline(always)]
    pub fn blk_tiotp(&self) -> BLK_TIOTP_R {
        BLK_TIOTP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - BLK_OTP"]
    #[inline(always)]
    pub fn blk_otp(&self) -> BLK_OTP_R {
        BLK_OTP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - BLK_TIOTP"]
    #[inline(always)]
    pub fn blk_tiotp(&mut self) -> BLK_TIOTP_W {
        BLK_TIOTP_W { w: self }
    }
    #[doc = "Bits 0:7 - BLK_OTP"]
    #[inline(always)]
    pub fn blk_otp(&mut self) -> BLK_OTP_W {
        BLK_OTP_W { w: self }
    }
}

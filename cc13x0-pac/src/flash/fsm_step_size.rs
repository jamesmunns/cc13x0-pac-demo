#[doc = "Reader of register FSM_STEP_SIZE"]
pub type R = crate::R<u32, super::FSM_STEP_SIZE>;
#[doc = "Writer for register FSM_STEP_SIZE"]
pub type W = crate::W<u32, super::FSM_STEP_SIZE>;
#[doc = "Register FSM_STEP_SIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_STEP_SIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EC_STEP_SIZE`"]
pub type EC_STEP_SIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EC_STEP_SIZE`"]
pub struct EC_STEP_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> EC_STEP_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:24 - EC_STEP_SIZE"]
    #[inline(always)]
    pub fn ec_step_size(&self) -> EC_STEP_SIZE_R {
        EC_STEP_SIZE_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:24 - EC_STEP_SIZE"]
    #[inline(always)]
    pub fn ec_step_size(&mut self) -> EC_STEP_SIZE_W {
        EC_STEP_SIZE_W { w: self }
    }
}

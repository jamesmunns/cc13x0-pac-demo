#[doc = "Reader of register MCUBUSCTL"]
pub type R = crate::R<u32, super::MCUBUSCTL>;
#[doc = "Writer for register MCUBUSCTL"]
pub type W = crate::W<u32, super::MCUBUSCTL>;
#[doc = "Register MCUBUSCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::MCUBUSCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DISCONNECT_REQ`"]
pub type DISCONNECT_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCONNECT_REQ`"]
pub struct DISCONNECT_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCONNECT_REQ_W<'a> {
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
    #[doc = "Bit 0 - DISCONNECT_REQ"]
    #[inline(always)]
    pub fn disconnect_req(&self) -> DISCONNECT_REQ_R {
        DISCONNECT_REQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DISCONNECT_REQ"]
    #[inline(always)]
    pub fn disconnect_req(&mut self) -> DISCONNECT_REQ_W {
        DISCONNECT_REQ_W { w: self }
    }
}

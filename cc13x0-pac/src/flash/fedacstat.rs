#[doc = "Reader of register FEDACSTAT"]
pub type R = crate::R<u32, super::FEDACSTAT>;
#[doc = "Writer for register FEDACSTAT"]
pub type W = crate::W<u32, super::FEDACSTAT>;
#[doc = "Register FEDACSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::FEDACSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RVF_INT`"]
pub type RVF_INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RVF_INT`"]
pub struct RVF_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RVF_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `FSM_DONE`"]
pub type FSM_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSM_DONE`"]
pub struct FSM_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 25 - RVF_INT"]
    #[inline(always)]
    pub fn rvf_int(&self) -> RVF_INT_R {
        RVF_INT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FSM_DONE"]
    #[inline(always)]
    pub fn fsm_done(&self) -> FSM_DONE_R {
        FSM_DONE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - RVF_INT"]
    #[inline(always)]
    pub fn rvf_int(&mut self) -> RVF_INT_W {
        RVF_INT_W { w: self }
    }
    #[doc = "Bit 24 - FSM_DONE"]
    #[inline(always)]
    pub fn fsm_done(&mut self) -> FSM_DONE_W {
        FSM_DONE_W { w: self }
    }
}

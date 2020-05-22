#[doc = "Reader of register PWRDWNREQ"]
pub type R = crate::R<u32, super::PWRDWNREQ>;
#[doc = "Writer for register PWRDWNREQ"]
pub type W = crate::W<u32, super::PWRDWNREQ>;
#[doc = "Register PWRDWNREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRDWNREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REQ`"]
pub type REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REQ`"]
pub struct REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_W<'a> {
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
    #[doc = "Bit 0 - REQ"]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REQ"]
    #[inline(always)]
    pub fn req(&mut self) -> REQ_W {
        REQ_W { w: self }
    }
}

#[doc = "Reader of register TDCCLKCTL"]
pub type R = crate::R<u32, super::TDCCLKCTL>;
#[doc = "Writer for register TDCCLKCTL"]
pub type W = crate::W<u32, super::TDCCLKCTL>;
#[doc = "Register TDCCLKCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TDCCLKCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACK`"]
pub type ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACK`"]
pub struct ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_W<'a> {
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
    #[doc = "Bit 1 - ACK"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - REQ"]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ACK"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
    }
    #[doc = "Bit 0 - REQ"]
    #[inline(always)]
    pub fn req(&mut self) -> REQ_W {
        REQ_W { w: self }
    }
}

#[doc = "Reader of register IRQTYPE"]
pub type R = crate::R<u32, super::IRQTYPE>;
#[doc = "Writer for register IRQTYPE"]
pub type W = crate::W<u32, super::IRQTYPE>;
#[doc = "Register IRQTYPE `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQTYPE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IEN`"]
pub type IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEN`"]
pub struct IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IEN_W<'a> {
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
    #[doc = "Bit 0 - IEN"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IEN"]
    #[inline(always)]
    pub fn ien(&mut self) -> IEN_W {
        IEN_W { w: self }
    }
}

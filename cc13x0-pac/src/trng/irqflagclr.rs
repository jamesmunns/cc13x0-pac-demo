#[doc = "Writer for register IRQFLAGCLR"]
pub type W = crate::W<u32, super::IRQFLAGCLR>;
#[doc = "Register IRQFLAGCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::IRQFLAGCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SHUTDOWN_OVF`"]
pub struct SHUTDOWN_OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> SHUTDOWN_OVF_W<'a> {
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
#[doc = "Write proxy for field `RDY`"]
pub struct RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RDY_W<'a> {
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
impl W {
    #[doc = "Bit 1 - SHUTDOWN_OVF"]
    #[inline(always)]
    pub fn shutdown_ovf(&mut self) -> SHUTDOWN_OVF_W {
        SHUTDOWN_OVF_W { w: self }
    }
    #[doc = "Bit 0 - RDY"]
    #[inline(always)]
    pub fn rdy(&mut self) -> RDY_W {
        RDY_W { w: self }
    }
}

#[doc = "Writer for register EFUSECFG"]
pub type W = crate::W<u32, super::EFUSECFG>;
#[doc = "Register EFUSECFG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::EFUSECFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Write proxy for field `IDLEGATING`"]
pub struct IDLEGATING_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLEGATING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `SLAVEPOWER`"]
pub struct SLAVEPOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVEPOWER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `GATING`"]
pub struct GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> GATING_W<'a> {
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
    #[doc = "Bit 8 - IDLEGATING"]
    #[inline(always)]
    pub fn idlegating(&mut self) -> IDLEGATING_W {
        IDLEGATING_W { w: self }
    }
    #[doc = "Bits 3:4 - SLAVEPOWER"]
    #[inline(always)]
    pub fn slavepower(&mut self) -> SLAVEPOWER_W {
        SLAVEPOWER_W { w: self }
    }
    #[doc = "Bit 0 - GATING"]
    #[inline(always)]
    pub fn gating(&mut self) -> GATING_W {
        GATING_W { w: self }
    }
}

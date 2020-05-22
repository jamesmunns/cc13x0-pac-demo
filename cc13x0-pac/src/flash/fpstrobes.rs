#[doc = "Reader of register FPSTROBES"]
pub type R = crate::R<u32, super::FPSTROBES>;
#[doc = "Writer for register FPSTROBES"]
pub type W = crate::W<u32, super::FPSTROBES>;
#[doc = "Register FPSTROBES `reset()`'s with value 0x0103"]
impl crate::ResetValue for super::FPSTROBES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0103
    }
}
#[doc = "Reader of field `EXECUTEZ`"]
pub type EXECUTEZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXECUTEZ`"]
pub struct EXECUTEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> EXECUTEZ_W<'a> {
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
#[doc = "Reader of field `V3PWRDNZ`"]
pub type V3PWRDNZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `V3PWRDNZ`"]
pub struct V3PWRDNZ_W<'a> {
    w: &'a mut W,
}
impl<'a> V3PWRDNZ_W<'a> {
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
#[doc = "Reader of field `V5PWRDNZ`"]
pub type V5PWRDNZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `V5PWRDNZ`"]
pub struct V5PWRDNZ_W<'a> {
    w: &'a mut W,
}
impl<'a> V5PWRDNZ_W<'a> {
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
    #[doc = "Bit 8 - EXECUTEZ"]
    #[inline(always)]
    pub fn executez(&self) -> EXECUTEZ_R {
        EXECUTEZ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1 - V3PWRDNZ"]
    #[inline(always)]
    pub fn v3pwrdnz(&self) -> V3PWRDNZ_R {
        V3PWRDNZ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - V5PWRDNZ"]
    #[inline(always)]
    pub fn v5pwrdnz(&self) -> V5PWRDNZ_R {
        V5PWRDNZ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - EXECUTEZ"]
    #[inline(always)]
    pub fn executez(&mut self) -> EXECUTEZ_W {
        EXECUTEZ_W { w: self }
    }
    #[doc = "Bit 1 - V3PWRDNZ"]
    #[inline(always)]
    pub fn v3pwrdnz(&mut self) -> V3PWRDNZ_W {
        V3PWRDNZ_W { w: self }
    }
    #[doc = "Bit 0 - V5PWRDNZ"]
    #[inline(always)]
    pub fn v5pwrdnz(&mut self) -> V5PWRDNZ_W {
        V5PWRDNZ_W { w: self }
    }
}

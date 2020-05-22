#[doc = "Reader of register SWEV"]
pub type R = crate::R<u32, super::SWEV>;
#[doc = "Writer for register SWEV"]
pub type W = crate::W<u32, super::SWEV>;
#[doc = "Register SWEV `reset()`'s with value 0"]
impl crate::ResetValue for super::SWEV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWEV3`"]
pub type SWEV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWEV3`"]
pub struct SWEV3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEV3_W<'a> {
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
#[doc = "Reader of field `SWEV2`"]
pub type SWEV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWEV2`"]
pub struct SWEV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEV2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SWEV1`"]
pub type SWEV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWEV1`"]
pub struct SWEV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEV1_W<'a> {
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
#[doc = "Reader of field `SWEV0`"]
pub type SWEV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWEV0`"]
pub struct SWEV0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEV0_W<'a> {
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
    #[doc = "Bit 24 - SWEV3"]
    #[inline(always)]
    pub fn swev3(&self) -> SWEV3_R {
        SWEV3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SWEV2"]
    #[inline(always)]
    pub fn swev2(&self) -> SWEV2_R {
        SWEV2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SWEV1"]
    #[inline(always)]
    pub fn swev1(&self) -> SWEV1_R {
        SWEV1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SWEV0"]
    #[inline(always)]
    pub fn swev0(&self) -> SWEV0_R {
        SWEV0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - SWEV3"]
    #[inline(always)]
    pub fn swev3(&mut self) -> SWEV3_W {
        SWEV3_W { w: self }
    }
    #[doc = "Bit 16 - SWEV2"]
    #[inline(always)]
    pub fn swev2(&mut self) -> SWEV2_W {
        SWEV2_W { w: self }
    }
    #[doc = "Bit 8 - SWEV1"]
    #[inline(always)]
    pub fn swev1(&mut self) -> SWEV1_W {
        SWEV1_W { w: self }
    }
    #[doc = "Bit 0 - SWEV0"]
    #[inline(always)]
    pub fn swev0(&mut self) -> SWEV0_W {
        SWEV0_W { w: self }
    }
}

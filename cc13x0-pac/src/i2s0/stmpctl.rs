#[doc = "Reader of register STMPCTL"]
pub type R = crate::R<u32, super::STMPCTL>;
#[doc = "Writer for register STMPCTL"]
pub type W = crate::W<u32, super::STMPCTL>;
#[doc = "Register STMPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::STMPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUT_RDY`"]
pub type OUT_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUT_RDY`"]
pub struct OUT_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `IN_RDY`"]
pub type IN_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_RDY`"]
pub struct IN_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_RDY_W<'a> {
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
#[doc = "Reader of field `STMP_EN`"]
pub type STMP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STMP_EN`"]
pub struct STMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> STMP_EN_W<'a> {
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
    #[doc = "Bit 2 - OUT_RDY"]
    #[inline(always)]
    pub fn out_rdy(&self) -> OUT_RDY_R {
        OUT_RDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - IN_RDY"]
    #[inline(always)]
    pub fn in_rdy(&self) -> IN_RDY_R {
        IN_RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - STMP_EN"]
    #[inline(always)]
    pub fn stmp_en(&self) -> STMP_EN_R {
        STMP_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - OUT_RDY"]
    #[inline(always)]
    pub fn out_rdy(&mut self) -> OUT_RDY_W {
        OUT_RDY_W { w: self }
    }
    #[doc = "Bit 1 - IN_RDY"]
    #[inline(always)]
    pub fn in_rdy(&mut self) -> IN_RDY_W {
        IN_RDY_W { w: self }
    }
    #[doc = "Bit 0 - STMP_EN"]
    #[inline(always)]
    pub fn stmp_en(&mut self) -> STMP_EN_W {
        STMP_EN_W { w: self }
    }
}

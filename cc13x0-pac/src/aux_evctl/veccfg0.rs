#[doc = "Reader of register VECCFG0"]
pub type R = crate::R<u32, super::VECCFG0>;
#[doc = "Writer for register VECCFG0"]
pub type W = crate::W<u32, super::VECCFG0>;
#[doc = "Register VECCFG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::VECCFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VEC1_POL`"]
pub type VEC1_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VEC1_POL`"]
pub struct VEC1_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC1_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `VEC1_EN`"]
pub type VEC1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VEC1_EN`"]
pub struct VEC1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `VEC1_EV`"]
pub type VEC1_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VEC1_EV`"]
pub struct VEC1_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC1_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `VEC0_POL`"]
pub type VEC0_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VEC0_POL`"]
pub struct VEC0_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC0_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `VEC0_EN`"]
pub type VEC0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VEC0_EN`"]
pub struct VEC0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC0_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `VEC0_EV`"]
pub type VEC0_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VEC0_EV`"]
pub struct VEC0_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC0_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 14 - VEC1_POL"]
    #[inline(always)]
    pub fn vec1_pol(&self) -> VEC1_POL_R {
        VEC1_POL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VEC1_EN"]
    #[inline(always)]
    pub fn vec1_en(&self) -> VEC1_EN_R {
        VEC1_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - VEC1_EV"]
    #[inline(always)]
    pub fn vec1_ev(&self) -> VEC1_EV_R {
        VEC1_EV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - VEC0_POL"]
    #[inline(always)]
    pub fn vec0_pol(&self) -> VEC0_POL_R {
        VEC0_POL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - VEC0_EN"]
    #[inline(always)]
    pub fn vec0_en(&self) -> VEC0_EN_R {
        VEC0_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - VEC0_EV"]
    #[inline(always)]
    pub fn vec0_ev(&self) -> VEC0_EV_R {
        VEC0_EV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 14 - VEC1_POL"]
    #[inline(always)]
    pub fn vec1_pol(&mut self) -> VEC1_POL_W {
        VEC1_POL_W { w: self }
    }
    #[doc = "Bit 13 - VEC1_EN"]
    #[inline(always)]
    pub fn vec1_en(&mut self) -> VEC1_EN_W {
        VEC1_EN_W { w: self }
    }
    #[doc = "Bits 8:12 - VEC1_EV"]
    #[inline(always)]
    pub fn vec1_ev(&mut self) -> VEC1_EV_W {
        VEC1_EV_W { w: self }
    }
    #[doc = "Bit 6 - VEC0_POL"]
    #[inline(always)]
    pub fn vec0_pol(&mut self) -> VEC0_POL_W {
        VEC0_POL_W { w: self }
    }
    #[doc = "Bit 5 - VEC0_EN"]
    #[inline(always)]
    pub fn vec0_en(&mut self) -> VEC0_EN_W {
        VEC0_EN_W { w: self }
    }
    #[doc = "Bits 0:4 - VEC0_EV"]
    #[inline(always)]
    pub fn vec0_ev(&mut self) -> VEC0_EV_W {
        VEC0_EV_W { w: self }
    }
}

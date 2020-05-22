#[doc = "Reader of register VECCFG1"]
pub type R = crate::R<u32, super::VECCFG1>;
#[doc = "Writer for register VECCFG1"]
pub type W = crate::W<u32, super::VECCFG1>;
#[doc = "Register VECCFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::VECCFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VEC3_POL`"]
pub type VEC3_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VEC3_POL`"]
pub struct VEC3_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC3_POL_W<'a> {
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
#[doc = "Reader of field `VEC3_EN`"]
pub type VEC3_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VEC3_EN`"]
pub struct VEC3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC3_EN_W<'a> {
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
#[doc = "Reader of field `VEC3_EV`"]
pub type VEC3_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VEC3_EV`"]
pub struct VEC3_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC3_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `VEC2_POL`"]
pub type VEC2_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VEC2_POL`"]
pub struct VEC2_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC2_POL_W<'a> {
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
#[doc = "Reader of field `VEC2_EN`"]
pub type VEC2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VEC2_EN`"]
pub struct VEC2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC2_EN_W<'a> {
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
#[doc = "Reader of field `VEC2_EV`"]
pub type VEC2_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VEC2_EV`"]
pub struct VEC2_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC2_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 14 - VEC3_POL"]
    #[inline(always)]
    pub fn vec3_pol(&self) -> VEC3_POL_R {
        VEC3_POL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VEC3_EN"]
    #[inline(always)]
    pub fn vec3_en(&self) -> VEC3_EN_R {
        VEC3_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - VEC3_EV"]
    #[inline(always)]
    pub fn vec3_ev(&self) -> VEC3_EV_R {
        VEC3_EV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - VEC2_POL"]
    #[inline(always)]
    pub fn vec2_pol(&self) -> VEC2_POL_R {
        VEC2_POL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - VEC2_EN"]
    #[inline(always)]
    pub fn vec2_en(&self) -> VEC2_EN_R {
        VEC2_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - VEC2_EV"]
    #[inline(always)]
    pub fn vec2_ev(&self) -> VEC2_EV_R {
        VEC2_EV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 14 - VEC3_POL"]
    #[inline(always)]
    pub fn vec3_pol(&mut self) -> VEC3_POL_W {
        VEC3_POL_W { w: self }
    }
    #[doc = "Bit 13 - VEC3_EN"]
    #[inline(always)]
    pub fn vec3_en(&mut self) -> VEC3_EN_W {
        VEC3_EN_W { w: self }
    }
    #[doc = "Bits 8:12 - VEC3_EV"]
    #[inline(always)]
    pub fn vec3_ev(&mut self) -> VEC3_EV_W {
        VEC3_EV_W { w: self }
    }
    #[doc = "Bit 6 - VEC2_POL"]
    #[inline(always)]
    pub fn vec2_pol(&mut self) -> VEC2_POL_W {
        VEC2_POL_W { w: self }
    }
    #[doc = "Bit 5 - VEC2_EN"]
    #[inline(always)]
    pub fn vec2_en(&mut self) -> VEC2_EN_W {
        VEC2_EN_W { w: self }
    }
    #[doc = "Bits 0:4 - VEC2_EV"]
    #[inline(always)]
    pub fn vec2_ev(&mut self) -> VEC2_EV_W {
        VEC2_EV_W { w: self }
    }
}

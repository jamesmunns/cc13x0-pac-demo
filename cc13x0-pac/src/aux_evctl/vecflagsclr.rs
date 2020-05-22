#[doc = "Reader of register VECFLAGSCLR"]
pub type R = crate::R<u32, super::VECFLAGSCLR>;
#[doc = "Writer for register VECFLAGSCLR"]
pub type W = crate::W<u32, super::VECFLAGSCLR>;
#[doc = "Register VECFLAGSCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::VECFLAGSCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VEC3`"]
pub type VEC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VEC3`"]
pub struct VEC3_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `VEC2`"]
pub type VEC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VEC2`"]
pub struct VEC2_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC2_W<'a> {
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
#[doc = "Reader of field `VEC1`"]
pub type VEC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VEC1`"]
pub struct VEC1_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC1_W<'a> {
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
#[doc = "Reader of field `VEC0`"]
pub type VEC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VEC0`"]
pub struct VEC0_W<'a> {
    w: &'a mut W,
}
impl<'a> VEC0_W<'a> {
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
    #[doc = "Bit 3 - VEC3"]
    #[inline(always)]
    pub fn vec3(&self) -> VEC3_R {
        VEC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VEC2"]
    #[inline(always)]
    pub fn vec2(&self) -> VEC2_R {
        VEC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - VEC1"]
    #[inline(always)]
    pub fn vec1(&self) -> VEC1_R {
        VEC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - VEC0"]
    #[inline(always)]
    pub fn vec0(&self) -> VEC0_R {
        VEC0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - VEC3"]
    #[inline(always)]
    pub fn vec3(&mut self) -> VEC3_W {
        VEC3_W { w: self }
    }
    #[doc = "Bit 2 - VEC2"]
    #[inline(always)]
    pub fn vec2(&mut self) -> VEC2_W {
        VEC2_W { w: self }
    }
    #[doc = "Bit 1 - VEC1"]
    #[inline(always)]
    pub fn vec1(&mut self) -> VEC1_W {
        VEC1_W { w: self }
    }
    #[doc = "Bit 0 - VEC0"]
    #[inline(always)]
    pub fn vec0(&mut self) -> VEC0_W {
        VEC0_W { w: self }
    }
}

#[doc = "Reader of register FSPRD"]
pub type R = crate::R<u32, super::FSPRD>;
#[doc = "Writer for register FSPRD"]
pub type W = crate::W<u32, super::FSPRD>;
#[doc = "Register FSPRD `reset()`'s with value 0"]
impl crate::ResetValue for super::FSPRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMBSEM`"]
pub type RMBSEM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMBSEM`"]
pub struct RMBSEM_W<'a> {
    w: &'a mut W,
}
impl<'a> RMBSEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RM1`"]
pub type RM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RM1`"]
pub struct RM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RM1_W<'a> {
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
#[doc = "Reader of field `RM0`"]
pub type RM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RM0`"]
pub struct RM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RM0_W<'a> {
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
    #[doc = "Bits 8:15 - RMBSEM"]
    #[inline(always)]
    pub fn rmbsem(&self) -> RMBSEM_R {
        RMBSEM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 1 - RM1"]
    #[inline(always)]
    pub fn rm1(&self) -> RM1_R {
        RM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RM0"]
    #[inline(always)]
    pub fn rm0(&self) -> RM0_R {
        RM0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:15 - RMBSEM"]
    #[inline(always)]
    pub fn rmbsem(&mut self) -> RMBSEM_W {
        RMBSEM_W { w: self }
    }
    #[doc = "Bit 1 - RM1"]
    #[inline(always)]
    pub fn rm1(&mut self) -> RM1_W {
        RM1_W { w: self }
    }
    #[doc = "Bit 0 - RM0"]
    #[inline(always)]
    pub fn rm0(&mut self) -> RM0_W {
        RM0_W { w: self }
    }
}

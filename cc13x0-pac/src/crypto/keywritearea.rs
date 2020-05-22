#[doc = "Reader of register KEYWRITEAREA"]
pub type R = crate::R<u32, super::KEYWRITEAREA>;
#[doc = "Writer for register KEYWRITEAREA"]
pub type W = crate::W<u32, super::KEYWRITEAREA>;
#[doc = "Register KEYWRITEAREA `reset()`'s with value 0"]
impl crate::ResetValue for super::KEYWRITEAREA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RAM_AREA7`"]
pub type RAM_AREA7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA7`"]
pub struct RAM_AREA7_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RAM_AREA6`"]
pub type RAM_AREA6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA6`"]
pub struct RAM_AREA6_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA6_W<'a> {
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
#[doc = "Reader of field `RAM_AREA5`"]
pub type RAM_AREA5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA5`"]
pub struct RAM_AREA5_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA5_W<'a> {
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
#[doc = "Reader of field `RAM_AREA4`"]
pub type RAM_AREA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA4`"]
pub struct RAM_AREA4_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RAM_AREA3`"]
pub type RAM_AREA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA3`"]
pub struct RAM_AREA3_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA3_W<'a> {
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
#[doc = "Reader of field `RAM_AREA2`"]
pub type RAM_AREA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA2`"]
pub struct RAM_AREA2_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA2_W<'a> {
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
#[doc = "Reader of field `RAM_AREA1`"]
pub type RAM_AREA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA1`"]
pub struct RAM_AREA1_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA1_W<'a> {
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
#[doc = "Reader of field `RAM_AREA0`"]
pub type RAM_AREA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_AREA0`"]
pub struct RAM_AREA0_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA0_W<'a> {
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
    #[doc = "Bit 7 - RAM_AREA7"]
    #[inline(always)]
    pub fn ram_area7(&self) -> RAM_AREA7_R {
        RAM_AREA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RAM_AREA6"]
    #[inline(always)]
    pub fn ram_area6(&self) -> RAM_AREA6_R {
        RAM_AREA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RAM_AREA5"]
    #[inline(always)]
    pub fn ram_area5(&self) -> RAM_AREA5_R {
        RAM_AREA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RAM_AREA4"]
    #[inline(always)]
    pub fn ram_area4(&self) -> RAM_AREA4_R {
        RAM_AREA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RAM_AREA3"]
    #[inline(always)]
    pub fn ram_area3(&self) -> RAM_AREA3_R {
        RAM_AREA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RAM_AREA2"]
    #[inline(always)]
    pub fn ram_area2(&self) -> RAM_AREA2_R {
        RAM_AREA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - RAM_AREA1"]
    #[inline(always)]
    pub fn ram_area1(&self) -> RAM_AREA1_R {
        RAM_AREA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RAM_AREA0"]
    #[inline(always)]
    pub fn ram_area0(&self) -> RAM_AREA0_R {
        RAM_AREA0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - RAM_AREA7"]
    #[inline(always)]
    pub fn ram_area7(&mut self) -> RAM_AREA7_W {
        RAM_AREA7_W { w: self }
    }
    #[doc = "Bit 6 - RAM_AREA6"]
    #[inline(always)]
    pub fn ram_area6(&mut self) -> RAM_AREA6_W {
        RAM_AREA6_W { w: self }
    }
    #[doc = "Bit 5 - RAM_AREA5"]
    #[inline(always)]
    pub fn ram_area5(&mut self) -> RAM_AREA5_W {
        RAM_AREA5_W { w: self }
    }
    #[doc = "Bit 4 - RAM_AREA4"]
    #[inline(always)]
    pub fn ram_area4(&mut self) -> RAM_AREA4_W {
        RAM_AREA4_W { w: self }
    }
    #[doc = "Bit 3 - RAM_AREA3"]
    #[inline(always)]
    pub fn ram_area3(&mut self) -> RAM_AREA3_W {
        RAM_AREA3_W { w: self }
    }
    #[doc = "Bit 2 - RAM_AREA2"]
    #[inline(always)]
    pub fn ram_area2(&mut self) -> RAM_AREA2_W {
        RAM_AREA2_W { w: self }
    }
    #[doc = "Bit 1 - RAM_AREA1"]
    #[inline(always)]
    pub fn ram_area1(&mut self) -> RAM_AREA1_W {
        RAM_AREA1_W { w: self }
    }
    #[doc = "Bit 0 - RAM_AREA0"]
    #[inline(always)]
    pub fn ram_area0(&mut self) -> RAM_AREA0_W {
        RAM_AREA0_W { w: self }
    }
}

#[doc = "Reader of register T1CFG"]
pub type R = crate::R<u32, super::T1CFG>;
#[doc = "Writer for register T1CFG"]
pub type W = crate::W<u32, super::T1CFG>;
#[doc = "Register T1CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::T1CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TICK_SRC_POL`"]
pub type TICK_SRC_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TICK_SRC_POL`"]
pub struct TICK_SRC_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_SRC_POL_W<'a> {
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
#[doc = "Reader of field `TICK_SRC`"]
pub type TICK_SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TICK_SRC`"]
pub struct TICK_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PRE`"]
pub type PRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRE`"]
pub struct PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
#[doc = "Reader of field `RELOAD`"]
pub type RELOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RELOAD`"]
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
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
    #[doc = "Bit 13 - TICK_SRC_POL"]
    #[inline(always)]
    pub fn tick_src_pol(&self) -> TICK_SRC_POL_R {
        TICK_SRC_POL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - TICK_SRC"]
    #[inline(always)]
    pub fn tick_src(&self) -> TICK_SRC_R {
        TICK_SRC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 4:7 - PRE"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RELOAD"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - TICK_SRC_POL"]
    #[inline(always)]
    pub fn tick_src_pol(&mut self) -> TICK_SRC_POL_W {
        TICK_SRC_POL_W { w: self }
    }
    #[doc = "Bits 8:12 - TICK_SRC"]
    #[inline(always)]
    pub fn tick_src(&mut self) -> TICK_SRC_W {
        TICK_SRC_W { w: self }
    }
    #[doc = "Bits 4:7 - PRE"]
    #[inline(always)]
    pub fn pre(&mut self) -> PRE_W {
        PRE_W { w: self }
    }
    #[doc = "Bit 1 - MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 0 - RELOAD"]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
}

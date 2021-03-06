#[doc = "Reader of register ADCCTL"]
pub type R = crate::R<u32, super::ADCCTL>;
#[doc = "Writer for register ADCCTL"]
pub type W = crate::W<u32, super::ADCCTL>;
#[doc = "Register ADCCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `START_POL`"]
pub type START_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START_POL`"]
pub struct START_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> START_POL_W<'a> {
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
#[doc = "Reader of field `START_SRC`"]
pub type START_SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `START_SRC`"]
pub struct START_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> START_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 13 - START_POL"]
    #[inline(always)]
    pub fn start_pol(&self) -> START_POL_R {
        START_POL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - START_SRC"]
    #[inline(always)]
    pub fn start_src(&self) -> START_SRC_R {
        START_SRC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:1 - CMD"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 13 - START_POL"]
    #[inline(always)]
    pub fn start_pol(&mut self) -> START_POL_W {
        START_POL_W { w: self }
    }
    #[doc = "Bits 8:12 - START_SRC"]
    #[inline(always)]
    pub fn start_src(&mut self) -> START_SRC_W {
        START_SRC_W { w: self }
    }
    #[doc = "Bits 0:1 - CMD"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
}

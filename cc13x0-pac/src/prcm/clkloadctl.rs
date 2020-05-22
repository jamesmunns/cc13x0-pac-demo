#[doc = "Reader of register CLKLOADCTL"]
pub type R = crate::R<u32, super::CLKLOADCTL>;
#[doc = "Writer for register CLKLOADCTL"]
pub type W = crate::W<u32, super::CLKLOADCTL>;
#[doc = "Register CLKLOADCTL `reset()`'s with value 0x02"]
impl crate::ResetValue for super::CLKLOADCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `LOAD_DONE`"]
pub type LOAD_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOAD_DONE`"]
pub struct LOAD_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOAD_DONE_W<'a> {
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
#[doc = "Reader of field `LOAD`"]
pub type LOAD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOAD`"]
pub struct LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LOAD_W<'a> {
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
    #[doc = "Bit 1 - LOAD_DONE"]
    #[inline(always)]
    pub fn load_done(&self) -> LOAD_DONE_R {
        LOAD_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LOAD"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - LOAD_DONE"]
    #[inline(always)]
    pub fn load_done(&mut self) -> LOAD_DONE_W {
        LOAD_DONE_W { w: self }
    }
    #[doc = "Bit 0 - LOAD"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W {
        LOAD_W { w: self }
    }
}

#[doc = "Reader of register COMP"]
pub type R = crate::R<u8, super::COMP>;
#[doc = "Writer for register COMP"]
pub type W = crate::W<u8, super::COMP>;
#[doc = "Register COMP `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMPA_REF_RES_EN`"]
pub type COMPA_REF_RES_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPA_REF_RES_EN`"]
pub struct COMPA_REF_RES_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPA_REF_RES_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `COMPA_REF_CURR_EN`"]
pub type COMPA_REF_CURR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPA_REF_CURR_EN`"]
pub struct COMPA_REF_CURR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPA_REF_CURR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `COMPB_TRIM`"]
pub type COMPB_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPB_TRIM`"]
pub struct COMPB_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPB_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u8) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `COMPB_EN`"]
pub type COMPB_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPB_EN`"]
pub struct COMPB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPB_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `COMPA_EN`"]
pub type COMPA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPA_EN`"]
pub struct COMPA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPA_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - COMPA_REF_RES_EN"]
    #[inline(always)]
    pub fn compa_ref_res_en(&self) -> COMPA_REF_RES_EN_R {
        COMPA_REF_RES_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - COMPA_REF_CURR_EN"]
    #[inline(always)]
    pub fn compa_ref_curr_en(&self) -> COMPA_REF_CURR_EN_R {
        COMPA_REF_CURR_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - COMPB_TRIM"]
    #[inline(always)]
    pub fn compb_trim(&self) -> COMPB_TRIM_R {
        COMPB_TRIM_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - COMPB_EN"]
    #[inline(always)]
    pub fn compb_en(&self) -> COMPB_EN_R {
        COMPB_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - COMPA_EN"]
    #[inline(always)]
    pub fn compa_en(&self) -> COMPA_EN_R {
        COMPA_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - COMPA_REF_RES_EN"]
    #[inline(always)]
    pub fn compa_ref_res_en(&mut self) -> COMPA_REF_RES_EN_W {
        COMPA_REF_RES_EN_W { w: self }
    }
    #[doc = "Bit 6 - COMPA_REF_CURR_EN"]
    #[inline(always)]
    pub fn compa_ref_curr_en(&mut self) -> COMPA_REF_CURR_EN_W {
        COMPA_REF_CURR_EN_W { w: self }
    }
    #[doc = "Bits 3:5 - COMPB_TRIM"]
    #[inline(always)]
    pub fn compb_trim(&mut self) -> COMPB_TRIM_W {
        COMPB_TRIM_W { w: self }
    }
    #[doc = "Bit 2 - COMPB_EN"]
    #[inline(always)]
    pub fn compb_en(&mut self) -> COMPB_EN_W {
        COMPB_EN_W { w: self }
    }
    #[doc = "Bit 0 - COMPA_EN"]
    #[inline(always)]
    pub fn compa_en(&mut self) -> COMPA_EN_W {
        COMPA_EN_W { w: self }
    }
}

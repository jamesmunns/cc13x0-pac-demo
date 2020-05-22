#[doc = "Reader of register TBMR"]
pub type R = crate::R<u32, super::TBMR>;
#[doc = "Writer for register TBMR"]
pub type W = crate::W<u32, super::TBMR>;
#[doc = "Register TBMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TBMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TCACT`"]
pub type TCACT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCACT`"]
pub struct TCACT_W<'a> {
    w: &'a mut W,
}
impl<'a> TCACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `TBCINTD`"]
pub type TBCINTD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBCINTD`"]
pub struct TBCINTD_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCINTD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TBPLO`"]
pub type TBPLO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBPLO`"]
pub struct TBPLO_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPLO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `TBMRSU`"]
pub type TBMRSU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMRSU`"]
pub struct TBMRSU_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMRSU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TBPWMIE`"]
pub type TBPWMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBPWMIE`"]
pub struct TBPWMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPWMIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TBILD`"]
pub type TBILD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBILD`"]
pub struct TBILD_W<'a> {
    w: &'a mut W,
}
impl<'a> TBILD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TBSNAPS`"]
pub type TBSNAPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBSNAPS`"]
pub struct TBSNAPS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBSNAPS_W<'a> {
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
#[doc = "Reader of field `TBWOT`"]
pub type TBWOT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBWOT`"]
pub struct TBWOT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBWOT_W<'a> {
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
#[doc = "Reader of field `TBMIE`"]
pub type TBMIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMIE`"]
pub struct TBMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMIE_W<'a> {
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
#[doc = "Reader of field `TBCDIR`"]
pub type TBCDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBCDIR`"]
pub struct TBCDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCDIR_W<'a> {
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
#[doc = "Reader of field `TBAMS`"]
pub type TBAMS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBAMS`"]
pub struct TBAMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBAMS_W<'a> {
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
#[doc = "Reader of field `TBCM`"]
pub type TBCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBCM`"]
pub struct TBCM_W<'a> {
    w: &'a mut W,
}
impl<'a> TBCM_W<'a> {
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
#[doc = "Reader of field `TBMR`"]
pub type TBMR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TBMR`"]
pub struct TBMR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 13:15 - TCACT"]
    #[inline(always)]
    pub fn tcact(&self) -> TCACT_R {
        TCACT_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 12 - TBCINTD"]
    #[inline(always)]
    pub fn tbcintd(&self) -> TBCINTD_R {
        TBCINTD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TBPLO"]
    #[inline(always)]
    pub fn tbplo(&self) -> TBPLO_R {
        TBPLO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TBMRSU"]
    #[inline(always)]
    pub fn tbmrsu(&self) -> TBMRSU_R {
        TBMRSU_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TBPWMIE"]
    #[inline(always)]
    pub fn tbpwmie(&self) -> TBPWMIE_R {
        TBPWMIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TBILD"]
    #[inline(always)]
    pub fn tbild(&self) -> TBILD_R {
        TBILD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TBSNAPS"]
    #[inline(always)]
    pub fn tbsnaps(&self) -> TBSNAPS_R {
        TBSNAPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TBWOT"]
    #[inline(always)]
    pub fn tbwot(&self) -> TBWOT_R {
        TBWOT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TBMIE"]
    #[inline(always)]
    pub fn tbmie(&self) -> TBMIE_R {
        TBMIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TBCDIR"]
    #[inline(always)]
    pub fn tbcdir(&self) -> TBCDIR_R {
        TBCDIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TBAMS"]
    #[inline(always)]
    pub fn tbams(&self) -> TBAMS_R {
        TBAMS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TBCM"]
    #[inline(always)]
    pub fn tbcm(&self) -> TBCM_R {
        TBCM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - TBMR"]
    #[inline(always)]
    pub fn tbmr(&self) -> TBMR_R {
        TBMR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 13:15 - TCACT"]
    #[inline(always)]
    pub fn tcact(&mut self) -> TCACT_W {
        TCACT_W { w: self }
    }
    #[doc = "Bit 12 - TBCINTD"]
    #[inline(always)]
    pub fn tbcintd(&mut self) -> TBCINTD_W {
        TBCINTD_W { w: self }
    }
    #[doc = "Bit 11 - TBPLO"]
    #[inline(always)]
    pub fn tbplo(&mut self) -> TBPLO_W {
        TBPLO_W { w: self }
    }
    #[doc = "Bit 10 - TBMRSU"]
    #[inline(always)]
    pub fn tbmrsu(&mut self) -> TBMRSU_W {
        TBMRSU_W { w: self }
    }
    #[doc = "Bit 9 - TBPWMIE"]
    #[inline(always)]
    pub fn tbpwmie(&mut self) -> TBPWMIE_W {
        TBPWMIE_W { w: self }
    }
    #[doc = "Bit 8 - TBILD"]
    #[inline(always)]
    pub fn tbild(&mut self) -> TBILD_W {
        TBILD_W { w: self }
    }
    #[doc = "Bit 7 - TBSNAPS"]
    #[inline(always)]
    pub fn tbsnaps(&mut self) -> TBSNAPS_W {
        TBSNAPS_W { w: self }
    }
    #[doc = "Bit 6 - TBWOT"]
    #[inline(always)]
    pub fn tbwot(&mut self) -> TBWOT_W {
        TBWOT_W { w: self }
    }
    #[doc = "Bit 5 - TBMIE"]
    #[inline(always)]
    pub fn tbmie(&mut self) -> TBMIE_W {
        TBMIE_W { w: self }
    }
    #[doc = "Bit 4 - TBCDIR"]
    #[inline(always)]
    pub fn tbcdir(&mut self) -> TBCDIR_W {
        TBCDIR_W { w: self }
    }
    #[doc = "Bit 3 - TBAMS"]
    #[inline(always)]
    pub fn tbams(&mut self) -> TBAMS_W {
        TBAMS_W { w: self }
    }
    #[doc = "Bit 2 - TBCM"]
    #[inline(always)]
    pub fn tbcm(&mut self) -> TBCM_W {
        TBCM_W { w: self }
    }
    #[doc = "Bits 0:1 - TBMR"]
    #[inline(always)]
    pub fn tbmr(&mut self) -> TBMR_W {
        TBMR_W { w: self }
    }
}

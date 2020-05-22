#[doc = "Reader of register RFHWIEN"]
pub type R = crate::R<u32, super::RFHWIEN>;
#[doc = "Writer for register RFHWIEN"]
pub type W = crate::W<u32, super::RFHWIEN>;
#[doc = "Register RFHWIEN `reset()`'s with value 0"]
impl crate::ResetValue for super::RFHWIEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RATCH7`"]
pub type RATCH7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RATCH7`"]
pub struct RATCH7_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `RATCH6`"]
pub type RATCH6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RATCH6`"]
pub struct RATCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `RATCH5`"]
pub type RATCH5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RATCH5`"]
pub struct RATCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RATCH4`"]
pub type RATCH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RATCH4`"]
pub struct RATCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `RATCH3`"]
pub type RATCH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RATCH3`"]
pub struct RATCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `RATCH2`"]
pub type RATCH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RATCH2`"]
pub struct RATCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH2_W<'a> {
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
#[doc = "Reader of field `RATCH1`"]
pub type RATCH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RATCH1`"]
pub struct RATCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH1_W<'a> {
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
#[doc = "Reader of field `RATCH0`"]
pub type RATCH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RATCH0`"]
pub struct RATCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RATCH0_W<'a> {
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
#[doc = "Reader of field `RFESOFT2`"]
pub type RFESOFT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFESOFT2`"]
pub struct RFESOFT2_W<'a> {
    w: &'a mut W,
}
impl<'a> RFESOFT2_W<'a> {
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
#[doc = "Reader of field `RFESOFT1`"]
pub type RFESOFT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFESOFT1`"]
pub struct RFESOFT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RFESOFT1_W<'a> {
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
#[doc = "Reader of field `RFESOFT0`"]
pub type RFESOFT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFESOFT0`"]
pub struct RFESOFT0_W<'a> {
    w: &'a mut W,
}
impl<'a> RFESOFT0_W<'a> {
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
#[doc = "Reader of field `RFEDONE`"]
pub type RFEDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFEDONE`"]
pub struct RFEDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFEDONE_W<'a> {
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
#[doc = "Reader of field `TRCTK`"]
pub type TRCTK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRCTK`"]
pub struct TRCTK_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCTK_W<'a> {
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
#[doc = "Reader of field `MDMSOFT`"]
pub type MDMSOFT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDMSOFT`"]
pub struct MDMSOFT_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMSOFT_W<'a> {
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
#[doc = "Reader of field `MDMOUT`"]
pub type MDMOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDMOUT`"]
pub struct MDMOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMOUT_W<'a> {
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
#[doc = "Reader of field `MDMIN`"]
pub type MDMIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDMIN`"]
pub struct MDMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMIN_W<'a> {
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
#[doc = "Reader of field `MDMDONE`"]
pub type MDMDONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDMDONE`"]
pub struct MDMDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMDONE_W<'a> {
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
#[doc = "Reader of field `FSCA`"]
pub type FSCA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSCA`"]
pub struct FSCA_W<'a> {
    w: &'a mut W,
}
impl<'a> FSCA_W<'a> {
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
impl R {
    #[doc = "Bit 19 - RATCH7"]
    #[inline(always)]
    pub fn ratch7(&self) -> RATCH7_R {
        RATCH7_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RATCH6"]
    #[inline(always)]
    pub fn ratch6(&self) -> RATCH6_R {
        RATCH6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RATCH5"]
    #[inline(always)]
    pub fn ratch5(&self) -> RATCH5_R {
        RATCH5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RATCH4"]
    #[inline(always)]
    pub fn ratch4(&self) -> RATCH4_R {
        RATCH4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RATCH3"]
    #[inline(always)]
    pub fn ratch3(&self) -> RATCH3_R {
        RATCH3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RATCH2"]
    #[inline(always)]
    pub fn ratch2(&self) -> RATCH2_R {
        RATCH2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RATCH1"]
    #[inline(always)]
    pub fn ratch1(&self) -> RATCH1_R {
        RATCH1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RATCH0"]
    #[inline(always)]
    pub fn ratch0(&self) -> RATCH0_R {
        RATCH0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RFESOFT2"]
    #[inline(always)]
    pub fn rfesoft2(&self) -> RFESOFT2_R {
        RFESOFT2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RFESOFT1"]
    #[inline(always)]
    pub fn rfesoft1(&self) -> RFESOFT1_R {
        RFESOFT1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RFESOFT0"]
    #[inline(always)]
    pub fn rfesoft0(&self) -> RFESOFT0_R {
        RFESOFT0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RFEDONE"]
    #[inline(always)]
    pub fn rfedone(&self) -> RFEDONE_R {
        RFEDONE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TRCTK"]
    #[inline(always)]
    pub fn trctk(&self) -> TRCTK_R {
        TRCTK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MDMSOFT"]
    #[inline(always)]
    pub fn mdmsoft(&self) -> MDMSOFT_R {
        MDMSOFT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MDMOUT"]
    #[inline(always)]
    pub fn mdmout(&self) -> MDMOUT_R {
        MDMOUT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MDMIN"]
    #[inline(always)]
    pub fn mdmin(&self) -> MDMIN_R {
        MDMIN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MDMDONE"]
    #[inline(always)]
    pub fn mdmdone(&self) -> MDMDONE_R {
        MDMDONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FSCA"]
    #[inline(always)]
    pub fn fsca(&self) -> FSCA_R {
        FSCA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 19 - RATCH7"]
    #[inline(always)]
    pub fn ratch7(&mut self) -> RATCH7_W {
        RATCH7_W { w: self }
    }
    #[doc = "Bit 18 - RATCH6"]
    #[inline(always)]
    pub fn ratch6(&mut self) -> RATCH6_W {
        RATCH6_W { w: self }
    }
    #[doc = "Bit 17 - RATCH5"]
    #[inline(always)]
    pub fn ratch5(&mut self) -> RATCH5_W {
        RATCH5_W { w: self }
    }
    #[doc = "Bit 16 - RATCH4"]
    #[inline(always)]
    pub fn ratch4(&mut self) -> RATCH4_W {
        RATCH4_W { w: self }
    }
    #[doc = "Bit 15 - RATCH3"]
    #[inline(always)]
    pub fn ratch3(&mut self) -> RATCH3_W {
        RATCH3_W { w: self }
    }
    #[doc = "Bit 14 - RATCH2"]
    #[inline(always)]
    pub fn ratch2(&mut self) -> RATCH2_W {
        RATCH2_W { w: self }
    }
    #[doc = "Bit 13 - RATCH1"]
    #[inline(always)]
    pub fn ratch1(&mut self) -> RATCH1_W {
        RATCH1_W { w: self }
    }
    #[doc = "Bit 12 - RATCH0"]
    #[inline(always)]
    pub fn ratch0(&mut self) -> RATCH0_W {
        RATCH0_W { w: self }
    }
    #[doc = "Bit 11 - RFESOFT2"]
    #[inline(always)]
    pub fn rfesoft2(&mut self) -> RFESOFT2_W {
        RFESOFT2_W { w: self }
    }
    #[doc = "Bit 10 - RFESOFT1"]
    #[inline(always)]
    pub fn rfesoft1(&mut self) -> RFESOFT1_W {
        RFESOFT1_W { w: self }
    }
    #[doc = "Bit 9 - RFESOFT0"]
    #[inline(always)]
    pub fn rfesoft0(&mut self) -> RFESOFT0_W {
        RFESOFT0_W { w: self }
    }
    #[doc = "Bit 8 - RFEDONE"]
    #[inline(always)]
    pub fn rfedone(&mut self) -> RFEDONE_W {
        RFEDONE_W { w: self }
    }
    #[doc = "Bit 6 - TRCTK"]
    #[inline(always)]
    pub fn trctk(&mut self) -> TRCTK_W {
        TRCTK_W { w: self }
    }
    #[doc = "Bit 5 - MDMSOFT"]
    #[inline(always)]
    pub fn mdmsoft(&mut self) -> MDMSOFT_W {
        MDMSOFT_W { w: self }
    }
    #[doc = "Bit 4 - MDMOUT"]
    #[inline(always)]
    pub fn mdmout(&mut self) -> MDMOUT_W {
        MDMOUT_W { w: self }
    }
    #[doc = "Bit 3 - MDMIN"]
    #[inline(always)]
    pub fn mdmin(&mut self) -> MDMIN_W {
        MDMIN_W { w: self }
    }
    #[doc = "Bit 2 - MDMDONE"]
    #[inline(always)]
    pub fn mdmdone(&mut self) -> MDMDONE_W {
        MDMDONE_W { w: self }
    }
    #[doc = "Bit 1 - FSCA"]
    #[inline(always)]
    pub fn fsca(&mut self) -> FSCA_W {
        FSCA_W { w: self }
    }
}

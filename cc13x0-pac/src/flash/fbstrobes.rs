#[doc = "Reader of register FBSTROBES"]
pub type R = crate::R<u32, super::FBSTROBES>;
#[doc = "Writer for register FBSTROBES"]
pub type W = crate::W<u32, super::FBSTROBES>;
#[doc = "Register FBSTROBES `reset()`'s with value 0x0104"]
impl crate::ResetValue for super::FBSTROBES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0104
    }
}
#[doc = "Reader of field `ECBIT`"]
pub type ECBIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECBIT`"]
pub struct ECBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ECBIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RWAIT2_FLCLK`"]
pub type RWAIT2_FLCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWAIT2_FLCLK`"]
pub struct RWAIT2_FLCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> RWAIT2_FLCLK_W<'a> {
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
#[doc = "Reader of field `RWAIT_FLCLK`"]
pub type RWAIT_FLCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWAIT_FLCLK`"]
pub struct RWAIT_FLCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> RWAIT_FLCLK_W<'a> {
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
#[doc = "Reader of field `FLCLKEN`"]
pub type FLCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLCLKEN`"]
pub struct FLCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLCLKEN_W<'a> {
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
#[doc = "Reader of field `CTRLENZ`"]
pub type CTRLENZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTRLENZ`"]
pub struct CTRLENZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLENZ_W<'a> {
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
#[doc = "Reader of field `NOCOLRED`"]
pub type NOCOLRED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOCOLRED`"]
pub struct NOCOLRED_W<'a> {
    w: &'a mut W,
}
impl<'a> NOCOLRED_W<'a> {
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
#[doc = "Reader of field `PRECOL`"]
pub type PRECOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRECOL`"]
pub struct PRECOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRECOL_W<'a> {
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
#[doc = "Reader of field `TI_OTP`"]
pub type TI_OTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TI_OTP`"]
pub struct TI_OTP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI_OTP_W<'a> {
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
#[doc = "Reader of field `OTP`"]
pub type OTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTP`"]
pub struct OTP_W<'a> {
    w: &'a mut W,
}
impl<'a> OTP_W<'a> {
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
#[doc = "Reader of field `TEZ`"]
pub type TEZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEZ`"]
pub struct TEZ_W<'a> {
    w: &'a mut W,
}
impl<'a> TEZ_W<'a> {
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
impl R {
    #[doc = "Bit 24 - ECBIT"]
    #[inline(always)]
    pub fn ecbit(&self) -> ECBIT_R {
        ECBIT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RWAIT2_FLCLK"]
    #[inline(always)]
    pub fn rwait2_flclk(&self) -> RWAIT2_FLCLK_R {
        RWAIT2_FLCLK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RWAIT_FLCLK"]
    #[inline(always)]
    pub fn rwait_flclk(&self) -> RWAIT_FLCLK_R {
        RWAIT_FLCLK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FLCLKEN"]
    #[inline(always)]
    pub fn flclken(&self) -> FLCLKEN_R {
        FLCLKEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CTRLENZ"]
    #[inline(always)]
    pub fn ctrlenz(&self) -> CTRLENZ_R {
        CTRLENZ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - NOCOLRED"]
    #[inline(always)]
    pub fn nocolred(&self) -> NOCOLRED_R {
        NOCOLRED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PRECOL"]
    #[inline(always)]
    pub fn precol(&self) -> PRECOL_R {
        PRECOL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TI_OTP"]
    #[inline(always)]
    pub fn ti_otp(&self) -> TI_OTP_R {
        TI_OTP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OTP"]
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TEZ"]
    #[inline(always)]
    pub fn tez(&self) -> TEZ_R {
        TEZ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - ECBIT"]
    #[inline(always)]
    pub fn ecbit(&mut self) -> ECBIT_W {
        ECBIT_W { w: self }
    }
    #[doc = "Bit 18 - RWAIT2_FLCLK"]
    #[inline(always)]
    pub fn rwait2_flclk(&mut self) -> RWAIT2_FLCLK_W {
        RWAIT2_FLCLK_W { w: self }
    }
    #[doc = "Bit 17 - RWAIT_FLCLK"]
    #[inline(always)]
    pub fn rwait_flclk(&mut self) -> RWAIT_FLCLK_W {
        RWAIT_FLCLK_W { w: self }
    }
    #[doc = "Bit 16 - FLCLKEN"]
    #[inline(always)]
    pub fn flclken(&mut self) -> FLCLKEN_W {
        FLCLKEN_W { w: self }
    }
    #[doc = "Bit 8 - CTRLENZ"]
    #[inline(always)]
    pub fn ctrlenz(&mut self) -> CTRLENZ_W {
        CTRLENZ_W { w: self }
    }
    #[doc = "Bit 6 - NOCOLRED"]
    #[inline(always)]
    pub fn nocolred(&mut self) -> NOCOLRED_W {
        NOCOLRED_W { w: self }
    }
    #[doc = "Bit 5 - PRECOL"]
    #[inline(always)]
    pub fn precol(&mut self) -> PRECOL_W {
        PRECOL_W { w: self }
    }
    #[doc = "Bit 4 - TI_OTP"]
    #[inline(always)]
    pub fn ti_otp(&mut self) -> TI_OTP_W {
        TI_OTP_W { w: self }
    }
    #[doc = "Bit 3 - OTP"]
    #[inline(always)]
    pub fn otp(&mut self) -> OTP_W {
        OTP_W { w: self }
    }
    #[doc = "Bit 2 - TEZ"]
    #[inline(always)]
    pub fn tez(&mut self) -> TEZ_W {
        TEZ_W { w: self }
    }
}

#[doc = "Reader of register DOE31_0"]
pub type R = crate::R<u32, super::DOE31_0>;
#[doc = "Writer for register DOE31_0"]
pub type W = crate::W<u32, super::DOE31_0>;
#[doc = "Register DOE31_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOE31_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIO31`"]
pub type DIO31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO31`"]
pub struct DIO31_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `DIO30`"]
pub type DIO30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO30`"]
pub struct DIO30_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DIO29`"]
pub type DIO29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO29`"]
pub struct DIO29_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `DIO28`"]
pub type DIO28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO28`"]
pub struct DIO28_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO28_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DIO27`"]
pub type DIO27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO27`"]
pub struct DIO27_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `DIO26`"]
pub type DIO26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO26`"]
pub struct DIO26_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `DIO25`"]
pub type DIO25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO25`"]
pub struct DIO25_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DIO24`"]
pub type DIO24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO24`"]
pub struct DIO24_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO24_W<'a> {
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
#[doc = "Reader of field `DIO23`"]
pub type DIO23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO23`"]
pub struct DIO23_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `DIO22`"]
pub type DIO22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO22`"]
pub struct DIO22_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `DIO21`"]
pub type DIO21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO21`"]
pub struct DIO21_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `DIO20`"]
pub type DIO20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO20`"]
pub struct DIO20_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `DIO19`"]
pub type DIO19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO19`"]
pub struct DIO19_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO19_W<'a> {
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
#[doc = "Reader of field `DIO18`"]
pub type DIO18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO18`"]
pub struct DIO18_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO18_W<'a> {
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
#[doc = "Reader of field `DIO17`"]
pub type DIO17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO17`"]
pub struct DIO17_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO17_W<'a> {
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
#[doc = "Reader of field `DIO16`"]
pub type DIO16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO16`"]
pub struct DIO16_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO16_W<'a> {
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
#[doc = "Reader of field `DIO15`"]
pub type DIO15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO15`"]
pub struct DIO15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO15_W<'a> {
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
#[doc = "Reader of field `DIO14`"]
pub type DIO14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO14`"]
pub struct DIO14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO14_W<'a> {
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
#[doc = "Reader of field `DIO13`"]
pub type DIO13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO13`"]
pub struct DIO13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO13_W<'a> {
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
#[doc = "Reader of field `DIO12`"]
pub type DIO12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO12`"]
pub struct DIO12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO12_W<'a> {
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
#[doc = "Reader of field `DIO11`"]
pub type DIO11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO11`"]
pub struct DIO11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO11_W<'a> {
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
#[doc = "Reader of field `DIO10`"]
pub type DIO10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO10`"]
pub struct DIO10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO10_W<'a> {
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
#[doc = "Reader of field `DIO9`"]
pub type DIO9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO9`"]
pub struct DIO9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO9_W<'a> {
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
#[doc = "Reader of field `DIO8`"]
pub type DIO8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO8`"]
pub struct DIO8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO8_W<'a> {
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
#[doc = "Reader of field `DIO7`"]
pub type DIO7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO7`"]
pub struct DIO7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO7_W<'a> {
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
#[doc = "Reader of field `DIO6`"]
pub type DIO6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO6`"]
pub struct DIO6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO6_W<'a> {
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
#[doc = "Reader of field `DIO5`"]
pub type DIO5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO5`"]
pub struct DIO5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO5_W<'a> {
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
#[doc = "Reader of field `DIO4`"]
pub type DIO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO4`"]
pub struct DIO4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO4_W<'a> {
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
#[doc = "Reader of field `DIO3`"]
pub type DIO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO3`"]
pub struct DIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO3_W<'a> {
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
#[doc = "Reader of field `DIO2`"]
pub type DIO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO2`"]
pub struct DIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO2_W<'a> {
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
#[doc = "Reader of field `DIO1`"]
pub type DIO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO1`"]
pub struct DIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO1_W<'a> {
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
#[doc = "Reader of field `DIO0`"]
pub type DIO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO0`"]
pub struct DIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO0_W<'a> {
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
    #[doc = "Bit 31 - DIO31"]
    #[inline(always)]
    pub fn dio31(&self) -> DIO31_R {
        DIO31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DIO30"]
    #[inline(always)]
    pub fn dio30(&self) -> DIO30_R {
        DIO30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DIO29"]
    #[inline(always)]
    pub fn dio29(&self) -> DIO29_R {
        DIO29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DIO28"]
    #[inline(always)]
    pub fn dio28(&self) -> DIO28_R {
        DIO28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DIO27"]
    #[inline(always)]
    pub fn dio27(&self) -> DIO27_R {
        DIO27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DIO26"]
    #[inline(always)]
    pub fn dio26(&self) -> DIO26_R {
        DIO26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DIO25"]
    #[inline(always)]
    pub fn dio25(&self) -> DIO25_R {
        DIO25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DIO24"]
    #[inline(always)]
    pub fn dio24(&self) -> DIO24_R {
        DIO24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DIO23"]
    #[inline(always)]
    pub fn dio23(&self) -> DIO23_R {
        DIO23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DIO22"]
    #[inline(always)]
    pub fn dio22(&self) -> DIO22_R {
        DIO22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - DIO21"]
    #[inline(always)]
    pub fn dio21(&self) -> DIO21_R {
        DIO21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DIO20"]
    #[inline(always)]
    pub fn dio20(&self) -> DIO20_R {
        DIO20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DIO19"]
    #[inline(always)]
    pub fn dio19(&self) -> DIO19_R {
        DIO19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DIO18"]
    #[inline(always)]
    pub fn dio18(&self) -> DIO18_R {
        DIO18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DIO17"]
    #[inline(always)]
    pub fn dio17(&self) -> DIO17_R {
        DIO17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DIO16"]
    #[inline(always)]
    pub fn dio16(&self) -> DIO16_R {
        DIO16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DIO15"]
    #[inline(always)]
    pub fn dio15(&self) -> DIO15_R {
        DIO15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DIO14"]
    #[inline(always)]
    pub fn dio14(&self) -> DIO14_R {
        DIO14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DIO13"]
    #[inline(always)]
    pub fn dio13(&self) -> DIO13_R {
        DIO13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DIO12"]
    #[inline(always)]
    pub fn dio12(&self) -> DIO12_R {
        DIO12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DIO11"]
    #[inline(always)]
    pub fn dio11(&self) -> DIO11_R {
        DIO11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DIO10"]
    #[inline(always)]
    pub fn dio10(&self) -> DIO10_R {
        DIO10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DIO9"]
    #[inline(always)]
    pub fn dio9(&self) -> DIO9_R {
        DIO9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIO8"]
    #[inline(always)]
    pub fn dio8(&self) -> DIO8_R {
        DIO8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DIO7"]
    #[inline(always)]
    pub fn dio7(&self) -> DIO7_R {
        DIO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DIO6"]
    #[inline(always)]
    pub fn dio6(&self) -> DIO6_R {
        DIO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DIO5"]
    #[inline(always)]
    pub fn dio5(&self) -> DIO5_R {
        DIO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DIO4"]
    #[inline(always)]
    pub fn dio4(&self) -> DIO4_R {
        DIO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DIO3"]
    #[inline(always)]
    pub fn dio3(&self) -> DIO3_R {
        DIO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DIO2"]
    #[inline(always)]
    pub fn dio2(&self) -> DIO2_R {
        DIO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DIO1"]
    #[inline(always)]
    pub fn dio1(&self) -> DIO1_R {
        DIO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DIO0"]
    #[inline(always)]
    pub fn dio0(&self) -> DIO0_R {
        DIO0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - DIO31"]
    #[inline(always)]
    pub fn dio31(&mut self) -> DIO31_W {
        DIO31_W { w: self }
    }
    #[doc = "Bit 30 - DIO30"]
    #[inline(always)]
    pub fn dio30(&mut self) -> DIO30_W {
        DIO30_W { w: self }
    }
    #[doc = "Bit 29 - DIO29"]
    #[inline(always)]
    pub fn dio29(&mut self) -> DIO29_W {
        DIO29_W { w: self }
    }
    #[doc = "Bit 28 - DIO28"]
    #[inline(always)]
    pub fn dio28(&mut self) -> DIO28_W {
        DIO28_W { w: self }
    }
    #[doc = "Bit 27 - DIO27"]
    #[inline(always)]
    pub fn dio27(&mut self) -> DIO27_W {
        DIO27_W { w: self }
    }
    #[doc = "Bit 26 - DIO26"]
    #[inline(always)]
    pub fn dio26(&mut self) -> DIO26_W {
        DIO26_W { w: self }
    }
    #[doc = "Bit 25 - DIO25"]
    #[inline(always)]
    pub fn dio25(&mut self) -> DIO25_W {
        DIO25_W { w: self }
    }
    #[doc = "Bit 24 - DIO24"]
    #[inline(always)]
    pub fn dio24(&mut self) -> DIO24_W {
        DIO24_W { w: self }
    }
    #[doc = "Bit 23 - DIO23"]
    #[inline(always)]
    pub fn dio23(&mut self) -> DIO23_W {
        DIO23_W { w: self }
    }
    #[doc = "Bit 22 - DIO22"]
    #[inline(always)]
    pub fn dio22(&mut self) -> DIO22_W {
        DIO22_W { w: self }
    }
    #[doc = "Bit 21 - DIO21"]
    #[inline(always)]
    pub fn dio21(&mut self) -> DIO21_W {
        DIO21_W { w: self }
    }
    #[doc = "Bit 20 - DIO20"]
    #[inline(always)]
    pub fn dio20(&mut self) -> DIO20_W {
        DIO20_W { w: self }
    }
    #[doc = "Bit 19 - DIO19"]
    #[inline(always)]
    pub fn dio19(&mut self) -> DIO19_W {
        DIO19_W { w: self }
    }
    #[doc = "Bit 18 - DIO18"]
    #[inline(always)]
    pub fn dio18(&mut self) -> DIO18_W {
        DIO18_W { w: self }
    }
    #[doc = "Bit 17 - DIO17"]
    #[inline(always)]
    pub fn dio17(&mut self) -> DIO17_W {
        DIO17_W { w: self }
    }
    #[doc = "Bit 16 - DIO16"]
    #[inline(always)]
    pub fn dio16(&mut self) -> DIO16_W {
        DIO16_W { w: self }
    }
    #[doc = "Bit 15 - DIO15"]
    #[inline(always)]
    pub fn dio15(&mut self) -> DIO15_W {
        DIO15_W { w: self }
    }
    #[doc = "Bit 14 - DIO14"]
    #[inline(always)]
    pub fn dio14(&mut self) -> DIO14_W {
        DIO14_W { w: self }
    }
    #[doc = "Bit 13 - DIO13"]
    #[inline(always)]
    pub fn dio13(&mut self) -> DIO13_W {
        DIO13_W { w: self }
    }
    #[doc = "Bit 12 - DIO12"]
    #[inline(always)]
    pub fn dio12(&mut self) -> DIO12_W {
        DIO12_W { w: self }
    }
    #[doc = "Bit 11 - DIO11"]
    #[inline(always)]
    pub fn dio11(&mut self) -> DIO11_W {
        DIO11_W { w: self }
    }
    #[doc = "Bit 10 - DIO10"]
    #[inline(always)]
    pub fn dio10(&mut self) -> DIO10_W {
        DIO10_W { w: self }
    }
    #[doc = "Bit 9 - DIO9"]
    #[inline(always)]
    pub fn dio9(&mut self) -> DIO9_W {
        DIO9_W { w: self }
    }
    #[doc = "Bit 8 - DIO8"]
    #[inline(always)]
    pub fn dio8(&mut self) -> DIO8_W {
        DIO8_W { w: self }
    }
    #[doc = "Bit 7 - DIO7"]
    #[inline(always)]
    pub fn dio7(&mut self) -> DIO7_W {
        DIO7_W { w: self }
    }
    #[doc = "Bit 6 - DIO6"]
    #[inline(always)]
    pub fn dio6(&mut self) -> DIO6_W {
        DIO6_W { w: self }
    }
    #[doc = "Bit 5 - DIO5"]
    #[inline(always)]
    pub fn dio5(&mut self) -> DIO5_W {
        DIO5_W { w: self }
    }
    #[doc = "Bit 4 - DIO4"]
    #[inline(always)]
    pub fn dio4(&mut self) -> DIO4_W {
        DIO4_W { w: self }
    }
    #[doc = "Bit 3 - DIO3"]
    #[inline(always)]
    pub fn dio3(&mut self) -> DIO3_W {
        DIO3_W { w: self }
    }
    #[doc = "Bit 2 - DIO2"]
    #[inline(always)]
    pub fn dio2(&mut self) -> DIO2_W {
        DIO2_W { w: self }
    }
    #[doc = "Bit 1 - DIO1"]
    #[inline(always)]
    pub fn dio1(&mut self) -> DIO1_W {
        DIO1_W { w: self }
    }
    #[doc = "Bit 0 - DIO0"]
    #[inline(always)]
    pub fn dio0(&mut self) -> DIO0_W {
        DIO0_W { w: self }
    }
}

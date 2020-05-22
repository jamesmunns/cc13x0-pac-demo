#[doc = "Reader of register ALGSEL"]
pub type R = crate::R<u32, super::ALGSEL>;
#[doc = "Writer for register ALGSEL"]
pub type W = crate::W<u32, super::ALGSEL>;
#[doc = "Register ALGSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::ALGSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TAG`"]
pub type TAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAG`"]
pub struct TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_W<'a> {
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
#[doc = "Reader of field `AES`"]
pub type AES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES`"]
pub struct AES_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_W<'a> {
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
#[doc = "Reader of field `KEY_STORE`"]
pub type KEY_STORE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEY_STORE`"]
pub struct KEY_STORE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_STORE_W<'a> {
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
    #[doc = "Bit 31 - TAG"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 1 - AES"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - KEY_STORE"]
    #[inline(always)]
    pub fn key_store(&self) -> KEY_STORE_R {
        KEY_STORE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - TAG"]
    #[inline(always)]
    pub fn tag(&mut self) -> TAG_W {
        TAG_W { w: self }
    }
    #[doc = "Bit 1 - AES"]
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W {
        AES_W { w: self }
    }
    #[doc = "Bit 0 - KEY_STORE"]
    #[inline(always)]
    pub fn key_store(&mut self) -> KEY_STORE_W {
        KEY_STORE_W { w: self }
    }
}

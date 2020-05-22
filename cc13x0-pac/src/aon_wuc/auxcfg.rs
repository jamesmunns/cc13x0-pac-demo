#[doc = "Reader of register AUXCFG"]
pub type R = crate::R<u32, super::AUXCFG>;
#[doc = "Writer for register AUXCFG"]
pub type W = crate::W<u32, super::AUXCFG>;
#[doc = "Register AUXCFG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::AUXCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `RAM_RET_EN`"]
pub type RAM_RET_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM_RET_EN`"]
pub struct RAM_RET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_RET_EN_W<'a> {
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
    #[doc = "Bit 0 - RAM_RET_EN"]
    #[inline(always)]
    pub fn ram_ret_en(&self) -> RAM_RET_EN_R {
        RAM_RET_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM_RET_EN"]
    #[inline(always)]
    pub fn ram_ret_en(&mut self) -> RAM_RET_EN_W {
        RAM_RET_EN_W { w: self }
    }
}

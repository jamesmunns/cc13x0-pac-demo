#[doc = "Reader of register ANDCCP"]
pub type R = crate::R<u32, super::ANDCCP>;
#[doc = "Writer for register ANDCCP"]
pub type W = crate::W<u32, super::ANDCCP>;
#[doc = "Register ANDCCP `reset()`'s with value 0"]
impl crate::ResetValue for super::ANDCCP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCP_AND_EN`"]
pub type CCP_AND_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCP_AND_EN`"]
pub struct CCP_AND_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCP_AND_EN_W<'a> {
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
    #[doc = "Bit 0 - CCP_AND_EN"]
    #[inline(always)]
    pub fn ccp_and_en(&self) -> CCP_AND_EN_R {
        CCP_AND_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCP_AND_EN"]
    #[inline(always)]
    pub fn ccp_and_en(&mut self) -> CCP_AND_EN_W {
        CCP_AND_EN_W { w: self }
    }
}

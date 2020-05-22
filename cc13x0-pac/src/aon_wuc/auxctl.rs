#[doc = "Reader of register AUXCTL"]
pub type R = crate::R<u32, super::AUXCTL>;
#[doc = "Writer for register AUXCTL"]
pub type W = crate::W<u32, super::AUXCTL>;
#[doc = "Register AUXCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::AUXCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESET_REQ`"]
pub type RESET_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_REQ`"]
pub struct RESET_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_REQ_W<'a> {
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
#[doc = "Reader of field `SCE_RUN_EN`"]
pub type SCE_RUN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCE_RUN_EN`"]
pub struct SCE_RUN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCE_RUN_EN_W<'a> {
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
#[doc = "Reader of field `SWEV`"]
pub type SWEV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWEV`"]
pub struct SWEV_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEV_W<'a> {
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
#[doc = "Reader of field `AUX_FORCE_ON`"]
pub type AUX_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_FORCE_ON`"]
pub struct AUX_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_FORCE_ON_W<'a> {
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
    #[doc = "Bit 31 - RESET_REQ"]
    #[inline(always)]
    pub fn reset_req(&self) -> RESET_REQ_R {
        RESET_REQ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SCE_RUN_EN"]
    #[inline(always)]
    pub fn sce_run_en(&self) -> SCE_RUN_EN_R {
        SCE_RUN_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SWEV"]
    #[inline(always)]
    pub fn swev(&self) -> SWEV_R {
        SWEV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - AUX_FORCE_ON"]
    #[inline(always)]
    pub fn aux_force_on(&self) -> AUX_FORCE_ON_R {
        AUX_FORCE_ON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - RESET_REQ"]
    #[inline(always)]
    pub fn reset_req(&mut self) -> RESET_REQ_W {
        RESET_REQ_W { w: self }
    }
    #[doc = "Bit 2 - SCE_RUN_EN"]
    #[inline(always)]
    pub fn sce_run_en(&mut self) -> SCE_RUN_EN_W {
        SCE_RUN_EN_W { w: self }
    }
    #[doc = "Bit 1 - SWEV"]
    #[inline(always)]
    pub fn swev(&mut self) -> SWEV_W {
        SWEV_W { w: self }
    }
    #[doc = "Bit 0 - AUX_FORCE_ON"]
    #[inline(always)]
    pub fn aux_force_on(&mut self) -> AUX_FORCE_ON_W {
        AUX_FORCE_ON_W { w: self }
    }
}

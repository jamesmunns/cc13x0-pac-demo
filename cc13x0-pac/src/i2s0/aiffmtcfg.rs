#[doc = "Reader of register AIFFMTCFG"]
pub type R = crate::R<u32, super::AIFFMTCFG>;
#[doc = "Writer for register AIFFMTCFG"]
pub type W = crate::W<u32, super::AIFFMTCFG>;
#[doc = "Register AIFFMTCFG `reset()`'s with value 0x0170"]
impl crate::ResetValue for super::AIFFMTCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0170
    }
}
#[doc = "Reader of field `DATA_DELAY`"]
pub type DATA_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_DELAY`"]
pub struct DATA_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `MEM_LEN_24`"]
pub type MEM_LEN_24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_LEN_24`"]
pub struct MEM_LEN_24_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_LEN_24_W<'a> {
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
#[doc = "Reader of field `SMPL_EDGE`"]
pub type SMPL_EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMPL_EDGE`"]
pub struct SMPL_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPL_EDGE_W<'a> {
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
#[doc = "Reader of field `DUAL_PHASE`"]
pub type DUAL_PHASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUAL_PHASE`"]
pub struct DUAL_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL_PHASE_W<'a> {
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
#[doc = "Reader of field `WORD_LEN`"]
pub type WORD_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WORD_LEN`"]
pub struct WORD_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WORD_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - DATA_DELAY"]
    #[inline(always)]
    pub fn data_delay(&self) -> DATA_DELAY_R {
        DATA_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7 - MEM_LEN_24"]
    #[inline(always)]
    pub fn mem_len_24(&self) -> MEM_LEN_24_R {
        MEM_LEN_24_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SMPL_EDGE"]
    #[inline(always)]
    pub fn smpl_edge(&self) -> SMPL_EDGE_R {
        SMPL_EDGE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DUAL_PHASE"]
    #[inline(always)]
    pub fn dual_phase(&self) -> DUAL_PHASE_R {
        DUAL_PHASE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - WORD_LEN"]
    #[inline(always)]
    pub fn word_len(&self) -> WORD_LEN_R {
        WORD_LEN_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - DATA_DELAY"]
    #[inline(always)]
    pub fn data_delay(&mut self) -> DATA_DELAY_W {
        DATA_DELAY_W { w: self }
    }
    #[doc = "Bit 7 - MEM_LEN_24"]
    #[inline(always)]
    pub fn mem_len_24(&mut self) -> MEM_LEN_24_W {
        MEM_LEN_24_W { w: self }
    }
    #[doc = "Bit 6 - SMPL_EDGE"]
    #[inline(always)]
    pub fn smpl_edge(&mut self) -> SMPL_EDGE_W {
        SMPL_EDGE_W { w: self }
    }
    #[doc = "Bit 5 - DUAL_PHASE"]
    #[inline(always)]
    pub fn dual_phase(&mut self) -> DUAL_PHASE_W {
        DUAL_PHASE_W { w: self }
    }
    #[doc = "Bits 0:4 - WORD_LEN"]
    #[inline(always)]
    pub fn word_len(&mut self) -> WORD_LEN_W {
        WORD_LEN_W { w: self }
    }
}

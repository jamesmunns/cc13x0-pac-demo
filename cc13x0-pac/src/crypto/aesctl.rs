#[doc = "Reader of register AESCTL"]
pub type R = crate::R<u32, super::AESCTL>;
#[doc = "Writer for register AESCTL"]
pub type W = crate::W<u32, super::AESCTL>;
#[doc = "Register AESCTL `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::AESCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `CONTEXT_RDY`"]
pub type CONTEXT_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONTEXT_RDY`"]
pub struct CONTEXT_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTEXT_RDY_W<'a> {
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
#[doc = "Reader of field `SAVED_CONTEXT_RDY`"]
pub type SAVED_CONTEXT_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAVED_CONTEXT_RDY`"]
pub struct SAVED_CONTEXT_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SAVED_CONTEXT_RDY_W<'a> {
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
#[doc = "Reader of field `SAVE_CONTEXT`"]
pub type SAVE_CONTEXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAVE_CONTEXT`"]
pub struct SAVE_CONTEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAVE_CONTEXT_W<'a> {
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
#[doc = "Reader of field `CCM_M`"]
pub type CCM_M_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCM_M`"]
pub struct CCM_M_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Reader of field `CCM_L`"]
pub type CCM_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCM_L`"]
pub struct CCM_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Reader of field `CCM`"]
pub type CCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCM`"]
pub struct CCM_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_W<'a> {
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
#[doc = "Reader of field `CBC_MAC`"]
pub type CBC_MAC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBC_MAC`"]
pub struct CBC_MAC_W<'a> {
    w: &'a mut W,
}
impl<'a> CBC_MAC_W<'a> {
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
#[doc = "Reader of field `CTR_WIDTH`"]
pub type CTR_WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTR_WIDTH`"]
pub struct CTR_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Reader of field `CTR`"]
pub type CTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTR`"]
pub struct CTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_W<'a> {
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
#[doc = "Reader of field `CBC`"]
pub type CBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBC`"]
pub struct CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> CBC_W<'a> {
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
#[doc = "Reader of field `KEY_SIZE`"]
pub type KEY_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEY_SIZE`"]
pub struct KEY_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
#[doc = "Reader of field `INPUT_RDY`"]
pub type INPUT_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INPUT_RDY`"]
pub struct INPUT_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_RDY_W<'a> {
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
#[doc = "Reader of field `OUTPUT_RDY`"]
pub type OUTPUT_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTPUT_RDY`"]
pub struct OUTPUT_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_RDY_W<'a> {
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
    #[doc = "Bit 31 - CONTEXT_RDY"]
    #[inline(always)]
    pub fn context_rdy(&self) -> CONTEXT_RDY_R {
        CONTEXT_RDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - SAVED_CONTEXT_RDY"]
    #[inline(always)]
    pub fn saved_context_rdy(&self) -> SAVED_CONTEXT_RDY_R {
        SAVED_CONTEXT_RDY_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SAVE_CONTEXT"]
    #[inline(always)]
    pub fn save_context(&self) -> SAVE_CONTEXT_R {
        SAVE_CONTEXT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 22:24 - CCM_M"]
    #[inline(always)]
    pub fn ccm_m(&self) -> CCM_M_R {
        CCM_M_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - CCM_L"]
    #[inline(always)]
    pub fn ccm_l(&self) -> CCM_L_R {
        CCM_L_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - CCM"]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CBC_MAC"]
    #[inline(always)]
    pub fn cbc_mac(&self) -> CBC_MAC_R {
        CBC_MAC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - CTR_WIDTH"]
    #[inline(always)]
    pub fn ctr_width(&self) -> CTR_WIDTH_R {
        CTR_WIDTH_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 6 - CTR"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CBC"]
    #[inline(always)]
    pub fn cbc(&self) -> CBC_R {
        CBC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - KEY_SIZE"]
    #[inline(always)]
    pub fn key_size(&self) -> KEY_SIZE_R {
        KEY_SIZE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - INPUT_RDY"]
    #[inline(always)]
    pub fn input_rdy(&self) -> INPUT_RDY_R {
        INPUT_RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - OUTPUT_RDY"]
    #[inline(always)]
    pub fn output_rdy(&self) -> OUTPUT_RDY_R {
        OUTPUT_RDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - CONTEXT_RDY"]
    #[inline(always)]
    pub fn context_rdy(&mut self) -> CONTEXT_RDY_W {
        CONTEXT_RDY_W { w: self }
    }
    #[doc = "Bit 30 - SAVED_CONTEXT_RDY"]
    #[inline(always)]
    pub fn saved_context_rdy(&mut self) -> SAVED_CONTEXT_RDY_W {
        SAVED_CONTEXT_RDY_W { w: self }
    }
    #[doc = "Bit 29 - SAVE_CONTEXT"]
    #[inline(always)]
    pub fn save_context(&mut self) -> SAVE_CONTEXT_W {
        SAVE_CONTEXT_W { w: self }
    }
    #[doc = "Bits 22:24 - CCM_M"]
    #[inline(always)]
    pub fn ccm_m(&mut self) -> CCM_M_W {
        CCM_M_W { w: self }
    }
    #[doc = "Bits 19:21 - CCM_L"]
    #[inline(always)]
    pub fn ccm_l(&mut self) -> CCM_L_W {
        CCM_L_W { w: self }
    }
    #[doc = "Bit 18 - CCM"]
    #[inline(always)]
    pub fn ccm(&mut self) -> CCM_W {
        CCM_W { w: self }
    }
    #[doc = "Bit 15 - CBC_MAC"]
    #[inline(always)]
    pub fn cbc_mac(&mut self) -> CBC_MAC_W {
        CBC_MAC_W { w: self }
    }
    #[doc = "Bits 7:8 - CTR_WIDTH"]
    #[inline(always)]
    pub fn ctr_width(&mut self) -> CTR_WIDTH_W {
        CTR_WIDTH_W { w: self }
    }
    #[doc = "Bit 6 - CTR"]
    #[inline(always)]
    pub fn ctr(&mut self) -> CTR_W {
        CTR_W { w: self }
    }
    #[doc = "Bit 5 - CBC"]
    #[inline(always)]
    pub fn cbc(&mut self) -> CBC_W {
        CBC_W { w: self }
    }
    #[doc = "Bits 3:4 - KEY_SIZE"]
    #[inline(always)]
    pub fn key_size(&mut self) -> KEY_SIZE_W {
        KEY_SIZE_W { w: self }
    }
    #[doc = "Bit 2 - DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 1 - INPUT_RDY"]
    #[inline(always)]
    pub fn input_rdy(&mut self) -> INPUT_RDY_W {
        INPUT_RDY_W { w: self }
    }
    #[doc = "Bit 0 - OUTPUT_RDY"]
    #[inline(always)]
    pub fn output_rdy(&mut self) -> OUTPUT_RDY_W {
        OUTPUT_RDY_W { w: self }
    }
}

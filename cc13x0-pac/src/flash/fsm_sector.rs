#[doc = "Reader of register FSM_SECTOR"]
pub type R = crate::R<u32, super::FSM_SECTOR>;
#[doc = "Writer for register FSM_SECTOR"]
pub type W = crate::W<u32, super::FSM_SECTOR>;
#[doc = "Register FSM_SECTOR `reset()`'s with value 0xffff_0000"]
impl crate::ResetValue for super::FSM_SECTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_0000
    }
}
#[doc = "Reader of field `SECT_ERASED`"]
pub type SECT_ERASED_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SECT_ERASED`"]
pub struct SECT_ERASED_W<'a> {
    w: &'a mut W,
}
impl<'a> SECT_ERASED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `FSM_SECTOR_EXTENSION`"]
pub type FSM_SECTOR_EXTENSION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSM_SECTOR_EXTENSION`"]
pub struct FSM_SECTOR_EXTENSION_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_SECTOR_EXTENSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SECTOR`"]
pub type SECTOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECTOR`"]
pub struct SECTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SEC_OUT`"]
pub type SEC_OUT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEC_OUT`"]
pub struct SEC_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - SECT_ERASED"]
    #[inline(always)]
    pub fn sect_erased(&self) -> SECT_ERASED_R {
        SECT_ERASED_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - FSM_SECTOR_EXTENSION"]
    #[inline(always)]
    pub fn fsm_sector_extension(&self) -> FSM_SECTOR_EXTENSION_R {
        FSM_SECTOR_EXTENSION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - SECTOR"]
    #[inline(always)]
    pub fn sector(&self) -> SECTOR_R {
        SECTOR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - SEC_OUT"]
    #[inline(always)]
    pub fn sec_out(&self) -> SEC_OUT_R {
        SEC_OUT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - SECT_ERASED"]
    #[inline(always)]
    pub fn sect_erased(&mut self) -> SECT_ERASED_W {
        SECT_ERASED_W { w: self }
    }
    #[doc = "Bits 8:15 - FSM_SECTOR_EXTENSION"]
    #[inline(always)]
    pub fn fsm_sector_extension(&mut self) -> FSM_SECTOR_EXTENSION_W {
        FSM_SECTOR_EXTENSION_W { w: self }
    }
    #[doc = "Bits 4:7 - SECTOR"]
    #[inline(always)]
    pub fn sector(&mut self) -> SECTOR_W {
        SECTOR_W { w: self }
    }
    #[doc = "Bits 0:3 - SEC_OUT"]
    #[inline(always)]
    pub fn sec_out(&mut self) -> SEC_OUT_W {
        SEC_OUT_W { w: self }
    }
}

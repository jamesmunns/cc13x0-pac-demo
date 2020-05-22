#[doc = "Reader of register FSM_PE_VH"]
pub type R = crate::R<u32, super::FSM_PE_VH>;
#[doc = "Writer for register FSM_PE_VH"]
pub type W = crate::W<u32, super::FSM_PE_VH>;
#[doc = "Register FSM_PE_VH `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::FSM_PE_VH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `PGM_VH`"]
pub type PGM_VH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PGM_VH`"]
pub struct PGM_VH_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_VH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - PGM_VH"]
    #[inline(always)]
    pub fn pgm_vh(&self) -> PGM_VH_R {
        PGM_VH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - PGM_VH"]
    #[inline(always)]
    pub fn pgm_vh(&mut self) -> PGM_VH_W {
        PGM_VH_W { w: self }
    }
}

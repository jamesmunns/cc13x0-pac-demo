#[doc = "Reader of register RCOSCHFCTL"]
pub type R = crate::R<u32, super::RCOSCHFCTL>;
#[doc = "Writer for register RCOSCHFCTL"]
pub type W = crate::W<u32, super::RCOSCHFCTL>;
#[doc = "Register RCOSCHFCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RCOSCHFCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RCOSCHF_CTRIM`"]
pub type RCOSCHF_CTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSCHF_CTRIM`"]
pub struct RCOSCHF_CTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHF_CTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - RCOSCHF_CTRIM"]
    #[inline(always)]
    pub fn rcoschf_ctrim(&self) -> RCOSCHF_CTRIM_R {
        RCOSCHF_CTRIM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - RCOSCHF_CTRIM"]
    #[inline(always)]
    pub fn rcoschf_ctrim(&mut self) -> RCOSCHF_CTRIM_W {
        RCOSCHF_CTRIM_W { w: self }
    }
}

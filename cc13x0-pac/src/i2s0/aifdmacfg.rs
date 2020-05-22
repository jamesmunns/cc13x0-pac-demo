#[doc = "Reader of register AIFDMACFG"]
pub type R = crate::R<u32, super::AIFDMACFG>;
#[doc = "Writer for register AIFDMACFG"]
pub type W = crate::W<u32, super::AIFDMACFG>;
#[doc = "Register AIFDMACFG `reset()`'s with value 0"]
impl crate::ResetValue for super::AIFDMACFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `END_FRAME_IDX`"]
pub type END_FRAME_IDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `END_FRAME_IDX`"]
pub struct END_FRAME_IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> END_FRAME_IDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - END_FRAME_IDX"]
    #[inline(always)]
    pub fn end_frame_idx(&self) -> END_FRAME_IDX_R {
        END_FRAME_IDX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - END_FRAME_IDX"]
    #[inline(always)]
    pub fn end_frame_idx(&mut self) -> END_FRAME_IDX_W {
        END_FRAME_IDX_W { w: self }
    }
}

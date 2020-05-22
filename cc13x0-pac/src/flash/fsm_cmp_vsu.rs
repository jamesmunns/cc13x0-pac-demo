#[doc = "Reader of register FSM_CMP_VSU"]
pub type R = crate::R<u32, super::FSM_CMP_VSU>;
#[doc = "Writer for register FSM_CMP_VSU"]
pub type W = crate::W<u32, super::FSM_CMP_VSU>;
#[doc = "Register FSM_CMP_VSU `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_CMP_VSU {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADD_EXZ`"]
pub type ADD_EXZ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADD_EXZ`"]
pub struct ADD_EXZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_EXZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - ADD_EXZ"]
    #[inline(always)]
    pub fn add_exz(&self) -> ADD_EXZ_R {
        ADD_EXZ_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - ADD_EXZ"]
    #[inline(always)]
    pub fn add_exz(&mut self) -> ADD_EXZ_W {
        ADD_EXZ_W { w: self }
    }
}

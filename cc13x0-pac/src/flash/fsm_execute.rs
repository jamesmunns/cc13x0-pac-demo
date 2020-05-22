#[doc = "Reader of register FSM_EXECUTE"]
pub type R = crate::R<u32, super::FSM_EXECUTE>;
#[doc = "Writer for register FSM_EXECUTE"]
pub type W = crate::W<u32, super::FSM_EXECUTE>;
#[doc = "Register FSM_EXECUTE `reset()`'s with value 0x000a_000a"]
impl crate::ResetValue for super::FSM_EXECUTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000a_000a
    }
}
#[doc = "Reader of field `SUSPEND_NOW`"]
pub type SUSPEND_NOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SUSPEND_NOW`"]
pub struct SUSPEND_NOW_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_NOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `FSMEXECUTE`"]
pub type FSMEXECUTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSMEXECUTE`"]
pub struct FSMEXECUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMEXECUTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - SUSPEND_NOW"]
    #[inline(always)]
    pub fn suspend_now(&self) -> SUSPEND_NOW_R {
        SUSPEND_NOW_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:4 - FSMEXECUTE"]
    #[inline(always)]
    pub fn fsmexecute(&self) -> FSMEXECUTE_R {
        FSMEXECUTE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - SUSPEND_NOW"]
    #[inline(always)]
    pub fn suspend_now(&mut self) -> SUSPEND_NOW_W {
        SUSPEND_NOW_W { w: self }
    }
    #[doc = "Bits 0:4 - FSMEXECUTE"]
    #[inline(always)]
    pub fn fsmexecute(&mut self) -> FSMEXECUTE_W {
        FSMEXECUTE_W { w: self }
    }
}

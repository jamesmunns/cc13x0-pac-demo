#[doc = "Writer for register EFUSEPROGRAM"]
pub type W = crate::W<u32, super::EFUSEPROGRAM>;
#[doc = "Register EFUSEPROGRAM `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSEPROGRAM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `COMPAREDISABLE`"]
pub struct COMPAREDISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREDISABLE_W<'a> {
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
#[doc = "Write proxy for field `CLOCKSTALL`"]
pub struct CLOCKSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKSTALL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 14)) | (((value as u32) & 0xffff) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `VPPTOVDD`"]
pub struct VPPTOVDD_W<'a> {
    w: &'a mut W,
}
impl<'a> VPPTOVDD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `ITERATIONS`"]
pub struct ITERATIONS_W<'a> {
    w: &'a mut W,
}
impl<'a> ITERATIONS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | (((value as u32) & 0x0f) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `WRITECLOCK`"]
pub struct WRITECLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITECLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl W {
    #[doc = "Bit 30 - COMPAREDISABLE"]
    #[inline(always)]
    pub fn comparedisable(&mut self) -> COMPAREDISABLE_W {
        COMPAREDISABLE_W { w: self }
    }
    #[doc = "Bits 14:29 - CLOCKSTALL"]
    #[inline(always)]
    pub fn clockstall(&mut self) -> CLOCKSTALL_W {
        CLOCKSTALL_W { w: self }
    }
    #[doc = "Bit 13 - VPPTOVDD"]
    #[inline(always)]
    pub fn vpptovdd(&mut self) -> VPPTOVDD_W {
        VPPTOVDD_W { w: self }
    }
    #[doc = "Bits 9:12 - ITERATIONS"]
    #[inline(always)]
    pub fn iterations(&mut self) -> ITERATIONS_W {
        ITERATIONS_W { w: self }
    }
    #[doc = "Bits 0:8 - WRITECLOCK"]
    #[inline(always)]
    pub fn writeclock(&mut self) -> WRITECLOCK_W {
        WRITECLOCK_W { w: self }
    }
}

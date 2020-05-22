#[doc = "Reader of register IOSTRP0"]
pub type R = crate::R<u32, super::IOSTRP0>;
#[doc = "Writer for register IOSTRP0"]
pub type W = crate::W<u32, super::IOSTRP0>;
#[doc = "Register IOSTRP0 `reset()`'s with value 0x28"]
impl crate::ResetValue for super::IOSTRP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x28
    }
}
#[doc = "Reader of field `CFG2`"]
pub type CFG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG2`"]
pub struct CFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CFG1`"]
pub type CFG1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG1`"]
pub struct CFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - CFG2"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - CFG1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - CFG2"]
    #[inline(always)]
    pub fn cfg2(&mut self) -> CFG2_W {
        CFG2_W { w: self }
    }
    #[doc = "Bits 0:3 - CFG1"]
    #[inline(always)]
    pub fn cfg1(&mut self) -> CFG1_W {
        CFG1_W { w: self }
    }
}

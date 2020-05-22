#[doc = "Reader of register AIFDIRCFG"]
pub type R = crate::R<u32, super::AIFDIRCFG>;
#[doc = "Writer for register AIFDIRCFG"]
pub type W = crate::W<u32, super::AIFDIRCFG>;
#[doc = "Register AIFDIRCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::AIFDIRCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AD2`"]
pub type AD2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AD2`"]
pub struct AD2_W<'a> {
    w: &'a mut W,
}
impl<'a> AD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `AD1`"]
pub type AD1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AD1`"]
pub struct AD1_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `AD0`"]
pub type AD0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AD0`"]
pub struct AD0_W<'a> {
    w: &'a mut W,
}
impl<'a> AD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - AD2"]
    #[inline(always)]
    pub fn ad2(&self) -> AD2_R {
        AD2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - AD1"]
    #[inline(always)]
    pub fn ad1(&self) -> AD1_R {
        AD1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - AD0"]
    #[inline(always)]
    pub fn ad0(&self) -> AD0_R {
        AD0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - AD2"]
    #[inline(always)]
    pub fn ad2(&mut self) -> AD2_W {
        AD2_W { w: self }
    }
    #[doc = "Bits 4:5 - AD1"]
    #[inline(always)]
    pub fn ad1(&mut self) -> AD1_W {
        AD1_W { w: self }
    }
    #[doc = "Bits 0:1 - AD0"]
    #[inline(always)]
    pub fn ad0(&mut self) -> AD0_W {
        AD0_W { w: self }
    }
}

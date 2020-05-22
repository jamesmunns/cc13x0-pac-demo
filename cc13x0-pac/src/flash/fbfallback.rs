#[doc = "Reader of register FBFALLBACK"]
pub type R = crate::R<u32, super::FBFALLBACK>;
#[doc = "Writer for register FBFALLBACK"]
pub type W = crate::W<u32, super::FBFALLBACK>;
#[doc = "Register FBFALLBACK `reset()`'s with value 0x0505_ffff"]
impl crate::ResetValue for super::FBFALLBACK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0505_ffff
    }
}
#[doc = "Reader of field `FSM_PWRSAV`"]
pub type FSM_PWRSAV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSM_PWRSAV`"]
pub struct FSM_PWRSAV_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_PWRSAV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `REG_PWRSAV`"]
pub type REG_PWRSAV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REG_PWRSAV`"]
pub struct REG_PWRSAV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_PWRSAV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `BANKPWR7`"]
pub type BANKPWR7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BANKPWR7`"]
pub struct BANKPWR7_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `BANKPWR6`"]
pub type BANKPWR6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BANKPWR6`"]
pub struct BANKPWR6_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `BANKPWR5`"]
pub type BANKPWR5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BANKPWR5`"]
pub struct BANKPWR5_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `BANKPWR4`"]
pub type BANKPWR4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BANKPWR4`"]
pub struct BANKPWR4_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `BANKPWR3`"]
pub type BANKPWR3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BANKPWR3`"]
pub struct BANKPWR3_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `BANKPWR2`"]
pub type BANKPWR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BANKPWR2`"]
pub struct BANKPWR2_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `BANKPWR1`"]
pub type BANKPWR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BANKPWR1`"]
pub struct BANKPWR1_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `BANKPWR0`"]
pub type BANKPWR0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BANKPWR0`"]
pub struct BANKPWR0_W<'a> {
    w: &'a mut W,
}
impl<'a> BANKPWR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:27 - FSM_PWRSAV"]
    #[inline(always)]
    pub fn fsm_pwrsav(&self) -> FSM_PWRSAV_R {
        FSM_PWRSAV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - REG_PWRSAV"]
    #[inline(always)]
    pub fn reg_pwrsav(&self) -> REG_PWRSAV_R {
        REG_PWRSAV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - BANKPWR7"]
    #[inline(always)]
    pub fn bankpwr7(&self) -> BANKPWR7_R {
        BANKPWR7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - BANKPWR6"]
    #[inline(always)]
    pub fn bankpwr6(&self) -> BANKPWR6_R {
        BANKPWR6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - BANKPWR5"]
    #[inline(always)]
    pub fn bankpwr5(&self) -> BANKPWR5_R {
        BANKPWR5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - BANKPWR4"]
    #[inline(always)]
    pub fn bankpwr4(&self) -> BANKPWR4_R {
        BANKPWR4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - BANKPWR3"]
    #[inline(always)]
    pub fn bankpwr3(&self) -> BANKPWR3_R {
        BANKPWR3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - BANKPWR2"]
    #[inline(always)]
    pub fn bankpwr2(&self) -> BANKPWR2_R {
        BANKPWR2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - BANKPWR1"]
    #[inline(always)]
    pub fn bankpwr1(&self) -> BANKPWR1_R {
        BANKPWR1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - BANKPWR0"]
    #[inline(always)]
    pub fn bankpwr0(&self) -> BANKPWR0_R {
        BANKPWR0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - FSM_PWRSAV"]
    #[inline(always)]
    pub fn fsm_pwrsav(&mut self) -> FSM_PWRSAV_W {
        FSM_PWRSAV_W { w: self }
    }
    #[doc = "Bits 16:19 - REG_PWRSAV"]
    #[inline(always)]
    pub fn reg_pwrsav(&mut self) -> REG_PWRSAV_W {
        REG_PWRSAV_W { w: self }
    }
    #[doc = "Bits 14:15 - BANKPWR7"]
    #[inline(always)]
    pub fn bankpwr7(&mut self) -> BANKPWR7_W {
        BANKPWR7_W { w: self }
    }
    #[doc = "Bits 12:13 - BANKPWR6"]
    #[inline(always)]
    pub fn bankpwr6(&mut self) -> BANKPWR6_W {
        BANKPWR6_W { w: self }
    }
    #[doc = "Bits 10:11 - BANKPWR5"]
    #[inline(always)]
    pub fn bankpwr5(&mut self) -> BANKPWR5_W {
        BANKPWR5_W { w: self }
    }
    #[doc = "Bits 8:9 - BANKPWR4"]
    #[inline(always)]
    pub fn bankpwr4(&mut self) -> BANKPWR4_W {
        BANKPWR4_W { w: self }
    }
    #[doc = "Bits 6:7 - BANKPWR3"]
    #[inline(always)]
    pub fn bankpwr3(&mut self) -> BANKPWR3_W {
        BANKPWR3_W { w: self }
    }
    #[doc = "Bits 4:5 - BANKPWR2"]
    #[inline(always)]
    pub fn bankpwr2(&mut self) -> BANKPWR2_W {
        BANKPWR2_W { w: self }
    }
    #[doc = "Bits 2:3 - BANKPWR1"]
    #[inline(always)]
    pub fn bankpwr1(&mut self) -> BANKPWR1_W {
        BANKPWR1_W { w: self }
    }
    #[doc = "Bits 0:1 - BANKPWR0"]
    #[inline(always)]
    pub fn bankpwr0(&mut self) -> BANKPWR0_W {
        BANKPWR0_W { w: self }
    }
}

#[doc = "Reader of register EVTOAONPOL"]
pub type R = crate::R<u32, super::EVTOAONPOL>;
#[doc = "Writer for register EVTOAONPOL"]
pub type W = crate::W<u32, super::EVTOAONPOL>;
#[doc = "Register EVTOAONPOL `reset()`'s with value 0"]
impl crate::ResetValue for super::EVTOAONPOL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMER1_EV`"]
pub type TIMER1_EV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER1_EV`"]
pub struct TIMER1_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_EV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TIMER0_EV`"]
pub type TIMER0_EV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER0_EV`"]
pub struct TIMER0_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_EV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TDC_DONE`"]
pub type TDC_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TDC_DONE`"]
pub struct TDC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDC_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ADC_DONE`"]
pub type ADC_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_DONE`"]
pub struct ADC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `AUX_COMPB`"]
pub type AUX_COMPB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_COMPB`"]
pub struct AUX_COMPB_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `AUX_COMPA`"]
pub type AUX_COMPA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_COMPA`"]
pub struct AUX_COMPA_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_COMPA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - TIMER1_EV"]
    #[inline(always)]
    pub fn timer1_ev(&self) -> TIMER1_EV_R {
        TIMER1_EV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TIMER0_EV"]
    #[inline(always)]
    pub fn timer0_ev(&self) -> TIMER0_EV_R {
        TIMER0_EV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TDC_DONE"]
    #[inline(always)]
    pub fn tdc_done(&self) -> TDC_DONE_R {
        TDC_DONE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC_DONE"]
    #[inline(always)]
    pub fn adc_done(&self) -> ADC_DONE_R {
        ADC_DONE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - TIMER1_EV"]
    #[inline(always)]
    pub fn timer1_ev(&mut self) -> TIMER1_EV_W {
        TIMER1_EV_W { w: self }
    }
    #[doc = "Bit 7 - TIMER0_EV"]
    #[inline(always)]
    pub fn timer0_ev(&mut self) -> TIMER0_EV_W {
        TIMER0_EV_W { w: self }
    }
    #[doc = "Bit 6 - TDC_DONE"]
    #[inline(always)]
    pub fn tdc_done(&mut self) -> TDC_DONE_W {
        TDC_DONE_W { w: self }
    }
    #[doc = "Bit 5 - ADC_DONE"]
    #[inline(always)]
    pub fn adc_done(&mut self) -> ADC_DONE_W {
        ADC_DONE_W { w: self }
    }
    #[doc = "Bit 4 - AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W {
        AUX_COMPB_W { w: self }
    }
    #[doc = "Bit 3 - AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W {
        AUX_COMPA_W { w: self }
    }
}

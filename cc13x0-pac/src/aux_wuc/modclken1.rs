#[doc = "Reader of register MODCLKEN1"]
pub type R = crate::R<u32, super::MODCLKEN1>;
#[doc = "Writer for register MODCLKEN1"]
pub type W = crate::W<u32, super::MODCLKEN1>;
#[doc = "Register MODCLKEN1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MODCLKEN1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AUX_ADI4`"]
pub type AUX_ADI4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_ADI4`"]
pub struct AUX_ADI4_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_ADI4_W<'a> {
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
#[doc = "Reader of field `AUX_DDI0_OSC`"]
pub type AUX_DDI0_OSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUX_DDI0_OSC`"]
pub struct AUX_DDI0_OSC_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_DDI0_OSC_W<'a> {
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
#[doc = "Reader of field `ANAIF`"]
pub type ANAIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANAIF`"]
pub struct ANAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ANAIF_W<'a> {
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
#[doc = "Reader of field `TIMER`"]
pub type TIMER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER`"]
pub struct TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_W<'a> {
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
#[doc = "Reader of field `AIODIO1`"]
pub type AIODIO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AIODIO1`"]
pub struct AIODIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> AIODIO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `AIODIO0`"]
pub type AIODIO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AIODIO0`"]
pub struct AIODIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> AIODIO0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SMPH`"]
pub type SMPH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMPH`"]
pub struct SMPH_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPH_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - AUX_ADI4"]
    #[inline(always)]
    pub fn aux_adi4(&self) -> AUX_ADI4_R {
        AUX_ADI4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AUX_DDI0_OSC"]
    #[inline(always)]
    pub fn aux_ddi0_osc(&self) -> AUX_DDI0_OSC_R {
        AUX_DDI0_OSC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ANAIF"]
    #[inline(always)]
    pub fn anaif(&self) -> ANAIF_R {
        ANAIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIMER"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AIODIO1"]
    #[inline(always)]
    pub fn aiodio1(&self) -> AIODIO1_R {
        AIODIO1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - AIODIO0"]
    #[inline(always)]
    pub fn aiodio0(&self) -> AIODIO0_R {
        AIODIO0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SMPH"]
    #[inline(always)]
    pub fn smph(&self) -> SMPH_R {
        SMPH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - AUX_ADI4"]
    #[inline(always)]
    pub fn aux_adi4(&mut self) -> AUX_ADI4_W {
        AUX_ADI4_W { w: self }
    }
    #[doc = "Bit 6 - AUX_DDI0_OSC"]
    #[inline(always)]
    pub fn aux_ddi0_osc(&mut self) -> AUX_DDI0_OSC_W {
        AUX_DDI0_OSC_W { w: self }
    }
    #[doc = "Bit 4 - ANAIF"]
    #[inline(always)]
    pub fn anaif(&mut self) -> ANAIF_W {
        ANAIF_W { w: self }
    }
    #[doc = "Bit 3 - TIMER"]
    #[inline(always)]
    pub fn timer(&mut self) -> TIMER_W {
        TIMER_W { w: self }
    }
    #[doc = "Bit 2 - AIODIO1"]
    #[inline(always)]
    pub fn aiodio1(&mut self) -> AIODIO1_W {
        AIODIO1_W { w: self }
    }
    #[doc = "Bit 1 - AIODIO0"]
    #[inline(always)]
    pub fn aiodio0(&mut self) -> AIODIO0_W {
        AIODIO0_W { w: self }
    }
    #[doc = "Bit 0 - SMPH"]
    #[inline(always)]
    pub fn smph(&mut self) -> SMPH_W {
        SMPH_W { w: self }
    }
}

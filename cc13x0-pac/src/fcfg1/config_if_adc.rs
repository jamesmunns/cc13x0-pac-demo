#[doc = "Reader of register CONFIG_IF_ADC"]
pub type R = crate::R<u32, super::CONFIG_IF_ADC>;
#[doc = "Reader of field `FF2ADJ`"]
pub type FF2ADJ_R = crate::R<u8, u8>;
#[doc = "Reader of field `FF3ADJ`"]
pub type FF3ADJ_R = crate::R<u8, u8>;
#[doc = "Reader of field `INT3ADJ`"]
pub type INT3ADJ_R = crate::R<u8, u8>;
#[doc = "Reader of field `FF1ADJ`"]
pub type FF1ADJ_R = crate::R<u8, u8>;
#[doc = "Reader of field `AAFCAP`"]
pub type AAFCAP_R = crate::R<u8, u8>;
#[doc = "Reader of field `INT2ADJ`"]
pub type INT2ADJ_R = crate::R<u8, u8>;
#[doc = "Reader of field `IFDIGLDO_TRIM_OUTPUT`"]
pub type IFDIGLDO_TRIM_OUTPUT_R = crate::R<u8, u8>;
#[doc = "Reader of field `IFANALDO_TRIM_OUTPUT`"]
pub type IFANALDO_TRIM_OUTPUT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 28:31 - FF2ADJ"]
    #[inline(always)]
    pub fn ff2adj(&self) -> FF2ADJ_R {
        FF2ADJ_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - FF3ADJ"]
    #[inline(always)]
    pub fn ff3adj(&self) -> FF3ADJ_R {
        FF3ADJ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - INT3ADJ"]
    #[inline(always)]
    pub fn int3adj(&self) -> INT3ADJ_R {
        INT3ADJ_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - FF1ADJ"]
    #[inline(always)]
    pub fn ff1adj(&self) -> FF1ADJ_R {
        FF1ADJ_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - AAFCAP"]
    #[inline(always)]
    pub fn aafcap(&self) -> AAFCAP_R {
        AAFCAP_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 10:13 - INT2ADJ"]
    #[inline(always)]
    pub fn int2adj(&self) -> INT2ADJ_R {
        INT2ADJ_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 5:9 - IFDIGLDO_TRIM_OUTPUT"]
    #[inline(always)]
    pub fn ifdigldo_trim_output(&self) -> IFDIGLDO_TRIM_OUTPUT_R {
        IFDIGLDO_TRIM_OUTPUT_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - IFANALDO_TRIM_OUTPUT"]
    #[inline(always)]
    pub fn ifanaldo_trim_output(&self) -> IFANALDO_TRIM_OUTPUT_R {
        IFANALDO_TRIM_OUTPUT_R::new((self.bits & 0x1f) as u8)
    }
}

#[doc = "Reader of register RCOSC_HF_TEMPCOMP"]
pub type R = crate::R<u32, super::RCOSC_HF_TEMPCOMP>;
#[doc = "Reader of field `FINE_RESISTOR`"]
pub type FINE_RESISTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `CTRIM`"]
pub type CTRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `CTRIMFRACT_QUAD`"]
pub type CTRIMFRACT_QUAD_R = crate::R<u8, u8>;
#[doc = "Reader of field `CTRIMFRACT_SLOPE`"]
pub type CTRIMFRACT_SLOPE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - FINE_RESISTOR"]
    #[inline(always)]
    pub fn fine_resistor(&self) -> FINE_RESISTOR_R {
        FINE_RESISTOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CTRIM"]
    #[inline(always)]
    pub fn ctrim(&self) -> CTRIM_R {
        CTRIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CTRIMFRACT_QUAD"]
    #[inline(always)]
    pub fn ctrimfract_quad(&self) -> CTRIMFRACT_QUAD_R {
        CTRIMFRACT_QUAD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - CTRIMFRACT_SLOPE"]
    #[inline(always)]
    pub fn ctrimfract_slope(&self) -> CTRIMFRACT_SLOPE_R {
        CTRIMFRACT_SLOPE_R::new((self.bits & 0xff) as u8)
    }
}

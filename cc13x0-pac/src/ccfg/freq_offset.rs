#[doc = "Reader of register FREQ_OFFSET"]
pub type R = crate::R<u32, super::FREQ_OFFSET>;
#[doc = "Reader of field `HF_COMP_P0`"]
pub type HF_COMP_P0_R = crate::R<u16, u16>;
#[doc = "Reader of field `HF_COMP_P1`"]
pub type HF_COMP_P1_R = crate::R<u8, u8>;
#[doc = "Reader of field `HF_COMP_P2`"]
pub type HF_COMP_P2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:31 - HF_COMP_P0"]
    #[inline(always)]
    pub fn hf_comp_p0(&self) -> HF_COMP_P0_R {
        HF_COMP_P0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - HF_COMP_P1"]
    #[inline(always)]
    pub fn hf_comp_p1(&self) -> HF_COMP_P1_R {
        HF_COMP_P1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - HF_COMP_P2"]
    #[inline(always)]
    pub fn hf_comp_p2(&self) -> HF_COMP_P2_R {
        HF_COMP_P2_R::new((self.bits & 0xff) as u8)
    }
}

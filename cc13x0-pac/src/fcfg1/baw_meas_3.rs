#[doc = "Reader of register BAW_MEAS_3"]
pub type R = crate::R<u32, super::BAW_MEAS_3>;
#[doc = "Reader of field `BAW_D3`"]
pub type BAW_D3_R = crate::R<u16, u16>;
#[doc = "Reader of field `BAW_T3`"]
pub type BAW_T3_R = crate::R<u8, u8>;
#[doc = "Reader of field `BAW_DT3`"]
pub type BAW_DT3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:31 - BAW_D3"]
    #[inline(always)]
    pub fn baw_d3(&self) -> BAW_D3_R {
        BAW_D3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - BAW_T3"]
    #[inline(always)]
    pub fn baw_t3(&self) -> BAW_T3_R {
        BAW_T3_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - BAW_DT3"]
    #[inline(always)]
    pub fn baw_dt3(&self) -> BAW_DT3_R {
        BAW_DT3_R::new((self.bits & 0xff) as u8)
    }
}

#[doc = "Reader of register BAW_MEAS_2"]
pub type R = crate::R<u32, super::BAW_MEAS_2>;
#[doc = "Reader of field `BAW_D2`"]
pub type BAW_D2_R = crate::R<u16, u16>;
#[doc = "Reader of field `BAW_T2`"]
pub type BAW_T2_R = crate::R<u8, u8>;
#[doc = "Reader of field `BAW_DT2`"]
pub type BAW_DT2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:31 - BAW_D2"]
    #[inline(always)]
    pub fn baw_d2(&self) -> BAW_D2_R {
        BAW_D2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - BAW_T2"]
    #[inline(always)]
    pub fn baw_t2(&self) -> BAW_T2_R {
        BAW_T2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - BAW_DT2"]
    #[inline(always)]
    pub fn baw_dt2(&self) -> BAW_DT2_R {
        BAW_DT2_R::new((self.bits & 0xff) as u8)
    }
}

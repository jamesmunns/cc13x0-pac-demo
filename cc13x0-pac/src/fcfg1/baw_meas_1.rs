#[doc = "Reader of register BAW_MEAS_1"]
pub type R = crate::R<u32, super::BAW_MEAS_1>;
#[doc = "Reader of field `BAW_D1`"]
pub type BAW_D1_R = crate::R<u16, u16>;
#[doc = "Reader of field `BAW_T1`"]
pub type BAW_T1_R = crate::R<u8, u8>;
#[doc = "Reader of field `BAW_DT1`"]
pub type BAW_DT1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:31 - BAW_D1"]
    #[inline(always)]
    pub fn baw_d1(&self) -> BAW_D1_R {
        BAW_D1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - BAW_T1"]
    #[inline(always)]
    pub fn baw_t1(&self) -> BAW_T1_R {
        BAW_T1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - BAW_DT1"]
    #[inline(always)]
    pub fn baw_dt1(&self) -> BAW_DT1_R {
        BAW_DT1_R::new((self.bits & 0xff) as u8)
    }
}

#[doc = "Reader of register BAW_MEAS_5"]
pub type R = crate::R<u32, super::BAW_MEAS_5>;
#[doc = "Reader of field `BAW_D5`"]
pub type BAW_D5_R = crate::R<u16, u16>;
#[doc = "Reader of field `BAW_T5`"]
pub type BAW_T5_R = crate::R<u8, u8>;
#[doc = "Reader of field `BAW_DT5`"]
pub type BAW_DT5_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:31 - BAW_D5"]
    #[inline(always)]
    pub fn baw_d5(&self) -> BAW_D5_R {
        BAW_D5_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - BAW_T5"]
    #[inline(always)]
    pub fn baw_t5(&self) -> BAW_T5_R {
        BAW_T5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - BAW_DT5"]
    #[inline(always)]
    pub fn baw_dt5(&self) -> BAW_DT5_R {
        BAW_DT5_R::new((self.bits & 0xff) as u8)
    }
}

#[doc = "Reader of register BAW_MEAS_4"]
pub type R = crate::R<u32, super::BAW_MEAS_4>;
#[doc = "Reader of field `BAW_D4`"]
pub type BAW_D4_R = crate::R<u16, u16>;
#[doc = "Reader of field `BAW_T4`"]
pub type BAW_T4_R = crate::R<u8, u8>;
#[doc = "Reader of field `BAW_DT4`"]
pub type BAW_DT4_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:31 - BAW_D4"]
    #[inline(always)]
    pub fn baw_d4(&self) -> BAW_D4_R {
        BAW_D4_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - BAW_T4"]
    #[inline(always)]
    pub fn baw_t4(&self) -> BAW_T4_R {
        BAW_T4_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - BAW_DT4"]
    #[inline(always)]
    pub fn baw_dt4(&self) -> BAW_DT4_R {
        BAW_DT4_R::new((self.bits & 0xff) as u8)
    }
}

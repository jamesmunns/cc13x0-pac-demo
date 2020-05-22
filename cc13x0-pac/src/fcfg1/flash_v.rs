#[doc = "Reader of register FLASH_V"]
pub type R = crate::R<u32, super::FLASH_V>;
#[doc = "Reader of field `VSL_P`"]
pub type VSL_P_R = crate::R<u8, u8>;
#[doc = "Reader of field `VWL_P`"]
pub type VWL_P_R = crate::R<u8, u8>;
#[doc = "Reader of field `V_READ`"]
pub type V_READ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - VSL_P"]
    #[inline(always)]
    pub fn vsl_p(&self) -> VSL_P_R {
        VSL_P_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - VWL_P"]
    #[inline(always)]
    pub fn vwl_p(&self) -> VWL_P_R {
        VWL_P_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - V_READ"]
    #[inline(always)]
    pub fn v_read(&self) -> V_READ_R {
        V_READ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}

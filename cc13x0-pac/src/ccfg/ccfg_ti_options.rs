#[doc = "Reader of register CCFG_TI_OPTIONS"]
pub type R = crate::R<u32, super::CCFG_TI_OPTIONS>;
#[doc = "Reader of field `TI_FA_ENABLE`"]
pub type TI_FA_ENABLE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - TI_FA_ENABLE"]
    #[inline(always)]
    pub fn ti_fa_enable(&self) -> TI_FA_ENABLE_R {
        TI_FA_ENABLE_R::new((self.bits & 0xff) as u8)
    }
}

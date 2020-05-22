#[doc = "Reader of register DMAPORTERR"]
pub type R = crate::R<u32, super::DMAPORTERR>;
#[doc = "Reader of field `AHB_ERR`"]
pub type AHB_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `LAST_CH`"]
pub type LAST_CH_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 12 - AHB_ERR"]
    #[inline(always)]
    pub fn ahb_err(&self) -> AHB_ERR_R {
        AHB_ERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LAST_CH"]
    #[inline(always)]
    pub fn last_ch(&self) -> LAST_CH_R {
        LAST_CH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}

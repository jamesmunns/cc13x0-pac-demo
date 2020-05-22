#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `BSY`"]
pub type BSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFF`"]
pub type RFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RNE`"]
pub type RNE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TNF`"]
pub type TNF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFE`"]
pub type TFE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 4 - BSY"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RFF"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RNE"]
    #[inline(always)]
    pub fn rne(&self) -> RNE_R {
        RNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TNF"]
    #[inline(always)]
    pub fn tnf(&self) -> TNF_R {
        TNF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TFE"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 0x01) != 0)
    }
}

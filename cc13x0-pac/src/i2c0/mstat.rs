#[doc = "Reader of register MSTAT"]
pub type R = crate::R<u32, super::MSTAT>;
#[doc = "Reader of field `BUSBSY`"]
pub type BUSBSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDLE`"]
pub type IDLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARBLST`"]
pub type ARBLST_R = crate::R<bool, bool>;
#[doc = "Reader of field `DATACK_N`"]
pub type DATACK_N_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADRACK_N`"]
pub type ADRACK_N_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 6 - BUSBSY"]
    #[inline(always)]
    pub fn busbsy(&self) -> BUSBSY_R {
        BUSBSY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IDLE"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ARBLST"]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DATACK_N"]
    #[inline(always)]
    pub fn datack_n(&self) -> DATACK_N_R {
        DATACK_N_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADRACK_N"]
    #[inline(always)]
    pub fn adrack_n(&self) -> ADRACK_N_R {
        ADRACK_N_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ERR"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
}

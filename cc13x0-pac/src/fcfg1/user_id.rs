#[doc = "Reader of register USER_ID"]
pub type R = crate::R<u32, super::USER_ID>;
#[doc = "Reader of field `PG_REV`"]
pub type PG_REV_R = crate::R<u8, u8>;
#[doc = "Reader of field `VER`"]
pub type VER_R = crate::R<u8, u8>;
#[doc = "Reader of field `SEQUENCE`"]
pub type SEQUENCE_R = crate::R<u8, u8>;
#[doc = "Reader of field `PKG`"]
pub type PKG_R = crate::R<u8, u8>;
#[doc = "Reader of field `PROTOCOL`"]
pub type PROTOCOL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 28:31 - PG_REV"]
    #[inline(always)]
    pub fn pg_rev(&self) -> PG_REV_R {
        PG_REV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 26:27 - VER"]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 19:22 - SEQUENCE"]
    #[inline(always)]
    pub fn sequence(&self) -> SEQUENCE_R {
        SEQUENCE_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - PKG"]
    #[inline(always)]
    pub fn pkg(&self) -> PKG_R {
        PKG_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:15 - PROTOCOL"]
    #[inline(always)]
    pub fn protocol(&self) -> PROTOCOL_R {
        PROTOCOL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}

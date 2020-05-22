#[doc = "Reader of register FSM_MODE"]
pub type R = crate::R<u32, super::FSM_MODE>;
#[doc = "Reader of field `RDV_SUBMODE`"]
pub type RDV_SUBMODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `PGM_SUBMODE`"]
pub type PGM_SUBMODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `ERA_SUBMODE`"]
pub type ERA_SUBMODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SUBMODE`"]
pub type SUBMODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SAV_PGM_CMD`"]
pub type SAV_PGM_CMD_R = crate::R<u8, u8>;
#[doc = "Reader of field `SAV_ERA_MODE`"]
pub type SAV_ERA_MODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 18:19 - RDV_SUBMODE"]
    #[inline(always)]
    pub fn rdv_submode(&self) -> RDV_SUBMODE_R {
        RDV_SUBMODE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - PGM_SUBMODE"]
    #[inline(always)]
    pub fn pgm_submode(&self) -> PGM_SUBMODE_R {
        PGM_SUBMODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - ERA_SUBMODE"]
    #[inline(always)]
    pub fn era_submode(&self) -> ERA_SUBMODE_R {
        ERA_SUBMODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - SUBMODE"]
    #[inline(always)]
    pub fn submode(&self) -> SUBMODE_R {
        SUBMODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 9:11 - SAV_PGM_CMD"]
    #[inline(always)]
    pub fn sav_pgm_cmd(&self) -> SAV_PGM_CMD_R {
        SAV_PGM_CMD_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - SAV_ERA_MODE"]
    #[inline(always)]
    pub fn sav_era_mode(&self) -> SAV_ERA_MODE_R {
        SAV_ERA_MODE_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - CMD"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x07) as u8)
    }
}

#[doc = "Reader of register FSM_ST_MACHINE"]
pub type R = crate::R<u32, super::FSM_ST_MACHINE>;
#[doc = "Writer for register FSM_ST_MACHINE"]
pub type W = crate::W<u32, super::FSM_ST_MACHINE>;
#[doc = "Register FSM_ST_MACHINE `reset()`'s with value 0x0080_0500"]
impl crate::ResetValue for super::FSM_ST_MACHINE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0080_0500
    }
}
#[doc = "Reader of field `DO_PRECOND`"]
pub type DO_PRECOND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DO_PRECOND`"]
pub struct DO_PRECOND_W<'a> {
    w: &'a mut W,
}
impl<'a> DO_PRECOND_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `FSM_INT_EN`"]
pub type FSM_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSM_INT_EN`"]
pub struct FSM_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_INT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ALL_BANKS`"]
pub type ALL_BANKS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALL_BANKS`"]
pub struct ALL_BANKS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALL_BANKS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `CMPV_ALLOWED`"]
pub type CMPV_ALLOWED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPV_ALLOWED`"]
pub struct CMPV_ALLOWED_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPV_ALLOWED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `RANDOM`"]
pub type RANDOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RANDOM`"]
pub struct RANDOM_W<'a> {
    w: &'a mut W,
}
impl<'a> RANDOM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `RV_SEC_EN`"]
pub type RV_SEC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RV_SEC_EN`"]
pub struct RV_SEC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RV_SEC_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `RV_RES`"]
pub type RV_RES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RV_RES`"]
pub struct RV_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> RV_RES_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RV_INT_EN`"]
pub type RV_INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RV_INT_EN`"]
pub struct RV_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RV_INT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `ONE_TIME_GOOD`"]
pub type ONE_TIME_GOOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONE_TIME_GOOD`"]
pub struct ONE_TIME_GOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ONE_TIME_GOOD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DO_REDU_COL`"]
pub type DO_REDU_COL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DO_REDU_COL`"]
pub struct DO_REDU_COL_W<'a> {
    w: &'a mut W,
}
impl<'a> DO_REDU_COL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DBG_SHORT_ROW`"]
pub type DBG_SHORT_ROW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBG_SHORT_ROW`"]
pub struct DBG_SHORT_ROW_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_SHORT_ROW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "Reader of field `PGM_SEC_COF_EN`"]
pub type PGM_SEC_COF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PGM_SEC_COF_EN`"]
pub struct PGM_SEC_COF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PGM_SEC_COF_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `PREC_STOP_EN`"]
pub type PREC_STOP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREC_STOP_EN`"]
pub struct PREC_STOP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREC_STOP_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DIS_TST_EN`"]
pub type DIS_TST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_TST_EN`"]
pub struct DIS_TST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_TST_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CMD_EN`"]
pub type CMD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_EN`"]
pub struct CMD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `INV_DATA`"]
pub type INV_DATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INV_DATA`"]
pub struct INV_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_DATA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `OVERRIDE`"]
pub type OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVERRIDE`"]
pub struct OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 23 - DO_PRECOND"]
    #[inline(always)]
    pub fn do_precond(&self) -> DO_PRECOND_R {
        DO_PRECOND_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - FSM_INT_EN"]
    #[inline(always)]
    pub fn fsm_int_en(&self) -> FSM_INT_EN_R {
        FSM_INT_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ALL_BANKS"]
    #[inline(always)]
    pub fn all_banks(&self) -> ALL_BANKS_R {
        ALL_BANKS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CMPV_ALLOWED"]
    #[inline(always)]
    pub fn cmpv_allowed(&self) -> CMPV_ALLOWED_R {
        CMPV_ALLOWED_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RANDOM"]
    #[inline(always)]
    pub fn random(&self) -> RANDOM_R {
        RANDOM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RV_SEC_EN"]
    #[inline(always)]
    pub fn rv_sec_en(&self) -> RV_SEC_EN_R {
        RV_SEC_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RV_RES"]
    #[inline(always)]
    pub fn rv_res(&self) -> RV_RES_R {
        RV_RES_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RV_INT_EN"]
    #[inline(always)]
    pub fn rv_int_en(&self) -> RV_INT_EN_R {
        RV_INT_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ONE_TIME_GOOD"]
    #[inline(always)]
    pub fn one_time_good(&self) -> ONE_TIME_GOOD_R {
        ONE_TIME_GOOD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DO_REDU_COL"]
    #[inline(always)]
    pub fn do_redu_col(&self) -> DO_REDU_COL_R {
        DO_REDU_COL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - DBG_SHORT_ROW"]
    #[inline(always)]
    pub fn dbg_short_row(&self) -> DBG_SHORT_ROW_R {
        DBG_SHORT_ROW_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - PGM_SEC_COF_EN"]
    #[inline(always)]
    pub fn pgm_sec_cof_en(&self) -> PGM_SEC_COF_EN_R {
        PGM_SEC_COF_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PREC_STOP_EN"]
    #[inline(always)]
    pub fn prec_stop_en(&self) -> PREC_STOP_EN_R {
        PREC_STOP_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DIS_TST_EN"]
    #[inline(always)]
    pub fn dis_tst_en(&self) -> DIS_TST_EN_R {
        DIS_TST_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CMD_EN"]
    #[inline(always)]
    pub fn cmd_en(&self) -> CMD_EN_R {
        CMD_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - INV_DATA"]
    #[inline(always)]
    pub fn inv_data(&self) -> INV_DATA_R {
        INV_DATA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - OVERRIDE"]
    #[inline(always)]
    pub fn override_(&self) -> OVERRIDE_R {
        OVERRIDE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - DO_PRECOND"]
    #[inline(always)]
    pub fn do_precond(&mut self) -> DO_PRECOND_W {
        DO_PRECOND_W { w: self }
    }
    #[doc = "Bit 22 - FSM_INT_EN"]
    #[inline(always)]
    pub fn fsm_int_en(&mut self) -> FSM_INT_EN_W {
        FSM_INT_EN_W { w: self }
    }
    #[doc = "Bit 21 - ALL_BANKS"]
    #[inline(always)]
    pub fn all_banks(&mut self) -> ALL_BANKS_W {
        ALL_BANKS_W { w: self }
    }
    #[doc = "Bit 20 - CMPV_ALLOWED"]
    #[inline(always)]
    pub fn cmpv_allowed(&mut self) -> CMPV_ALLOWED_W {
        CMPV_ALLOWED_W { w: self }
    }
    #[doc = "Bit 19 - RANDOM"]
    #[inline(always)]
    pub fn random(&mut self) -> RANDOM_W {
        RANDOM_W { w: self }
    }
    #[doc = "Bit 18 - RV_SEC_EN"]
    #[inline(always)]
    pub fn rv_sec_en(&mut self) -> RV_SEC_EN_W {
        RV_SEC_EN_W { w: self }
    }
    #[doc = "Bit 17 - RV_RES"]
    #[inline(always)]
    pub fn rv_res(&mut self) -> RV_RES_W {
        RV_RES_W { w: self }
    }
    #[doc = "Bit 16 - RV_INT_EN"]
    #[inline(always)]
    pub fn rv_int_en(&mut self) -> RV_INT_EN_W {
        RV_INT_EN_W { w: self }
    }
    #[doc = "Bit 14 - ONE_TIME_GOOD"]
    #[inline(always)]
    pub fn one_time_good(&mut self) -> ONE_TIME_GOOD_W {
        ONE_TIME_GOOD_W { w: self }
    }
    #[doc = "Bit 11 - DO_REDU_COL"]
    #[inline(always)]
    pub fn do_redu_col(&mut self) -> DO_REDU_COL_W {
        DO_REDU_COL_W { w: self }
    }
    #[doc = "Bits 7:10 - DBG_SHORT_ROW"]
    #[inline(always)]
    pub fn dbg_short_row(&mut self) -> DBG_SHORT_ROW_W {
        DBG_SHORT_ROW_W { w: self }
    }
    #[doc = "Bit 5 - PGM_SEC_COF_EN"]
    #[inline(always)]
    pub fn pgm_sec_cof_en(&mut self) -> PGM_SEC_COF_EN_W {
        PGM_SEC_COF_EN_W { w: self }
    }
    #[doc = "Bit 4 - PREC_STOP_EN"]
    #[inline(always)]
    pub fn prec_stop_en(&mut self) -> PREC_STOP_EN_W {
        PREC_STOP_EN_W { w: self }
    }
    #[doc = "Bit 3 - DIS_TST_EN"]
    #[inline(always)]
    pub fn dis_tst_en(&mut self) -> DIS_TST_EN_W {
        DIS_TST_EN_W { w: self }
    }
    #[doc = "Bit 2 - CMD_EN"]
    #[inline(always)]
    pub fn cmd_en(&mut self) -> CMD_EN_W {
        CMD_EN_W { w: self }
    }
    #[doc = "Bit 1 - INV_DATA"]
    #[inline(always)]
    pub fn inv_data(&mut self) -> INV_DATA_W {
        INV_DATA_W { w: self }
    }
    #[doc = "Bit 0 - OVERRIDE"]
    #[inline(always)]
    pub fn override_(&mut self) -> OVERRIDE_W {
        OVERRIDE_W { w: self }
    }
}

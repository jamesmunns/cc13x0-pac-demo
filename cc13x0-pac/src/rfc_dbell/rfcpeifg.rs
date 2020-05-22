#[doc = "Reader of register RFCPEIFG"]
pub type R = crate::R<u32, super::RFCPEIFG>;
#[doc = "Writer for register RFCPEIFG"]
pub type W = crate::W<u32, super::RFCPEIFG>;
#[doc = "Register RFCPEIFG `reset()`'s with value 0"]
impl crate::ResetValue for super::RFCPEIFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTERNAL_ERROR`"]
pub type INTERNAL_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTERNAL_ERROR`"]
pub struct INTERNAL_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERNAL_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `BOOT_DONE`"]
pub type BOOT_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_DONE`"]
pub struct BOOT_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `MODULES_UNLOCKED`"]
pub type MODULES_UNLOCKED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODULES_UNLOCKED`"]
pub struct MODULES_UNLOCKED_W<'a> {
    w: &'a mut W,
}
impl<'a> MODULES_UNLOCKED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SYNTH_NO_LOCK`"]
pub type SYNTH_NO_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNTH_NO_LOCK`"]
pub struct SYNTH_NO_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNTH_NO_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `IRQ27`"]
pub type IRQ27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ27`"]
pub struct IRQ27_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `RX_ABORTED`"]
pub type RX_ABORTED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_ABORTED`"]
pub struct RX_ABORTED_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ABORTED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `RX_N_DATA_WRITTEN`"]
pub type RX_N_DATA_WRITTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_N_DATA_WRITTEN`"]
pub struct RX_N_DATA_WRITTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_N_DATA_WRITTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `RX_DATA_WRITTEN`"]
pub type RX_DATA_WRITTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_DATA_WRITTEN`"]
pub struct RX_DATA_WRITTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_WRITTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RX_ENTRY_DONE`"]
pub type RX_ENTRY_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_ENTRY_DONE`"]
pub struct RX_ENTRY_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ENTRY_DONE_W<'a> {
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
#[doc = "Reader of field `RX_BUF_FULL`"]
pub type RX_BUF_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_BUF_FULL`"]
pub struct RX_BUF_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BUF_FULL_W<'a> {
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
#[doc = "Reader of field `RX_CTRL_ACK`"]
pub type RX_CTRL_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_CTRL_ACK`"]
pub struct RX_CTRL_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CTRL_ACK_W<'a> {
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
#[doc = "Reader of field `RX_CTRL`"]
pub type RX_CTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_CTRL`"]
pub struct RX_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CTRL_W<'a> {
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
#[doc = "Reader of field `RX_EMPTY`"]
pub type RX_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_EMPTY`"]
pub struct RX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EMPTY_W<'a> {
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
#[doc = "Reader of field `RX_IGNORED`"]
pub type RX_IGNORED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_IGNORED`"]
pub struct RX_IGNORED_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_IGNORED_W<'a> {
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
#[doc = "Reader of field `RX_NOK`"]
pub type RX_NOK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_NOK`"]
pub struct RX_NOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_NOK_W<'a> {
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
#[doc = "Reader of field `RX_OK`"]
pub type RX_OK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_OK`"]
pub struct RX_OK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OK_W<'a> {
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
#[doc = "Reader of field `IRQ15`"]
pub type IRQ15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ15`"]
pub struct IRQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `IRQ14`"]
pub type IRQ14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ14`"]
pub struct IRQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ14_W<'a> {
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
#[doc = "Reader of field `IRQ13`"]
pub type IRQ13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ13`"]
pub struct IRQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `IRQ12`"]
pub type IRQ12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQ12`"]
pub struct IRQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TX_BUFFER_CHANGED`"]
pub type TX_BUFFER_CHANGED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_BUFFER_CHANGED`"]
pub struct TX_BUFFER_CHANGED_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BUFFER_CHANGED_W<'a> {
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
#[doc = "Reader of field `TX_ENTRY_DONE`"]
pub type TX_ENTRY_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_ENTRY_DONE`"]
pub struct TX_ENTRY_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ENTRY_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `TX_RETRANS`"]
pub type TX_RETRANS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_RETRANS`"]
pub struct TX_RETRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_RETRANS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TX_CTRL_ACK_ACK`"]
pub type TX_CTRL_ACK_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_CTRL_ACK_ACK`"]
pub struct TX_CTRL_ACK_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CTRL_ACK_ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TX_CTRL_ACK`"]
pub type TX_CTRL_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_CTRL_ACK`"]
pub struct TX_CTRL_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CTRL_ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TX_CTRL`"]
pub type TX_CTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_CTRL`"]
pub struct TX_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TX_ACK`"]
pub type TX_ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_ACK`"]
pub struct TX_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ACK_W<'a> {
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
#[doc = "Reader of field `TX_DONE`"]
pub type TX_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DONE`"]
pub struct TX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DONE_W<'a> {
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
#[doc = "Reader of field `LAST_FG_COMMAND_DONE`"]
pub type LAST_FG_COMMAND_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LAST_FG_COMMAND_DONE`"]
pub struct LAST_FG_COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_FG_COMMAND_DONE_W<'a> {
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
#[doc = "Reader of field `FG_COMMAND_DONE`"]
pub type FG_COMMAND_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FG_COMMAND_DONE`"]
pub struct FG_COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> FG_COMMAND_DONE_W<'a> {
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
#[doc = "Reader of field `LAST_COMMAND_DONE`"]
pub type LAST_COMMAND_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LAST_COMMAND_DONE`"]
pub struct LAST_COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> LAST_COMMAND_DONE_W<'a> {
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
#[doc = "Reader of field `COMMAND_DONE`"]
pub type COMMAND_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMMAND_DONE`"]
pub struct COMMAND_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMAND_DONE_W<'a> {
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
    #[doc = "Bit 31 - INTERNAL_ERROR"]
    #[inline(always)]
    pub fn internal_error(&self) -> INTERNAL_ERROR_R {
        INTERNAL_ERROR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - BOOT_DONE"]
    #[inline(always)]
    pub fn boot_done(&self) -> BOOT_DONE_R {
        BOOT_DONE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - MODULES_UNLOCKED"]
    #[inline(always)]
    pub fn modules_unlocked(&self) -> MODULES_UNLOCKED_R {
        MODULES_UNLOCKED_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SYNTH_NO_LOCK"]
    #[inline(always)]
    pub fn synth_no_lock(&self) -> SYNTH_NO_LOCK_R {
        SYNTH_NO_LOCK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - IRQ27"]
    #[inline(always)]
    pub fn irq27(&self) -> IRQ27_R {
        IRQ27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - RX_ABORTED"]
    #[inline(always)]
    pub fn rx_aborted(&self) -> RX_ABORTED_R {
        RX_ABORTED_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RX_N_DATA_WRITTEN"]
    #[inline(always)]
    pub fn rx_n_data_written(&self) -> RX_N_DATA_WRITTEN_R {
        RX_N_DATA_WRITTEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RX_DATA_WRITTEN"]
    #[inline(always)]
    pub fn rx_data_written(&self) -> RX_DATA_WRITTEN_R {
        RX_DATA_WRITTEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - RX_ENTRY_DONE"]
    #[inline(always)]
    pub fn rx_entry_done(&self) -> RX_ENTRY_DONE_R {
        RX_ENTRY_DONE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - RX_BUF_FULL"]
    #[inline(always)]
    pub fn rx_buf_full(&self) -> RX_BUF_FULL_R {
        RX_BUF_FULL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - RX_CTRL_ACK"]
    #[inline(always)]
    pub fn rx_ctrl_ack(&self) -> RX_CTRL_ACK_R {
        RX_CTRL_ACK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - RX_CTRL"]
    #[inline(always)]
    pub fn rx_ctrl(&self) -> RX_CTRL_R {
        RX_CTRL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RX_EMPTY"]
    #[inline(always)]
    pub fn rx_empty(&self) -> RX_EMPTY_R {
        RX_EMPTY_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RX_IGNORED"]
    #[inline(always)]
    pub fn rx_ignored(&self) -> RX_IGNORED_R {
        RX_IGNORED_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RX_NOK"]
    #[inline(always)]
    pub fn rx_nok(&self) -> RX_NOK_R {
        RX_NOK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RX_OK"]
    #[inline(always)]
    pub fn rx_ok(&self) -> RX_OK_R {
        RX_OK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IRQ15"]
    #[inline(always)]
    pub fn irq15(&self) -> IRQ15_R {
        IRQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - IRQ14"]
    #[inline(always)]
    pub fn irq14(&self) -> IRQ14_R {
        IRQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - IRQ13"]
    #[inline(always)]
    pub fn irq13(&self) -> IRQ13_R {
        IRQ13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IRQ12"]
    #[inline(always)]
    pub fn irq12(&self) -> IRQ12_R {
        IRQ12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TX_BUFFER_CHANGED"]
    #[inline(always)]
    pub fn tx_buffer_changed(&self) -> TX_BUFFER_CHANGED_R {
        TX_BUFFER_CHANGED_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TX_ENTRY_DONE"]
    #[inline(always)]
    pub fn tx_entry_done(&self) -> TX_ENTRY_DONE_R {
        TX_ENTRY_DONE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TX_RETRANS"]
    #[inline(always)]
    pub fn tx_retrans(&self) -> TX_RETRANS_R {
        TX_RETRANS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TX_CTRL_ACK_ACK"]
    #[inline(always)]
    pub fn tx_ctrl_ack_ack(&self) -> TX_CTRL_ACK_ACK_R {
        TX_CTRL_ACK_ACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TX_CTRL_ACK"]
    #[inline(always)]
    pub fn tx_ctrl_ack(&self) -> TX_CTRL_ACK_R {
        TX_CTRL_ACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TX_CTRL"]
    #[inline(always)]
    pub fn tx_ctrl(&self) -> TX_CTRL_R {
        TX_CTRL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TX_ACK"]
    #[inline(always)]
    pub fn tx_ack(&self) -> TX_ACK_R {
        TX_ACK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TX_DONE"]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LAST_FG_COMMAND_DONE"]
    #[inline(always)]
    pub fn last_fg_command_done(&self) -> LAST_FG_COMMAND_DONE_R {
        LAST_FG_COMMAND_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FG_COMMAND_DONE"]
    #[inline(always)]
    pub fn fg_command_done(&self) -> FG_COMMAND_DONE_R {
        FG_COMMAND_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LAST_COMMAND_DONE"]
    #[inline(always)]
    pub fn last_command_done(&self) -> LAST_COMMAND_DONE_R {
        LAST_COMMAND_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - COMMAND_DONE"]
    #[inline(always)]
    pub fn command_done(&self) -> COMMAND_DONE_R {
        COMMAND_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - INTERNAL_ERROR"]
    #[inline(always)]
    pub fn internal_error(&mut self) -> INTERNAL_ERROR_W {
        INTERNAL_ERROR_W { w: self }
    }
    #[doc = "Bit 30 - BOOT_DONE"]
    #[inline(always)]
    pub fn boot_done(&mut self) -> BOOT_DONE_W {
        BOOT_DONE_W { w: self }
    }
    #[doc = "Bit 29 - MODULES_UNLOCKED"]
    #[inline(always)]
    pub fn modules_unlocked(&mut self) -> MODULES_UNLOCKED_W {
        MODULES_UNLOCKED_W { w: self }
    }
    #[doc = "Bit 28 - SYNTH_NO_LOCK"]
    #[inline(always)]
    pub fn synth_no_lock(&mut self) -> SYNTH_NO_LOCK_W {
        SYNTH_NO_LOCK_W { w: self }
    }
    #[doc = "Bit 27 - IRQ27"]
    #[inline(always)]
    pub fn irq27(&mut self) -> IRQ27_W {
        IRQ27_W { w: self }
    }
    #[doc = "Bit 26 - RX_ABORTED"]
    #[inline(always)]
    pub fn rx_aborted(&mut self) -> RX_ABORTED_W {
        RX_ABORTED_W { w: self }
    }
    #[doc = "Bit 25 - RX_N_DATA_WRITTEN"]
    #[inline(always)]
    pub fn rx_n_data_written(&mut self) -> RX_N_DATA_WRITTEN_W {
        RX_N_DATA_WRITTEN_W { w: self }
    }
    #[doc = "Bit 24 - RX_DATA_WRITTEN"]
    #[inline(always)]
    pub fn rx_data_written(&mut self) -> RX_DATA_WRITTEN_W {
        RX_DATA_WRITTEN_W { w: self }
    }
    #[doc = "Bit 23 - RX_ENTRY_DONE"]
    #[inline(always)]
    pub fn rx_entry_done(&mut self) -> RX_ENTRY_DONE_W {
        RX_ENTRY_DONE_W { w: self }
    }
    #[doc = "Bit 22 - RX_BUF_FULL"]
    #[inline(always)]
    pub fn rx_buf_full(&mut self) -> RX_BUF_FULL_W {
        RX_BUF_FULL_W { w: self }
    }
    #[doc = "Bit 21 - RX_CTRL_ACK"]
    #[inline(always)]
    pub fn rx_ctrl_ack(&mut self) -> RX_CTRL_ACK_W {
        RX_CTRL_ACK_W { w: self }
    }
    #[doc = "Bit 20 - RX_CTRL"]
    #[inline(always)]
    pub fn rx_ctrl(&mut self) -> RX_CTRL_W {
        RX_CTRL_W { w: self }
    }
    #[doc = "Bit 19 - RX_EMPTY"]
    #[inline(always)]
    pub fn rx_empty(&mut self) -> RX_EMPTY_W {
        RX_EMPTY_W { w: self }
    }
    #[doc = "Bit 18 - RX_IGNORED"]
    #[inline(always)]
    pub fn rx_ignored(&mut self) -> RX_IGNORED_W {
        RX_IGNORED_W { w: self }
    }
    #[doc = "Bit 17 - RX_NOK"]
    #[inline(always)]
    pub fn rx_nok(&mut self) -> RX_NOK_W {
        RX_NOK_W { w: self }
    }
    #[doc = "Bit 16 - RX_OK"]
    #[inline(always)]
    pub fn rx_ok(&mut self) -> RX_OK_W {
        RX_OK_W { w: self }
    }
    #[doc = "Bit 15 - IRQ15"]
    #[inline(always)]
    pub fn irq15(&mut self) -> IRQ15_W {
        IRQ15_W { w: self }
    }
    #[doc = "Bit 14 - IRQ14"]
    #[inline(always)]
    pub fn irq14(&mut self) -> IRQ14_W {
        IRQ14_W { w: self }
    }
    #[doc = "Bit 13 - IRQ13"]
    #[inline(always)]
    pub fn irq13(&mut self) -> IRQ13_W {
        IRQ13_W { w: self }
    }
    #[doc = "Bit 12 - IRQ12"]
    #[inline(always)]
    pub fn irq12(&mut self) -> IRQ12_W {
        IRQ12_W { w: self }
    }
    #[doc = "Bit 11 - TX_BUFFER_CHANGED"]
    #[inline(always)]
    pub fn tx_buffer_changed(&mut self) -> TX_BUFFER_CHANGED_W {
        TX_BUFFER_CHANGED_W { w: self }
    }
    #[doc = "Bit 10 - TX_ENTRY_DONE"]
    #[inline(always)]
    pub fn tx_entry_done(&mut self) -> TX_ENTRY_DONE_W {
        TX_ENTRY_DONE_W { w: self }
    }
    #[doc = "Bit 9 - TX_RETRANS"]
    #[inline(always)]
    pub fn tx_retrans(&mut self) -> TX_RETRANS_W {
        TX_RETRANS_W { w: self }
    }
    #[doc = "Bit 8 - TX_CTRL_ACK_ACK"]
    #[inline(always)]
    pub fn tx_ctrl_ack_ack(&mut self) -> TX_CTRL_ACK_ACK_W {
        TX_CTRL_ACK_ACK_W { w: self }
    }
    #[doc = "Bit 7 - TX_CTRL_ACK"]
    #[inline(always)]
    pub fn tx_ctrl_ack(&mut self) -> TX_CTRL_ACK_W {
        TX_CTRL_ACK_W { w: self }
    }
    #[doc = "Bit 6 - TX_CTRL"]
    #[inline(always)]
    pub fn tx_ctrl(&mut self) -> TX_CTRL_W {
        TX_CTRL_W { w: self }
    }
    #[doc = "Bit 5 - TX_ACK"]
    #[inline(always)]
    pub fn tx_ack(&mut self) -> TX_ACK_W {
        TX_ACK_W { w: self }
    }
    #[doc = "Bit 4 - TX_DONE"]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TX_DONE_W {
        TX_DONE_W { w: self }
    }
    #[doc = "Bit 3 - LAST_FG_COMMAND_DONE"]
    #[inline(always)]
    pub fn last_fg_command_done(&mut self) -> LAST_FG_COMMAND_DONE_W {
        LAST_FG_COMMAND_DONE_W { w: self }
    }
    #[doc = "Bit 2 - FG_COMMAND_DONE"]
    #[inline(always)]
    pub fn fg_command_done(&mut self) -> FG_COMMAND_DONE_W {
        FG_COMMAND_DONE_W { w: self }
    }
    #[doc = "Bit 1 - LAST_COMMAND_DONE"]
    #[inline(always)]
    pub fn last_command_done(&mut self) -> LAST_COMMAND_DONE_W {
        LAST_COMMAND_DONE_W { w: self }
    }
    #[doc = "Bit 0 - COMMAND_DONE"]
    #[inline(always)]
    pub fn command_done(&mut self) -> COMMAND_DONE_W {
        COMMAND_DONE_W { w: self }
    }
}

#[doc = "Reader of register FETCHSTAT"]
pub type R = crate::R<u32, super::FETCHSTAT>;
#[doc = "Reader of field `OPCODE`"]
pub type OPCODE_R = crate::R<u16, u16>;
#[doc = "Reader of field `PC`"]
pub type PC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - OPCODE"]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - PC"]
    #[inline(always)]
    pub fn pc(&self) -> PC_R {
        PC_R::new((self.bits & 0xffff) as u16)
    }
}

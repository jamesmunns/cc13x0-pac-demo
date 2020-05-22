#[doc = "Reader of register REG3_2"]
pub type R = crate::R<u32, super::REG3_2>;
#[doc = "Reader of field `REG3`"]
pub type REG3_R = crate::R<u16, u16>;
#[doc = "Reader of field `REG2`"]
pub type REG2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - REG3"]
    #[inline(always)]
    pub fn reg3(&self) -> REG3_R {
        REG3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - REG2"]
    #[inline(always)]
    pub fn reg2(&self) -> REG2_R {
        REG2_R::new((self.bits & 0xffff) as u16)
    }
}

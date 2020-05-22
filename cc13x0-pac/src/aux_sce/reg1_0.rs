#[doc = "Reader of register REG1_0"]
pub type R = crate::R<u32, super::REG1_0>;
#[doc = "Reader of field `REG1`"]
pub type REG1_R = crate::R<u16, u16>;
#[doc = "Reader of field `REG0`"]
pub type REG0_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - REG1"]
    #[inline(always)]
    pub fn reg1(&self) -> REG1_R {
        REG1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - REG0"]
    #[inline(always)]
    pub fn reg0(&self) -> REG0_R {
        REG0_R::new((self.bits & 0xffff) as u16)
    }
}

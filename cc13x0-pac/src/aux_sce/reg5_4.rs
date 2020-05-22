#[doc = "Reader of register REG5_4"]
pub type R = crate::R<u32, super::REG5_4>;
#[doc = "Reader of field `REG5`"]
pub type REG5_R = crate::R<u16, u16>;
#[doc = "Reader of field `REG4`"]
pub type REG4_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - REG5"]
    #[inline(always)]
    pub fn reg5(&self) -> REG5_R {
        REG5_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - REG4"]
    #[inline(always)]
    pub fn reg4(&self) -> REG4_R {
        REG4_R::new((self.bits & 0xffff) as u16)
    }
}

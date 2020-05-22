#[doc = "Reader of register REG7_6"]
pub type R = crate::R<u32, super::REG7_6>;
#[doc = "Reader of field `REG7`"]
pub type REG7_R = crate::R<u16, u16>;
#[doc = "Reader of field `REG6`"]
pub type REG6_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - REG7"]
    #[inline(always)]
    pub fn reg7(&self) -> REG7_R {
        REG7_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - REG6"]
    #[inline(always)]
    pub fn reg6(&self) -> REG6_R {
        REG6_R::new((self.bits & 0xffff) as u16)
    }
}

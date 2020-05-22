#[doc = "Reader of register HWVER1"]
pub type R = crate::R<u32, super::HWVER1>;
#[doc = "Reader of field `REV`"]
pub type REV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - REV"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0xff) as u8)
    }
}

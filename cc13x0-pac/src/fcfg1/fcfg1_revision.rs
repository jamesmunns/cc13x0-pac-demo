#[doc = "Reader of register FCFG1_REVISION"]
pub type R = crate::R<u32, super::FCFG1_REVISION>;
#[doc = "Reader of field `REV`"]
pub type REV_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - REV"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

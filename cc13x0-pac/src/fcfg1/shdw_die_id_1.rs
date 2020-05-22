#[doc = "Reader of register SHDW_DIE_ID_1"]
pub type R = crate::R<u32, super::SHDW_DIE_ID_1>;
#[doc = "Reader of field `ID_63_32`"]
pub type ID_63_32_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ID_63_32"]
    #[inline(always)]
    pub fn id_63_32(&self) -> ID_63_32_R {
        ID_63_32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

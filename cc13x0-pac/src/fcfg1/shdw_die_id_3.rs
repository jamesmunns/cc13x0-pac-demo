#[doc = "Reader of register SHDW_DIE_ID_3"]
pub type R = crate::R<u32, super::SHDW_DIE_ID_3>;
#[doc = "Reader of field `ID_127_96`"]
pub type ID_127_96_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ID_127_96"]
    #[inline(always)]
    pub fn id_127_96(&self) -> ID_127_96_R {
        ID_127_96_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

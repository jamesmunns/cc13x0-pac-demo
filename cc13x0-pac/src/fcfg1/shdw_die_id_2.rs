#[doc = "Reader of register SHDW_DIE_ID_2"]
pub type R = crate::R<u32, super::SHDW_DIE_ID_2>;
#[doc = "Reader of field `ID_95_64`"]
pub type ID_95_64_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ID_95_64"]
    #[inline(always)]
    pub fn id_95_64(&self) -> ID_95_64_R {
        ID_95_64_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

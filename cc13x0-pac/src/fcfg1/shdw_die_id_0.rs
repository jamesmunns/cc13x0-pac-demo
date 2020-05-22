#[doc = "Reader of register SHDW_DIE_ID_0"]
pub type R = crate::R<u32, super::SHDW_DIE_ID_0>;
#[doc = "Reader of field `ID_31_0`"]
pub type ID_31_0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ID_31_0"]
    #[inline(always)]
    pub fn id_31_0(&self) -> ID_31_0_R {
        ID_31_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

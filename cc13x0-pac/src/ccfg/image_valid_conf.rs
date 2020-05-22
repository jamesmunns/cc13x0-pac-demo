#[doc = "Reader of register IMAGE_VALID_CONF"]
pub type R = crate::R<u32, super::IMAGE_VALID_CONF>;
#[doc = "Reader of field `IMAGE_VALID`"]
pub type IMAGE_VALID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IMAGE_VALID"]
    #[inline(always)]
    pub fn image_valid(&self) -> IMAGE_VALID_R {
        IMAGE_VALID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}

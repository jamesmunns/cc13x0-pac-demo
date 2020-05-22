#[doc = "Reader of register FLASH_COORDINATE"]
pub type R = crate::R<u32, super::FLASH_COORDINATE>;
#[doc = "Reader of field `XCOORDINATE`"]
pub type XCOORDINATE_R = crate::R<u16, u16>;
#[doc = "Reader of field `YCOORDINATE`"]
pub type YCOORDINATE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - XCOORDINATE"]
    #[inline(always)]
    pub fn xcoordinate(&self) -> XCOORDINATE_R {
        XCOORDINATE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - YCOORDINATE"]
    #[inline(always)]
    pub fn ycoordinate(&self) -> YCOORDINATE_R {
        YCOORDINATE_R::new((self.bits & 0xffff) as u16)
    }
}

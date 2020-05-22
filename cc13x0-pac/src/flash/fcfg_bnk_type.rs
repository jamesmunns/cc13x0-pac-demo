#[doc = "Reader of register FCFG_BNK_TYPE"]
pub type R = crate::R<u32, super::FCFG_BNK_TYPE>;
#[doc = "Reader of field `B7_TYPE`"]
pub type B7_TYPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `B6_TYPE`"]
pub type B6_TYPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `B5_TYPE`"]
pub type B5_TYPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `B4_TYPE`"]
pub type B4_TYPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `B3_TYPE`"]
pub type B3_TYPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `B2_TYPE`"]
pub type B2_TYPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `B1_TYPE`"]
pub type B1_TYPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `B0_TYPE`"]
pub type B0_TYPE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 28:31 - B7_TYPE"]
    #[inline(always)]
    pub fn b7_type(&self) -> B7_TYPE_R {
        B7_TYPE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - B6_TYPE"]
    #[inline(always)]
    pub fn b6_type(&self) -> B6_TYPE_R {
        B6_TYPE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - B5_TYPE"]
    #[inline(always)]
    pub fn b5_type(&self) -> B5_TYPE_R {
        B5_TYPE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - B4_TYPE"]
    #[inline(always)]
    pub fn b4_type(&self) -> B4_TYPE_R {
        B4_TYPE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - B3_TYPE"]
    #[inline(always)]
    pub fn b3_type(&self) -> B3_TYPE_R {
        B3_TYPE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - B2_TYPE"]
    #[inline(always)]
    pub fn b2_type(&self) -> B2_TYPE_R {
        B2_TYPE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - B1_TYPE"]
    #[inline(always)]
    pub fn b1_type(&self) -> B1_TYPE_R {
        B1_TYPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - B0_TYPE"]
    #[inline(always)]
    pub fn b0_type(&self) -> B0_TYPE_R {
        B0_TYPE_R::new((self.bits & 0x0f) as u8)
    }
}

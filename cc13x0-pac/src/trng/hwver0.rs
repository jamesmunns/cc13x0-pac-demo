#[doc = "Reader of register HWVER0"]
pub type R = crate::R<u32, super::HWVER0>;
#[doc = "Reader of field `HW_MAJOR_VER`"]
pub type HW_MAJOR_VER_R = crate::R<u8, u8>;
#[doc = "Reader of field `HW_MINOR_VER`"]
pub type HW_MINOR_VER_R = crate::R<u8, u8>;
#[doc = "Reader of field `HW_PATCH_LVL`"]
pub type HW_PATCH_LVL_R = crate::R<u8, u8>;
#[doc = "Reader of field `EIP_NUM_COMPL`"]
pub type EIP_NUM_COMPL_R = crate::R<u8, u8>;
#[doc = "Reader of field `EIP_NUM`"]
pub type EIP_NUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:27 - HW_MAJOR_VER"]
    #[inline(always)]
    pub fn hw_major_ver(&self) -> HW_MAJOR_VER_R {
        HW_MAJOR_VER_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - HW_MINOR_VER"]
    #[inline(always)]
    pub fn hw_minor_ver(&self) -> HW_MINOR_VER_R {
        HW_MINOR_VER_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - HW_PATCH_LVL"]
    #[inline(always)]
    pub fn hw_patch_lvl(&self) -> HW_PATCH_LVL_R {
        HW_PATCH_LVL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - EIP_NUM_COMPL"]
    #[inline(always)]
    pub fn eip_num_compl(&self) -> EIP_NUM_COMPL_R {
        EIP_NUM_COMPL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - EIP_NUM"]
    #[inline(always)]
    pub fn eip_num(&self) -> EIP_NUM_R {
        EIP_NUM_R::new((self.bits & 0xff) as u8)
    }
}

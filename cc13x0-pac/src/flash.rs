#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 28usize],
    #[doc = "0x1c - FMC and Efuse Status"]
    pub stat: STAT,
    _reserved1: [u8; 4usize],
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    pub cfg: CFG,
    #[doc = "0x28 - Internal. Only to be used through TI provided API."]
    pub syscode_start: SYSCODE_START,
    #[doc = "0x2c - Internal. Only to be used through TI provided API."]
    pub flash_size: FLASH_SIZE,
    _reserved4: [u8; 12usize],
    #[doc = "0x3c - Internal. Only to be used through TI provided API."]
    pub fwlock: FWLOCK,
    #[doc = "0x40 - Internal. Only to be used through TI provided API."]
    pub fwflag: FWFLAG,
    _reserved6: [u8; 4028usize],
    #[doc = "0x1000 - Internal. Only to be used through TI provided API."]
    pub efuse: EFUSE,
    #[doc = "0x1004 - Internal. Only to be used through TI provided API."]
    pub efuseaddr: EFUSEADDR,
    #[doc = "0x1008 - Internal. Only to be used through TI provided API."]
    pub dataupper: DATAUPPER,
    #[doc = "0x100c - Internal. Only to be used through TI provided API."]
    pub datalower: DATALOWER,
    #[doc = "0x1010 - Internal. Only to be used through TI provided API."]
    pub efusecfg: EFUSECFG,
    #[doc = "0x1014 - Internal. Only to be used through TI provided API."]
    pub efusestat: EFUSESTAT,
    #[doc = "0x1018 - Internal. Only to be used through TI provided API."]
    pub acc: ACC,
    #[doc = "0x101c - Internal. Only to be used through TI provided API."]
    pub boundary: BOUNDARY,
    #[doc = "0x1020 - Internal. Only to be used through TI provided API."]
    pub efuseflag: EFUSEFLAG,
    #[doc = "0x1024 - Internal. Only to be used through TI provided API."]
    pub efusekey: EFUSEKEY,
    #[doc = "0x1028 - Internal. Only to be used through TI provided API."]
    pub efuserelease: EFUSERELEASE,
    #[doc = "0x102c - Internal. Only to be used through TI provided API."]
    pub efusepins: EFUSEPINS,
    #[doc = "0x1030 - Internal. Only to be used through TI provided API."]
    pub efusecra: EFUSECRA,
    #[doc = "0x1034 - Internal. Only to be used through TI provided API."]
    pub efuseread: EFUSEREAD,
    #[doc = "0x1038 - Internal. Only to be used through TI provided API."]
    pub efuseprogram: EFUSEPROGRAM,
    #[doc = "0x103c - Internal. Only to be used through TI provided API."]
    pub efuseerror: EFUSEERROR,
    #[doc = "0x1040 - Internal. Only to be used through TI provided API."]
    pub singlebit: SINGLEBIT,
    #[doc = "0x1044 - Internal. Only to be used through TI provided API."]
    pub twobit: TWOBIT,
    #[doc = "0x1048 - Internal. Only to be used through TI provided API."]
    pub selftestcyc: SELFTESTCYC,
    #[doc = "0x104c - Internal. Only to be used through TI provided API."]
    pub selftestsign: SELFTESTSIGN,
    _reserved26: [u8; 4016usize],
    #[doc = "0x2000 - Internal. Only to be used through TI provided API."]
    pub frdctl: FRDCTL,
    #[doc = "0x2004 - Internal. Only to be used through TI provided API."]
    pub fsprd: FSPRD,
    #[doc = "0x2008 - Internal. Only to be used through TI provided API."]
    pub fedacctl1: FEDACCTL1,
    _reserved29: [u8; 16usize],
    #[doc = "0x201c - Internal. Only to be used through TI provided API."]
    pub fedacstat: FEDACSTAT,
    _reserved30: [u8; 16usize],
    #[doc = "0x2030 - Internal. Only to be used through TI provided API."]
    pub fbprot: FBPROT,
    #[doc = "0x2034 - Internal. Only to be used through TI provided API."]
    pub fbse: FBSE,
    #[doc = "0x2038 - Internal. Only to be used through TI provided API."]
    pub fbbusy: FBBUSY,
    #[doc = "0x203c - Internal. Only to be used through TI provided API."]
    pub fbac: FBAC,
    #[doc = "0x2040 - Internal. Only to be used through TI provided API."]
    pub fbfallback: FBFALLBACK,
    #[doc = "0x2044 - Internal. Only to be used through TI provided API."]
    pub fbprdy: FBPRDY,
    #[doc = "0x2048 - Internal. Only to be used through TI provided API."]
    pub fpac1: FPAC1,
    #[doc = "0x204c - Internal. Only to be used through TI provided API."]
    pub fpac2: FPAC2,
    #[doc = "0x2050 - Internal. Only to be used through TI provided API."]
    pub fmac: FMAC,
    #[doc = "0x2054 - Internal. Only to be used through TI provided API."]
    pub fmstat: FMSTAT,
    _reserved40: [u8; 12usize],
    #[doc = "0x2064 - Internal. Only to be used through TI provided API."]
    pub flock: FLOCK,
    _reserved41: [u8; 24usize],
    #[doc = "0x2080 - Internal. Only to be used through TI provided API."]
    pub fvreadct: FVREADCT,
    #[doc = "0x2084 - Internal. Only to be used through TI provided API."]
    pub fvhvct1: FVHVCT1,
    #[doc = "0x2088 - Internal. Only to be used through TI provided API."]
    pub fvhvct2: FVHVCT2,
    #[doc = "0x208c - Internal. Only to be used through TI provided API."]
    pub fvhvct3: FVHVCT3,
    #[doc = "0x2090 - Internal. Only to be used through TI provided API."]
    pub fvnvct: FVNVCT,
    #[doc = "0x2094 - Internal. Only to be used through TI provided API."]
    pub fvslp: FVSLP,
    #[doc = "0x2098 - Internal. Only to be used through TI provided API."]
    pub fvwlct: FVWLCT,
    #[doc = "0x209c - Internal. Only to be used through TI provided API."]
    pub fefusectl: FEFUSECTL,
    #[doc = "0x20a0 - Internal. Only to be used through TI provided API."]
    pub fefusestat: FEFUSESTAT,
    #[doc = "0x20a4 - Internal. Only to be used through TI provided API."]
    pub fefusedata: FEFUSEDATA,
    #[doc = "0x20a8 - Internal. Only to be used through TI provided API."]
    pub fseqpmp: FSEQPMP,
    _reserved52: [u8; 84usize],
    #[doc = "0x2100 - Internal. Only to be used through TI provided API."]
    pub fbstrobes: FBSTROBES,
    #[doc = "0x2104 - Internal. Only to be used through TI provided API."]
    pub fpstrobes: FPSTROBES,
    #[doc = "0x2108 - Internal. Only to be used through TI provided API."]
    pub fbmode: FBMODE,
    #[doc = "0x210c - Internal. Only to be used through TI provided API."]
    pub ftcr: FTCR,
    #[doc = "0x2110 - Internal. Only to be used through TI provided API."]
    pub faddr: FADDR,
    _reserved57: [u8; 8usize],
    #[doc = "0x211c - Internal. Only to be used through TI provided API."]
    pub ftctl: FTCTL,
    #[doc = "0x2120 - Internal. Only to be used through TI provided API."]
    pub fwpwrite0: FWPWRITE0,
    #[doc = "0x2124 - Internal. Only to be used through TI provided API."]
    pub fwpwrite1: FWPWRITE1,
    #[doc = "0x2128 - Internal. Only to be used through TI provided API."]
    pub fwpwrite2: FWPWRITE2,
    #[doc = "0x212c - Internal. Only to be used through TI provided API."]
    pub fwpwrite3: FWPWRITE3,
    #[doc = "0x2130 - Internal. Only to be used through TI provided API."]
    pub fwpwrite4: FWPWRITE4,
    #[doc = "0x2134 - Internal. Only to be used through TI provided API."]
    pub fwpwrite5: FWPWRITE5,
    #[doc = "0x2138 - Internal. Only to be used through TI provided API."]
    pub fwpwrite6: FWPWRITE6,
    #[doc = "0x213c - Internal. Only to be used through TI provided API."]
    pub fwpwrite7: FWPWRITE7,
    #[doc = "0x2140 - Internal. Only to be used through TI provided API."]
    pub fwpwrite_ecc: FWPWRITE_ECC,
    #[doc = "0x2144 - Internal. Only to be used through TI provided API."]
    pub fswstat: FSWSTAT,
    _reserved68: [u8; 184usize],
    #[doc = "0x2200 - Internal. Only to be used through TI provided API."]
    pub fsm_glbctl: FSM_GLBCTL,
    #[doc = "0x2204 - Internal. Only to be used through TI provided API."]
    pub fsm_state: FSM_STATE,
    #[doc = "0x2208 - Internal. Only to be used through TI provided API."]
    pub fsm_stat: FSM_STAT,
    #[doc = "0x220c - Internal. Only to be used through TI provided API."]
    pub fsm_cmd: FSM_CMD,
    #[doc = "0x2210 - Internal. Only to be used through TI provided API."]
    pub fsm_pe_osu: FSM_PE_OSU,
    #[doc = "0x2214 - Internal. Only to be used through TI provided API."]
    pub fsm_vstat: FSM_VSTAT,
    #[doc = "0x2218 - Internal. Only to be used through TI provided API."]
    pub fsm_pe_vsu: FSM_PE_VSU,
    #[doc = "0x221c - Internal. Only to be used through TI provided API."]
    pub fsm_cmp_vsu: FSM_CMP_VSU,
    #[doc = "0x2220 - Internal. Only to be used through TI provided API."]
    pub fsm_ex_val: FSM_EX_VAL,
    #[doc = "0x2224 - Internal. Only to be used through TI provided API."]
    pub fsm_rd_h: FSM_RD_H,
    #[doc = "0x2228 - Internal. Only to be used through TI provided API."]
    pub fsm_p_oh: FSM_P_OH,
    #[doc = "0x222c - Internal. Only to be used through TI provided API."]
    pub fsm_era_oh: FSM_ERA_OH,
    #[doc = "0x2230 - Internal. Only to be used through TI provided API."]
    pub fsm_sav_ppul: FSM_SAV_PPUL,
    #[doc = "0x2234 - Internal. Only to be used through TI provided API."]
    pub fsm_pe_vh: FSM_PE_VH,
    _reserved82: [u8; 8usize],
    #[doc = "0x2240 - Internal. Only to be used through TI provided API."]
    pub fsm_prg_pw: FSM_PRG_PW,
    #[doc = "0x2244 - Internal. Only to be used through TI provided API."]
    pub fsm_era_pw: FSM_ERA_PW,
    _reserved84: [u8; 12usize],
    #[doc = "0x2254 - Internal. Only to be used through TI provided API."]
    pub fsm_sav_era_pul: FSM_SAV_ERA_PUL,
    #[doc = "0x2258 - Internal. Only to be used through TI provided API."]
    pub fsm_timer: FSM_TIMER,
    #[doc = "0x225c - Internal. Only to be used through TI provided API."]
    pub fsm_mode: FSM_MODE,
    #[doc = "0x2260 - Internal. Only to be used through TI provided API."]
    pub fsm_pgm: FSM_PGM,
    #[doc = "0x2264 - Internal. Only to be used through TI provided API."]
    pub fsm_era: FSM_ERA,
    #[doc = "0x2268 - Internal. Only to be used through TI provided API."]
    pub fsm_prg_pul: FSM_PRG_PUL,
    #[doc = "0x226c - Internal. Only to be used through TI provided API."]
    pub fsm_era_pul: FSM_ERA_PUL,
    #[doc = "0x2270 - Internal. Only to be used through TI provided API."]
    pub fsm_step_size: FSM_STEP_SIZE,
    #[doc = "0x2274 - Internal. Only to be used through TI provided API."]
    pub fsm_pul_cntr: FSM_PUL_CNTR,
    #[doc = "0x2278 - Internal. Only to be used through TI provided API."]
    pub fsm_ec_step_height: FSM_EC_STEP_HEIGHT,
    #[doc = "0x227c - Internal. Only to be used through TI provided API."]
    pub fsm_st_machine: FSM_ST_MACHINE,
    #[doc = "0x2280 - Internal. Only to be used through TI provided API."]
    pub fsm_fles: FSM_FLES,
    _reserved96: [u8; 4usize],
    #[doc = "0x2288 - Internal. Only to be used through TI provided API."]
    pub fsm_wr_ena: FSM_WR_ENA,
    #[doc = "0x228c - Internal. Only to be used through TI provided API."]
    pub fsm_acc_pp: FSM_ACC_PP,
    #[doc = "0x2290 - Internal. Only to be used through TI provided API."]
    pub fsm_acc_ep: FSM_ACC_EP,
    _reserved99: [u8; 12usize],
    #[doc = "0x22a0 - Internal. Only to be used through TI provided API."]
    pub fsm_addr: FSM_ADDR,
    #[doc = "0x22a4 - Internal. Only to be used through TI provided API."]
    pub fsm_sector: FSM_SECTOR,
    #[doc = "0x22a8 - Internal. Only to be used through TI provided API."]
    pub fmc_rev_id: FMC_REV_ID,
    #[doc = "0x22ac - Internal. Only to be used through TI provided API."]
    pub fsm_err_addr: FSM_ERR_ADDR,
    #[doc = "0x22b0 - Internal. Only to be used through TI provided API."]
    pub fsm_pgm_maxpul: FSM_PGM_MAXPUL,
    #[doc = "0x22b4 - Internal. Only to be used through TI provided API."]
    pub fsm_execute: FSM_EXECUTE,
    _reserved105: [u8; 8usize],
    #[doc = "0x22c0 - Internal. Only to be used through TI provided API."]
    pub fsm_sector1: FSM_SECTOR1,
    #[doc = "0x22c4 - Internal. Only to be used through TI provided API."]
    pub fsm_sector2: FSM_SECTOR2,
    _reserved107: [u8; 24usize],
    #[doc = "0x22e0 - Internal. Only to be used through TI provided API."]
    pub fsm_bsle0: FSM_BSLE0,
    #[doc = "0x22e4 - Internal. Only to be used through TI provided API."]
    pub fsm_bsle1: FSM_BSLE1,
    _reserved109: [u8; 8usize],
    #[doc = "0x22f0 - Internal. Only to be used through TI provided API."]
    pub fsm_bslp0: FSM_BSLP0,
    #[doc = "0x22f4 - Internal. Only to be used through TI provided API."]
    pub fsm_bslp1: FSM_BSLP1,
    _reserved111: [u8; 264usize],
    #[doc = "0x2400 - Internal. Only to be used through TI provided API."]
    pub fcfg_bank: FCFG_BANK,
    #[doc = "0x2404 - Internal. Only to be used through TI provided API."]
    pub fcfg_wrapper: FCFG_WRAPPER,
    #[doc = "0x2408 - Internal. Only to be used through TI provided API."]
    pub fcfg_bnk_type: FCFG_BNK_TYPE,
    _reserved114: [u8; 4usize],
    #[doc = "0x2410 - Internal. Only to be used through TI provided API."]
    pub fcfg_b0_start: FCFG_B0_START,
    #[doc = "0x2414 - Internal. Only to be used through TI provided API."]
    pub fcfg_b1_start: FCFG_B1_START,
    #[doc = "0x2418 - Internal. Only to be used through TI provided API."]
    pub fcfg_b2_start: FCFG_B2_START,
    #[doc = "0x241c - Internal. Only to be used through TI provided API."]
    pub fcfg_b3_start: FCFG_B3_START,
    #[doc = "0x2420 - Internal. Only to be used through TI provided API."]
    pub fcfg_b4_start: FCFG_B4_START,
    #[doc = "0x2424 - Internal. Only to be used through TI provided API."]
    pub fcfg_b5_start: FCFG_B5_START,
    #[doc = "0x2428 - Internal. Only to be used through TI provided API."]
    pub fcfg_b6_start: FCFG_B6_START,
    #[doc = "0x242c - Internal. Only to be used through TI provided API."]
    pub fcfg_b7_start: FCFG_B7_START,
    #[doc = "0x2430 - Internal. Only to be used through TI provided API."]
    pub fcfg_b0_ssize0: FCFG_B0_SSIZE0,
}
#[doc = "FMC and Efuse Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "FMC and Efuse Status"]
pub mod stat;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cfg;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscode_start](syscode_start) module"]
pub type SYSCODE_START = crate::Reg<u32, _SYSCODE_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCODE_START;
#[doc = "`read()` method returns [syscode_start::R](syscode_start::R) reader structure"]
impl crate::Readable for SYSCODE_START {}
#[doc = "`write(|w| ..)` method takes [syscode_start::W](syscode_start::W) writer structure"]
impl crate::Writable for SYSCODE_START {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod syscode_start;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_size](flash_size) module"]
pub type FLASH_SIZE = crate::Reg<u32, _FLASH_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASH_SIZE;
#[doc = "`read()` method returns [flash_size::R](flash_size::R) reader structure"]
impl crate::Readable for FLASH_SIZE {}
#[doc = "`write(|w| ..)` method takes [flash_size::W](flash_size::W) writer structure"]
impl crate::Writable for FLASH_SIZE {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flash_size;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwlock](fwlock) module"]
pub type FWLOCK = crate::Reg<u32, _FWLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWLOCK;
#[doc = "`read()` method returns [fwlock::R](fwlock::R) reader structure"]
impl crate::Readable for FWLOCK {}
#[doc = "`write(|w| ..)` method takes [fwlock::W](fwlock::W) writer structure"]
impl crate::Writable for FWLOCK {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwlock;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwflag](fwflag) module"]
pub type FWFLAG = crate::Reg<u32, _FWFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWFLAG;
#[doc = "`read()` method returns [fwflag::R](fwflag::R) reader structure"]
impl crate::Readable for FWFLAG {}
#[doc = "`write(|w| ..)` method takes [fwflag::W](fwflag::W) writer structure"]
impl crate::Writable for FWFLAG {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwflag;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse](efuse) module"]
pub type EFUSE = crate::Reg<u32, _EFUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSE;
#[doc = "`write(|w| ..)` method takes [efuse::W](efuse::W) writer structure"]
impl crate::Writable for EFUSE {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuse;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuseaddr](efuseaddr) module"]
pub type EFUSEADDR = crate::Reg<u32, _EFUSEADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSEADDR;
#[doc = "`write(|w| ..)` method takes [efuseaddr::W](efuseaddr::W) writer structure"]
impl crate::Writable for EFUSEADDR {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseaddr;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dataupper](dataupper) module"]
pub type DATAUPPER = crate::Reg<u32, _DATAUPPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATAUPPER;
#[doc = "`write(|w| ..)` method takes [dataupper::W](dataupper::W) writer structure"]
impl crate::Writable for DATAUPPER {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod dataupper;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datalower](datalower) module"]
pub type DATALOWER = crate::Reg<u32, _DATALOWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATALOWER;
#[doc = "`write(|w| ..)` method takes [datalower::W](datalower::W) writer structure"]
impl crate::Writable for DATALOWER {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod datalower;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efusecfg](efusecfg) module"]
pub type EFUSECFG = crate::Reg<u32, _EFUSECFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSECFG;
#[doc = "`write(|w| ..)` method takes [efusecfg::W](efusecfg::W) writer structure"]
impl crate::Writable for EFUSECFG {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusecfg;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efusestat](efusestat) module"]
pub type EFUSESTAT = crate::Reg<u32, _EFUSESTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSESTAT;
#[doc = "`write(|w| ..)` method takes [efusestat::W](efusestat::W) writer structure"]
impl crate::Writable for EFUSESTAT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusestat;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc](acc) module"]
pub type ACC = crate::Reg<u32, _ACC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC;
#[doc = "`write(|w| ..)` method takes [acc::W](acc::W) writer structure"]
impl crate::Writable for ACC {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod acc;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boundary](boundary) module"]
pub type BOUNDARY = crate::Reg<u32, _BOUNDARY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOUNDARY;
#[doc = "`write(|w| ..)` method takes [boundary::W](boundary::W) writer structure"]
impl crate::Writable for BOUNDARY {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod boundary;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuseflag](efuseflag) module"]
pub type EFUSEFLAG = crate::Reg<u32, _EFUSEFLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSEFLAG;
#[doc = "`write(|w| ..)` method takes [efuseflag::W](efuseflag::W) writer structure"]
impl crate::Writable for EFUSEFLAG {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseflag;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efusekey](efusekey) module"]
pub type EFUSEKEY = crate::Reg<u32, _EFUSEKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSEKEY;
#[doc = "`write(|w| ..)` method takes [efusekey::W](efusekey::W) writer structure"]
impl crate::Writable for EFUSEKEY {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusekey;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuserelease](efuserelease) module"]
pub type EFUSERELEASE = crate::Reg<u32, _EFUSERELEASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSERELEASE;
#[doc = "`write(|w| ..)` method takes [efuserelease::W](efuserelease::W) writer structure"]
impl crate::Writable for EFUSERELEASE {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuserelease;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efusepins](efusepins) module"]
pub type EFUSEPINS = crate::Reg<u32, _EFUSEPINS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSEPINS;
#[doc = "`write(|w| ..)` method takes [efusepins::W](efusepins::W) writer structure"]
impl crate::Writable for EFUSEPINS {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusepins;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efusecra](efusecra) module"]
pub type EFUSECRA = crate::Reg<u32, _EFUSECRA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSECRA;
#[doc = "`write(|w| ..)` method takes [efusecra::W](efusecra::W) writer structure"]
impl crate::Writable for EFUSECRA {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efusecra;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuseread](efuseread) module"]
pub type EFUSEREAD = crate::Reg<u32, _EFUSEREAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSEREAD;
#[doc = "`write(|w| ..)` method takes [efuseread::W](efuseread::W) writer structure"]
impl crate::Writable for EFUSEREAD {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseread;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuseprogram](efuseprogram) module"]
pub type EFUSEPROGRAM = crate::Reg<u32, _EFUSEPROGRAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSEPROGRAM;
#[doc = "`write(|w| ..)` method takes [efuseprogram::W](efuseprogram::W) writer structure"]
impl crate::Writable for EFUSEPROGRAM {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseprogram;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuseerror](efuseerror) module"]
pub type EFUSEERROR = crate::Reg<u32, _EFUSEERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EFUSEERROR;
#[doc = "`write(|w| ..)` method takes [efuseerror::W](efuseerror::W) writer structure"]
impl crate::Writable for EFUSEERROR {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod efuseerror;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singlebit](singlebit) module"]
pub type SINGLEBIT = crate::Reg<u32, _SINGLEBIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGLEBIT;
#[doc = "`write(|w| ..)` method takes [singlebit::W](singlebit::W) writer structure"]
impl crate::Writable for SINGLEBIT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod singlebit;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twobit](twobit) module"]
pub type TWOBIT = crate::Reg<u32, _TWOBIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWOBIT;
#[doc = "`write(|w| ..)` method takes [twobit::W](twobit::W) writer structure"]
impl crate::Writable for TWOBIT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod twobit;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [selftestcyc](selftestcyc) module"]
pub type SELFTESTCYC = crate::Reg<u32, _SELFTESTCYC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SELFTESTCYC;
#[doc = "`write(|w| ..)` method takes [selftestcyc::W](selftestcyc::W) writer structure"]
impl crate::Writable for SELFTESTCYC {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod selftestcyc;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [selftestsign](selftestsign) module"]
pub type SELFTESTSIGN = crate::Reg<u32, _SELFTESTSIGN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SELFTESTSIGN;
#[doc = "`write(|w| ..)` method takes [selftestsign::W](selftestsign::W) writer structure"]
impl crate::Writable for SELFTESTSIGN {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod selftestsign;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frdctl](frdctl) module"]
pub type FRDCTL = crate::Reg<u32, _FRDCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRDCTL;
#[doc = "`read()` method returns [frdctl::R](frdctl::R) reader structure"]
impl crate::Readable for FRDCTL {}
#[doc = "`write(|w| ..)` method takes [frdctl::W](frdctl::W) writer structure"]
impl crate::Writable for FRDCTL {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod frdctl;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsprd](fsprd) module"]
pub type FSPRD = crate::Reg<u32, _FSPRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSPRD;
#[doc = "`read()` method returns [fsprd::R](fsprd::R) reader structure"]
impl crate::Readable for FSPRD {}
#[doc = "`write(|w| ..)` method takes [fsprd::W](fsprd::W) writer structure"]
impl crate::Writable for FSPRD {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsprd;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fedacctl1](fedacctl1) module"]
pub type FEDACCTL1 = crate::Reg<u32, _FEDACCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FEDACCTL1;
#[doc = "`read()` method returns [fedacctl1::R](fedacctl1::R) reader structure"]
impl crate::Readable for FEDACCTL1 {}
#[doc = "`write(|w| ..)` method takes [fedacctl1::W](fedacctl1::W) writer structure"]
impl crate::Writable for FEDACCTL1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacctl1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fedacstat](fedacstat) module"]
pub type FEDACSTAT = crate::Reg<u32, _FEDACSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FEDACSTAT;
#[doc = "`read()` method returns [fedacstat::R](fedacstat::R) reader structure"]
impl crate::Readable for FEDACSTAT {}
#[doc = "`write(|w| ..)` method takes [fedacstat::W](fedacstat::W) writer structure"]
impl crate::Writable for FEDACSTAT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fedacstat;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbprot](fbprot) module"]
pub type FBPROT = crate::Reg<u32, _FBPROT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBPROT;
#[doc = "`read()` method returns [fbprot::R](fbprot::R) reader structure"]
impl crate::Readable for FBPROT {}
#[doc = "`write(|w| ..)` method takes [fbprot::W](fbprot::W) writer structure"]
impl crate::Writable for FBPROT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbprot;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbse](fbse) module"]
pub type FBSE = crate::Reg<u32, _FBSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBSE;
#[doc = "`read()` method returns [fbse::R](fbse::R) reader structure"]
impl crate::Readable for FBSE {}
#[doc = "`write(|w| ..)` method takes [fbse::W](fbse::W) writer structure"]
impl crate::Writable for FBSE {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbse;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbbusy](fbbusy) module"]
pub type FBBUSY = crate::Reg<u32, _FBBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBBUSY;
#[doc = "`read()` method returns [fbbusy::R](fbbusy::R) reader structure"]
impl crate::Readable for FBBUSY {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbbusy;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbac](fbac) module"]
pub type FBAC = crate::Reg<u32, _FBAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBAC;
#[doc = "`read()` method returns [fbac::R](fbac::R) reader structure"]
impl crate::Readable for FBAC {}
#[doc = "`write(|w| ..)` method takes [fbac::W](fbac::W) writer structure"]
impl crate::Writable for FBAC {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbac;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbfallback](fbfallback) module"]
pub type FBFALLBACK = crate::Reg<u32, _FBFALLBACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBFALLBACK;
#[doc = "`read()` method returns [fbfallback::R](fbfallback::R) reader structure"]
impl crate::Readable for FBFALLBACK {}
#[doc = "`write(|w| ..)` method takes [fbfallback::W](fbfallback::W) writer structure"]
impl crate::Writable for FBFALLBACK {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbfallback;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbprdy](fbprdy) module"]
pub type FBPRDY = crate::Reg<u32, _FBPRDY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBPRDY;
#[doc = "`read()` method returns [fbprdy::R](fbprdy::R) reader structure"]
impl crate::Readable for FBPRDY {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbprdy;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpac1](fpac1) module"]
pub type FPAC1 = crate::Reg<u32, _FPAC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPAC1;
#[doc = "`read()` method returns [fpac1::R](fpac1::R) reader structure"]
impl crate::Readable for FPAC1 {}
#[doc = "`write(|w| ..)` method takes [fpac1::W](fpac1::W) writer structure"]
impl crate::Writable for FPAC1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpac1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpac2](fpac2) module"]
pub type FPAC2 = crate::Reg<u32, _FPAC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPAC2;
#[doc = "`read()` method returns [fpac2::R](fpac2::R) reader structure"]
impl crate::Readable for FPAC2 {}
#[doc = "`write(|w| ..)` method takes [fpac2::W](fpac2::W) writer structure"]
impl crate::Writable for FPAC2 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpac2;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmac](fmac) module"]
pub type FMAC = crate::Reg<u32, _FMAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMAC;
#[doc = "`read()` method returns [fmac::R](fmac::R) reader structure"]
impl crate::Readable for FMAC {}
#[doc = "`write(|w| ..)` method takes [fmac::W](fmac::W) writer structure"]
impl crate::Writable for FMAC {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fmac;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmstat](fmstat) module"]
pub type FMSTAT = crate::Reg<u32, _FMSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMSTAT;
#[doc = "`read()` method returns [fmstat::R](fmstat::R) reader structure"]
impl crate::Readable for FMSTAT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fmstat;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flock](flock) module"]
pub type FLOCK = crate::Reg<u32, _FLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLOCK;
#[doc = "`read()` method returns [flock::R](flock::R) reader structure"]
impl crate::Readable for FLOCK {}
#[doc = "`write(|w| ..)` method takes [flock::W](flock::W) writer structure"]
impl crate::Writable for FLOCK {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flock;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvreadct](fvreadct) module"]
pub type FVREADCT = crate::Reg<u32, _FVREADCT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FVREADCT;
#[doc = "`read()` method returns [fvreadct::R](fvreadct::R) reader structure"]
impl crate::Readable for FVREADCT {}
#[doc = "`write(|w| ..)` method takes [fvreadct::W](fvreadct::W) writer structure"]
impl crate::Writable for FVREADCT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvreadct;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvhvct1](fvhvct1) module"]
pub type FVHVCT1 = crate::Reg<u32, _FVHVCT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FVHVCT1;
#[doc = "`read()` method returns [fvhvct1::R](fvhvct1::R) reader structure"]
impl crate::Readable for FVHVCT1 {}
#[doc = "`write(|w| ..)` method takes [fvhvct1::W](fvhvct1::W) writer structure"]
impl crate::Writable for FVHVCT1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvhvct1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvhvct2](fvhvct2) module"]
pub type FVHVCT2 = crate::Reg<u32, _FVHVCT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FVHVCT2;
#[doc = "`read()` method returns [fvhvct2::R](fvhvct2::R) reader structure"]
impl crate::Readable for FVHVCT2 {}
#[doc = "`write(|w| ..)` method takes [fvhvct2::W](fvhvct2::W) writer structure"]
impl crate::Writable for FVHVCT2 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvhvct2;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvhvct3](fvhvct3) module"]
pub type FVHVCT3 = crate::Reg<u32, _FVHVCT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FVHVCT3;
#[doc = "`read()` method returns [fvhvct3::R](fvhvct3::R) reader structure"]
impl crate::Readable for FVHVCT3 {}
#[doc = "`write(|w| ..)` method takes [fvhvct3::W](fvhvct3::W) writer structure"]
impl crate::Writable for FVHVCT3 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvhvct3;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvnvct](fvnvct) module"]
pub type FVNVCT = crate::Reg<u32, _FVNVCT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FVNVCT;
#[doc = "`read()` method returns [fvnvct::R](fvnvct::R) reader structure"]
impl crate::Readable for FVNVCT {}
#[doc = "`write(|w| ..)` method takes [fvnvct::W](fvnvct::W) writer structure"]
impl crate::Writable for FVNVCT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvnvct;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvslp](fvslp) module"]
pub type FVSLP = crate::Reg<u32, _FVSLP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FVSLP;
#[doc = "`read()` method returns [fvslp::R](fvslp::R) reader structure"]
impl crate::Readable for FVSLP {}
#[doc = "`write(|w| ..)` method takes [fvslp::W](fvslp::W) writer structure"]
impl crate::Writable for FVSLP {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvslp;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvwlct](fvwlct) module"]
pub type FVWLCT = crate::Reg<u32, _FVWLCT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FVWLCT;
#[doc = "`read()` method returns [fvwlct::R](fvwlct::R) reader structure"]
impl crate::Readable for FVWLCT {}
#[doc = "`write(|w| ..)` method takes [fvwlct::W](fvwlct::W) writer structure"]
impl crate::Writable for FVWLCT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fvwlct;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fefusectl](fefusectl) module"]
pub type FEFUSECTL = crate::Reg<u32, _FEFUSECTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FEFUSECTL;
#[doc = "`read()` method returns [fefusectl::R](fefusectl::R) reader structure"]
impl crate::Readable for FEFUSECTL {}
#[doc = "`write(|w| ..)` method takes [fefusectl::W](fefusectl::W) writer structure"]
impl crate::Writable for FEFUSECTL {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fefusectl;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fefusestat](fefusestat) module"]
pub type FEFUSESTAT = crate::Reg<u32, _FEFUSESTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FEFUSESTAT;
#[doc = "`read()` method returns [fefusestat::R](fefusestat::R) reader structure"]
impl crate::Readable for FEFUSESTAT {}
#[doc = "`write(|w| ..)` method takes [fefusestat::W](fefusestat::W) writer structure"]
impl crate::Writable for FEFUSESTAT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fefusestat;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fefusedata](fefusedata) module"]
pub type FEFUSEDATA = crate::Reg<u32, _FEFUSEDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FEFUSEDATA;
#[doc = "`read()` method returns [fefusedata::R](fefusedata::R) reader structure"]
impl crate::Readable for FEFUSEDATA {}
#[doc = "`write(|w| ..)` method takes [fefusedata::W](fefusedata::W) writer structure"]
impl crate::Writable for FEFUSEDATA {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fefusedata;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fseqpmp](fseqpmp) module"]
pub type FSEQPMP = crate::Reg<u32, _FSEQPMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSEQPMP;
#[doc = "`read()` method returns [fseqpmp::R](fseqpmp::R) reader structure"]
impl crate::Readable for FSEQPMP {}
#[doc = "`write(|w| ..)` method takes [fseqpmp::W](fseqpmp::W) writer structure"]
impl crate::Writable for FSEQPMP {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fseqpmp;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbstrobes](fbstrobes) module"]
pub type FBSTROBES = crate::Reg<u32, _FBSTROBES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBSTROBES;
#[doc = "`read()` method returns [fbstrobes::R](fbstrobes::R) reader structure"]
impl crate::Readable for FBSTROBES {}
#[doc = "`write(|w| ..)` method takes [fbstrobes::W](fbstrobes::W) writer structure"]
impl crate::Writable for FBSTROBES {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbstrobes;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpstrobes](fpstrobes) module"]
pub type FPSTROBES = crate::Reg<u32, _FPSTROBES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPSTROBES;
#[doc = "`read()` method returns [fpstrobes::R](fpstrobes::R) reader structure"]
impl crate::Readable for FPSTROBES {}
#[doc = "`write(|w| ..)` method takes [fpstrobes::W](fpstrobes::W) writer structure"]
impl crate::Writable for FPSTROBES {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fpstrobes;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbmode](fbmode) module"]
pub type FBMODE = crate::Reg<u32, _FBMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FBMODE;
#[doc = "`read()` method returns [fbmode::R](fbmode::R) reader structure"]
impl crate::Readable for FBMODE {}
#[doc = "`write(|w| ..)` method takes [fbmode::W](fbmode::W) writer structure"]
impl crate::Writable for FBMODE {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fbmode;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftcr](ftcr) module"]
pub type FTCR = crate::Reg<u32, _FTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTCR;
#[doc = "`read()` method returns [ftcr::R](ftcr::R) reader structure"]
impl crate::Readable for FTCR {}
#[doc = "`write(|w| ..)` method takes [ftcr::W](ftcr::W) writer structure"]
impl crate::Writable for FTCR {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ftcr;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faddr](faddr) module"]
pub type FADDR = crate::Reg<u32, _FADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FADDR;
#[doc = "`read()` method returns [faddr::R](faddr::R) reader structure"]
impl crate::Readable for FADDR {}
#[doc = "`write(|w| ..)` method takes [faddr::W](faddr::W) writer structure"]
impl crate::Writable for FADDR {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod faddr;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftctl](ftctl) module"]
pub type FTCTL = crate::Reg<u32, _FTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTCTL;
#[doc = "`read()` method returns [ftctl::R](ftctl::R) reader structure"]
impl crate::Readable for FTCTL {}
#[doc = "`write(|w| ..)` method takes [ftctl::W](ftctl::W) writer structure"]
impl crate::Writable for FTCTL {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ftctl;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpwrite0](fwpwrite0) module"]
pub type FWPWRITE0 = crate::Reg<u32, _FWPWRITE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWPWRITE0;
#[doc = "`read()` method returns [fwpwrite0::R](fwpwrite0::R) reader structure"]
impl crate::Readable for FWPWRITE0 {}
#[doc = "`write(|w| ..)` method takes [fwpwrite0::W](fwpwrite0::W) writer structure"]
impl crate::Writable for FWPWRITE0 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite0;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpwrite1](fwpwrite1) module"]
pub type FWPWRITE1 = crate::Reg<u32, _FWPWRITE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWPWRITE1;
#[doc = "`read()` method returns [fwpwrite1::R](fwpwrite1::R) reader structure"]
impl crate::Readable for FWPWRITE1 {}
#[doc = "`write(|w| ..)` method takes [fwpwrite1::W](fwpwrite1::W) writer structure"]
impl crate::Writable for FWPWRITE1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpwrite2](fwpwrite2) module"]
pub type FWPWRITE2 = crate::Reg<u32, _FWPWRITE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWPWRITE2;
#[doc = "`read()` method returns [fwpwrite2::R](fwpwrite2::R) reader structure"]
impl crate::Readable for FWPWRITE2 {}
#[doc = "`write(|w| ..)` method takes [fwpwrite2::W](fwpwrite2::W) writer structure"]
impl crate::Writable for FWPWRITE2 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite2;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpwrite3](fwpwrite3) module"]
pub type FWPWRITE3 = crate::Reg<u32, _FWPWRITE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWPWRITE3;
#[doc = "`read()` method returns [fwpwrite3::R](fwpwrite3::R) reader structure"]
impl crate::Readable for FWPWRITE3 {}
#[doc = "`write(|w| ..)` method takes [fwpwrite3::W](fwpwrite3::W) writer structure"]
impl crate::Writable for FWPWRITE3 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite3;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpwrite4](fwpwrite4) module"]
pub type FWPWRITE4 = crate::Reg<u32, _FWPWRITE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWPWRITE4;
#[doc = "`read()` method returns [fwpwrite4::R](fwpwrite4::R) reader structure"]
impl crate::Readable for FWPWRITE4 {}
#[doc = "`write(|w| ..)` method takes [fwpwrite4::W](fwpwrite4::W) writer structure"]
impl crate::Writable for FWPWRITE4 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite4;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpwrite5](fwpwrite5) module"]
pub type FWPWRITE5 = crate::Reg<u32, _FWPWRITE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWPWRITE5;
#[doc = "`read()` method returns [fwpwrite5::R](fwpwrite5::R) reader structure"]
impl crate::Readable for FWPWRITE5 {}
#[doc = "`write(|w| ..)` method takes [fwpwrite5::W](fwpwrite5::W) writer structure"]
impl crate::Writable for FWPWRITE5 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite5;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpwrite6](fwpwrite6) module"]
pub type FWPWRITE6 = crate::Reg<u32, _FWPWRITE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWPWRITE6;
#[doc = "`read()` method returns [fwpwrite6::R](fwpwrite6::R) reader structure"]
impl crate::Readable for FWPWRITE6 {}
#[doc = "`write(|w| ..)` method takes [fwpwrite6::W](fwpwrite6::W) writer structure"]
impl crate::Writable for FWPWRITE6 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite6;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpwrite7](fwpwrite7) module"]
pub type FWPWRITE7 = crate::Reg<u32, _FWPWRITE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWPWRITE7;
#[doc = "`read()` method returns [fwpwrite7::R](fwpwrite7::R) reader structure"]
impl crate::Readable for FWPWRITE7 {}
#[doc = "`write(|w| ..)` method takes [fwpwrite7::W](fwpwrite7::W) writer structure"]
impl crate::Writable for FWPWRITE7 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite7;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpwrite_ecc](fwpwrite_ecc) module"]
pub type FWPWRITE_ECC = crate::Reg<u32, _FWPWRITE_ECC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWPWRITE_ECC;
#[doc = "`read()` method returns [fwpwrite_ecc::R](fwpwrite_ecc::R) reader structure"]
impl crate::Readable for FWPWRITE_ECC {}
#[doc = "`write(|w| ..)` method takes [fwpwrite_ecc::W](fwpwrite_ecc::W) writer structure"]
impl crate::Writable for FWPWRITE_ECC {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fwpwrite_ecc;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fswstat](fswstat) module"]
pub type FSWSTAT = crate::Reg<u32, _FSWSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSWSTAT;
#[doc = "`read()` method returns [fswstat::R](fswstat::R) reader structure"]
impl crate::Readable for FSWSTAT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fswstat;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_glbctl](fsm_glbctl) module"]
pub type FSM_GLBCTL = crate::Reg<u32, _FSM_GLBCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_GLBCTL;
#[doc = "`read()` method returns [fsm_glbctl::R](fsm_glbctl::R) reader structure"]
impl crate::Readable for FSM_GLBCTL {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_glbctl;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_state](fsm_state) module"]
pub type FSM_STATE = crate::Reg<u32, _FSM_STATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_STATE;
#[doc = "`read()` method returns [fsm_state::R](fsm_state::R) reader structure"]
impl crate::Readable for FSM_STATE {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_state;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_stat](fsm_stat) module"]
pub type FSM_STAT = crate::Reg<u32, _FSM_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_STAT;
#[doc = "`read()` method returns [fsm_stat::R](fsm_stat::R) reader structure"]
impl crate::Readable for FSM_STAT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_stat;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_cmd](fsm_cmd) module"]
pub type FSM_CMD = crate::Reg<u32, _FSM_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_CMD;
#[doc = "`read()` method returns [fsm_cmd::R](fsm_cmd::R) reader structure"]
impl crate::Readable for FSM_CMD {}
#[doc = "`write(|w| ..)` method takes [fsm_cmd::W](fsm_cmd::W) writer structure"]
impl crate::Writable for FSM_CMD {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_cmd;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_pe_osu](fsm_pe_osu) module"]
pub type FSM_PE_OSU = crate::Reg<u32, _FSM_PE_OSU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_PE_OSU;
#[doc = "`read()` method returns [fsm_pe_osu::R](fsm_pe_osu::R) reader structure"]
impl crate::Readable for FSM_PE_OSU {}
#[doc = "`write(|w| ..)` method takes [fsm_pe_osu::W](fsm_pe_osu::W) writer structure"]
impl crate::Writable for FSM_PE_OSU {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pe_osu;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_vstat](fsm_vstat) module"]
pub type FSM_VSTAT = crate::Reg<u32, _FSM_VSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_VSTAT;
#[doc = "`read()` method returns [fsm_vstat::R](fsm_vstat::R) reader structure"]
impl crate::Readable for FSM_VSTAT {}
#[doc = "`write(|w| ..)` method takes [fsm_vstat::W](fsm_vstat::W) writer structure"]
impl crate::Writable for FSM_VSTAT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_vstat;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_pe_vsu](fsm_pe_vsu) module"]
pub type FSM_PE_VSU = crate::Reg<u32, _FSM_PE_VSU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_PE_VSU;
#[doc = "`read()` method returns [fsm_pe_vsu::R](fsm_pe_vsu::R) reader structure"]
impl crate::Readable for FSM_PE_VSU {}
#[doc = "`write(|w| ..)` method takes [fsm_pe_vsu::W](fsm_pe_vsu::W) writer structure"]
impl crate::Writable for FSM_PE_VSU {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pe_vsu;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_cmp_vsu](fsm_cmp_vsu) module"]
pub type FSM_CMP_VSU = crate::Reg<u32, _FSM_CMP_VSU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_CMP_VSU;
#[doc = "`read()` method returns [fsm_cmp_vsu::R](fsm_cmp_vsu::R) reader structure"]
impl crate::Readable for FSM_CMP_VSU {}
#[doc = "`write(|w| ..)` method takes [fsm_cmp_vsu::W](fsm_cmp_vsu::W) writer structure"]
impl crate::Writable for FSM_CMP_VSU {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_cmp_vsu;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_ex_val](fsm_ex_val) module"]
pub type FSM_EX_VAL = crate::Reg<u32, _FSM_EX_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_EX_VAL;
#[doc = "`read()` method returns [fsm_ex_val::R](fsm_ex_val::R) reader structure"]
impl crate::Readable for FSM_EX_VAL {}
#[doc = "`write(|w| ..)` method takes [fsm_ex_val::W](fsm_ex_val::W) writer structure"]
impl crate::Writable for FSM_EX_VAL {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_ex_val;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_rd_h](fsm_rd_h) module"]
pub type FSM_RD_H = crate::Reg<u32, _FSM_RD_H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_RD_H;
#[doc = "`read()` method returns [fsm_rd_h::R](fsm_rd_h::R) reader structure"]
impl crate::Readable for FSM_RD_H {}
#[doc = "`write(|w| ..)` method takes [fsm_rd_h::W](fsm_rd_h::W) writer structure"]
impl crate::Writable for FSM_RD_H {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_rd_h;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_p_oh](fsm_p_oh) module"]
pub type FSM_P_OH = crate::Reg<u32, _FSM_P_OH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_P_OH;
#[doc = "`read()` method returns [fsm_p_oh::R](fsm_p_oh::R) reader structure"]
impl crate::Readable for FSM_P_OH {}
#[doc = "`write(|w| ..)` method takes [fsm_p_oh::W](fsm_p_oh::W) writer structure"]
impl crate::Writable for FSM_P_OH {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_p_oh;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_era_oh](fsm_era_oh) module"]
pub type FSM_ERA_OH = crate::Reg<u32, _FSM_ERA_OH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_ERA_OH;
#[doc = "`read()` method returns [fsm_era_oh::R](fsm_era_oh::R) reader structure"]
impl crate::Readable for FSM_ERA_OH {}
#[doc = "`write(|w| ..)` method takes [fsm_era_oh::W](fsm_era_oh::W) writer structure"]
impl crate::Writable for FSM_ERA_OH {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era_oh;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_sav_ppul](fsm_sav_ppul) module"]
pub type FSM_SAV_PPUL = crate::Reg<u32, _FSM_SAV_PPUL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_SAV_PPUL;
#[doc = "`read()` method returns [fsm_sav_ppul::R](fsm_sav_ppul::R) reader structure"]
impl crate::Readable for FSM_SAV_PPUL {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sav_ppul;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_pe_vh](fsm_pe_vh) module"]
pub type FSM_PE_VH = crate::Reg<u32, _FSM_PE_VH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_PE_VH;
#[doc = "`read()` method returns [fsm_pe_vh::R](fsm_pe_vh::R) reader structure"]
impl crate::Readable for FSM_PE_VH {}
#[doc = "`write(|w| ..)` method takes [fsm_pe_vh::W](fsm_pe_vh::W) writer structure"]
impl crate::Writable for FSM_PE_VH {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pe_vh;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_prg_pw](fsm_prg_pw) module"]
pub type FSM_PRG_PW = crate::Reg<u32, _FSM_PRG_PW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_PRG_PW;
#[doc = "`read()` method returns [fsm_prg_pw::R](fsm_prg_pw::R) reader structure"]
impl crate::Readable for FSM_PRG_PW {}
#[doc = "`write(|w| ..)` method takes [fsm_prg_pw::W](fsm_prg_pw::W) writer structure"]
impl crate::Writable for FSM_PRG_PW {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_prg_pw;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_era_pw](fsm_era_pw) module"]
pub type FSM_ERA_PW = crate::Reg<u32, _FSM_ERA_PW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_ERA_PW;
#[doc = "`read()` method returns [fsm_era_pw::R](fsm_era_pw::R) reader structure"]
impl crate::Readable for FSM_ERA_PW {}
#[doc = "`write(|w| ..)` method takes [fsm_era_pw::W](fsm_era_pw::W) writer structure"]
impl crate::Writable for FSM_ERA_PW {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era_pw;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_sav_era_pul](fsm_sav_era_pul) module"]
pub type FSM_SAV_ERA_PUL = crate::Reg<u32, _FSM_SAV_ERA_PUL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_SAV_ERA_PUL;
#[doc = "`read()` method returns [fsm_sav_era_pul::R](fsm_sav_era_pul::R) reader structure"]
impl crate::Readable for FSM_SAV_ERA_PUL {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sav_era_pul;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_timer](fsm_timer) module"]
pub type FSM_TIMER = crate::Reg<u32, _FSM_TIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_TIMER;
#[doc = "`read()` method returns [fsm_timer::R](fsm_timer::R) reader structure"]
impl crate::Readable for FSM_TIMER {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_timer;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_mode](fsm_mode) module"]
pub type FSM_MODE = crate::Reg<u32, _FSM_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_MODE;
#[doc = "`read()` method returns [fsm_mode::R](fsm_mode::R) reader structure"]
impl crate::Readable for FSM_MODE {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_mode;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_pgm](fsm_pgm) module"]
pub type FSM_PGM = crate::Reg<u32, _FSM_PGM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_PGM;
#[doc = "`read()` method returns [fsm_pgm::R](fsm_pgm::R) reader structure"]
impl crate::Readable for FSM_PGM {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pgm;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_era](fsm_era) module"]
pub type FSM_ERA = crate::Reg<u32, _FSM_ERA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_ERA;
#[doc = "`read()` method returns [fsm_era::R](fsm_era::R) reader structure"]
impl crate::Readable for FSM_ERA {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_prg_pul](fsm_prg_pul) module"]
pub type FSM_PRG_PUL = crate::Reg<u32, _FSM_PRG_PUL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_PRG_PUL;
#[doc = "`read()` method returns [fsm_prg_pul::R](fsm_prg_pul::R) reader structure"]
impl crate::Readable for FSM_PRG_PUL {}
#[doc = "`write(|w| ..)` method takes [fsm_prg_pul::W](fsm_prg_pul::W) writer structure"]
impl crate::Writable for FSM_PRG_PUL {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_prg_pul;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_era_pul](fsm_era_pul) module"]
pub type FSM_ERA_PUL = crate::Reg<u32, _FSM_ERA_PUL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_ERA_PUL;
#[doc = "`read()` method returns [fsm_era_pul::R](fsm_era_pul::R) reader structure"]
impl crate::Readable for FSM_ERA_PUL {}
#[doc = "`write(|w| ..)` method takes [fsm_era_pul::W](fsm_era_pul::W) writer structure"]
impl crate::Writable for FSM_ERA_PUL {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_era_pul;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_step_size](fsm_step_size) module"]
pub type FSM_STEP_SIZE = crate::Reg<u32, _FSM_STEP_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_STEP_SIZE;
#[doc = "`read()` method returns [fsm_step_size::R](fsm_step_size::R) reader structure"]
impl crate::Readable for FSM_STEP_SIZE {}
#[doc = "`write(|w| ..)` method takes [fsm_step_size::W](fsm_step_size::W) writer structure"]
impl crate::Writable for FSM_STEP_SIZE {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_step_size;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_pul_cntr](fsm_pul_cntr) module"]
pub type FSM_PUL_CNTR = crate::Reg<u32, _FSM_PUL_CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_PUL_CNTR;
#[doc = "`read()` method returns [fsm_pul_cntr::R](fsm_pul_cntr::R) reader structure"]
impl crate::Readable for FSM_PUL_CNTR {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pul_cntr;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_ec_step_height](fsm_ec_step_height) module"]
pub type FSM_EC_STEP_HEIGHT = crate::Reg<u32, _FSM_EC_STEP_HEIGHT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_EC_STEP_HEIGHT;
#[doc = "`read()` method returns [fsm_ec_step_height::R](fsm_ec_step_height::R) reader structure"]
impl crate::Readable for FSM_EC_STEP_HEIGHT {}
#[doc = "`write(|w| ..)` method takes [fsm_ec_step_height::W](fsm_ec_step_height::W) writer structure"]
impl crate::Writable for FSM_EC_STEP_HEIGHT {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_ec_step_height;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_st_machine](fsm_st_machine) module"]
pub type FSM_ST_MACHINE = crate::Reg<u32, _FSM_ST_MACHINE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_ST_MACHINE;
#[doc = "`read()` method returns [fsm_st_machine::R](fsm_st_machine::R) reader structure"]
impl crate::Readable for FSM_ST_MACHINE {}
#[doc = "`write(|w| ..)` method takes [fsm_st_machine::W](fsm_st_machine::W) writer structure"]
impl crate::Writable for FSM_ST_MACHINE {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_st_machine;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_fles](fsm_fles) module"]
pub type FSM_FLES = crate::Reg<u32, _FSM_FLES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_FLES;
#[doc = "`read()` method returns [fsm_fles::R](fsm_fles::R) reader structure"]
impl crate::Readable for FSM_FLES {}
#[doc = "`write(|w| ..)` method takes [fsm_fles::W](fsm_fles::W) writer structure"]
impl crate::Writable for FSM_FLES {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_fles;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_wr_ena](fsm_wr_ena) module"]
pub type FSM_WR_ENA = crate::Reg<u32, _FSM_WR_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_WR_ENA;
#[doc = "`read()` method returns [fsm_wr_ena::R](fsm_wr_ena::R) reader structure"]
impl crate::Readable for FSM_WR_ENA {}
#[doc = "`write(|w| ..)` method takes [fsm_wr_ena::W](fsm_wr_ena::W) writer structure"]
impl crate::Writable for FSM_WR_ENA {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_wr_ena;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_acc_pp](fsm_acc_pp) module"]
pub type FSM_ACC_PP = crate::Reg<u32, _FSM_ACC_PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_ACC_PP;
#[doc = "`read()` method returns [fsm_acc_pp::R](fsm_acc_pp::R) reader structure"]
impl crate::Readable for FSM_ACC_PP {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_acc_pp;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_acc_ep](fsm_acc_ep) module"]
pub type FSM_ACC_EP = crate::Reg<u32, _FSM_ACC_EP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_ACC_EP;
#[doc = "`read()` method returns [fsm_acc_ep::R](fsm_acc_ep::R) reader structure"]
impl crate::Readable for FSM_ACC_EP {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_acc_ep;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_addr](fsm_addr) module"]
pub type FSM_ADDR = crate::Reg<u32, _FSM_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_ADDR;
#[doc = "`read()` method returns [fsm_addr::R](fsm_addr::R) reader structure"]
impl crate::Readable for FSM_ADDR {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_addr;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_sector](fsm_sector) module"]
pub type FSM_SECTOR = crate::Reg<u32, _FSM_SECTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_SECTOR;
#[doc = "`read()` method returns [fsm_sector::R](fsm_sector::R) reader structure"]
impl crate::Readable for FSM_SECTOR {}
#[doc = "`write(|w| ..)` method takes [fsm_sector::W](fsm_sector::W) writer structure"]
impl crate::Writable for FSM_SECTOR {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sector;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_rev_id](fmc_rev_id) module"]
pub type FMC_REV_ID = crate::Reg<u32, _FMC_REV_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC_REV_ID;
#[doc = "`read()` method returns [fmc_rev_id::R](fmc_rev_id::R) reader structure"]
impl crate::Readable for FMC_REV_ID {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fmc_rev_id;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_err_addr](fsm_err_addr) module"]
pub type FSM_ERR_ADDR = crate::Reg<u32, _FSM_ERR_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_ERR_ADDR;
#[doc = "`read()` method returns [fsm_err_addr::R](fsm_err_addr::R) reader structure"]
impl crate::Readable for FSM_ERR_ADDR {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_err_addr;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_pgm_maxpul](fsm_pgm_maxpul) module"]
pub type FSM_PGM_MAXPUL = crate::Reg<u32, _FSM_PGM_MAXPUL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_PGM_MAXPUL;
#[doc = "`read()` method returns [fsm_pgm_maxpul::R](fsm_pgm_maxpul::R) reader structure"]
impl crate::Readable for FSM_PGM_MAXPUL {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_pgm_maxpul;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_execute](fsm_execute) module"]
pub type FSM_EXECUTE = crate::Reg<u32, _FSM_EXECUTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_EXECUTE;
#[doc = "`read()` method returns [fsm_execute::R](fsm_execute::R) reader structure"]
impl crate::Readable for FSM_EXECUTE {}
#[doc = "`write(|w| ..)` method takes [fsm_execute::W](fsm_execute::W) writer structure"]
impl crate::Writable for FSM_EXECUTE {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_execute;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_sector1](fsm_sector1) module"]
pub type FSM_SECTOR1 = crate::Reg<u32, _FSM_SECTOR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_SECTOR1;
#[doc = "`read()` method returns [fsm_sector1::R](fsm_sector1::R) reader structure"]
impl crate::Readable for FSM_SECTOR1 {}
#[doc = "`write(|w| ..)` method takes [fsm_sector1::W](fsm_sector1::W) writer structure"]
impl crate::Writable for FSM_SECTOR1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sector1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_sector2](fsm_sector2) module"]
pub type FSM_SECTOR2 = crate::Reg<u32, _FSM_SECTOR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_SECTOR2;
#[doc = "`read()` method returns [fsm_sector2::R](fsm_sector2::R) reader structure"]
impl crate::Readable for FSM_SECTOR2 {}
#[doc = "`write(|w| ..)` method takes [fsm_sector2::W](fsm_sector2::W) writer structure"]
impl crate::Writable for FSM_SECTOR2 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_sector2;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_bsle0](fsm_bsle0) module"]
pub type FSM_BSLE0 = crate::Reg<u32, _FSM_BSLE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_BSLE0;
#[doc = "`read()` method returns [fsm_bsle0::R](fsm_bsle0::R) reader structure"]
impl crate::Readable for FSM_BSLE0 {}
#[doc = "`write(|w| ..)` method takes [fsm_bsle0::W](fsm_bsle0::W) writer structure"]
impl crate::Writable for FSM_BSLE0 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bsle0;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_bsle1](fsm_bsle1) module"]
pub type FSM_BSLE1 = crate::Reg<u32, _FSM_BSLE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_BSLE1;
#[doc = "`read()` method returns [fsm_bsle1::R](fsm_bsle1::R) reader structure"]
impl crate::Readable for FSM_BSLE1 {}
#[doc = "`write(|w| ..)` method takes [fsm_bsle1::W](fsm_bsle1::W) writer structure"]
impl crate::Writable for FSM_BSLE1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bsle1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_bslp0](fsm_bslp0) module"]
pub type FSM_BSLP0 = crate::Reg<u32, _FSM_BSLP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_BSLP0;
#[doc = "`read()` method returns [fsm_bslp0::R](fsm_bslp0::R) reader structure"]
impl crate::Readable for FSM_BSLP0 {}
#[doc = "`write(|w| ..)` method takes [fsm_bslp0::W](fsm_bslp0::W) writer structure"]
impl crate::Writable for FSM_BSLP0 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bslp0;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_bslp1](fsm_bslp1) module"]
pub type FSM_BSLP1 = crate::Reg<u32, _FSM_BSLP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM_BSLP1;
#[doc = "`read()` method returns [fsm_bslp1::R](fsm_bslp1::R) reader structure"]
impl crate::Readable for FSM_BSLP1 {}
#[doc = "`write(|w| ..)` method takes [fsm_bslp1::W](fsm_bslp1::W) writer structure"]
impl crate::Writable for FSM_BSLP1 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fsm_bslp1;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_bank](fcfg_bank) module"]
pub type FCFG_BANK = crate::Reg<u32, _FCFG_BANK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG_BANK;
#[doc = "`read()` method returns [fcfg_bank::R](fcfg_bank::R) reader structure"]
impl crate::Readable for FCFG_BANK {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_bank;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_wrapper](fcfg_wrapper) module"]
pub type FCFG_WRAPPER = crate::Reg<u32, _FCFG_WRAPPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG_WRAPPER;
#[doc = "`read()` method returns [fcfg_wrapper::R](fcfg_wrapper::R) reader structure"]
impl crate::Readable for FCFG_WRAPPER {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_wrapper;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_bnk_type](fcfg_bnk_type) module"]
pub type FCFG_BNK_TYPE = crate::Reg<u32, _FCFG_BNK_TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG_BNK_TYPE;
#[doc = "`read()` method returns [fcfg_bnk_type::R](fcfg_bnk_type::R) reader structure"]
impl crate::Readable for FCFG_BNK_TYPE {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_bnk_type;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_b0_start](fcfg_b0_start) module"]
pub type FCFG_B0_START = crate::Reg<u32, _FCFG_B0_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG_B0_START;
#[doc = "`read()` method returns [fcfg_b0_start::R](fcfg_b0_start::R) reader structure"]
impl crate::Readable for FCFG_B0_START {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_start;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_b1_start](fcfg_b1_start) module"]
pub type FCFG_B1_START = crate::Reg<u32, _FCFG_B1_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG_B1_START;
#[doc = "`read()` method returns [fcfg_b1_start::R](fcfg_b1_start::R) reader structure"]
impl crate::Readable for FCFG_B1_START {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b1_start;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_b2_start](fcfg_b2_start) module"]
pub type FCFG_B2_START = crate::Reg<u32, _FCFG_B2_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG_B2_START;
#[doc = "`read()` method returns [fcfg_b2_start::R](fcfg_b2_start::R) reader structure"]
impl crate::Readable for FCFG_B2_START {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b2_start;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_b3_start](fcfg_b3_start) module"]
pub type FCFG_B3_START = crate::Reg<u32, _FCFG_B3_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG_B3_START;
#[doc = "`read()` method returns [fcfg_b3_start::R](fcfg_b3_start::R) reader structure"]
impl crate::Readable for FCFG_B3_START {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b3_start;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_b4_start](fcfg_b4_start) module"]
pub type FCFG_B4_START = crate::Reg<u32, _FCFG_B4_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG_B4_START;
#[doc = "`read()` method returns [fcfg_b4_start::R](fcfg_b4_start::R) reader structure"]
impl crate::Readable for FCFG_B4_START {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b4_start;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_b5_start](fcfg_b5_start) module"]
pub type FCFG_B5_START = crate::Reg<u32, _FCFG_B5_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG_B5_START;
#[doc = "`read()` method returns [fcfg_b5_start::R](fcfg_b5_start::R) reader structure"]
impl crate::Readable for FCFG_B5_START {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b5_start;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_b6_start](fcfg_b6_start) module"]
pub type FCFG_B6_START = crate::Reg<u32, _FCFG_B6_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG_B6_START;
#[doc = "`read()` method returns [fcfg_b6_start::R](fcfg_b6_start::R) reader structure"]
impl crate::Readable for FCFG_B6_START {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b6_start;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_b7_start](fcfg_b7_start) module"]
pub type FCFG_B7_START = crate::Reg<u32, _FCFG_B7_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG_B7_START;
#[doc = "`read()` method returns [fcfg_b7_start::R](fcfg_b7_start::R) reader structure"]
impl crate::Readable for FCFG_B7_START {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b7_start;
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_b0_ssize0](fcfg_b0_ssize0) module"]
pub type FCFG_B0_SSIZE0 = crate::Reg<u32, _FCFG_B0_SSIZE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCFG_B0_SSIZE0;
#[doc = "`read()` method returns [fcfg_b0_ssize0::R](fcfg_b0_ssize0::R) reader structure"]
impl crate::Readable for FCFG_B0_SSIZE0 {}
#[doc = "Internal. Only to be used through TI provided API."]
pub mod fcfg_b0_ssize0;

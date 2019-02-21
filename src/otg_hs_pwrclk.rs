#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power and clock gating control register"]
    pub otg_hs_pcgcr: OTG_HS_PCGCR,
}
#[doc = "Power and clock gating control register"]
pub struct OTG_HS_PCGCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power and clock gating control register"]
pub mod otg_hs_pcgcr;

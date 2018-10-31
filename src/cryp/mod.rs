#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - status register"]
    pub sr: SR,
    #[doc = "0x08 - data input register"]
    pub din: DIN,
    #[doc = "0x0c - data output register"]
    pub dout: DOUT,
    #[doc = "0x10 - DMA control register"]
    pub dmacr: DMACR,
    #[doc = "0x14 - interrupt mask set/clear register"]
    pub imscr: IMSCR,
    #[doc = "0x18 - raw interrupt status register"]
    pub risr: RISR,
    #[doc = "0x1c - masked interrupt status register"]
    pub misr: MISR,
    #[doc = "0x20 - key registers"]
    pub k0lr: K0LR,
    #[doc = "0x24 - key registers"]
    pub k0rr: K0RR,
    #[doc = "0x28 - key registers"]
    pub k1lr: K1LR,
    #[doc = "0x2c - key registers"]
    pub k1rr: K1RR,
    #[doc = "0x30 - key registers"]
    pub k2lr: K2LR,
    #[doc = "0x34 - key registers"]
    pub k2rr: K2RR,
    #[doc = "0x38 - key registers"]
    pub k3lr: K3LR,
    #[doc = "0x3c - key registers"]
    pub k3rr: K3RR,
    #[doc = "0x40 - initialization vector registers"]
    pub iv0lr: IV0LR,
    #[doc = "0x44 - initialization vector registers"]
    pub iv0rr: IV0RR,
    #[doc = "0x48 - initialization vector registers"]
    pub iv1lr: IV1LR,
    #[doc = "0x4c - initialization vector registers"]
    pub iv1rr: IV1RR,
    #[doc = "0x50 - context swap register"]
    pub csgcmccm0r: CSGCMCCM0R,
    #[doc = "0x54 - context swap register"]
    pub csgcmccm1r: CSGCMCCM1R,
    #[doc = "0x58 - context swap register"]
    pub csgcmccm2r: CSGCMCCM2R,
    #[doc = "0x5c - context swap register"]
    pub csgcmccm3r: CSGCMCCM3R,
    #[doc = "0x60 - context swap register"]
    pub csgcmccm4r: CSGCMCCM4R,
    #[doc = "0x64 - context swap register"]
    pub csgcmccm5r: CSGCMCCM5R,
    #[doc = "0x68 - context swap register"]
    pub csgcmccm6r: CSGCMCCM6R,
    #[doc = "0x6c - context swap register"]
    pub csgcmccm7r: CSGCMCCM7R,
    #[doc = "0x70 - context swap register"]
    pub csgcm0r: CSGCM0R,
    #[doc = "0x74 - context swap register"]
    pub csgcm1r: CSGCM1R,
    #[doc = "0x78 - context swap register"]
    pub csgcm2r: CSGCM2R,
    #[doc = "0x7c - context swap register"]
    pub csgcm3r: CSGCM3R,
    #[doc = "0x80 - context swap register"]
    pub csgcm4r: CSGCM4R,
    #[doc = "0x84 - context swap register"]
    pub csgcm5r: CSGCM5R,
    #[doc = "0x88 - context swap register"]
    pub csgcm6r: CSGCM6R,
    #[doc = "0x8c - context swap register"]
    pub csgcm7r: CSGCM7R,
}
#[doc = "control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "data input register"]
pub struct DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data input register"]
pub mod din;
#[doc = "data output register"]
pub struct DOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data output register"]
pub mod dout;
#[doc = "DMA control register"]
pub struct DMACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA control register"]
pub mod dmacr;
#[doc = "interrupt mask set/clear register"]
pub struct IMSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt mask set/clear register"]
pub mod imscr;
#[doc = "raw interrupt status register"]
pub struct RISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "raw interrupt status register"]
pub mod risr;
#[doc = "masked interrupt status register"]
pub struct MISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "masked interrupt status register"]
pub mod misr;
#[doc = "key registers"]
pub struct K0LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key registers"]
pub mod k0lr;
#[doc = "key registers"]
pub struct K0RR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key registers"]
pub mod k0rr;
#[doc = "key registers"]
pub struct K1LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key registers"]
pub mod k1lr;
#[doc = "key registers"]
pub struct K1RR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key registers"]
pub mod k1rr;
#[doc = "key registers"]
pub struct K2LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key registers"]
pub mod k2lr;
#[doc = "key registers"]
pub struct K2RR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key registers"]
pub mod k2rr;
#[doc = "key registers"]
pub struct K3LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key registers"]
pub mod k3lr;
#[doc = "key registers"]
pub struct K3RR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key registers"]
pub mod k3rr;
#[doc = "initialization vector registers"]
pub struct IV0LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector registers"]
pub mod iv0lr;
#[doc = "initialization vector registers"]
pub struct IV0RR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector registers"]
pub mod iv0rr;
#[doc = "initialization vector registers"]
pub struct IV1LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector registers"]
pub mod iv1lr;
#[doc = "initialization vector registers"]
pub struct IV1RR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector registers"]
pub mod iv1rr;
#[doc = "context swap register"]
pub struct CSGCMCCM0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcmccm0r;
#[doc = "context swap register"]
pub struct CSGCMCCM1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcmccm1r;
#[doc = "context swap register"]
pub struct CSGCMCCM2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcmccm2r;
#[doc = "context swap register"]
pub struct CSGCMCCM3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcmccm3r;
#[doc = "context swap register"]
pub struct CSGCMCCM4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcmccm4r;
#[doc = "context swap register"]
pub struct CSGCMCCM5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcmccm5r;
#[doc = "context swap register"]
pub struct CSGCMCCM6R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcmccm6r;
#[doc = "context swap register"]
pub struct CSGCMCCM7R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcmccm7r;
#[doc = "context swap register"]
pub struct CSGCM0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcm0r;
#[doc = "context swap register"]
pub struct CSGCM1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcm1r;
#[doc = "context swap register"]
pub struct CSGCM2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcm2r;
#[doc = "context swap register"]
pub struct CSGCM3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcm3r;
#[doc = "context swap register"]
pub struct CSGCM4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcm4r;
#[doc = "context swap register"]
pub struct CSGCM5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcm5r;
#[doc = "context swap register"]
pub struct CSGCM6R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcm6r;
#[doc = "context swap register"]
pub struct CSGCM7R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap register"]
pub mod csgcm7r;

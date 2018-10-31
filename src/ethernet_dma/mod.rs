#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet DMA bus mode register"]
    pub dmabmr: DMABMR,
    #[doc = "0x04 - Ethernet DMA transmit poll demand register"]
    pub dmatpdr: DMATPDR,
    #[doc = "0x08 - EHERNET DMA receive poll demand register"]
    pub dmarpdr: DMARPDR,
    #[doc = "0x0c - Ethernet DMA receive descriptor list address register"]
    pub dmardlar: DMARDLAR,
    #[doc = "0x10 - Ethernet DMA transmit descriptor list address register"]
    pub dmatdlar: DMATDLAR,
    #[doc = "0x14 - Ethernet DMA status register"]
    pub dmasr: DMASR,
    #[doc = "0x18 - Ethernet DMA operation mode register"]
    pub dmaomr: DMAOMR,
    #[doc = "0x1c - Ethernet DMA interrupt enable register"]
    pub dmaier: DMAIER,
    #[doc = "0x20 - Ethernet DMA missed frame and buffer overflow counter register"]
    pub dmamfbocr: DMAMFBOCR,
    #[doc = "0x24 - Ethernet DMA receive status watchdog timer register"]
    pub dmarswtr: DMARSWTR,
    _reserved0: [u8; 32usize],
    #[doc = "0x48 - Ethernet DMA current host transmit descriptor register"]
    pub dmachtdr: DMACHTDR,
    #[doc = "0x4c - Ethernet DMA current host receive descriptor register"]
    pub dmachrdr: DMACHRDR,
    #[doc = "0x50 - Ethernet DMA current host transmit buffer address register"]
    pub dmachtbar: DMACHTBAR,
    #[doc = "0x54 - Ethernet DMA current host receive buffer address register"]
    pub dmachrbar: DMACHRBAR,
}
#[doc = "Ethernet DMA bus mode register"]
pub struct DMABMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet DMA bus mode register"]
pub mod dmabmr;
#[doc = "Ethernet DMA transmit poll demand register"]
pub struct DMATPDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet DMA transmit poll demand register"]
pub mod dmatpdr;
#[doc = "EHERNET DMA receive poll demand register"]
pub struct DMARPDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EHERNET DMA receive poll demand register"]
pub mod dmarpdr;
#[doc = "Ethernet DMA receive descriptor list address register"]
pub struct DMARDLAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet DMA receive descriptor list address register"]
pub mod dmardlar;
#[doc = "Ethernet DMA transmit descriptor list address register"]
pub struct DMATDLAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet DMA transmit descriptor list address register"]
pub mod dmatdlar;
#[doc = "Ethernet DMA status register"]
pub struct DMASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet DMA status register"]
pub mod dmasr;
#[doc = "Ethernet DMA operation mode register"]
pub struct DMAOMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet DMA operation mode register"]
pub mod dmaomr;
#[doc = "Ethernet DMA interrupt enable register"]
pub struct DMAIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet DMA interrupt enable register"]
pub mod dmaier;
#[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
pub struct DMAMFBOCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet DMA missed frame and buffer overflow counter register"]
pub mod dmamfbocr;
#[doc = "Ethernet DMA receive status watchdog timer register"]
pub struct DMARSWTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet DMA receive status watchdog timer register"]
pub mod dmarswtr;
#[doc = "Ethernet DMA current host transmit descriptor register"]
pub struct DMACHTDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet DMA current host transmit descriptor register"]
pub mod dmachtdr;
#[doc = "Ethernet DMA current host receive descriptor register"]
pub struct DMACHRDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet DMA current host receive descriptor register"]
pub mod dmachrdr;
#[doc = "Ethernet DMA current host transmit buffer address register"]
pub struct DMACHTBAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet DMA current host transmit buffer address register"]
pub mod dmachtbar;
#[doc = "Ethernet DMA current host receive buffer address register"]
pub struct DMACHRBAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet DMA current host receive buffer address register"]
pub mod dmachrbar;

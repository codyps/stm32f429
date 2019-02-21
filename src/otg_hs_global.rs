#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS control and status register"]
    pub otg_hs_gotgctl: OTG_HS_GOTGCTL,
    #[doc = "0x04 - OTG_HS interrupt register"]
    pub otg_hs_gotgint: OTG_HS_GOTGINT,
    #[doc = "0x08 - OTG_HS AHB configuration register"]
    pub otg_hs_gahbcfg: OTG_HS_GAHBCFG,
    #[doc = "0x0c - OTG_HS USB configuration register"]
    pub otg_hs_gusbcfg: OTG_HS_GUSBCFG,
    #[doc = "0x10 - OTG_HS reset register"]
    pub otg_hs_grstctl: OTG_HS_GRSTCTL,
    #[doc = "0x14 - OTG_HS core interrupt register"]
    pub otg_hs_gintsts: OTG_HS_GINTSTS,
    #[doc = "0x18 - OTG_HS interrupt mask register"]
    pub otg_hs_gintmsk: OTG_HS_GINTMSK,
    #[doc = "0x1c - OTG_HS Receive status debug read register (host mode)"]
    pub otg_hs_grxstsr_host: OTG_HS_GRXSTSR_HOST,
    #[doc = "0x20 - OTG_HS status read and pop register (host mode)"]
    pub otg_hs_grxstsp_host: OTG_HS_GRXSTSP_HOST,
    #[doc = "0x24 - OTG_HS Receive FIFO size register"]
    pub otg_hs_grxfsiz: OTG_HS_GRXFSIZ,
    #[doc = "0x28 - OTG_HS nonperiodic transmit FIFO size register (host mode)"]
    pub otg_hs_gnptxfsiz_host: OTG_HS_GNPTXFSIZ_HOST,
    #[doc = "0x2c - OTG_HS nonperiodic transmit FIFO/queue status register"]
    pub otg_hs_gnptxsts: OTG_HS_GNPTXSTS,
    _reserved0: [u8; 8usize],
    #[doc = "0x38 - OTG_HS general core configuration register"]
    pub otg_hs_gccfg: OTG_HS_GCCFG,
    #[doc = "0x3c - OTG_HS core ID register"]
    pub otg_hs_cid: OTG_HS_CID,
    _reserved1: [u8; 192usize],
    #[doc = "0x100 - OTG_HS Host periodic transmit FIFO size register"]
    pub otg_hs_hptxfsiz: OTG_HS_HPTXFSIZ,
    #[doc = "0x104 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf1: OTG_HS_DIEPTXF1,
    #[doc = "0x108 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf2: OTG_HS_DIEPTXF2,
    _reserved2: [u8; 16usize],
    #[doc = "0x11c - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf3: OTG_HS_DIEPTXF3,
    #[doc = "0x120 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf4: OTG_HS_DIEPTXF4,
    #[doc = "0x124 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf5: OTG_HS_DIEPTXF5,
    #[doc = "0x128 - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf6: OTG_HS_DIEPTXF6,
    #[doc = "0x12c - OTG_HS device IN endpoint transmit FIFO size register"]
    pub otg_hs_dieptxf7: OTG_HS_DIEPTXF7,
}
#[doc = "OTG_HS control and status register"]
pub struct OTG_HS_GOTGCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS control and status register"]
pub mod otg_hs_gotgctl;
#[doc = "OTG_HS interrupt register"]
pub struct OTG_HS_GOTGINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS interrupt register"]
pub mod otg_hs_gotgint;
#[doc = "OTG_HS AHB configuration register"]
pub struct OTG_HS_GAHBCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS AHB configuration register"]
pub mod otg_hs_gahbcfg;
#[doc = "OTG_HS USB configuration register"]
pub struct OTG_HS_GUSBCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS USB configuration register"]
pub mod otg_hs_gusbcfg;
#[doc = "OTG_HS reset register"]
pub struct OTG_HS_GRSTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS reset register"]
pub mod otg_hs_grstctl;
#[doc = "OTG_HS core interrupt register"]
pub struct OTG_HS_GINTSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS core interrupt register"]
pub mod otg_hs_gintsts;
#[doc = "OTG_HS interrupt mask register"]
pub struct OTG_HS_GINTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS interrupt mask register"]
pub mod otg_hs_gintmsk;
#[doc = "OTG_HS Receive status debug read register (host mode)"]
pub struct OTG_HS_GRXSTSR_HOST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS Receive status debug read register (host mode)"]
pub mod otg_hs_grxstsr_host;
#[doc = "OTG_HS status read and pop register (host mode)"]
pub struct OTG_HS_GRXSTSP_HOST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS status read and pop register (host mode)"]
pub mod otg_hs_grxstsp_host;
#[doc = "OTG_HS Receive FIFO size register"]
pub struct OTG_HS_GRXFSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS Receive FIFO size register"]
pub mod otg_hs_grxfsiz;
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)"]
pub struct OTG_HS_GNPTXFSIZ_HOST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)"]
pub mod otg_hs_gnptxfsiz_host;
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)"]
pub struct OTG_HS_TX0FSIZ_PERIPHERAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)"]
pub mod otg_hs_tx0fsiz_peripheral;
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register"]
pub struct OTG_HS_GNPTXSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register"]
pub mod otg_hs_gnptxsts;
#[doc = "OTG_HS general core configuration register"]
pub struct OTG_HS_GCCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS general core configuration register"]
pub mod otg_hs_gccfg;
#[doc = "OTG_HS core ID register"]
pub struct OTG_HS_CID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS core ID register"]
pub mod otg_hs_cid;
#[doc = "OTG_HS Host periodic transmit FIFO size register"]
pub struct OTG_HS_HPTXFSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS Host periodic transmit FIFO size register"]
pub mod otg_hs_hptxfsiz;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub struct OTG_HS_DIEPTXF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf1;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub struct OTG_HS_DIEPTXF2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf2;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub struct OTG_HS_DIEPTXF3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf3;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub struct OTG_HS_DIEPTXF4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf4;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub struct OTG_HS_DIEPTXF5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf5;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub struct OTG_HS_DIEPTXF6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf6;
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub struct OTG_HS_DIEPTXF7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register"]
pub mod otg_hs_dieptxf7;
#[doc = "OTG_HS Receive status debug read register (peripheral mode mode)"]
pub struct OTG_HS_GRXSTSR_PERIPHERAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS Receive status debug read register (peripheral mode mode)"]
pub mod otg_hs_grxstsr_peripheral;
#[doc = "OTG_HS status read and pop register (peripheral mode)"]
pub struct OTG_HS_GRXSTSP_PERIPHERAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS status read and pop register (peripheral mode)"]
pub mod otg_hs_grxstsp_peripheral;

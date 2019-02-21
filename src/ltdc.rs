#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - Synchronization Size Configuration Register"]
    pub sscr: SSCR,
    #[doc = "0x0c - Back Porch Configuration Register"]
    pub bpcr: BPCR,
    #[doc = "0x10 - Active Width Configuration Register"]
    pub awcr: AWCR,
    #[doc = "0x14 - Total Width Configuration Register"]
    pub twcr: TWCR,
    #[doc = "0x18 - Global Control Register"]
    pub gcr: GCR,
    _reserved1: [u8; 8usize],
    #[doc = "0x24 - Shadow Reload Configuration Register"]
    pub srcr: SRCR,
    _reserved2: [u8; 4usize],
    #[doc = "0x2c - Background Color Configuration Register"]
    pub bccr: BCCR,
    _reserved3: [u8; 4usize],
    #[doc = "0x34 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x38 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x3c - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x40 - Line Interrupt Position Configuration Register"]
    pub lipcr: LIPCR,
    #[doc = "0x44 - Current Position Status Register"]
    pub cpsr: CPSR,
    #[doc = "0x48 - Current Display Status Register"]
    pub cdsr: CDSR,
    _reserved4: [u8; 56usize],
    #[doc = "0x84 - Layerx Control Register"]
    pub l1cr: L1CR,
    #[doc = "0x88 - Layerx Window Horizontal Position Configuration Register"]
    pub l1whpcr: L1WHPCR,
    #[doc = "0x8c - Layerx Window Vertical Position Configuration Register"]
    pub l1wvpcr: L1WVPCR,
    #[doc = "0x90 - Layerx Color Keying Configuration Register"]
    pub l1ckcr: L1CKCR,
    #[doc = "0x94 - Layerx Pixel Format Configuration Register"]
    pub l1pfcr: L1PFCR,
    #[doc = "0x98 - Layerx Constant Alpha Configuration Register"]
    pub l1cacr: L1CACR,
    #[doc = "0x9c - Layerx Default Color Configuration Register"]
    pub l1dccr: L1DCCR,
    #[doc = "0xa0 - Layerx Blending Factors Configuration Register"]
    pub l1bfcr: L1BFCR,
    _reserved5: [u8; 8usize],
    #[doc = "0xac - Layerx Color Frame Buffer Address Register"]
    pub l1cfbar: L1CFBAR,
    #[doc = "0xb0 - Layerx Color Frame Buffer Length Register"]
    pub l1cfblr: L1CFBLR,
    #[doc = "0xb4 - Layerx ColorFrame Buffer Line Number Register"]
    pub l1cfblnr: L1CFBLNR,
    _reserved6: [u8; 12usize],
    #[doc = "0xc4 - Layerx CLUT Write Register"]
    pub l1clutwr: L1CLUTWR,
    _reserved7: [u8; 60usize],
    #[doc = "0x104 - Layerx Control Register"]
    pub l2cr: L2CR,
    #[doc = "0x108 - Layerx Window Horizontal Position Configuration Register"]
    pub l2whpcr: L2WHPCR,
    #[doc = "0x10c - Layerx Window Vertical Position Configuration Register"]
    pub l2wvpcr: L2WVPCR,
    #[doc = "0x110 - Layerx Color Keying Configuration Register"]
    pub l2ckcr: L2CKCR,
    #[doc = "0x114 - Layerx Pixel Format Configuration Register"]
    pub l2pfcr: L2PFCR,
    #[doc = "0x118 - Layerx Constant Alpha Configuration Register"]
    pub l2cacr: L2CACR,
    #[doc = "0x11c - Layerx Default Color Configuration Register"]
    pub l2dccr: L2DCCR,
    #[doc = "0x120 - Layerx Blending Factors Configuration Register"]
    pub l2bfcr: L2BFCR,
    _reserved8: [u8; 8usize],
    #[doc = "0x12c - Layerx Color Frame Buffer Address Register"]
    pub l2cfbar: L2CFBAR,
    #[doc = "0x130 - Layerx Color Frame Buffer Length Register"]
    pub l2cfblr: L2CFBLR,
    #[doc = "0x134 - Layerx ColorFrame Buffer Line Number Register"]
    pub l2cfblnr: L2CFBLNR,
    _reserved9: [u8; 12usize],
    #[doc = "0x144 - Layerx CLUT Write Register"]
    pub l2clutwr: L2CLUTWR,
}
#[doc = "Synchronization Size Configuration Register"]
pub struct SSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Synchronization Size Configuration Register"]
pub mod sscr;
#[doc = "Back Porch Configuration Register"]
pub struct BPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Back Porch Configuration Register"]
pub mod bpcr;
#[doc = "Active Width Configuration Register"]
pub struct AWCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Active Width Configuration Register"]
pub mod awcr;
#[doc = "Total Width Configuration Register"]
pub struct TWCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Total Width Configuration Register"]
pub mod twcr;
#[doc = "Global Control Register"]
pub struct GCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Control Register"]
pub mod gcr;
#[doc = "Shadow Reload Configuration Register"]
pub struct SRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Shadow Reload Configuration Register"]
pub mod srcr;
#[doc = "Background Color Configuration Register"]
pub struct BCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Background Color Configuration Register"]
pub mod bccr;
#[doc = "Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Status Register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Interrupt Clear Register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "Line Interrupt Position Configuration Register"]
pub struct LIPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Line Interrupt Position Configuration Register"]
pub mod lipcr;
#[doc = "Current Position Status Register"]
pub struct CPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Position Status Register"]
pub mod cpsr;
#[doc = "Current Display Status Register"]
pub struct CDSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Display Status Register"]
pub mod cdsr;
#[doc = "Layerx Control Register"]
pub struct L1CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Control Register"]
pub mod l1cr;
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub struct L1WHPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub mod l1whpcr;
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub struct L1WVPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub mod l1wvpcr;
#[doc = "Layerx Color Keying Configuration Register"]
pub struct L1CKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Color Keying Configuration Register"]
pub mod l1ckcr;
#[doc = "Layerx Pixel Format Configuration Register"]
pub struct L1PFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Pixel Format Configuration Register"]
pub mod l1pfcr;
#[doc = "Layerx Constant Alpha Configuration Register"]
pub struct L1CACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Constant Alpha Configuration Register"]
pub mod l1cacr;
#[doc = "Layerx Default Color Configuration Register"]
pub struct L1DCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Default Color Configuration Register"]
pub mod l1dccr;
#[doc = "Layerx Blending Factors Configuration Register"]
pub struct L1BFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Blending Factors Configuration Register"]
pub mod l1bfcr;
#[doc = "Layerx Color Frame Buffer Address Register"]
pub struct L1CFBAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Color Frame Buffer Address Register"]
pub mod l1cfbar;
#[doc = "Layerx Color Frame Buffer Length Register"]
pub struct L1CFBLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Color Frame Buffer Length Register"]
pub mod l1cfblr;
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub struct L1CFBLNR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub mod l1cfblnr;
#[doc = "Layerx CLUT Write Register"]
pub struct L1CLUTWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx CLUT Write Register"]
pub mod l1clutwr;
#[doc = "Layerx Control Register"]
pub struct L2CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Control Register"]
pub mod l2cr;
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub struct L2WHPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub mod l2whpcr;
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub struct L2WVPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub mod l2wvpcr;
#[doc = "Layerx Color Keying Configuration Register"]
pub struct L2CKCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Color Keying Configuration Register"]
pub mod l2ckcr;
#[doc = "Layerx Pixel Format Configuration Register"]
pub struct L2PFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Pixel Format Configuration Register"]
pub mod l2pfcr;
#[doc = "Layerx Constant Alpha Configuration Register"]
pub struct L2CACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Constant Alpha Configuration Register"]
pub mod l2cacr;
#[doc = "Layerx Default Color Configuration Register"]
pub struct L2DCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Default Color Configuration Register"]
pub mod l2dccr;
#[doc = "Layerx Blending Factors Configuration Register"]
pub struct L2BFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Blending Factors Configuration Register"]
pub mod l2bfcr;
#[doc = "Layerx Color Frame Buffer Address Register"]
pub struct L2CFBAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Color Frame Buffer Address Register"]
pub mod l2cfbar;
#[doc = "Layerx Color Frame Buffer Length Register"]
pub struct L2CFBLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx Color Frame Buffer Length Register"]
pub mod l2cfblr;
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub struct L2CFBLNR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub mod l2cfblnr;
#[doc = "Layerx CLUT Write Register"]
pub struct L2CLUTWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Layerx CLUT Write Register"]
pub mod l2clutwr;

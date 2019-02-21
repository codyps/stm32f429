#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet MAC configuration register"]
    pub maccr: MACCR,
    #[doc = "0x04 - Ethernet MAC frame filter register"]
    pub macffr: MACFFR,
    #[doc = "0x08 - Ethernet MAC hash table high register"]
    pub machthr: MACHTHR,
    #[doc = "0x0c - Ethernet MAC hash table low register"]
    pub machtlr: MACHTLR,
    #[doc = "0x10 - Ethernet MAC MII address register"]
    pub macmiiar: MACMIIAR,
    #[doc = "0x14 - Ethernet MAC MII data register"]
    pub macmiidr: MACMIIDR,
    #[doc = "0x18 - Ethernet MAC flow control register"]
    pub macfcr: MACFCR,
    #[doc = "0x1c - Ethernet MAC VLAN tag register"]
    pub macvlantr: MACVLANTR,
    _reserved0: [u8; 12usize],
    #[doc = "0x2c - Ethernet MAC PMT control and status register"]
    pub macpmtcsr: MACPMTCSR,
    _reserved1: [u8; 4usize],
    #[doc = "0x34 - Ethernet MAC debug register"]
    pub macdbgr: MACDBGR,
    #[doc = "0x38 - Ethernet MAC interrupt status register"]
    pub macsr: MACSR,
    #[doc = "0x3c - Ethernet MAC interrupt mask register"]
    pub macimr: MACIMR,
    #[doc = "0x40 - Ethernet MAC address 0 high register"]
    pub maca0hr: MACA0HR,
    #[doc = "0x44 - Ethernet MAC address 0 low register"]
    pub maca0lr: MACA0LR,
    #[doc = "0x48 - Ethernet MAC address 1 high register"]
    pub maca1hr: MACA1HR,
    #[doc = "0x4c - Ethernet MAC address1 low register"]
    pub maca1lr: MACA1LR,
    #[doc = "0x50 - Ethernet MAC address 2 high register"]
    pub maca2hr: MACA2HR,
    #[doc = "0x54 - Ethernet MAC address 2 low register"]
    pub maca2lr: MACA2LR,
    #[doc = "0x58 - Ethernet MAC address 3 high register"]
    pub maca3hr: MACA3HR,
    #[doc = "0x5c - Ethernet MAC address 3 low register"]
    pub maca3lr: MACA3LR,
}
#[doc = "Ethernet MAC configuration register"]
pub struct MACCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC configuration register"]
pub mod maccr;
#[doc = "Ethernet MAC frame filter register"]
pub struct MACFFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC frame filter register"]
pub mod macffr;
#[doc = "Ethernet MAC hash table high register"]
pub struct MACHTHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC hash table high register"]
pub mod machthr;
#[doc = "Ethernet MAC hash table low register"]
pub struct MACHTLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC hash table low register"]
pub mod machtlr;
#[doc = "Ethernet MAC MII address register"]
pub struct MACMIIAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC MII address register"]
pub mod macmiiar;
#[doc = "Ethernet MAC MII data register"]
pub struct MACMIIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC MII data register"]
pub mod macmiidr;
#[doc = "Ethernet MAC flow control register"]
pub struct MACFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC flow control register"]
pub mod macfcr;
#[doc = "Ethernet MAC VLAN tag register"]
pub struct MACVLANTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC VLAN tag register"]
pub mod macvlantr;
#[doc = "Ethernet MAC PMT control and status register"]
pub struct MACPMTCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC PMT control and status register"]
pub mod macpmtcsr;
#[doc = "Ethernet MAC debug register"]
pub struct MACDBGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC debug register"]
pub mod macdbgr;
#[doc = "Ethernet MAC interrupt status register"]
pub struct MACSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC interrupt status register"]
pub mod macsr;
#[doc = "Ethernet MAC interrupt mask register"]
pub struct MACIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC interrupt mask register"]
pub mod macimr;
#[doc = "Ethernet MAC address 0 high register"]
pub struct MACA0HR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 0 high register"]
pub mod maca0hr;
#[doc = "Ethernet MAC address 0 low register"]
pub struct MACA0LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 0 low register"]
pub mod maca0lr;
#[doc = "Ethernet MAC address 1 high register"]
pub struct MACA1HR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 1 high register"]
pub mod maca1hr;
#[doc = "Ethernet MAC address1 low register"]
pub struct MACA1LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address1 low register"]
pub mod maca1lr;
#[doc = "Ethernet MAC address 2 high register"]
pub struct MACA2HR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 2 high register"]
pub mod maca2hr;
#[doc = "Ethernet MAC address 2 low register"]
pub struct MACA2LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 2 low register"]
pub mod maca2lr;
#[doc = "Ethernet MAC address 3 high register"]
pub struct MACA3HR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 3 high register"]
pub mod maca3hr;
#[doc = "Ethernet MAC address 3 low register"]
pub struct MACA3LR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet MAC address 3 low register"]
pub mod maca3lr;

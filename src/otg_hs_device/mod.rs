#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_HS device configuration register"]
    pub otg_hs_dcfg: OTG_HS_DCFG,
    #[doc = "0x04 - OTG_HS device control register"]
    pub otg_hs_dctl: OTG_HS_DCTL,
    #[doc = "0x08 - OTG_HS device status register"]
    pub otg_hs_dsts: OTG_HS_DSTS,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - OTG_HS device IN endpoint common interrupt mask register"]
    pub otg_hs_diepmsk: OTG_HS_DIEPMSK,
    #[doc = "0x14 - OTG_HS device OUT endpoint common interrupt mask register"]
    pub otg_hs_doepmsk: OTG_HS_DOEPMSK,
    #[doc = "0x18 - OTG_HS device all endpoints interrupt register"]
    pub otg_hs_daint: OTG_HS_DAINT,
    #[doc = "0x1c - OTG_HS all endpoints interrupt mask register"]
    pub otg_hs_daintmsk: OTG_HS_DAINTMSK,
    _reserved1: [u8; 8usize],
    #[doc = "0x28 - OTG_HS device VBUS discharge time register"]
    pub otg_hs_dvbusdis: OTG_HS_DVBUSDIS,
    #[doc = "0x2c - OTG_HS device VBUS pulsing time register"]
    pub otg_hs_dvbuspulse: OTG_HS_DVBUSPULSE,
    #[doc = "0x30 - OTG_HS Device threshold control register"]
    pub otg_hs_dthrctl: OTG_HS_DTHRCTL,
    #[doc = "0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register"]
    pub otg_hs_diepempmsk: OTG_HS_DIEPEMPMSK,
    #[doc = "0x38 - OTG_HS device each endpoint interrupt register"]
    pub otg_hs_deachint: OTG_HS_DEACHINT,
    #[doc = "0x3c - OTG_HS device each endpoint interrupt register mask"]
    pub otg_hs_deachintmsk: OTG_HS_DEACHINTMSK,
    #[doc = "0x40 - OTG_HS device each in endpoint-1 interrupt register"]
    pub otg_hs_diepeachmsk1: OTG_HS_DIEPEACHMSK1,
    _reserved2: [u8; 60usize],
    #[doc = "0x80 - OTG_HS device each OUT endpoint-1 interrupt register"]
    pub otg_hs_doepeachmsk1: OTG_HS_DOEPEACHMSK1,
    _reserved3: [u8; 124usize],
    #[doc = "0x100 - OTG device endpoint-0 control register"]
    pub otg_hs_diepctl0: OTG_HS_DIEPCTL0,
    _reserved4: [u8; 4usize],
    #[doc = "0x108 - OTG device endpoint-0 interrupt register"]
    pub otg_hs_diepint0: OTG_HS_DIEPINT0,
    _reserved5: [u8; 4usize],
    #[doc = "0x110 - OTG_HS device IN endpoint 0 transfer size register"]
    pub otg_hs_dieptsiz0: OTG_HS_DIEPTSIZ0,
    #[doc = "0x114 - OTG_HS device endpoint-1 DMA address register"]
    pub otg_hs_diepdma1: OTG_HS_DIEPDMA1,
    #[doc = "0x118 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts0: OTG_HS_DTXFSTS0,
    _reserved6: [u8; 4usize],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub otg_hs_diepctl1: OTG_HS_DIEPCTL1,
    _reserved7: [u8; 4usize],
    #[doc = "0x128 - OTG device endpoint-1 interrupt register"]
    pub otg_hs_diepint1: OTG_HS_DIEPINT1,
    _reserved8: [u8; 4usize],
    #[doc = "0x130 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz1: OTG_HS_DIEPTSIZ1,
    #[doc = "0x134 - OTG_HS device endpoint-2 DMA address register"]
    pub otg_hs_diepdma2: OTG_HS_DIEPDMA2,
    #[doc = "0x138 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts1: OTG_HS_DTXFSTS1,
    _reserved9: [u8; 4usize],
    #[doc = "0x140 - OTG device endpoint-2 control register"]
    pub otg_hs_diepctl2: OTG_HS_DIEPCTL2,
    _reserved10: [u8; 4usize],
    #[doc = "0x148 - OTG device endpoint-2 interrupt register"]
    pub otg_hs_diepint2: OTG_HS_DIEPINT2,
    _reserved11: [u8; 4usize],
    #[doc = "0x150 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz2: OTG_HS_DIEPTSIZ2,
    #[doc = "0x154 - OTG_HS device endpoint-3 DMA address register"]
    pub otg_hs_diepdma3: OTG_HS_DIEPDMA3,
    #[doc = "0x158 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts2: OTG_HS_DTXFSTS2,
    _reserved12: [u8; 4usize],
    #[doc = "0x160 - OTG device endpoint-3 control register"]
    pub otg_hs_diepctl3: OTG_HS_DIEPCTL3,
    _reserved13: [u8; 4usize],
    #[doc = "0x168 - OTG device endpoint-3 interrupt register"]
    pub otg_hs_diepint3: OTG_HS_DIEPINT3,
    _reserved14: [u8; 4usize],
    #[doc = "0x170 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz3: OTG_HS_DIEPTSIZ3,
    #[doc = "0x174 - OTG_HS device endpoint-4 DMA address register"]
    pub otg_hs_diepdma4: OTG_HS_DIEPDMA4,
    #[doc = "0x178 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts3: OTG_HS_DTXFSTS3,
    _reserved15: [u8; 4usize],
    #[doc = "0x180 - OTG device endpoint-4 control register"]
    pub otg_hs_diepctl4: OTG_HS_DIEPCTL4,
    _reserved16: [u8; 4usize],
    #[doc = "0x188 - OTG device endpoint-4 interrupt register"]
    pub otg_hs_diepint4: OTG_HS_DIEPINT4,
    _reserved17: [u8; 4usize],
    #[doc = "0x190 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz4: OTG_HS_DIEPTSIZ4,
    #[doc = "0x194 - OTG_HS device endpoint-5 DMA address register"]
    pub otg_hs_diepdma5: OTG_HS_DIEPDMA5,
    #[doc = "0x198 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts4: OTG_HS_DTXFSTS4,
    _reserved18: [u8; 4usize],
    #[doc = "0x1a0 - OTG device endpoint-5 control register"]
    pub otg_hs_diepctl5: OTG_HS_DIEPCTL5,
    _reserved19: [u8; 4usize],
    #[doc = "0x1a8 - OTG device endpoint-5 interrupt register"]
    pub otg_hs_diepint5: OTG_HS_DIEPINT5,
    _reserved20: [u8; 4usize],
    #[doc = "0x1b0 - OTG_HS device endpoint transfer size register"]
    pub otg_hs_dieptsiz5: OTG_HS_DIEPTSIZ5,
    _reserved21: [u8; 4usize],
    #[doc = "0x1b8 - OTG_HS device IN endpoint transmit FIFO status register"]
    pub otg_hs_dtxfsts5: OTG_HS_DTXFSTS5,
    _reserved22: [u8; 4usize],
    #[doc = "0x1c0 - OTG device endpoint-6 control register"]
    pub otg_hs_diepctl6: OTG_HS_DIEPCTL6,
    _reserved23: [u8; 4usize],
    #[doc = "0x1c8 - OTG device endpoint-6 interrupt register"]
    pub otg_hs_diepint6: OTG_HS_DIEPINT6,
    _reserved24: [u8; 20usize],
    #[doc = "0x1e0 - OTG device endpoint-7 control register"]
    pub otg_hs_diepctl7: OTG_HS_DIEPCTL7,
    _reserved25: [u8; 4usize],
    #[doc = "0x1e8 - OTG device endpoint-7 interrupt register"]
    pub otg_hs_diepint7: OTG_HS_DIEPINT7,
    _reserved26: [u8; 276usize],
    #[doc = "0x300 - OTG_HS device control OUT endpoint 0 control register"]
    pub otg_hs_doepctl0: OTG_HS_DOEPCTL0,
    _reserved27: [u8; 4usize],
    #[doc = "0x308 - OTG_HS device endpoint-0 interrupt register"]
    pub otg_hs_doepint0: OTG_HS_DOEPINT0,
    _reserved28: [u8; 4usize],
    #[doc = "0x310 - OTG_HS device endpoint-1 transfer size register"]
    pub otg_hs_doeptsiz0: OTG_HS_DOEPTSIZ0,
    _reserved29: [u8; 12usize],
    #[doc = "0x320 - OTG device endpoint-1 control register"]
    pub otg_hs_doepctl1: OTG_HS_DOEPCTL1,
    _reserved30: [u8; 4usize],
    #[doc = "0x328 - OTG_HS device endpoint-1 interrupt register"]
    pub otg_hs_doepint1: OTG_HS_DOEPINT1,
    _reserved31: [u8; 4usize],
    #[doc = "0x330 - OTG_HS device endpoint-2 transfer size register"]
    pub otg_hs_doeptsiz1: OTG_HS_DOEPTSIZ1,
    _reserved32: [u8; 12usize],
    #[doc = "0x340 - OTG device endpoint-2 control register"]
    pub otg_hs_doepctl2: OTG_HS_DOEPCTL2,
    _reserved33: [u8; 4usize],
    #[doc = "0x348 - OTG_HS device endpoint-2 interrupt register"]
    pub otg_hs_doepint2: OTG_HS_DOEPINT2,
    _reserved34: [u8; 4usize],
    #[doc = "0x350 - OTG_HS device endpoint-3 transfer size register"]
    pub otg_hs_doeptsiz2: OTG_HS_DOEPTSIZ2,
    _reserved35: [u8; 12usize],
    #[doc = "0x360 - OTG device endpoint-3 control register"]
    pub otg_hs_doepctl3: OTG_HS_DOEPCTL3,
    _reserved36: [u8; 4usize],
    #[doc = "0x368 - OTG_HS device endpoint-3 interrupt register"]
    pub otg_hs_doepint3: OTG_HS_DOEPINT3,
    _reserved37: [u8; 4usize],
    #[doc = "0x370 - OTG_HS device endpoint-4 transfer size register"]
    pub otg_hs_doeptsiz3: OTG_HS_DOEPTSIZ3,
    _reserved38: [u8; 20usize],
    #[doc = "0x388 - OTG_HS device endpoint-4 interrupt register"]
    pub otg_hs_doepint4: OTG_HS_DOEPINT4,
    _reserved39: [u8; 4usize],
    #[doc = "0x390 - OTG_HS device endpoint-5 transfer size register"]
    pub otg_hs_doeptsiz4: OTG_HS_DOEPTSIZ4,
    _reserved40: [u8; 20usize],
    #[doc = "0x3a8 - OTG_HS device endpoint-5 interrupt register"]
    pub otg_hs_doepint5: OTG_HS_DOEPINT5,
    _reserved41: [u8; 28usize],
    #[doc = "0x3c8 - OTG_HS device endpoint-6 interrupt register"]
    pub otg_hs_doepint6: OTG_HS_DOEPINT6,
    _reserved42: [u8; 28usize],
    #[doc = "0x3e8 - OTG_HS device endpoint-7 interrupt register"]
    pub otg_hs_doepint7: OTG_HS_DOEPINT7,
}
#[doc = "OTG_HS device configuration register"]
pub struct OTG_HS_DCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device configuration register"]
pub mod otg_hs_dcfg;
#[doc = "OTG_HS device control register"]
pub struct OTG_HS_DCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device control register"]
pub mod otg_hs_dctl;
#[doc = "OTG_HS device status register"]
pub struct OTG_HS_DSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device status register"]
pub mod otg_hs_dsts;
#[doc = "OTG_HS device IN endpoint common interrupt mask register"]
pub struct OTG_HS_DIEPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint common interrupt mask register"]
pub mod otg_hs_diepmsk;
#[doc = "OTG_HS device OUT endpoint common interrupt mask register"]
pub struct OTG_HS_DOEPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device OUT endpoint common interrupt mask register"]
pub mod otg_hs_doepmsk;
#[doc = "OTG_HS device all endpoints interrupt register"]
pub struct OTG_HS_DAINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device all endpoints interrupt register"]
pub mod otg_hs_daint;
#[doc = "OTG_HS all endpoints interrupt mask register"]
pub struct OTG_HS_DAINTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS all endpoints interrupt mask register"]
pub mod otg_hs_daintmsk;
#[doc = "OTG_HS device VBUS discharge time register"]
pub struct OTG_HS_DVBUSDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device VBUS discharge time register"]
pub mod otg_hs_dvbusdis;
#[doc = "OTG_HS device VBUS pulsing time register"]
pub struct OTG_HS_DVBUSPULSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device VBUS pulsing time register"]
pub mod otg_hs_dvbuspulse;
#[doc = "OTG_HS Device threshold control register"]
pub struct OTG_HS_DTHRCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS Device threshold control register"]
pub mod otg_hs_dthrctl;
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register"]
pub struct OTG_HS_DIEPEMPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint FIFO empty interrupt mask register"]
pub mod otg_hs_diepempmsk;
#[doc = "OTG_HS device each endpoint interrupt register"]
pub struct OTG_HS_DEACHINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device each endpoint interrupt register"]
pub mod otg_hs_deachint;
#[doc = "OTG_HS device each endpoint interrupt register mask"]
pub struct OTG_HS_DEACHINTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device each endpoint interrupt register mask"]
pub mod otg_hs_deachintmsk;
#[doc = "OTG_HS device each in endpoint-1 interrupt register"]
pub struct OTG_HS_DIEPEACHMSK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device each in endpoint-1 interrupt register"]
pub mod otg_hs_diepeachmsk1;
#[doc = "OTG_HS device each OUT endpoint-1 interrupt register"]
pub struct OTG_HS_DOEPEACHMSK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device each OUT endpoint-1 interrupt register"]
pub mod otg_hs_doepeachmsk1;
#[doc = "OTG device endpoint-0 control register"]
pub struct OTG_HS_DIEPCTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-0 control register"]
pub mod otg_hs_diepctl0;
#[doc = "OTG device endpoint-1 control register"]
pub struct OTG_HS_DIEPCTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-1 control register"]
pub mod otg_hs_diepctl1;
#[doc = "OTG device endpoint-2 control register"]
pub struct OTG_HS_DIEPCTL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-2 control register"]
pub mod otg_hs_diepctl2;
#[doc = "OTG device endpoint-3 control register"]
pub struct OTG_HS_DIEPCTL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-3 control register"]
pub mod otg_hs_diepctl3;
#[doc = "OTG device endpoint-4 control register"]
pub struct OTG_HS_DIEPCTL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-4 control register"]
pub mod otg_hs_diepctl4;
#[doc = "OTG device endpoint-5 control register"]
pub struct OTG_HS_DIEPCTL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-5 control register"]
pub mod otg_hs_diepctl5;
#[doc = "OTG device endpoint-6 control register"]
pub struct OTG_HS_DIEPCTL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-6 control register"]
pub mod otg_hs_diepctl6;
#[doc = "OTG device endpoint-7 control register"]
pub struct OTG_HS_DIEPCTL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-7 control register"]
pub mod otg_hs_diepctl7;
#[doc = "OTG device endpoint-0 interrupt register"]
pub struct OTG_HS_DIEPINT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-0 interrupt register"]
pub mod otg_hs_diepint0;
#[doc = "OTG device endpoint-1 interrupt register"]
pub struct OTG_HS_DIEPINT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-1 interrupt register"]
pub mod otg_hs_diepint1;
#[doc = "OTG device endpoint-2 interrupt register"]
pub struct OTG_HS_DIEPINT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-2 interrupt register"]
pub mod otg_hs_diepint2;
#[doc = "OTG device endpoint-3 interrupt register"]
pub struct OTG_HS_DIEPINT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-3 interrupt register"]
pub mod otg_hs_diepint3;
#[doc = "OTG device endpoint-4 interrupt register"]
pub struct OTG_HS_DIEPINT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-4 interrupt register"]
pub mod otg_hs_diepint4;
#[doc = "OTG device endpoint-5 interrupt register"]
pub struct OTG_HS_DIEPINT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-5 interrupt register"]
pub mod otg_hs_diepint5;
#[doc = "OTG device endpoint-6 interrupt register"]
pub struct OTG_HS_DIEPINT6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-6 interrupt register"]
pub mod otg_hs_diepint6;
#[doc = "OTG device endpoint-7 interrupt register"]
pub struct OTG_HS_DIEPINT7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-7 interrupt register"]
pub mod otg_hs_diepint7;
#[doc = "OTG_HS device IN endpoint 0 transfer size register"]
pub struct OTG_HS_DIEPTSIZ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint 0 transfer size register"]
pub mod otg_hs_dieptsiz0;
#[doc = "OTG_HS device endpoint-1 DMA address register"]
pub struct OTG_HS_DIEPDMA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-1 DMA address register"]
pub mod otg_hs_diepdma1;
#[doc = "OTG_HS device endpoint-2 DMA address register"]
pub struct OTG_HS_DIEPDMA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-2 DMA address register"]
pub mod otg_hs_diepdma2;
#[doc = "OTG_HS device endpoint-3 DMA address register"]
pub struct OTG_HS_DIEPDMA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-3 DMA address register"]
pub mod otg_hs_diepdma3;
#[doc = "OTG_HS device endpoint-4 DMA address register"]
pub struct OTG_HS_DIEPDMA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-4 DMA address register"]
pub mod otg_hs_diepdma4;
#[doc = "OTG_HS device endpoint-5 DMA address register"]
pub struct OTG_HS_DIEPDMA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-5 DMA address register"]
pub mod otg_hs_diepdma5;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub struct OTG_HS_DTXFSTS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts0;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub struct OTG_HS_DTXFSTS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts1;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub struct OTG_HS_DTXFSTS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts2;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub struct OTG_HS_DTXFSTS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts3;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub struct OTG_HS_DTXFSTS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts4;
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub struct OTG_HS_DTXFSTS5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register"]
pub mod otg_hs_dtxfsts5;
#[doc = "OTG_HS device endpoint transfer size register"]
pub struct OTG_HS_DIEPTSIZ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz1;
#[doc = "OTG_HS device endpoint transfer size register"]
pub struct OTG_HS_DIEPTSIZ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz2;
#[doc = "OTG_HS device endpoint transfer size register"]
pub struct OTG_HS_DIEPTSIZ3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz3;
#[doc = "OTG_HS device endpoint transfer size register"]
pub struct OTG_HS_DIEPTSIZ4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz4;
#[doc = "OTG_HS device endpoint transfer size register"]
pub struct OTG_HS_DIEPTSIZ5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint transfer size register"]
pub mod otg_hs_dieptsiz5;
#[doc = "OTG_HS device control OUT endpoint 0 control register"]
pub struct OTG_HS_DOEPCTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device control OUT endpoint 0 control register"]
pub mod otg_hs_doepctl0;
#[doc = "OTG device endpoint-1 control register"]
pub struct OTG_HS_DOEPCTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-1 control register"]
pub mod otg_hs_doepctl1;
#[doc = "OTG device endpoint-2 control register"]
pub struct OTG_HS_DOEPCTL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-2 control register"]
pub mod otg_hs_doepctl2;
#[doc = "OTG device endpoint-3 control register"]
pub struct OTG_HS_DOEPCTL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-3 control register"]
pub mod otg_hs_doepctl3;
#[doc = "OTG_HS device endpoint-0 interrupt register"]
pub struct OTG_HS_DOEPINT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-0 interrupt register"]
pub mod otg_hs_doepint0;
#[doc = "OTG_HS device endpoint-1 interrupt register"]
pub struct OTG_HS_DOEPINT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-1 interrupt register"]
pub mod otg_hs_doepint1;
#[doc = "OTG_HS device endpoint-2 interrupt register"]
pub struct OTG_HS_DOEPINT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-2 interrupt register"]
pub mod otg_hs_doepint2;
#[doc = "OTG_HS device endpoint-3 interrupt register"]
pub struct OTG_HS_DOEPINT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-3 interrupt register"]
pub mod otg_hs_doepint3;
#[doc = "OTG_HS device endpoint-4 interrupt register"]
pub struct OTG_HS_DOEPINT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-4 interrupt register"]
pub mod otg_hs_doepint4;
#[doc = "OTG_HS device endpoint-5 interrupt register"]
pub struct OTG_HS_DOEPINT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-5 interrupt register"]
pub mod otg_hs_doepint5;
#[doc = "OTG_HS device endpoint-6 interrupt register"]
pub struct OTG_HS_DOEPINT6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-6 interrupt register"]
pub mod otg_hs_doepint6;
#[doc = "OTG_HS device endpoint-7 interrupt register"]
pub struct OTG_HS_DOEPINT7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-7 interrupt register"]
pub mod otg_hs_doepint7;
#[doc = "OTG_HS device endpoint-1 transfer size register"]
pub struct OTG_HS_DOEPTSIZ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-1 transfer size register"]
pub mod otg_hs_doeptsiz0;
#[doc = "OTG_HS device endpoint-2 transfer size register"]
pub struct OTG_HS_DOEPTSIZ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-2 transfer size register"]
pub mod otg_hs_doeptsiz1;
#[doc = "OTG_HS device endpoint-3 transfer size register"]
pub struct OTG_HS_DOEPTSIZ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-3 transfer size register"]
pub mod otg_hs_doeptsiz2;
#[doc = "OTG_HS device endpoint-4 transfer size register"]
pub struct OTG_HS_DOEPTSIZ3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-4 transfer size register"]
pub mod otg_hs_doeptsiz3;
#[doc = "OTG_HS device endpoint-5 transfer size register"]
pub struct OTG_HS_DOEPTSIZ4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_HS device endpoint-5 transfer size register"]
pub mod otg_hs_doeptsiz4;

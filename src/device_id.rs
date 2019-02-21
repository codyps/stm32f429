#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - X and Y coordinates on the wafer"]
    pub uid1: UID1,
    #[doc = "0x04 - Lot/wafer number"]
    pub uid2: UID2,
    #[doc = "0x08 - Lot number"]
    pub uid3: UID3,
}
#[doc = "X and Y coordinates on the wafer"]
pub struct UID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "X and Y coordinates on the wafer"]
pub mod uid1;
#[doc = "Lot/wafer number"]
pub struct UID2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lot/wafer number"]
pub mod uid2;
#[doc = "Lot number"]
pub struct UID3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lot number"]
pub mod uid3;

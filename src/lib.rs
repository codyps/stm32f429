#![doc = "Peripheral access API for STM32F429 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn TAMP_STAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_STREAM0();
    fn DMA1_STREAM1();
    fn DMA1_STREAM2();
    fn DMA1_STREAM3();
    fn DMA1_STREAM4();
    fn DMA1_STREAM5();
    fn DMA1_STREAM6();
    fn ADC();
    fn CAN1_TX();
    fn CAN1_RX0();
    fn CAN1_RX1();
    fn CAN1_SCE();
    fn EXTI9_5();
    fn TIM1_BRK_TIM9();
    fn TIM1_UP_TIM10();
    fn TIM1_TRG_COM_TIM11();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn OTG_FS_WKUP();
    fn TIM8_BRK_TIM12();
    fn TIM8_UP_TIM13();
    fn TIM8_TRG_COM_TIM14();
    fn TIM8_CC();
    fn DMA1_STREAM7();
    fn FMC();
    fn SDIO();
    fn TIM5();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6_DAC();
    fn TIM7();
    fn DMA2_STREAM0();
    fn DMA2_STREAM1();
    fn DMA2_STREAM2();
    fn DMA2_STREAM3();
    fn DMA2_STREAM4();
    fn ETH();
    fn ETH_WKUP();
    fn CAN2_TX();
    fn CAN2_RX0();
    fn CAN2_RX1();
    fn CAN2_SCE();
    fn OTG_FS();
    fn DMA2_STREAM5();
    fn DMA2_STREAM6();
    fn DMA2_STREAM7();
    fn USART6();
    fn I2C3_EV();
    fn I2C3_ER();
    fn OTG_HS_EP1_OUT();
    fn OTG_HS_EP1_IN();
    fn OTG_HS_WKUP();
    fn OTG_HS();
    fn DCMI();
    fn CRYP();
    fn HASH_RNG();
    fn FPU();
    fn UART7();
    fn UART8();
    fn SPI4();
    fn SPI5();
    fn SPI6();
    fn SAI1();
    fn LCD_TFT();
    fn LCD_TFT_1();
    fn DMA2D();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 91] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector {
        _handler: TAMP_STAMP,
    },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector {
        _handler: DMA1_STREAM0,
    },
    Vector {
        _handler: DMA1_STREAM1,
    },
    Vector {
        _handler: DMA1_STREAM2,
    },
    Vector {
        _handler: DMA1_STREAM3,
    },
    Vector {
        _handler: DMA1_STREAM4,
    },
    Vector {
        _handler: DMA1_STREAM5,
    },
    Vector {
        _handler: DMA1_STREAM6,
    },
    Vector { _handler: ADC },
    Vector { _handler: CAN1_TX },
    Vector { _handler: CAN1_RX0 },
    Vector { _handler: CAN1_RX1 },
    Vector { _handler: CAN1_SCE },
    Vector { _handler: EXTI9_5 },
    Vector {
        _handler: TIM1_BRK_TIM9,
    },
    Vector {
        _handler: TIM1_UP_TIM10,
    },
    Vector {
        _handler: TIM1_TRG_COM_TIM11,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector {
        _handler: OTG_FS_WKUP,
    },
    Vector {
        _handler: TIM8_BRK_TIM12,
    },
    Vector {
        _handler: TIM8_UP_TIM13,
    },
    Vector {
        _handler: TIM8_TRG_COM_TIM14,
    },
    Vector { _handler: TIM8_CC },
    Vector {
        _handler: DMA1_STREAM7,
    },
    Vector { _handler: FMC },
    Vector { _handler: SDIO },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: TIM6_DAC },
    Vector { _handler: TIM7 },
    Vector {
        _handler: DMA2_STREAM0,
    },
    Vector {
        _handler: DMA2_STREAM1,
    },
    Vector {
        _handler: DMA2_STREAM2,
    },
    Vector {
        _handler: DMA2_STREAM3,
    },
    Vector {
        _handler: DMA2_STREAM4,
    },
    Vector { _handler: ETH },
    Vector { _handler: ETH_WKUP },
    Vector { _handler: CAN2_TX },
    Vector { _handler: CAN2_RX0 },
    Vector { _handler: CAN2_RX1 },
    Vector { _handler: CAN2_SCE },
    Vector { _handler: OTG_FS },
    Vector {
        _handler: DMA2_STREAM5,
    },
    Vector {
        _handler: DMA2_STREAM6,
    },
    Vector {
        _handler: DMA2_STREAM7,
    },
    Vector { _handler: USART6 },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector {
        _handler: OTG_HS_EP1_OUT,
    },
    Vector {
        _handler: OTG_HS_EP1_IN,
    },
    Vector {
        _handler: OTG_HS_WKUP,
    },
    Vector { _handler: OTG_HS },
    Vector { _handler: DCMI },
    Vector { _handler: CRYP },
    Vector { _handler: HASH_RNG },
    Vector { _handler: FPU },
    Vector { _handler: UART7 },
    Vector { _handler: UART8 },
    Vector { _handler: SPI4 },
    Vector { _handler: SPI5 },
    Vector { _handler: SPI6 },
    Vector { _handler: SAI1 },
    Vector { _handler: LCD_TFT },
    Vector {
        _handler: LCD_TFT_1,
    },
    Vector { _handler: DMA2D },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"]
    WWDG,
    #[doc = "1 - PVD through EXTI line detection interrupt"]
    PVD,
    #[doc = "2 - Tamper and TimeStamp interrupts through the EXTI line"]
    TAMP_STAMP,
    #[doc = "3 - RTC Wakeup interrupt through the EXTI line"]
    RTC_WKUP,
    #[doc = "4 - Flash global interrupt"]
    FLASH,
    #[doc = "5 - RCC global interrupt"]
    RCC,
    #[doc = "6 - EXTI Line0 interrupt"]
    EXTI0,
    #[doc = "7 - EXTI Line1 interrupt"]
    EXTI1,
    #[doc = "8 - EXTI Line2 interrupt"]
    EXTI2,
    #[doc = "9 - EXTI Line3 interrupt"]
    EXTI3,
    #[doc = "10 - EXTI Line4 interrupt"]
    EXTI4,
    #[doc = "11 - DMA1 Stream0 global interrupt"]
    DMA1_STREAM0,
    #[doc = "12 - DMA1 Stream1 global interrupt"]
    DMA1_STREAM1,
    #[doc = "13 - DMA1 Stream2 global interrupt"]
    DMA1_STREAM2,
    #[doc = "14 - DMA1 Stream3 global interrupt"]
    DMA1_STREAM3,
    #[doc = "15 - DMA1 Stream4 global interrupt"]
    DMA1_STREAM4,
    #[doc = "16 - DMA1 Stream5 global interrupt"]
    DMA1_STREAM5,
    #[doc = "17 - DMA1 Stream6 global interrupt"]
    DMA1_STREAM6,
    #[doc = "18 - ADC2 global interrupts"]
    ADC,
    #[doc = "19 - CAN1 TX interrupts"]
    CAN1_TX,
    #[doc = "20 - CAN1 RX0 interrupts"]
    CAN1_RX0,
    #[doc = "21 - CAN1 RX1 interrupts"]
    CAN1_RX1,
    #[doc = "22 - CAN1 SCE interrupt"]
    CAN1_SCE,
    #[doc = "23 - EXTI Line\\[9:5\\] interrupts"]
    EXTI9_5,
    #[doc = "24 - TIM1 Break interrupt and TIM9 global interrupt"]
    TIM1_BRK_TIM9,
    #[doc = "25 - TIM1 Update interrupt and TIM10 global interrupt"]
    TIM1_UP_TIM10,
    #[doc = "26 - TIM1 Trigger and Commutation interrupts and TIM11 global interrupt"]
    TIM1_TRG_COM_TIM11,
    #[doc = "27 - TIM1 Capture Compare interrupt"]
    TIM1_CC,
    #[doc = "28 - TIM2 global interrupt"]
    TIM2,
    #[doc = "29 - TIM3 global interrupt"]
    TIM3,
    #[doc = "30 - TIM4 global interrupt"]
    TIM4,
    #[doc = "31 - I2C1 event interrupt"]
    I2C1_EV,
    #[doc = "32 - I2C1 error interrupt"]
    I2C1_ER,
    #[doc = "33 - I2C2 event interrupt"]
    I2C2_EV,
    #[doc = "34 - I2C2 error interrupt"]
    I2C2_ER,
    #[doc = "35 - SPI1 global interrupt"]
    SPI1,
    #[doc = "36 - SPI2 global interrupt"]
    SPI2,
    #[doc = "37 - USART1 global interrupt"]
    USART1,
    #[doc = "38 - USART2 global interrupt"]
    USART2,
    #[doc = "39 - USART3 global interrupt"]
    USART3,
    #[doc = "40 - EXTI Line\\[15:10\\] interrupts"]
    EXTI15_10,
    #[doc = "41 - RTC Alarms (A and B) through EXTI line interrupt"]
    RTC_ALARM,
    #[doc = "42 - USB On-The-Go FS Wakeup through EXTI line interrupt"]
    OTG_FS_WKUP,
    #[doc = "43 - TIM8 Break interrupt and TIM12 global interrupt"]
    TIM8_BRK_TIM12,
    #[doc = "44 - TIM8 Update interrupt and TIM13 global interrupt"]
    TIM8_UP_TIM13,
    #[doc = "45 - TIM8 Trigger and Commutation interrupts and TIM14 global interrupt"]
    TIM8_TRG_COM_TIM14,
    #[doc = "46 - TIM8 Capture Compare interrupt"]
    TIM8_CC,
    #[doc = "47 - DMA1 Stream7 global interrupt"]
    DMA1_STREAM7,
    #[doc = "48 - FMC global interrupt"]
    FMC,
    #[doc = "49 - SDIO global interrupt"]
    SDIO,
    #[doc = "50 - TIM5 global interrupt"]
    TIM5,
    #[doc = "51 - SPI3 global interrupt"]
    SPI3,
    #[doc = "52 - UART4 global interrupt"]
    UART4,
    #[doc = "53 - UART5 global interrupt"]
    UART5,
    #[doc = "54 - TIM6 global interrupt, DAC1 and DAC2 underrun error interrupt"]
    TIM6_DAC,
    #[doc = "55 - TIM7 global interrupt"]
    TIM7,
    #[doc = "56 - DMA2 Stream0 global interrupt"]
    DMA2_STREAM0,
    #[doc = "57 - DMA2 Stream1 global interrupt"]
    DMA2_STREAM1,
    #[doc = "58 - DMA2 Stream2 global interrupt"]
    DMA2_STREAM2,
    #[doc = "59 - DMA2 Stream3 global interrupt"]
    DMA2_STREAM3,
    #[doc = "60 - DMA2 Stream4 global interrupt"]
    DMA2_STREAM4,
    #[doc = "61 - Ethernet global interrupt"]
    ETH,
    #[doc = "62 - Ethernet Wakeup through EXTI line interrupt"]
    ETH_WKUP,
    #[doc = "63 - CAN2 TX interrupts"]
    CAN2_TX,
    #[doc = "64 - CAN2 RX0 interrupts"]
    CAN2_RX0,
    #[doc = "65 - CAN2 RX1 interrupts"]
    CAN2_RX1,
    #[doc = "66 - CAN2 SCE interrupt"]
    CAN2_SCE,
    #[doc = "67 - USB On The Go FS global interrupt"]
    OTG_FS,
    #[doc = "68 - DMA2 Stream5 global interrupt"]
    DMA2_STREAM5,
    #[doc = "69 - DMA2 Stream6 global interrupt"]
    DMA2_STREAM6,
    #[doc = "70 - DMA2 Stream7 global interrupt"]
    DMA2_STREAM7,
    #[doc = "71 - USART6 global interrupt"]
    USART6,
    #[doc = "72 - I2C3 event interrupt"]
    I2C3_EV,
    #[doc = "73 - I2C3 error interrupt"]
    I2C3_ER,
    #[doc = "74 - USB On The Go HS End Point 1 Out global interrupt"]
    OTG_HS_EP1_OUT,
    #[doc = "75 - USB On The Go HS End Point 1 In global interrupt"]
    OTG_HS_EP1_IN,
    #[doc = "76 - USB On The Go HS Wakeup through EXTI interrupt"]
    OTG_HS_WKUP,
    #[doc = "77 - USB On The Go HS global interrupt"]
    OTG_HS,
    #[doc = "78 - DCMI global interrupt"]
    DCMI,
    #[doc = "79 - CRYP crypto global interrupt"]
    CRYP,
    #[doc = "80 - Hash and Rng global interrupt"]
    HASH_RNG,
    #[doc = "81 - Floating point interrupt"]
    FPU,
    #[doc = "82 - UART 7 global interrupt"]
    UART7,
    #[doc = "83 - UART 8 global interrupt"]
    UART8,
    #[doc = "84 - SPI 4 global interrupt"]
    SPI4,
    #[doc = "85 - SPI 5 global interrupt"]
    SPI5,
    #[doc = "86 - SPI 6 global interrupt"]
    SPI6,
    #[doc = "87 - SAI1 global interrupt"]
    SAI1,
    #[doc = "88 - LTDC global interrupt"]
    LCD_TFT,
    #[doc = "89 - LTDC global error interrupt"]
    LCD_TFT_1,
    #[doc = "90 - DMA2D global interrupt"]
    DMA2D,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WWDG => 0,
            Interrupt::PVD => 1,
            Interrupt::TAMP_STAMP => 2,
            Interrupt::RTC_WKUP => 3,
            Interrupt::FLASH => 4,
            Interrupt::RCC => 5,
            Interrupt::EXTI0 => 6,
            Interrupt::EXTI1 => 7,
            Interrupt::EXTI2 => 8,
            Interrupt::EXTI3 => 9,
            Interrupt::EXTI4 => 10,
            Interrupt::DMA1_STREAM0 => 11,
            Interrupt::DMA1_STREAM1 => 12,
            Interrupt::DMA1_STREAM2 => 13,
            Interrupt::DMA1_STREAM3 => 14,
            Interrupt::DMA1_STREAM4 => 15,
            Interrupt::DMA1_STREAM5 => 16,
            Interrupt::DMA1_STREAM6 => 17,
            Interrupt::ADC => 18,
            Interrupt::CAN1_TX => 19,
            Interrupt::CAN1_RX0 => 20,
            Interrupt::CAN1_RX1 => 21,
            Interrupt::CAN1_SCE => 22,
            Interrupt::EXTI9_5 => 23,
            Interrupt::TIM1_BRK_TIM9 => 24,
            Interrupt::TIM1_UP_TIM10 => 25,
            Interrupt::TIM1_TRG_COM_TIM11 => 26,
            Interrupt::TIM1_CC => 27,
            Interrupt::TIM2 => 28,
            Interrupt::TIM3 => 29,
            Interrupt::TIM4 => 30,
            Interrupt::I2C1_EV => 31,
            Interrupt::I2C1_ER => 32,
            Interrupt::I2C2_EV => 33,
            Interrupt::I2C2_ER => 34,
            Interrupt::SPI1 => 35,
            Interrupt::SPI2 => 36,
            Interrupt::USART1 => 37,
            Interrupt::USART2 => 38,
            Interrupt::USART3 => 39,
            Interrupt::EXTI15_10 => 40,
            Interrupt::RTC_ALARM => 41,
            Interrupt::OTG_FS_WKUP => 42,
            Interrupt::TIM8_BRK_TIM12 => 43,
            Interrupt::TIM8_UP_TIM13 => 44,
            Interrupt::TIM8_TRG_COM_TIM14 => 45,
            Interrupt::TIM8_CC => 46,
            Interrupt::DMA1_STREAM7 => 47,
            Interrupt::FMC => 48,
            Interrupt::SDIO => 49,
            Interrupt::TIM5 => 50,
            Interrupt::SPI3 => 51,
            Interrupt::UART4 => 52,
            Interrupt::UART5 => 53,
            Interrupt::TIM6_DAC => 54,
            Interrupt::TIM7 => 55,
            Interrupt::DMA2_STREAM0 => 56,
            Interrupt::DMA2_STREAM1 => 57,
            Interrupt::DMA2_STREAM2 => 58,
            Interrupt::DMA2_STREAM3 => 59,
            Interrupt::DMA2_STREAM4 => 60,
            Interrupt::ETH => 61,
            Interrupt::ETH_WKUP => 62,
            Interrupt::CAN2_TX => 63,
            Interrupt::CAN2_RX0 => 64,
            Interrupt::CAN2_RX1 => 65,
            Interrupt::CAN2_SCE => 66,
            Interrupt::OTG_FS => 67,
            Interrupt::DMA2_STREAM5 => 68,
            Interrupt::DMA2_STREAM6 => 69,
            Interrupt::DMA2_STREAM7 => 70,
            Interrupt::USART6 => 71,
            Interrupt::I2C3_EV => 72,
            Interrupt::I2C3_ER => 73,
            Interrupt::OTG_HS_EP1_OUT => 74,
            Interrupt::OTG_HS_EP1_IN => 75,
            Interrupt::OTG_HS_WKUP => 76,
            Interrupt::OTG_HS => 77,
            Interrupt::DCMI => 78,
            Interrupt::CRYP => 79,
            Interrupt::HASH_RNG => 80,
            Interrupt::FPU => 81,
            Interrupt::UART7 => 82,
            Interrupt::UART8 => 83,
            Interrupt::SPI4 => 84,
            Interrupt::SPI5 => 85,
            Interrupt::SPI6 => 86,
            Interrupt::SAI1 => 87,
            Interrupt::LCD_TFT => 88,
            Interrupt::LCD_TFT_1 => 89,
            Interrupt::DMA2D => 90,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Random number generator"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rng::RegisterBlock {
        1342572544 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    fn deref(&self) -> &rng::RegisterBlock {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "Random number generator"]
pub mod rng;
#[doc = "Hash processor"]
pub struct HASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HASH {}
impl HASH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hash::RegisterBlock {
        1342571520 as *const _
    }
}
impl Deref for HASH {
    type Target = hash::RegisterBlock;
    fn deref(&self) -> &hash::RegisterBlock {
        unsafe { &*HASH::ptr() }
    }
}
#[doc = "Hash processor"]
pub mod hash;
#[doc = "Cryptographic processor"]
pub struct CRYP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYP {}
impl CRYP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const cryp::RegisterBlock {
        1342570496 as *const _
    }
}
impl Deref for CRYP {
    type Target = cryp::RegisterBlock;
    fn deref(&self) -> &cryp::RegisterBlock {
        unsafe { &*CRYP::ptr() }
    }
}
#[doc = "Cryptographic processor"]
pub mod cryp;
#[doc = "Digital camera interface"]
pub struct DCMI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCMI {}
impl DCMI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dcmi::RegisterBlock {
        1342504960 as *const _
    }
}
impl Deref for DCMI {
    type Target = dcmi::RegisterBlock;
    fn deref(&self) -> &dcmi::RegisterBlock {
        unsafe { &*DCMI::ptr() }
    }
}
#[doc = "Digital camera interface"]
pub mod dcmi;
#[doc = "Flexible memory controller"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fmc::RegisterBlock {
        2684354560 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    fn deref(&self) -> &fmc::RegisterBlock {
        unsafe { &*FMC::ptr() }
    }
}
#[doc = "Flexible memory controller"]
pub mod fmc;
#[doc = "Debug support"]
pub struct DBG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBG {}
impl DBG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dbg::RegisterBlock {
        3758366720 as *const _
    }
}
impl Deref for DBG {
    type Target = dbg::RegisterBlock;
    fn deref(&self) -> &dbg::RegisterBlock {
        unsafe { &*DBG::ptr() }
    }
}
#[doc = "Debug support"]
pub mod dbg;
#[doc = "DMA controller"]
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma2::RegisterBlock {
        1073898496 as *const _
    }
}
impl Deref for DMA2 {
    type Target = dma2::RegisterBlock;
    fn deref(&self) -> &dma2::RegisterBlock {
        unsafe { &*DMA2::ptr() }
    }
}
#[doc = "DMA controller"]
pub mod dma2;
#[doc = "DMA1"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma2::RegisterBlock {
        1073897472 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma2::RegisterBlock;
    fn deref(&self) -> &dma2::RegisterBlock {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "Reset and clock control"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rcc::RegisterBlock {
        1073887232 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    fn deref(&self) -> &rcc::RegisterBlock {
        unsafe { &*RCC::ptr() }
    }
}
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "General-purpose I/Os"]
pub struct GPIOK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOK {}
impl GPIOK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiok::RegisterBlock {
        1073883136 as *const _
    }
}
impl Deref for GPIOK {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        unsafe { &*GPIOK::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiok;
#[doc = "GPIOJ"]
pub struct GPIOJ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOJ {}
impl GPIOJ {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiok::RegisterBlock {
        1073882112 as *const _
    }
}
impl Deref for GPIOJ {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        unsafe { &*GPIOJ::ptr() }
    }
}
#[doc = "GPIOI"]
pub struct GPIOI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOI {}
impl GPIOI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiok::RegisterBlock {
        1073881088 as *const _
    }
}
impl Deref for GPIOI {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        unsafe { &*GPIOI::ptr() }
    }
}
#[doc = "GPIOH"]
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiok::RegisterBlock {
        1073880064 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        unsafe { &*GPIOH::ptr() }
    }
}
#[doc = "GPIOG"]
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOG {}
impl GPIOG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiok::RegisterBlock {
        1073879040 as *const _
    }
}
impl Deref for GPIOG {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        unsafe { &*GPIOG::ptr() }
    }
}
#[doc = "GPIOF"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiok::RegisterBlock {
        1073878016 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "GPIOE"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiok::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "GPIOD"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiok::RegisterBlock {
        1073875968 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "GPIOC"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiok::RegisterBlock {
        1073874944 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpiok::RegisterBlock;
    fn deref(&self) -> &gpiok::RegisterBlock {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiob::RegisterBlock {
        1073873920 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &gpiob::RegisterBlock {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiob;
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "System configuration controller"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const syscfg::RegisterBlock {
        1073821696 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    fn deref(&self) -> &syscfg::RegisterBlock {
        unsafe { &*SYSCFG::ptr() }
    }
}
#[doc = "System configuration controller"]
pub mod syscfg;
#[doc = "Serial peripheral interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub mod spi1;
#[doc = "SPI2"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073756160 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "SPI3"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073757184 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI3::ptr() }
    }
}
#[doc = "I2S2ext"]
pub struct I2S2EXT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S2EXT {}
impl I2S2EXT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073755136 as *const _
    }
}
impl Deref for I2S2EXT {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*I2S2EXT::ptr() }
    }
}
#[doc = "I2S3ext"]
pub struct I2S3EXT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S3EXT {}
impl I2S3EXT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for I2S3EXT {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*I2S3EXT::ptr() }
    }
}
#[doc = "SPI4"]
pub struct SPI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI4 {}
impl SPI4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073820672 as *const _
    }
}
impl Deref for SPI4 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI4::ptr() }
    }
}
#[doc = "SPI5"]
pub struct SPI5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI5 {}
impl SPI5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073827840 as *const _
    }
}
impl Deref for SPI5 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI5::ptr() }
    }
}
#[doc = "SPI6"]
pub struct SPI6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI6 {}
impl SPI6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073828864 as *const _
    }
}
impl Deref for SPI6 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI6::ptr() }
    }
}
#[doc = "Secure digital input/output interface"]
pub struct SDIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDIO {}
impl SDIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sdio::RegisterBlock {
        1073818624 as *const _
    }
}
impl Deref for SDIO {
    type Target = sdio::RegisterBlock;
    fn deref(&self) -> &sdio::RegisterBlock {
        unsafe { &*SDIO::ptr() }
    }
}
#[doc = "Secure digital input/output interface"]
pub mod sdio;
#[doc = "Analog-to-digital converter"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1::RegisterBlock {
        1073815552 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Analog-to-digital converter"]
pub mod adc1;
#[doc = "ADC2"]
pub struct ADC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC2 {}
impl ADC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1::RegisterBlock {
        1073815808 as *const _
    }
}
impl Deref for ADC2 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        unsafe { &*ADC2::ptr() }
    }
}
#[doc = "ADC3"]
pub struct ADC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC3 {}
impl ADC3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1::RegisterBlock {
        1073816064 as *const _
    }
}
impl Deref for ADC3 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        unsafe { &*ADC3::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART6 {}
impl USART6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart6::RegisterBlock {
        1073812480 as *const _
    }
}
impl Deref for USART6 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        unsafe { &*USART6::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart6;
#[doc = "USART1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart6::RegisterBlock {
        1073811456 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "USART2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart6::RegisterBlock {
        1073759232 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "USART3"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart6::RegisterBlock {
        1073760256 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "UART7"]
pub struct UART7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART7 {}
impl UART7 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart6::RegisterBlock {
        1073772544 as *const _
    }
}
impl Deref for UART7 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        unsafe { &*UART7::ptr() }
    }
}
#[doc = "UART8"]
pub struct UART8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART8 {}
impl UART8 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart6::RegisterBlock {
        1073773568 as *const _
    }
}
impl Deref for UART8 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        unsafe { &*UART8::ptr() }
    }
}
#[doc = "Digital-to-analog converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac::RegisterBlock {
        1073771520 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &dac::RegisterBlock {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital-to-analog converter"]
pub mod dac;
#[doc = "Power control"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwr::RegisterBlock {
        1073770496 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    fn deref(&self) -> &pwr::RegisterBlock {
        unsafe { &*PWR::ptr() }
    }
}
#[doc = "Power control"]
pub mod pwr;
#[doc = "Independent watchdog"]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iwdg::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    fn deref(&self) -> &iwdg::RegisterBlock {
        unsafe { &*IWDG::ptr() }
    }
}
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "Window watchdog"]
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdg::RegisterBlock {
        1073753088 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    fn deref(&self) -> &wwdg::RegisterBlock {
        unsafe { &*WWDG::ptr() }
    }
}
#[doc = "Window watchdog"]
pub mod wwdg;
#[doc = "Real-time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073752064 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart4::RegisterBlock {
        1073761280 as *const _
    }
}
impl Deref for UART4 {
    type Target = uart4::RegisterBlock;
    fn deref(&self) -> &uart4::RegisterBlock {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod uart4;
#[doc = "UART5"]
pub struct UART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART5 {}
impl UART5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart4::RegisterBlock {
        1073762304 as *const _
    }
}
impl Deref for UART5 {
    type Target = uart4::RegisterBlock;
    fn deref(&self) -> &uart4::RegisterBlock {
        unsafe { &*UART5::ptr() }
    }
}
#[doc = "Common ADC registers"]
pub struct C_ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for C_ADC {}
impl C_ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const c_adc::RegisterBlock {
        1073816320 as *const _
    }
}
impl Deref for C_ADC {
    type Target = c_adc::RegisterBlock;
    fn deref(&self) -> &c_adc::RegisterBlock {
        unsafe { &*C_ADC::ptr() }
    }
}
#[doc = "Common ADC registers"]
pub mod c_adc;
#[doc = "Advanced-timers"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim1::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        unsafe { &*TIM1::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub mod tim1;
#[doc = "TIM8"]
pub struct TIM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM8 {}
impl TIM8 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim1::RegisterBlock {
        1073808384 as *const _
    }
}
impl Deref for TIM8 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        unsafe { &*TIM8::ptr() }
    }
}
#[doc = "General purpose timers"]
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM2::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim2;
#[doc = "General purpose timers"]
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim3::RegisterBlock {
        1073742848 as *const _
    }
}
impl Deref for TIM3 {
    type Target = tim3::RegisterBlock;
    fn deref(&self) -> &tim3::RegisterBlock {
        unsafe { &*TIM3::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim3;
#[doc = "TIM4"]
pub struct TIM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM4 {}
impl TIM4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim3::RegisterBlock {
        1073743872 as *const _
    }
}
impl Deref for TIM4 {
    type Target = tim3::RegisterBlock;
    fn deref(&self) -> &tim3::RegisterBlock {
        unsafe { &*TIM4::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM5 {}
impl TIM5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim5::RegisterBlock {
        1073744896 as *const _
    }
}
impl Deref for TIM5 {
    type Target = tim5::RegisterBlock;
    fn deref(&self) -> &tim5::RegisterBlock {
        unsafe { &*TIM5::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim5;
#[doc = "General purpose timers"]
pub struct TIM9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM9 {}
impl TIM9 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim9::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for TIM9 {
    type Target = tim9::RegisterBlock;
    fn deref(&self) -> &tim9::RegisterBlock {
        unsafe { &*TIM9::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim9;
#[doc = "TIM12"]
pub struct TIM12 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM12 {}
impl TIM12 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim9::RegisterBlock {
        1073747968 as *const _
    }
}
impl Deref for TIM12 {
    type Target = tim9::RegisterBlock;
    fn deref(&self) -> &tim9::RegisterBlock {
        unsafe { &*TIM12::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM10 {}
impl TIM10 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim10::RegisterBlock {
        1073824768 as *const _
    }
}
impl Deref for TIM10 {
    type Target = tim10::RegisterBlock;
    fn deref(&self) -> &tim10::RegisterBlock {
        unsafe { &*TIM10::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim10;
#[doc = "TIM13"]
pub struct TIM13 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM13 {}
impl TIM13 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim10::RegisterBlock {
        1073748992 as *const _
    }
}
impl Deref for TIM13 {
    type Target = tim10::RegisterBlock;
    fn deref(&self) -> &tim10::RegisterBlock {
        unsafe { &*TIM13::ptr() }
    }
}
#[doc = "TIM14"]
pub struct TIM14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM14 {}
impl TIM14 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim10::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for TIM14 {
    type Target = tim10::RegisterBlock;
    fn deref(&self) -> &tim10::RegisterBlock {
        unsafe { &*TIM14::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM11 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM11 {}
impl TIM11 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim11::RegisterBlock {
        1073825792 as *const _
    }
}
impl Deref for TIM11 {
    type Target = tim11::RegisterBlock;
    fn deref(&self) -> &tim11::RegisterBlock {
        unsafe { &*TIM11::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim11;
#[doc = "Basic timers"]
pub struct TIM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM6 {}
impl TIM6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim6::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        unsafe { &*TIM6::ptr() }
    }
}
#[doc = "Basic timers"]
pub mod tim6;
#[doc = "TIM7"]
pub struct TIM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM7 {}
impl TIM7 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim6::RegisterBlock {
        1073746944 as *const _
    }
}
impl Deref for TIM7 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        unsafe { &*TIM7::ptr() }
    }
}
#[doc = "Ethernet: media access control (MAC)"]
pub struct ETHERNET_MAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_MAC {}
impl ETHERNET_MAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ethernet_mac::RegisterBlock {
        1073905664 as *const _
    }
}
impl Deref for ETHERNET_MAC {
    type Target = ethernet_mac::RegisterBlock;
    fn deref(&self) -> &ethernet_mac::RegisterBlock {
        unsafe { &*ETHERNET_MAC::ptr() }
    }
}
#[doc = "Ethernet: media access control (MAC)"]
pub mod ethernet_mac;
#[doc = "Ethernet: MAC management counters"]
pub struct ETHERNET_MMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_MMC {}
impl ETHERNET_MMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ethernet_mmc::RegisterBlock {
        1073905920 as *const _
    }
}
impl Deref for ETHERNET_MMC {
    type Target = ethernet_mmc::RegisterBlock;
    fn deref(&self) -> &ethernet_mmc::RegisterBlock {
        unsafe { &*ETHERNET_MMC::ptr() }
    }
}
#[doc = "Ethernet: MAC management counters"]
pub mod ethernet_mmc;
#[doc = "Ethernet: Precision time protocol"]
pub struct ETHERNET_PTP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_PTP {}
impl ETHERNET_PTP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ethernet_ptp::RegisterBlock {
        1073907456 as *const _
    }
}
impl Deref for ETHERNET_PTP {
    type Target = ethernet_ptp::RegisterBlock;
    fn deref(&self) -> &ethernet_ptp::RegisterBlock {
        unsafe { &*ETHERNET_PTP::ptr() }
    }
}
#[doc = "Ethernet: Precision time protocol"]
pub mod ethernet_ptp;
#[doc = "Ethernet: DMA controller operation"]
pub struct ETHERNET_DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_DMA {}
impl ETHERNET_DMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ethernet_dma::RegisterBlock {
        1073909760 as *const _
    }
}
impl Deref for ETHERNET_DMA {
    type Target = ethernet_dma::RegisterBlock;
    fn deref(&self) -> &ethernet_dma::RegisterBlock {
        unsafe { &*ETHERNET_DMA::ptr() }
    }
}
#[doc = "Ethernet: DMA controller operation"]
pub mod ethernet_dma;
#[doc = "Cryptographic processor"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1073885184 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cryptographic processor"]
pub mod crc;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_GLOBAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_GLOBAL {}
impl OTG_FS_GLOBAL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_global::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for OTG_FS_GLOBAL {
    type Target = otg_fs_global::RegisterBlock;
    fn deref(&self) -> &otg_fs_global::RegisterBlock {
        unsafe { &*OTG_FS_GLOBAL::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_global;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_HOST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_HOST {}
impl OTG_FS_HOST {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_host::RegisterBlock {
        1342178304 as *const _
    }
}
impl Deref for OTG_FS_HOST {
    type Target = otg_fs_host::RegisterBlock;
    fn deref(&self) -> &otg_fs_host::RegisterBlock {
        unsafe { &*OTG_FS_HOST::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_host;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_DEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_DEVICE {}
impl OTG_FS_DEVICE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_device::RegisterBlock {
        1342179328 as *const _
    }
}
impl Deref for OTG_FS_DEVICE {
    type Target = otg_fs_device::RegisterBlock;
    fn deref(&self) -> &otg_fs_device::RegisterBlock {
        unsafe { &*OTG_FS_DEVICE::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_device;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_PWRCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_PWRCLK {}
impl OTG_FS_PWRCLK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_pwrclk::RegisterBlock {
        1342180864 as *const _
    }
}
impl Deref for OTG_FS_PWRCLK {
    type Target = otg_fs_pwrclk::RegisterBlock;
    fn deref(&self) -> &otg_fs_pwrclk::RegisterBlock {
        unsafe { &*OTG_FS_PWRCLK::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_pwrclk;
#[doc = "Controller area network"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can1::RegisterBlock {
        1073767424 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &can1::RegisterBlock {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "Controller area network"]
pub mod can1;
#[doc = "CAN2"]
pub struct CAN2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN2 {}
impl CAN2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can1::RegisterBlock {
        1073768448 as *const _
    }
}
impl Deref for CAN2 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &can1::RegisterBlock {
        unsafe { &*CAN2::ptr() }
    }
}
#[doc = "FLASH"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flash::RegisterBlock {
        1073888256 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &flash::RegisterBlock {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "FLASH"]
pub mod flash;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const exti::RegisterBlock {
        1073822720 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    fn deref(&self) -> &exti::RegisterBlock {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "USB on the go high speed"]
pub struct OTG_HS_GLOBAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_HS_GLOBAL {}
impl OTG_HS_GLOBAL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_hs_global::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for OTG_HS_GLOBAL {
    type Target = otg_hs_global::RegisterBlock;
    fn deref(&self) -> &otg_hs_global::RegisterBlock {
        unsafe { &*OTG_HS_GLOBAL::ptr() }
    }
}
#[doc = "USB on the go high speed"]
pub mod otg_hs_global;
#[doc = "USB on the go high speed"]
pub struct OTG_HS_HOST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_HS_HOST {}
impl OTG_HS_HOST {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_hs_host::RegisterBlock {
        1074004992 as *const _
    }
}
impl Deref for OTG_HS_HOST {
    type Target = otg_hs_host::RegisterBlock;
    fn deref(&self) -> &otg_hs_host::RegisterBlock {
        unsafe { &*OTG_HS_HOST::ptr() }
    }
}
#[doc = "USB on the go high speed"]
pub mod otg_hs_host;
#[doc = "USB on the go high speed"]
pub struct OTG_HS_DEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_HS_DEVICE {}
impl OTG_HS_DEVICE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_hs_device::RegisterBlock {
        1074006016 as *const _
    }
}
impl Deref for OTG_HS_DEVICE {
    type Target = otg_hs_device::RegisterBlock;
    fn deref(&self) -> &otg_hs_device::RegisterBlock {
        unsafe { &*OTG_HS_DEVICE::ptr() }
    }
}
#[doc = "USB on the go high speed"]
pub mod otg_hs_device;
#[doc = "USB on the go high speed"]
pub struct OTG_HS_PWRCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_HS_PWRCLK {}
impl OTG_HS_PWRCLK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_hs_pwrclk::RegisterBlock {
        1074007552 as *const _
    }
}
impl Deref for OTG_HS_PWRCLK {
    type Target = otg_hs_pwrclk::RegisterBlock;
    fn deref(&self) -> &otg_hs_pwrclk::RegisterBlock {
        unsafe { &*OTG_HS_PWRCLK::ptr() }
    }
}
#[doc = "USB on the go high speed"]
pub mod otg_hs_pwrclk;
#[doc = "LCD-TFT Controller"]
pub struct LTDC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LTDC {}
impl LTDC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ltdc::RegisterBlock {
        1073833984 as *const _
    }
}
impl Deref for LTDC {
    type Target = ltdc::RegisterBlock;
    fn deref(&self) -> &ltdc::RegisterBlock {
        unsafe { &*LTDC::ptr() }
    }
}
#[doc = "LCD-TFT Controller"]
pub mod ltdc;
#[doc = "Serial audio interface"]
pub struct SAI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI {}
impl SAI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sai::RegisterBlock {
        1073829888 as *const _
    }
}
impl Deref for SAI {
    type Target = sai::RegisterBlock;
    fn deref(&self) -> &sai::RegisterBlock {
        unsafe { &*SAI::ptr() }
    }
}
#[doc = "Serial audio interface"]
pub mod sai;
#[doc = "DMA2D controller"]
pub struct DMA2D {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2D {}
impl DMA2D {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma2d::RegisterBlock {
        1073917952 as *const _
    }
}
impl Deref for DMA2D {
    type Target = dma2d::RegisterBlock;
    fn deref(&self) -> &dma2d::RegisterBlock {
        unsafe { &*DMA2D::ptr() }
    }
}
#[doc = "DMA2D controller"]
pub mod dma2d;
#[doc = "Inter-integrated circuit"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c3::RegisterBlock {
        1073765376 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c3::RegisterBlock;
    fn deref(&self) -> &i2c3::RegisterBlock {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub mod i2c3;
#[doc = "I2C2"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c3::RegisterBlock {
        1073764352 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c3::RegisterBlock;
    fn deref(&self) -> &i2c3::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "I2C1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c3::RegisterBlock {
        1073763328 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c3::RegisterBlock;
    fn deref(&self) -> &i2c3::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Floting point unit"]
pub struct FPU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPU {}
impl FPU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fpu::RegisterBlock {
        3758157620 as *const _
    }
}
impl Deref for FPU {
    type Target = fpu::RegisterBlock;
    fn deref(&self) -> &fpu::RegisterBlock {
        unsafe { &*FPU::ptr() }
    }
}
#[doc = "Floting point unit"]
pub mod fpu;
#[doc = "SysTick timer"]
pub struct STK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STK {}
impl STK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const stk::RegisterBlock {
        3758153744 as *const _
    }
}
impl Deref for STK {
    type Target = stk::RegisterBlock;
    fn deref(&self) -> &stk::RegisterBlock {
        unsafe { &*STK::ptr() }
    }
}
#[doc = "SysTick timer"]
pub mod stk;
#[doc = "Nested vectored interrupt controller"]
pub struct NVIC_STIR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVIC_STIR {}
impl NVIC_STIR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nvic_stir::RegisterBlock {
        3758157568 as *const _
    }
}
impl Deref for NVIC_STIR {
    type Target = nvic_stir::RegisterBlock;
    fn deref(&self) -> &nvic_stir::RegisterBlock {
        unsafe { &*NVIC_STIR::ptr() }
    }
}
#[doc = "Nested vectored interrupt controller"]
pub mod nvic_stir;
#[doc = "Floating point unit CPACR"]
pub struct FPU_CPACR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FPU_CPACR {}
impl FPU_CPACR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fpu_cpacr::RegisterBlock {
        3758157192 as *const _
    }
}
impl Deref for FPU_CPACR {
    type Target = fpu_cpacr::RegisterBlock;
    fn deref(&self) -> &fpu_cpacr::RegisterBlock {
        unsafe { &*FPU_CPACR::ptr() }
    }
}
#[doc = "Floating point unit CPACR"]
pub mod fpu_cpacr;
#[doc = "System control block ACTLR"]
pub struct SCB_ACTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCB_ACTRL {}
impl SCB_ACTRL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const scb_actrl::RegisterBlock {
        3758153736 as *const _
    }
}
impl Deref for SCB_ACTRL {
    type Target = scb_actrl::RegisterBlock;
    fn deref(&self) -> &scb_actrl::RegisterBlock {
        unsafe { &*SCB_ACTRL::ptr() }
    }
}
#[doc = "System control block ACTLR"]
pub mod scb_actrl;
#[doc = "Unique device ID"]
pub struct DEVICE_ID {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DEVICE_ID {}
impl DEVICE_ID {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const device_id::RegisterBlock {
        536836624 as *const _
    }
}
impl Deref for DEVICE_ID {
    type Target = device_id::RegisterBlock;
    fn deref(&self) -> &device_id::RegisterBlock {
        unsafe { &*DEVICE_ID::ptr() }
    }
}
#[doc = "Unique device ID"]
pub mod device_id;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "HASH"]
    pub HASH: HASH,
    #[doc = "CRYP"]
    pub CRYP: CRYP,
    #[doc = "DCMI"]
    pub DCMI: DCMI,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "DBG"]
    pub DBG: DBG,
    #[doc = "DMA2"]
    pub DMA2: DMA2,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "GPIOK"]
    pub GPIOK: GPIOK,
    #[doc = "GPIOJ"]
    pub GPIOJ: GPIOJ,
    #[doc = "GPIOI"]
    pub GPIOI: GPIOI,
    #[doc = "GPIOH"]
    pub GPIOH: GPIOH,
    #[doc = "GPIOG"]
    pub GPIOG: GPIOG,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "I2S2EXT"]
    pub I2S2EXT: I2S2EXT,
    #[doc = "I2S3EXT"]
    pub I2S3EXT: I2S3EXT,
    #[doc = "SPI4"]
    pub SPI4: SPI4,
    #[doc = "SPI5"]
    pub SPI5: SPI5,
    #[doc = "SPI6"]
    pub SPI6: SPI6,
    #[doc = "SDIO"]
    pub SDIO: SDIO,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "ADC2"]
    pub ADC2: ADC2,
    #[doc = "ADC3"]
    pub ADC3: ADC3,
    #[doc = "USART6"]
    pub USART6: USART6,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "UART7"]
    pub UART7: UART7,
    #[doc = "UART8"]
    pub UART8: UART8,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "IWDG"]
    pub IWDG: IWDG,
    #[doc = "WWDG"]
    pub WWDG: WWDG,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "UART5"]
    pub UART5: UART5,
    #[doc = "C_ADC"]
    pub C_ADC: C_ADC,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "TIM8"]
    pub TIM8: TIM8,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "TIM3"]
    pub TIM3: TIM3,
    #[doc = "TIM4"]
    pub TIM4: TIM4,
    #[doc = "TIM5"]
    pub TIM5: TIM5,
    #[doc = "TIM9"]
    pub TIM9: TIM9,
    #[doc = "TIM12"]
    pub TIM12: TIM12,
    #[doc = "TIM10"]
    pub TIM10: TIM10,
    #[doc = "TIM13"]
    pub TIM13: TIM13,
    #[doc = "TIM14"]
    pub TIM14: TIM14,
    #[doc = "TIM11"]
    pub TIM11: TIM11,
    #[doc = "TIM6"]
    pub TIM6: TIM6,
    #[doc = "TIM7"]
    pub TIM7: TIM7,
    #[doc = "ETHERNET_MAC"]
    pub ETHERNET_MAC: ETHERNET_MAC,
    #[doc = "ETHERNET_MMC"]
    pub ETHERNET_MMC: ETHERNET_MMC,
    #[doc = "ETHERNET_PTP"]
    pub ETHERNET_PTP: ETHERNET_PTP,
    #[doc = "ETHERNET_DMA"]
    pub ETHERNET_DMA: ETHERNET_DMA,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "OTG_FS_GLOBAL"]
    pub OTG_FS_GLOBAL: OTG_FS_GLOBAL,
    #[doc = "OTG_FS_HOST"]
    pub OTG_FS_HOST: OTG_FS_HOST,
    #[doc = "OTG_FS_DEVICE"]
    pub OTG_FS_DEVICE: OTG_FS_DEVICE,
    #[doc = "OTG_FS_PWRCLK"]
    pub OTG_FS_PWRCLK: OTG_FS_PWRCLK,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "CAN2"]
    pub CAN2: CAN2,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "OTG_HS_GLOBAL"]
    pub OTG_HS_GLOBAL: OTG_HS_GLOBAL,
    #[doc = "OTG_HS_HOST"]
    pub OTG_HS_HOST: OTG_HS_HOST,
    #[doc = "OTG_HS_DEVICE"]
    pub OTG_HS_DEVICE: OTG_HS_DEVICE,
    #[doc = "OTG_HS_PWRCLK"]
    pub OTG_HS_PWRCLK: OTG_HS_PWRCLK,
    #[doc = "LTDC"]
    pub LTDC: LTDC,
    #[doc = "SAI"]
    pub SAI: SAI,
    #[doc = "DMA2D"]
    pub DMA2D: DMA2D,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "FPU"]
    pub FPU: FPU,
    #[doc = "STK"]
    pub STK: STK,
    #[doc = "NVIC_STIR"]
    pub NVIC_STIR: NVIC_STIR,
    #[doc = "FPU_CPACR"]
    pub FPU_CPACR: FPU_CPACR,
    #[doc = "SCB_ACTRL"]
    pub SCB_ACTRL: SCB_ACTRL,
    #[doc = "DEVICE_ID"]
    pub DEVICE_ID: DEVICE_ID,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            RNG: RNG {
                _marker: PhantomData,
            },
            HASH: HASH {
                _marker: PhantomData,
            },
            CRYP: CRYP {
                _marker: PhantomData,
            },
            DCMI: DCMI {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            DBG: DBG {
                _marker: PhantomData,
            },
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            GPIOK: GPIOK {
                _marker: PhantomData,
            },
            GPIOJ: GPIOJ {
                _marker: PhantomData,
            },
            GPIOI: GPIOI {
                _marker: PhantomData,
            },
            GPIOH: GPIOH {
                _marker: PhantomData,
            },
            GPIOG: GPIOG {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            I2S2EXT: I2S2EXT {
                _marker: PhantomData,
            },
            I2S3EXT: I2S3EXT {
                _marker: PhantomData,
            },
            SPI4: SPI4 {
                _marker: PhantomData,
            },
            SPI5: SPI5 {
                _marker: PhantomData,
            },
            SPI6: SPI6 {
                _marker: PhantomData,
            },
            SDIO: SDIO {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            ADC2: ADC2 {
                _marker: PhantomData,
            },
            ADC3: ADC3 {
                _marker: PhantomData,
            },
            USART6: USART6 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            UART7: UART7 {
                _marker: PhantomData,
            },
            UART8: UART8 {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            IWDG: IWDG {
                _marker: PhantomData,
            },
            WWDG: WWDG {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            UART5: UART5 {
                _marker: PhantomData,
            },
            C_ADC: C_ADC {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            TIM8: TIM8 {
                _marker: PhantomData,
            },
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            TIM3: TIM3 {
                _marker: PhantomData,
            },
            TIM4: TIM4 {
                _marker: PhantomData,
            },
            TIM5: TIM5 {
                _marker: PhantomData,
            },
            TIM9: TIM9 {
                _marker: PhantomData,
            },
            TIM12: TIM12 {
                _marker: PhantomData,
            },
            TIM10: TIM10 {
                _marker: PhantomData,
            },
            TIM13: TIM13 {
                _marker: PhantomData,
            },
            TIM14: TIM14 {
                _marker: PhantomData,
            },
            TIM11: TIM11 {
                _marker: PhantomData,
            },
            TIM6: TIM6 {
                _marker: PhantomData,
            },
            TIM7: TIM7 {
                _marker: PhantomData,
            },
            ETHERNET_MAC: ETHERNET_MAC {
                _marker: PhantomData,
            },
            ETHERNET_MMC: ETHERNET_MMC {
                _marker: PhantomData,
            },
            ETHERNET_PTP: ETHERNET_PTP {
                _marker: PhantomData,
            },
            ETHERNET_DMA: ETHERNET_DMA {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            OTG_FS_GLOBAL: OTG_FS_GLOBAL {
                _marker: PhantomData,
            },
            OTG_FS_HOST: OTG_FS_HOST {
                _marker: PhantomData,
            },
            OTG_FS_DEVICE: OTG_FS_DEVICE {
                _marker: PhantomData,
            },
            OTG_FS_PWRCLK: OTG_FS_PWRCLK {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            CAN2: CAN2 {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            OTG_HS_GLOBAL: OTG_HS_GLOBAL {
                _marker: PhantomData,
            },
            OTG_HS_HOST: OTG_HS_HOST {
                _marker: PhantomData,
            },
            OTG_HS_DEVICE: OTG_HS_DEVICE {
                _marker: PhantomData,
            },
            OTG_HS_PWRCLK: OTG_HS_PWRCLK {
                _marker: PhantomData,
            },
            LTDC: LTDC {
                _marker: PhantomData,
            },
            SAI: SAI {
                _marker: PhantomData,
            },
            DMA2D: DMA2D {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            FPU: FPU {
                _marker: PhantomData,
            },
            STK: STK {
                _marker: PhantomData,
            },
            NVIC_STIR: NVIC_STIR {
                _marker: PhantomData,
            },
            FPU_CPACR: FPU_CPACR {
                _marker: PhantomData,
            },
            SCB_ACTRL: SCB_ACTRL {
                _marker: PhantomData,
            },
            DEVICE_ID: DEVICE_ID {
                _marker: PhantomData,
            },
        }
    }
}

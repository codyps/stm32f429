#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLLCFGR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct PLLQR {
    bits: u8,
}
impl PLLQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PLLSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLSRCR {
    #[doc = "undocumented"]
    HSI,
    #[doc = "undocumented"]
    HSE,
}
impl PLLSRCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            PLLSRCR::HSI => false,
            PLLSRCR::HSE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLSRCR {
        match value {
            false => PLLSRCR::HSI,
            true => PLLSRCR::HSE,
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline]
    pub fn is_hsi(&self) -> bool {
        *self == PLLSRCR::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRCR::HSE
    }
}
#[doc = "Possible values of the field `PLLP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLPR {
    #[doc = "undocumented"]
    DIV2,
    #[doc = "undocumented"]
    DIV4,
    #[doc = "undocumented"]
    DIV6,
    #[doc = "undocumented"]
    DIV8,
}
impl PLLPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLLPR::DIV2 => 0,
            PLLPR::DIV4 => 1,
            PLLPR::DIV6 => 2,
            PLLPR::DIV8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLLPR {
        match value {
            0 => PLLPR::DIV2,
            1 => PLLPR::DIV4,
            2 => PLLPR::DIV6,
            3 => PLLPR::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PLLPR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PLLPR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline]
    pub fn is_div6(&self) -> bool {
        *self == PLLPR::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PLLPR::DIV8
    }
}
#[doc = r" Value of the field"]
pub struct PLLNR {
    bits: u16,
}
impl PLLNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PLLMR {
    bits: u8,
}
impl PLLMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PLLQW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLSRC`"]
pub enum PLLSRCW {
    #[doc = "`0`"]
    HSI,
    #[doc = "`1`"]
    HSE,
}
impl PLLSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLSRCW::HSI => false,
            PLLSRCW::HSE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn hsi(self) -> &'a mut W {
        self.variant(PLLSRCW::HSI)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn hse(self) -> &'a mut W {
        self.variant(PLLSRCW::HSE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLLP`"]
pub enum PLLPW {
    #[doc = "`0`"]
    DIV2,
    #[doc = "`1`"]
    DIV4,
    #[doc = "`10`"]
    DIV6,
    #[doc = "`11`"]
    DIV8,
}
impl PLLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLLPW::DIV2 => 0,
            PLLPW::DIV4 => 1,
            PLLPW::DIV6 => 2,
            PLLPW::DIV8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLPW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLPW::DIV2)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PLLPW::DIV4)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn div6(self) -> &'a mut W {
        self.variant(PLLPW::DIV6)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PLLPW::DIV8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLLNW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLLMW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 24:27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline]
    pub fn pllq(&self) -> PLLQR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLLQR { bits }
    }
    #[doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline]
    pub fn pllsrc(&self) -> PLLSRCR {
        PLLSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:17 - Main PLL (PLL) division factor for main system clock"]
    #[inline]
    pub fn pllp(&self) -> PLLPR {
        PLLPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:14 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline]
    pub fn plln(&self) -> PLLNR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PLLNR { bits }
    }
    #[doc = "Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline]
    pub fn pllm(&self) -> PLLMR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLLMR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 603992080 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline]
    pub fn pllq(&mut self) -> _PLLQW {
        _PLLQW { w: self }
    }
    #[doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline]
    pub fn pllsrc(&mut self) -> _PLLSRCW {
        _PLLSRCW { w: self }
    }
    #[doc = "Bits 16:17 - Main PLL (PLL) division factor for main system clock"]
    #[inline]
    pub fn pllp(&mut self) -> _PLLPW {
        _PLLPW { w: self }
    }
    #[doc = "Bits 6:14 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline]
    pub fn plln(&mut self) -> _PLLNW {
        _PLLNW { w: self }
    }
    #[doc = "Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline]
    pub fn pllm(&mut self) -> _PLLMW {
        _PLLMW { w: self }
    }
}

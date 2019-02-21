#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCKCFGR {
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
pub struct PLLI2SDIVQR {
    bits: u8,
}
impl PLLI2SDIVQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PLLSAIDIVQR {
    bits: u8,
}
impl PLLSAIDIVQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PLLSAIDIVRR {
    bits: u8,
}
impl PLLSAIDIVRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAI1ASRCR {
    bits: u8,
}
impl SAI1ASRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAI1BSRCR {
    bits: u8,
}
impl SAI1BSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TIMPRER {
    bits: bool,
}
impl TIMPRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _PLLI2SDIVQW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLI2SDIVQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLLSAIDIVQW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLSAIDIVQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLLSAIDIVRW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLSAIDIVRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SAI1ASRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI1ASRCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SAI1BSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI1BSRCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIMPREW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMPREW<'a> {
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:4 - PLLI2S division factor for SAI1 clock"]
    #[inline]
    pub fn plli2sdivq(&self) -> PLLI2SDIVQR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLLI2SDIVQR { bits }
    }
    #[doc = "Bits 8:12 - PLLSAI division factor for SAI1 clock"]
    #[inline]
    pub fn pllsaidivq(&self) -> PLLSAIDIVQR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLLSAIDIVQR { bits }
    }
    #[doc = "Bits 16:17 - division factor for LCD_CLK"]
    #[inline]
    pub fn pllsaidivr(&self) -> PLLSAIDIVRR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLLSAIDIVRR { bits }
    }
    #[doc = "Bits 20:21 - SAI1-A clock source selection"]
    #[inline]
    pub fn sai1asrc(&self) -> SAI1ASRCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAI1ASRCR { bits }
    }
    #[doc = "Bits 22:23 - SAI1-B clock source selection"]
    #[inline]
    pub fn sai1bsrc(&self) -> SAI1BSRCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAI1BSRCR { bits }
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline]
    pub fn timpre(&self) -> TIMPRER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIMPRER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - PLLI2S division factor for SAI1 clock"]
    #[inline]
    pub fn plli2sdivq(&mut self) -> _PLLI2SDIVQW {
        _PLLI2SDIVQW { w: self }
    }
    #[doc = "Bits 8:12 - PLLSAI division factor for SAI1 clock"]
    #[inline]
    pub fn pllsaidivq(&mut self) -> _PLLSAIDIVQW {
        _PLLSAIDIVQW { w: self }
    }
    #[doc = "Bits 16:17 - division factor for LCD_CLK"]
    #[inline]
    pub fn pllsaidivr(&mut self) -> _PLLSAIDIVRW {
        _PLLSAIDIVRW { w: self }
    }
    #[doc = "Bits 20:21 - SAI1-A clock source selection"]
    #[inline]
    pub fn sai1asrc(&mut self) -> _SAI1ASRCW {
        _SAI1ASRCW { w: self }
    }
    #[doc = "Bits 22:23 - SAI1-B clock source selection"]
    #[inline]
    pub fn sai1bsrc(&mut self) -> _SAI1BSRCW {
        _SAI1BSRCW { w: self }
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline]
    pub fn timpre(&mut self) -> _TIMPREW {
        _TIMPREW { w: self }
    }
}

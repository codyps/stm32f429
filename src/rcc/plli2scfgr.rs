#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLLI2SCFGR {
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
pub struct PLLI2SRR {
    bits: u8,
}
impl PLLI2SRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PLLI2SQR {
    bits: u8,
}
impl PLLI2SQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PLLI2SNR {
    bits: u16,
}
impl PLLI2SNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PLLI2SRW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLI2SRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLLI2SQW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLI2SQW<'a> {
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
#[doc = r" Proxy"]
pub struct _PLLI2SNW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLI2SNW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline]
    pub fn plli2sr(&self) -> PLLI2SRR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLLI2SRR { bits }
    }
    #[doc = "Bits 24:27 - PLLI2S division factor for SAI1 clock"]
    #[inline]
    pub fn plli2sq(&self) -> PLLI2SQR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLLI2SQR { bits }
    }
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline]
    pub fn plli2sn(&self) -> PLLI2SNR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        PLLI2SNR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 536883200 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline]
    pub fn plli2sr(&mut self) -> _PLLI2SRW {
        _PLLI2SRW { w: self }
    }
    #[doc = "Bits 24:27 - PLLI2S division factor for SAI1 clock"]
    #[inline]
    pub fn plli2sq(&mut self) -> _PLLI2SQW {
        _PLLI2SQW { w: self }
    }
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline]
    pub fn plli2sn(&mut self) -> _PLLI2SNW {
        _PLLI2SNW { w: self }
    }
}

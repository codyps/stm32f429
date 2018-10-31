#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACMIIAR {
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
pub struct MBR {
    bits: bool,
}
impl MBR {
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
#[doc = r" Value of the field"]
pub struct MWR {
    bits: bool,
}
impl MWR {
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
#[doc = "Possible values of the field `CR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRR {
    #[doc = "60-100 MHz HCLK/42"]
    DIV42,
    #[doc = "100-150 MHz HCLK/62"]
    DIV62,
    #[doc = "20-35 MHz HCLK/16"]
    DIV16,
    #[doc = "35-60 MHz HCLK/26"]
    DIV26,
    #[doc = "150-168 MHz HCLK/102"]
    DIV102,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRR::DIV42 => 0,
            CRR::DIV62 => 1,
            CRR::DIV16 => 2,
            CRR::DIV26 => 3,
            CRR::DIV102 => 4,
            CRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRR {
        match value {
            0 => CRR::DIV42,
            1 => CRR::DIV62,
            2 => CRR::DIV16,
            3 => CRR::DIV26,
            4 => CRR::DIV102,
            i => CRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV42`"]
    #[inline]
    pub fn is_div42(&self) -> bool {
        *self == CRR::DIV42
    }
    #[doc = "Checks if the value of the field is `DIV62`"]
    #[inline]
    pub fn is_div62(&self) -> bool {
        *self == CRR::DIV62
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == CRR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV26`"]
    #[inline]
    pub fn is_div26(&self) -> bool {
        *self == CRR::DIV26
    }
    #[doc = "Checks if the value of the field is `DIV102`"]
    #[inline]
    pub fn is_div102(&self) -> bool {
        *self == CRR::DIV102
    }
}
#[doc = r" Value of the field"]
pub struct MRR {
    bits: u8,
}
impl MRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PAR {
    bits: u8,
}
impl PAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _MBW<'a> {
    w: &'a mut W,
}
impl<'a> _MBW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MWW<'a> {
    w: &'a mut W,
}
impl<'a> _MWW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CR`"]
pub enum CRW {
    #[doc = "60-100 MHz HCLK/42"]
    DIV42,
    #[doc = "100-150 MHz HCLK/62"]
    DIV62,
    #[doc = "20-35 MHz HCLK/16"]
    DIV16,
    #[doc = "35-60 MHz HCLK/26"]
    DIV26,
    #[doc = "150-168 MHz HCLK/102"]
    DIV102,
}
impl CRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRW::DIV42 => 0,
            CRW::DIV62 => 1,
            CRW::DIV16 => 2,
            CRW::DIV26 => 3,
            CRW::DIV102 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRW<'a> {
    w: &'a mut W,
}
impl<'a> _CRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "60-100 MHz HCLK/42"]
    #[inline]
    pub fn div42(self) -> &'a mut W {
        self.variant(CRW::DIV42)
    }
    #[doc = "100-150 MHz HCLK/62"]
    #[inline]
    pub fn div62(self) -> &'a mut W {
        self.variant(CRW::DIV62)
    }
    #[doc = "20-35 MHz HCLK/16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(CRW::DIV16)
    }
    #[doc = "35-60 MHz HCLK/26"]
    #[inline]
    pub fn div26(self) -> &'a mut W {
        self.variant(CRW::DIV26)
    }
    #[doc = "150-168 MHz HCLK/102"]
    #[inline]
    pub fn div102(self) -> &'a mut W {
        self.variant(CRW::DIV102)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MRW<'a> {
    w: &'a mut W,
}
impl<'a> _MRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAW<'a> {
    w: &'a mut W,
}
impl<'a> _PAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - MB"]
    #[inline]
    pub fn mb(&self) -> MBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MBR { bits }
    }
    #[doc = "Bit 1 - MW"]
    #[inline]
    pub fn mw(&self) -> MWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MWR { bits }
    }
    #[doc = "Bits 2:4 - Clock Range"]
    #[inline]
    pub fn cr(&self) -> CRR {
        CRR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:10 - MII Register - select the desired MII register in the PHY device"]
    #[inline]
    pub fn mr(&self) -> MRR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MRR { bits }
    }
    #[doc = "Bits 11:15 - PHY address - select which of possible 32 phys is being accessed"]
    #[inline]
    pub fn pa(&self) -> PAR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PAR { bits }
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
    #[doc = "Bit 0 - MB"]
    #[inline]
    pub fn mb(&mut self) -> _MBW {
        _MBW { w: self }
    }
    #[doc = "Bit 1 - MW"]
    #[inline]
    pub fn mw(&mut self) -> _MWW {
        _MWW { w: self }
    }
    #[doc = "Bits 2:4 - Clock Range"]
    #[inline]
    pub fn cr(&mut self) -> _CRW {
        _CRW { w: self }
    }
    #[doc = "Bits 6:10 - MII Register - select the desired MII register in the PHY device"]
    #[inline]
    pub fn mr(&mut self) -> _MRW {
        _MRW { w: self }
    }
    #[doc = "Bits 11:15 - PHY address - select which of possible 32 phys is being accessed"]
    #[inline]
    pub fn pa(&mut self) -> _PAW {
        _PAW { w: self }
    }
}

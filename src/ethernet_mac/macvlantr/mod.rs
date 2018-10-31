#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACVLANTR {
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
pub struct VLANTIR {
    bits: u16,
}
impl VLANTIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `VLANTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLANTCR {
    #[doc = "undocumented"]
    CMP16,
    #[doc = "undocumented"]
    CMP12,
}
impl VLANTCR {
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
            VLANTCR::CMP16 => false,
            VLANTCR::CMP12 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VLANTCR {
        match value {
            false => VLANTCR::CMP16,
            true => VLANTCR::CMP12,
        }
    }
    #[doc = "Checks if the value of the field is `CMP16`"]
    #[inline]
    pub fn is_cmp16(&self) -> bool {
        *self == VLANTCR::CMP16
    }
    #[doc = "Checks if the value of the field is `CMP12`"]
    #[inline]
    pub fn is_cmp12(&self) -> bool {
        *self == VLANTCR::CMP12
    }
}
#[doc = r" Proxy"]
pub struct _VLANTIW<'a> {
    w: &'a mut W,
}
impl<'a> _VLANTIW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VLANTC`"]
pub enum VLANTCW {
    #[doc = "`0`"]
    CMP16,
    #[doc = "`1`"]
    CMP12,
}
impl VLANTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VLANTCW::CMP16 => false,
            VLANTCW::CMP12 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VLANTCW<'a> {
    w: &'a mut W,
}
impl<'a> _VLANTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VLANTCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn cmp16(self) -> &'a mut W {
        self.variant(VLANTCW::CMP16)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn cmp12(self) -> &'a mut W {
        self.variant(VLANTCW::CMP12)
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
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:15 - VLAN tag identifier (for recieve frames)"]
    #[inline]
    pub fn vlanti(&self) -> VLANTIR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VLANTIR { bits }
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline]
    pub fn vlantc(&self) -> VLANTCR {
        VLANTCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bits 0:15 - VLAN tag identifier (for recieve frames)"]
    #[inline]
    pub fn vlanti(&mut self) -> _VLANTIW {
        _VLANTIW { w: self }
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline]
    pub fn vlantc(&mut self) -> _VLANTCW {
        _VLANTCW { w: self }
    }
}

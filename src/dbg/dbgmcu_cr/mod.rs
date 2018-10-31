#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DBGMCU_CR {
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
pub struct DBG_SLEEPR {
    bits: bool,
}
impl DBG_SLEEPR {
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
pub struct DBG_STOPR {
    bits: bool,
}
impl DBG_STOPR {
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
pub struct DBG_STANDBYR {
    bits: bool,
}
impl DBG_STANDBYR {
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
pub struct TRACE_IOENR {
    bits: bool,
}
impl TRACE_IOENR {
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
#[doc = "Possible values of the field `TRACE_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACE_MODER {
    #[doc = "undocumented"]
    ASYNC,
    #[doc = "undocumented"]
    SYNC1,
    #[doc = "undocumented"]
    SYNC2,
    #[doc = "undocumented"]
    SYNC4,
}
impl TRACE_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRACE_MODER::ASYNC => 0,
            TRACE_MODER::SYNC1 => 1,
            TRACE_MODER::SYNC2 => 2,
            TRACE_MODER::SYNC4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRACE_MODER {
        match value {
            0 => TRACE_MODER::ASYNC,
            1 => TRACE_MODER::SYNC1,
            2 => TRACE_MODER::SYNC2,
            3 => TRACE_MODER::SYNC4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline]
    pub fn is_async(&self) -> bool {
        *self == TRACE_MODER::ASYNC
    }
    #[doc = "Checks if the value of the field is `SYNC1`"]
    #[inline]
    pub fn is_sync1(&self) -> bool {
        *self == TRACE_MODER::SYNC1
    }
    #[doc = "Checks if the value of the field is `SYNC2`"]
    #[inline]
    pub fn is_sync2(&self) -> bool {
        *self == TRACE_MODER::SYNC2
    }
    #[doc = "Checks if the value of the field is `SYNC4`"]
    #[inline]
    pub fn is_sync4(&self) -> bool {
        *self == TRACE_MODER::SYNC4
    }
}
#[doc = r" Proxy"]
pub struct _DBG_SLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_SLEEPW<'a> {
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
pub struct _DBG_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_STOPW<'a> {
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
#[doc = r" Proxy"]
pub struct _DBG_STANDBYW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_STANDBYW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRACE_IOENW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACE_IOENW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRACE_MODE`"]
pub enum TRACE_MODEW {
    #[doc = "`0`"]
    ASYNC,
    #[doc = "`1`"]
    SYNC1,
    #[doc = "`10`"]
    SYNC2,
    #[doc = "`11`"]
    SYNC4,
}
impl TRACE_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRACE_MODEW::ASYNC => 0,
            TRACE_MODEW::SYNC1 => 1,
            TRACE_MODEW::SYNC2 => 2,
            TRACE_MODEW::SYNC4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRACE_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACE_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRACE_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn async(self) -> &'a mut W {
        self.variant(TRACE_MODEW::ASYNC)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn sync1(self) -> &'a mut W {
        self.variant(TRACE_MODEW::SYNC1)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn sync2(self) -> &'a mut W {
        self.variant(TRACE_MODEW::SYNC2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn sync4(self) -> &'a mut W {
        self.variant(TRACE_MODEW::SYNC4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline]
    pub fn dbg_sleep(&self) -> DBG_SLEEPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_SLEEPR { bits }
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline]
    pub fn dbg_stop(&self) -> DBG_STOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_STOPR { bits }
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline]
    pub fn dbg_standby(&self) -> DBG_STANDBYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBG_STANDBYR { bits }
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline]
    pub fn trace_ioen(&self) -> TRACE_IOENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TRACE_IOENR { bits }
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline]
    pub fn trace_mode(&self) -> TRACE_MODER {
        TRACE_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline]
    pub fn dbg_sleep(&mut self) -> _DBG_SLEEPW {
        _DBG_SLEEPW { w: self }
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline]
    pub fn dbg_stop(&mut self) -> _DBG_STOPW {
        _DBG_STOPW { w: self }
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline]
    pub fn dbg_standby(&mut self) -> _DBG_STANDBYW {
        _DBG_STANDBYW { w: self }
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline]
    pub fn trace_ioen(&mut self) -> _TRACE_IOENW {
        _TRACE_IOENW { w: self }
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline]
    pub fn trace_mode(&mut self) -> _TRACE_MODEW {
        _TRACE_MODEW { w: self }
    }
}

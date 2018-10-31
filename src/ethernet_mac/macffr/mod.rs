#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACFFR {
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
pub struct PMR {
    bits: bool,
}
impl PMR {
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
pub struct HUR {
    bits: bool,
}
impl HUR {
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
pub struct HMR {
    bits: bool,
}
impl HMR {
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
pub struct DAIFR {
    bits: bool,
}
impl DAIFR {
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
pub struct PAMR {
    bits: bool,
}
impl PAMR {
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
pub struct BFDR {
    bits: bool,
}
impl BFDR {
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
#[doc = "Possible values of the field `PCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCFR {
    #[doc = "undocumented"]
    NO_CONTROL_FRAMES,
    #[doc = "undocumented"]
    NO_PAUSE_FRAMES,
    #[doc = "undocumented"]
    CONTROL_FRAMES_BYPASS_FILTER,
    #[doc = "undocumented"]
    CONTROL_FRAMES,
}
impl PCFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCFR::NO_CONTROL_FRAMES => 0,
            PCFR::NO_PAUSE_FRAMES => 1,
            PCFR::CONTROL_FRAMES_BYPASS_FILTER => 2,
            PCFR::CONTROL_FRAMES => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCFR {
        match value {
            0 => PCFR::NO_CONTROL_FRAMES,
            1 => PCFR::NO_PAUSE_FRAMES,
            2 => PCFR::CONTROL_FRAMES_BYPASS_FILTER,
            3 => PCFR::CONTROL_FRAMES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CONTROL_FRAMES`"]
    #[inline]
    pub fn is_no_control_frames(&self) -> bool {
        *self == PCFR::NO_CONTROL_FRAMES
    }
    #[doc = "Checks if the value of the field is `NO_PAUSE_FRAMES`"]
    #[inline]
    pub fn is_no_pause_frames(&self) -> bool {
        *self == PCFR::NO_PAUSE_FRAMES
    }
    #[doc = "Checks if the value of the field is `CONTROL_FRAMES_BYPASS_FILTER`"]
    #[inline]
    pub fn is_control_frames_bypass_filter(&self) -> bool {
        *self == PCFR::CONTROL_FRAMES_BYPASS_FILTER
    }
    #[doc = "Checks if the value of the field is `CONTROL_FRAMES`"]
    #[inline]
    pub fn is_control_frames(&self) -> bool {
        *self == PCFR::CONTROL_FRAMES
    }
}
#[doc = r" Value of the field"]
pub struct SAIFR {
    bits: bool,
}
impl SAIFR {
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
pub struct SAFR {
    bits: bool,
}
impl SAFR {
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
#[doc = "Possible values of the field `HPF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPFR {
    #[doc = "undocumented"]
    BOTH_FILTERS,
    #[doc = "undocumented"]
    ONLY_HASH_FILTER,
}
impl HPFR {
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
            HPFR::BOTH_FILTERS => false,
            HPFR::ONLY_HASH_FILTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HPFR {
        match value {
            false => HPFR::BOTH_FILTERS,
            true => HPFR::ONLY_HASH_FILTER,
        }
    }
    #[doc = "Checks if the value of the field is `BOTH_FILTERS`"]
    #[inline]
    pub fn is_both_filters(&self) -> bool {
        *self == HPFR::BOTH_FILTERS
    }
    #[doc = "Checks if the value of the field is `ONLY_HASH_FILTER`"]
    #[inline]
    pub fn is_only_hash_filter(&self) -> bool {
        *self == HPFR::ONLY_HASH_FILTER
    }
}
#[doc = r" Value of the field"]
pub struct RAR {
    bits: bool,
}
impl RAR {
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
pub struct _PMW<'a> {
    w: &'a mut W,
}
impl<'a> _PMW<'a> {
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
pub struct _HUW<'a> {
    w: &'a mut W,
}
impl<'a> _HUW<'a> {
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
pub struct _HMW<'a> {
    w: &'a mut W,
}
impl<'a> _HMW<'a> {
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
pub struct _DAIFW<'a> {
    w: &'a mut W,
}
impl<'a> _DAIFW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAMW<'a> {
    w: &'a mut W,
}
impl<'a> _PAMW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BFDW<'a> {
    w: &'a mut W,
}
impl<'a> _BFDW<'a> {
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
#[doc = "Values that can be written to the field `PCF`"]
pub enum PCFW {
    #[doc = "`0`"]
    NO_CONTROL_FRAMES,
    #[doc = "`1`"]
    NO_PAUSE_FRAMES,
    #[doc = "`10`"]
    CONTROL_FRAMES_BYPASS_FILTER,
    #[doc = "`11`"]
    CONTROL_FRAMES,
}
impl PCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCFW::NO_CONTROL_FRAMES => 0,
            PCFW::NO_PAUSE_FRAMES => 1,
            PCFW::CONTROL_FRAMES_BYPASS_FILTER => 2,
            PCFW::CONTROL_FRAMES => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCFW<'a> {
    w: &'a mut W,
}
impl<'a> _PCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn no_control_frames(self) -> &'a mut W {
        self.variant(PCFW::NO_CONTROL_FRAMES)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn no_pause_frames(self) -> &'a mut W {
        self.variant(PCFW::NO_PAUSE_FRAMES)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn control_frames_bypass_filter(self) -> &'a mut W {
        self.variant(PCFW::CONTROL_FRAMES_BYPASS_FILTER)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn control_frames(self) -> &'a mut W {
        self.variant(PCFW::CONTROL_FRAMES)
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
#[doc = r" Proxy"]
pub struct _SAIFW<'a> {
    w: &'a mut W,
}
impl<'a> _SAIFW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SAFW<'a> {
    w: &'a mut W,
}
impl<'a> _SAFW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HPF`"]
pub enum HPFW {
    #[doc = "`0`"]
    BOTH_FILTERS,
    #[doc = "`1`"]
    ONLY_HASH_FILTER,
}
impl HPFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HPFW::BOTH_FILTERS => false,
            HPFW::ONLY_HASH_FILTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPFW<'a> {
    w: &'a mut W,
}
impl<'a> _HPFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn both_filters(self) -> &'a mut W {
        self.variant(HPFW::BOTH_FILTERS)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn only_hash_filter(self) -> &'a mut W {
        self.variant(HPFW::ONLY_HASH_FILTER)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RAW<'a> {
    w: &'a mut W,
}
impl<'a> _RAW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline]
    pub fn pm(&self) -> PMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PMR { bits }
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline]
    pub fn hu(&self) -> HUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HUR { bits }
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline]
    pub fn hm(&self) -> HMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HMR { bits }
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline]
    pub fn daif(&self) -> DAIFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DAIFR { bits }
    }
    #[doc = "Bit 4 - Pass all multicast"]
    #[inline]
    pub fn pam(&self) -> PAMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAMR { bits }
    }
    #[doc = "Bit 5 - Broadcast frames enable"]
    #[inline]
    pub fn bfd(&self) -> BFDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BFDR { bits }
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline]
    pub fn pcf(&self) -> PCFR {
        PCFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline]
    pub fn saif(&self) -> SAIFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SAIFR { bits }
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline]
    pub fn saf(&self) -> SAFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SAFR { bits }
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline]
    pub fn hpf(&self) -> HPFR {
        HPFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Recieve All"]
    #[inline]
    pub fn ra(&self) -> RAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RAR { bits }
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
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline]
    pub fn pm(&mut self) -> _PMW {
        _PMW { w: self }
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline]
    pub fn hu(&mut self) -> _HUW {
        _HUW { w: self }
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline]
    pub fn hm(&mut self) -> _HMW {
        _HMW { w: self }
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline]
    pub fn daif(&mut self) -> _DAIFW {
        _DAIFW { w: self }
    }
    #[doc = "Bit 4 - Pass all multicast"]
    #[inline]
    pub fn pam(&mut self) -> _PAMW {
        _PAMW { w: self }
    }
    #[doc = "Bit 5 - Broadcast frames enable"]
    #[inline]
    pub fn bfd(&mut self) -> _BFDW {
        _BFDW { w: self }
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline]
    pub fn pcf(&mut self) -> _PCFW {
        _PCFW { w: self }
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline]
    pub fn saif(&mut self) -> _SAIFW {
        _SAIFW { w: self }
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline]
    pub fn saf(&mut self) -> _SAFW {
        _SAFW { w: self }
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline]
    pub fn hpf(&mut self) -> _HPFW {
        _HPFW { w: self }
    }
    #[doc = "Bit 31 - Recieve All"]
    #[inline]
    pub fn ra(&mut self) -> _RAW {
        _RAW { w: self }
    }
}

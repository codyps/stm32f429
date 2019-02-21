#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MACCR {
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
pub struct RER {
    bits: bool,
}
impl RER {
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
pub struct TER {
    bits: bool,
}
impl TER {
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
pub struct DCR {
    bits: bool,
}
impl DCR {
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
#[doc = "Possible values of the field `BL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLR {
    #[doc = "undocumented"]
    R10,
    #[doc = "undocumented"]
    R8,
    #[doc = "undocumented"]
    R4,
    #[doc = "undocumented"]
    R1,
}
impl BLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BLR::R10 => 0,
            BLR::R8 => 1,
            BLR::R4 => 2,
            BLR::R1 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BLR {
        match value {
            0 => BLR::R10,
            1 => BLR::R8,
            2 => BLR::R4,
            3 => BLR::R1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `R10`"]
    #[inline]
    pub fn is_r10(&self) -> bool {
        *self == BLR::R10
    }
    #[doc = "Checks if the value of the field is `R8`"]
    #[inline]
    pub fn is_r8(&self) -> bool {
        *self == BLR::R8
    }
    #[doc = "Checks if the value of the field is `R4`"]
    #[inline]
    pub fn is_r4(&self) -> bool {
        *self == BLR::R4
    }
    #[doc = "Checks if the value of the field is `R1`"]
    #[inline]
    pub fn is_r1(&self) -> bool {
        *self == BLR::R1
    }
}
#[doc = r" Value of the field"]
pub struct APCSR {
    bits: bool,
}
impl APCSR {
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
pub struct RDR {
    bits: bool,
}
impl RDR {
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
pub struct IPCOR {
    bits: bool,
}
impl IPCOR {
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
pub struct DMR {
    bits: bool,
}
impl DMR {
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
pub struct LMR {
    bits: bool,
}
impl LMR {
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
pub struct RODR {
    bits: bool,
}
impl RODR {
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
#[doc = "Possible values of the field `FES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FESR {
    #[doc = "undocumented"]
    RATE_10,
    #[doc = "undocumented"]
    RATE_100,
}
impl FESR {
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
            FESR::RATE_10 => false,
            FESR::RATE_100 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FESR {
        match value {
            false => FESR::RATE_10,
            true => FESR::RATE_100,
        }
    }
    #[doc = "Checks if the value of the field is `RATE_10`"]
    #[inline]
    pub fn is_rate_10(&self) -> bool {
        *self == FESR::RATE_10
    }
    #[doc = "Checks if the value of the field is `RATE_100`"]
    #[inline]
    pub fn is_rate_100(&self) -> bool {
        *self == FESR::RATE_100
    }
}
#[doc = r" Value of the field"]
pub struct CSDR {
    bits: bool,
}
impl CSDR {
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
pub struct IFGR {
    bits: u8,
}
impl IFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `JD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JDR {
    #[doc = "undocumented"]
    LIMIT2048,
    #[doc = "undocumented"]
    LIMIT16384,
}
impl JDR {
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
            JDR::LIMIT2048 => false,
            JDR::LIMIT16384 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JDR {
        match value {
            false => JDR::LIMIT2048,
            true => JDR::LIMIT16384,
        }
    }
    #[doc = "Checks if the value of the field is `LIMIT2048`"]
    #[inline]
    pub fn is_limit2048(&self) -> bool {
        *self == JDR::LIMIT2048
    }
    #[doc = "Checks if the value of the field is `LIMIT16384`"]
    #[inline]
    pub fn is_limit16384(&self) -> bool {
        *self == JDR::LIMIT16384
    }
}
#[doc = "Possible values of the field `WD`"]
pub type WDR = JDR;
#[doc = "Possible values of the field `CSTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSTFR {
    #[doc = "undocumented"]
    KEEP_ETHER_CRC,
    #[doc = "undocumented"]
    STRIP_ETHER_CRC,
}
impl CSTFR {
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
            CSTFR::KEEP_ETHER_CRC => false,
            CSTFR::STRIP_ETHER_CRC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CSTFR {
        match value {
            false => CSTFR::KEEP_ETHER_CRC,
            true => CSTFR::STRIP_ETHER_CRC,
        }
    }
    #[doc = "Checks if the value of the field is `KEEP_ETHER_CRC`"]
    #[inline]
    pub fn is_keep_ether_crc(&self) -> bool {
        *self == CSTFR::KEEP_ETHER_CRC
    }
    #[doc = "Checks if the value of the field is `STRIP_ETHER_CRC`"]
    #[inline]
    pub fn is_strip_ether_crc(&self) -> bool {
        *self == CSTFR::STRIP_ETHER_CRC
    }
}
#[doc = r" Proxy"]
pub struct _REW<'a> {
    w: &'a mut W,
}
impl<'a> _REW<'a> {
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
pub struct _TEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEW<'a> {
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
pub struct _DCW<'a> {
    w: &'a mut W,
}
impl<'a> _DCW<'a> {
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
#[doc = "Values that can be written to the field `BL`"]
pub enum BLW {
    #[doc = "`0`"]
    R10,
    #[doc = "`1`"]
    R8,
    #[doc = "`10`"]
    R4,
    #[doc = "`11`"]
    R1,
}
impl BLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BLW::R10 => 0,
            BLW::R8 => 1,
            BLW::R4 => 2,
            BLW::R1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BLW<'a> {
    w: &'a mut W,
}
impl<'a> _BLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn r10(self) -> &'a mut W {
        self.variant(BLW::R10)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn r8(self) -> &'a mut W {
        self.variant(BLW::R8)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn r4(self) -> &'a mut W {
        self.variant(BLW::R4)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn r1(self) -> &'a mut W {
        self.variant(BLW::R1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _APCSW<'a> {
    w: &'a mut W,
}
impl<'a> _APCSW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RDW<'a> {
    w: &'a mut W,
}
impl<'a> _RDW<'a> {
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
#[doc = r" Proxy"]
pub struct _IPCOW<'a> {
    w: &'a mut W,
}
impl<'a> _IPCOW<'a> {
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
pub struct _DMW<'a> {
    w: &'a mut W,
}
impl<'a> _DMW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LMW<'a> {
    w: &'a mut W,
}
impl<'a> _LMW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RODW<'a> {
    w: &'a mut W,
}
impl<'a> _RODW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FES`"]
pub enum FESW {
    #[doc = "`0`"]
    RATE_10,
    #[doc = "`1`"]
    RATE_100,
}
impl FESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FESW::RATE_10 => false,
            FESW::RATE_100 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FESW<'a> {
    w: &'a mut W,
}
impl<'a> _FESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn rate_10(self) -> &'a mut W {
        self.variant(FESW::RATE_10)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn rate_100(self) -> &'a mut W {
        self.variant(FESW::RATE_100)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSDW<'a> {
    w: &'a mut W,
}
impl<'a> _CSDW<'a> {
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
#[doc = r" Proxy"]
pub struct _IFGW<'a> {
    w: &'a mut W,
}
impl<'a> _IFGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `JD`"]
pub enum JDW {
    #[doc = "`0`"]
    LIMIT2048,
    #[doc = "`1`"]
    LIMIT16384,
}
impl JDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            JDW::LIMIT2048 => false,
            JDW::LIMIT16384 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _JDW<'a> {
    w: &'a mut W,
}
impl<'a> _JDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn limit2048(self) -> &'a mut W {
        self.variant(JDW::LIMIT2048)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn limit16384(self) -> &'a mut W {
        self.variant(JDW::LIMIT16384)
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
#[doc = "Values that can be written to the field `WD`"]
pub type WDW = JDW;
#[doc = r" Proxy"]
pub struct _WDW<'a> {
    w: &'a mut W,
}
impl<'a> _WDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn limit2048(self) -> &'a mut W {
        self.variant(JDW::LIMIT2048)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn limit16384(self) -> &'a mut W {
        self.variant(JDW::LIMIT16384)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSTF`"]
pub enum CSTFW {
    #[doc = "`0`"]
    KEEP_ETHER_CRC,
    #[doc = "`1`"]
    STRIP_ETHER_CRC,
}
impl CSTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CSTFW::KEEP_ETHER_CRC => false,
            CSTFW::STRIP_ETHER_CRC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSTFW<'a> {
    w: &'a mut W,
}
impl<'a> _CSTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn keep_ether_crc(self) -> &'a mut W {
        self.variant(CSTFW::KEEP_ETHER_CRC)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn strip_ether_crc(self) -> &'a mut W {
        self.variant(CSTFW::STRIP_ETHER_CRC)
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
        const OFFSET: u8 = 25;
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
    #[doc = "Bit 2 - Recieve enable"]
    #[inline]
    pub fn re(&self) -> RER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RER { bits }
    }
    #[doc = "Bit 3 - Transmit enable"]
    #[inline]
    pub fn te(&self) -> TER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TER { bits }
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline]
    pub fn dc(&self) -> DCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCR { bits }
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline]
    pub fn bl(&self) -> BLR {
        BLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline]
    pub fn apcs(&self) -> APCSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        APCSR { bits }
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline]
    pub fn rd(&self) -> RDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RDR { bits }
    }
    #[doc = "Bit 10 - IPv4 Checksum offload"]
    #[inline]
    pub fn ipco(&self) -> IPCOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IPCOR { bits }
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline]
    pub fn dm(&self) -> DMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMR { bits }
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline]
    pub fn lm(&self) -> LMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LMR { bits }
    }
    #[doc = "Bit 13 - Recieve own disable"]
    #[inline]
    pub fn rod(&self) -> RODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RODR { bits }
    }
    #[doc = "Bit 14 - Fast Ethernet Speed"]
    #[inline]
    pub fn fes(&self) -> FESR {
        FESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline]
    pub fn csd(&self) -> CSDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSDR { bits }
    }
    #[doc = "Bits 17:19 - Interframe gap (96,86,80, ... 40)"]
    #[inline]
    pub fn ifg(&self) -> IFGR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IFGR { bits }
    }
    #[doc = "Bit 22 - Jabber disable (transmitter)"]
    #[inline]
    pub fn jd(&self) -> JDR {
        JDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Watchdog Disable (reciever)"]
    #[inline]
    pub fn wd(&self) -> WDR {
        WDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - CRC stripping for type frames"]
    #[inline]
    pub fn cstf(&self) -> CSTFR {
        CSTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32768 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Recieve enable"]
    #[inline]
    pub fn re(&mut self) -> _REW {
        _REW { w: self }
    }
    #[doc = "Bit 3 - Transmit enable"]
    #[inline]
    pub fn te(&mut self) -> _TEW {
        _TEW { w: self }
    }
    #[doc = "Bit 4 - Deferral check"]
    #[inline]
    pub fn dc(&mut self) -> _DCW {
        _DCW { w: self }
    }
    #[doc = "Bits 5:6 - Back-off limit"]
    #[inline]
    pub fn bl(&mut self) -> _BLW {
        _BLW { w: self }
    }
    #[doc = "Bit 7 - Automatic pad/CRC stripping"]
    #[inline]
    pub fn apcs(&mut self) -> _APCSW {
        _APCSW { w: self }
    }
    #[doc = "Bit 9 - Retry disable"]
    #[inline]
    pub fn rd(&mut self) -> _RDW {
        _RDW { w: self }
    }
    #[doc = "Bit 10 - IPv4 Checksum offload"]
    #[inline]
    pub fn ipco(&mut self) -> _IPCOW {
        _IPCOW { w: self }
    }
    #[doc = "Bit 11 - Duplex mode"]
    #[inline]
    pub fn dm(&mut self) -> _DMW {
        _DMW { w: self }
    }
    #[doc = "Bit 12 - Loopback mode"]
    #[inline]
    pub fn lm(&mut self) -> _LMW {
        _LMW { w: self }
    }
    #[doc = "Bit 13 - Recieve own disable"]
    #[inline]
    pub fn rod(&mut self) -> _RODW {
        _RODW { w: self }
    }
    #[doc = "Bit 14 - Fast Ethernet Speed"]
    #[inline]
    pub fn fes(&mut self) -> _FESW {
        _FESW { w: self }
    }
    #[doc = "Bit 16 - Carrier sense disable"]
    #[inline]
    pub fn csd(&mut self) -> _CSDW {
        _CSDW { w: self }
    }
    #[doc = "Bits 17:19 - Interframe gap (96,86,80, ... 40)"]
    #[inline]
    pub fn ifg(&mut self) -> _IFGW {
        _IFGW { w: self }
    }
    #[doc = "Bit 22 - Jabber disable (transmitter)"]
    #[inline]
    pub fn jd(&mut self) -> _JDW {
        _JDW { w: self }
    }
    #[doc = "Bit 23 - Watchdog Disable (reciever)"]
    #[inline]
    pub fn wd(&mut self) -> _WDW {
        _WDW { w: self }
    }
    #[doc = "Bit 25 - CRC stripping for type frames"]
    #[inline]
    pub fn cstf(&mut self) -> _CSTFW {
        _CSTFW { w: self }
    }
}

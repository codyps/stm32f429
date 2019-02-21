#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::UID2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct LOT_NUMR {
    bits: u32,
}
impl LOT_NUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WAF_NUMR {
    bits: u8,
}
impl WAF_NUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:31 - Lot number (ASCII encoded, end)"]
    #[inline]
    pub fn lot_num(&self) -> LOT_NUMR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        LOT_NUMR { bits }
    }
    #[doc = "Bits 0:7 - Wafer number (ASCII encoded)"]
    #[inline]
    pub fn waf_num(&self) -> WAF_NUMR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WAF_NUMR { bits }
    }
}

#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DATA2R {
    bits: u16,
}
impl DATA2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA1R {
    bits: u16,
}
impl DATA1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 16:31 - 2nd data item of a pair of regular conversions"]
    #[inline]
    pub fn data2(&self) -> DATA2R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DATA2R { bits }
    }
    #[doc = "Bits 0:15 - 1st data item of a pair of regular conversions"]
    #[inline]
    pub fn data1(&self) -> DATA1R {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DATA1R { bits }
    }
}

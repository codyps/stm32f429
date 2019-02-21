#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::L2CLUTWR {
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
}
#[doc = r" Proxy"]
pub struct _CLUTADDW<'a> {
    w: &'a mut W,
}
impl<'a> _CLUTADDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _REDW<'a> {
    w: &'a mut W,
}
impl<'a> _REDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GREENW<'a> {
    w: &'a mut W,
}
impl<'a> _GREENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BLUEW<'a> {
    w: &'a mut W,
}
impl<'a> _BLUEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bits 24:31 - CLUT Address"]
    #[inline]
    pub fn clutadd(&mut self) -> _CLUTADDW {
        _CLUTADDW { w: self }
    }
    #[doc = "Bits 16:23 - Red value"]
    #[inline]
    pub fn red(&mut self) -> _REDW {
        _REDW { w: self }
    }
    #[doc = "Bits 8:15 - Green value"]
    #[inline]
    pub fn green(&mut self) -> _GREENW {
        _GREENW { w: self }
    }
    #[doc = "Bits 0:7 - Blue value"]
    #[inline]
    pub fn blue(&mut self) -> _BLUEW {
        _BLUEW { w: self }
    }
}

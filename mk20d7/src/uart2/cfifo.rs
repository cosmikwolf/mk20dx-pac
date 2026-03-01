#[doc = "Register `CFIFO` reader"]
pub type R = crate::R<CfifoSpec>;
#[doc = "Register `CFIFO` writer"]
pub type W = crate::W<CfifoSpec>;
#[doc = "Receive FIFO Underflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxufe {
    #[doc = "0: RXUF interrupt disabled"]
    Disabled = 0,
    #[doc = "1: RXUF interrupt enabled"]
    Enabled = 1,
}
impl From<Rxufe> for bool {
    #[inline(always)]
    fn from(variant: Rxufe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXUFE` reader - Receive FIFO Underflow Interrupt Enable"]
pub type RxufeR = crate::BitReader<Rxufe>;
impl RxufeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxufe {
        match self.bits {
            false => Rxufe::Disabled,
            true => Rxufe::Enabled,
        }
    }
    #[doc = "RXUF interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxufe::Disabled
    }
    #[doc = "RXUF interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxufe::Enabled
    }
}
#[doc = "Field `RXUFE` writer - Receive FIFO Underflow Interrupt Enable"]
pub type RxufeW<'a, REG> = crate::BitWriter<'a, REG, Rxufe>;
impl<'a, REG> RxufeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RXUF interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxufe::Disabled)
    }
    #[doc = "RXUF interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxufe::Enabled)
    }
}
#[doc = "Transmit FIFO Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txofe {
    #[doc = "0: TXOF interrupt disabled"]
    Disabled = 0,
    #[doc = "1: TXOF interrupt enabled"]
    Enabled = 1,
}
impl From<Txofe> for bool {
    #[inline(always)]
    fn from(variant: Txofe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXOFE` reader - Transmit FIFO Overflow Interrupt Enable"]
pub type TxofeR = crate::BitReader<Txofe>;
impl TxofeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txofe {
        match self.bits {
            false => Txofe::Disabled,
            true => Txofe::Enabled,
        }
    }
    #[doc = "TXOF interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txofe::Disabled
    }
    #[doc = "TXOF interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txofe::Enabled
    }
}
#[doc = "Field `TXOFE` writer - Transmit FIFO Overflow Interrupt Enable"]
pub type TxofeW<'a, REG> = crate::BitWriter<'a, REG, Txofe>;
impl<'a, REG> TxofeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXOF interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txofe::Disabled)
    }
    #[doc = "TXOF interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Txofe::Enabled)
    }
}
#[doc = "Receive FIFO/Buffer Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxflush {
    #[doc = "0: No flush"]
    NoEffect = 0,
    #[doc = "1: Flush receive FIFO"]
    Flush = 1,
}
impl From<Rxflush> for bool {
    #[inline(always)]
    fn from(variant: Rxflush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFLUSH` writer - Receive FIFO/Buffer Flush"]
pub type RxflushW<'a, REG> = crate::BitWriter<'a, REG, Rxflush>;
impl<'a, REG> RxflushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No flush"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Rxflush::NoEffect)
    }
    #[doc = "Flush receive FIFO"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Rxflush::Flush)
    }
}
#[doc = "Transmit FIFO/Buffer Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txflush {
    #[doc = "0: No flush"]
    NoEffect = 0,
    #[doc = "1: Flush transmit FIFO"]
    Flush = 1,
}
impl From<Txflush> for bool {
    #[inline(always)]
    fn from(variant: Txflush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFLUSH` writer - Transmit FIFO/Buffer Flush"]
pub type TxflushW<'a, REG> = crate::BitWriter<'a, REG, Txflush>;
impl<'a, REG> TxflushW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No flush"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Txflush::NoEffect)
    }
    #[doc = "Flush transmit FIFO"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Txflush::Flush)
    }
}
impl R {
    #[doc = "Bit 0 - Receive FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxufe(&self) -> RxufeR {
        RxufeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txofe(&self) -> TxofeR {
        TxofeR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxufe(&mut self) -> RxufeW<'_, CfifoSpec> {
        RxufeW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txofe(&mut self) -> TxofeW<'_, CfifoSpec> {
        TxofeW::new(self, 1)
    }
    #[doc = "Bit 6 - Receive FIFO/Buffer Flush"]
    #[inline(always)]
    pub fn rxflush(&mut self) -> RxflushW<'_, CfifoSpec> {
        RxflushW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit FIFO/Buffer Flush"]
    #[inline(always)]
    pub fn txflush(&mut self) -> TxflushW<'_, CfifoSpec> {
        TxflushW::new(self, 7)
    }
}
#[doc = "UART FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfifoSpec;
impl crate::RegisterSpec for CfifoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cfifo::R`](R) reader structure"]
impl crate::Readable for CfifoSpec {}
#[doc = "`write(|w| ..)` method takes [`cfifo::W`](W) writer structure"]
impl crate::Writable for CfifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFIFO to value 0"]
impl crate::Resettable for CfifoSpec {}

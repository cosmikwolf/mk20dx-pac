#[doc = "Register `SFIFO` reader"]
pub type R = crate::R<SfifoSpec>;
#[doc = "Register `SFIFO` writer"]
pub type W = crate::W<SfifoSpec>;
#[doc = "Receiver Buffer Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxuf {
    #[doc = "0: No receive underflow"]
    NoUnderflow = 0,
    #[doc = "1: Receive FIFO underflow"]
    Underflow = 1,
}
impl From<Rxuf> for bool {
    #[inline(always)]
    fn from(variant: Rxuf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXUF` reader - Receiver Buffer Underflow Flag"]
pub type RxufR = crate::BitReader<Rxuf>;
impl RxufR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxuf {
        match self.bits {
            false => Rxuf::NoUnderflow,
            true => Rxuf::Underflow,
        }
    }
    #[doc = "No receive underflow"]
    #[inline(always)]
    pub fn is_no_underflow(&self) -> bool {
        *self == Rxuf::NoUnderflow
    }
    #[doc = "Receive FIFO underflow"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        *self == Rxuf::Underflow
    }
}
#[doc = "Field `RXUF` writer - Receiver Buffer Underflow Flag"]
pub type RxufW<'a, REG> = crate::BitWriter<'a, REG, Rxuf>;
impl<'a, REG> RxufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No receive underflow"]
    #[inline(always)]
    pub fn no_underflow(self) -> &'a mut crate::W<REG> {
        self.variant(Rxuf::NoUnderflow)
    }
    #[doc = "Receive FIFO underflow"]
    #[inline(always)]
    pub fn underflow(self) -> &'a mut crate::W<REG> {
        self.variant(Rxuf::Underflow)
    }
}
#[doc = "Transmitter Buffer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txof {
    #[doc = "0: No transmit overflow"]
    NoOverflow = 0,
    #[doc = "1: Transmit FIFO overflow"]
    Overflow = 1,
}
impl From<Txof> for bool {
    #[inline(always)]
    fn from(variant: Txof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXOF` reader - Transmitter Buffer Overflow Flag"]
pub type TxofR = crate::BitReader<Txof>;
impl TxofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txof {
        match self.bits {
            false => Txof::NoOverflow,
            true => Txof::Overflow,
        }
    }
    #[doc = "No transmit overflow"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Txof::NoOverflow
    }
    #[doc = "Transmit FIFO overflow"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Txof::Overflow
    }
}
#[doc = "Field `TXOF` writer - Transmitter Buffer Overflow Flag"]
pub type TxofW<'a, REG> = crate::BitWriter<'a, REG, Txof>;
impl<'a, REG> TxofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No transmit overflow"]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Txof::NoOverflow)
    }
    #[doc = "Transmit FIFO overflow"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Txof::Overflow)
    }
}
#[doc = "Receive Buffer/FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxempt {
    #[doc = "0: Receive buffer not empty"]
    NotEmpty = 0,
    #[doc = "1: Receive buffer empty"]
    Empty = 1,
}
impl From<Rxempt> for bool {
    #[inline(always)]
    fn from(variant: Rxempt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEMPT` reader - Receive Buffer/FIFO Empty"]
pub type RxemptR = crate::BitReader<Rxempt>;
impl RxemptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxempt {
        match self.bits {
            false => Rxempt::NotEmpty,
            true => Rxempt::Empty,
        }
    }
    #[doc = "Receive buffer not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Rxempt::NotEmpty
    }
    #[doc = "Receive buffer empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Rxempt::Empty
    }
}
#[doc = "Transmit Buffer/FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txempt {
    #[doc = "0: Transmit buffer not empty"]
    NotEmpty = 0,
    #[doc = "1: Transmit buffer empty"]
    Empty = 1,
}
impl From<Txempt> for bool {
    #[inline(always)]
    fn from(variant: Txempt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPT` reader - Transmit Buffer/FIFO Empty"]
pub type TxemptR = crate::BitReader<Txempt>;
impl TxemptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txempt {
        match self.bits {
            false => Txempt::NotEmpty,
            true => Txempt::Empty,
        }
    }
    #[doc = "Transmit buffer not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Txempt::NotEmpty
    }
    #[doc = "Transmit buffer empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Txempt::Empty
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Buffer Underflow Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RxufR {
        RxufR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Buffer Overflow Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TxofR {
        TxofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Buffer/FIFO Empty"]
    #[inline(always)]
    pub fn rxempt(&self) -> RxemptR {
        RxemptR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Buffer/FIFO Empty"]
    #[inline(always)]
    pub fn txempt(&self) -> TxemptR {
        TxemptR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Buffer Underflow Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RxufW<'_, SfifoSpec> {
        RxufW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitter Buffer Overflow Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TxofW<'_, SfifoSpec> {
        TxofW::new(self, 1)
    }
}
#[doc = "UART FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfifoSpec;
impl crate::RegisterSpec for SfifoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sfifo::R`](R) reader structure"]
impl crate::Readable for SfifoSpec {}
#[doc = "`write(|w| ..)` method takes [`sfifo::W`](W) writer structure"]
impl crate::Writable for SfifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFIFO to value 0xc0"]
impl crate::Resettable for SfifoSpec {
    const RESET_VALUE: u8 = 0xc0;
}

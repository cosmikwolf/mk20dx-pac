#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `POPNXTPTR` reader - Pop Next Pointer"]
pub type PopnxtptrR = crate::FieldReader;
#[doc = "Field `RXCTR` reader - RX FIFO Counter"]
pub type RxctrR = crate::FieldReader;
#[doc = "Field `TXNXTPTR` reader - Transmit Next Pointer"]
pub type TxnxtptrR = crate::FieldReader;
#[doc = "Field `TXCTR` reader - TX FIFO Counter"]
pub type TxctrR = crate::FieldReader;
#[doc = "Receive FIFO Drain Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdf {
    #[doc = "0: RX FIFO is empty"]
    Empty = 0,
    #[doc = "1: RX FIFO is not empty"]
    NotEmpty = 1,
}
impl From<Rfdf> for bool {
    #[inline(always)]
    fn from(variant: Rfdf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFDF` reader - Receive FIFO Drain Flag"]
pub type RfdfR = crate::BitReader<Rfdf>;
impl RfdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfdf {
        match self.bits {
            false => Rfdf::Empty,
            true => Rfdf::NotEmpty,
        }
    }
    #[doc = "RX FIFO is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Rfdf::Empty
    }
    #[doc = "RX FIFO is not empty"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == Rfdf::NotEmpty
    }
}
#[doc = "Field `RFDF` writer - Receive FIFO Drain Flag"]
pub type RfdfW<'a, REG> = crate::BitWriter1C<'a, REG, Rfdf>;
impl<'a, REG> RfdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX FIFO is empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdf::Empty)
    }
    #[doc = "RX FIFO is not empty"]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdf::NotEmpty)
    }
}
#[doc = "Receive FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfof {
    #[doc = "0: No RX FIFO overflow"]
    NoOverflow = 0,
    #[doc = "1: RX FIFO overflow has occurred"]
    Overflow = 1,
}
impl From<Rfof> for bool {
    #[inline(always)]
    fn from(variant: Rfof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFOF` reader - Receive FIFO Overflow Flag"]
pub type RfofR = crate::BitReader<Rfof>;
impl RfofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfof {
        match self.bits {
            false => Rfof::NoOverflow,
            true => Rfof::Overflow,
        }
    }
    #[doc = "No RX FIFO overflow"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Rfof::NoOverflow
    }
    #[doc = "RX FIFO overflow has occurred"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Rfof::Overflow
    }
}
#[doc = "Field `RFOF` writer - Receive FIFO Overflow Flag"]
pub type RfofW<'a, REG> = crate::BitWriter1C<'a, REG, Rfof>;
impl<'a, REG> RfofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No RX FIFO overflow"]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Rfof::NoOverflow)
    }
    #[doc = "RX FIFO overflow has occurred"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Rfof::Overflow)
    }
}
#[doc = "Transmit FIFO Fill Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfff {
    #[doc = "0: TX FIFO is full"]
    Full = 0,
    #[doc = "1: TX FIFO is not full"]
    NotFull = 1,
}
impl From<Tfff> for bool {
    #[inline(always)]
    fn from(variant: Tfff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFFF` reader - Transmit FIFO Fill Flag"]
pub type TfffR = crate::BitReader<Tfff>;
impl TfffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfff {
        match self.bits {
            false => Tfff::Full,
            true => Tfff::NotFull,
        }
    }
    #[doc = "TX FIFO is full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Tfff::Full
    }
    #[doc = "TX FIFO is not full"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == Tfff::NotFull
    }
}
#[doc = "Field `TFFF` writer - Transmit FIFO Fill Flag"]
pub type TfffW<'a, REG> = crate::BitWriter1C<'a, REG, Tfff>;
impl<'a, REG> TfffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX FIFO is full"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Tfff::Full)
    }
    #[doc = "TX FIFO is not full"]
    #[inline(always)]
    pub fn not_full(self) -> &'a mut crate::W<REG> {
        self.variant(Tfff::NotFull)
    }
}
#[doc = "Transmit FIFO Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfuf {
    #[doc = "0: No TX underflow"]
    NoUnderflow = 0,
    #[doc = "1: TX FIFO underflow has occurred"]
    Underflow = 1,
}
impl From<Tfuf> for bool {
    #[inline(always)]
    fn from(variant: Tfuf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFUF` reader - Transmit FIFO Underflow Flag"]
pub type TfufR = crate::BitReader<Tfuf>;
impl TfufR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfuf {
        match self.bits {
            false => Tfuf::NoUnderflow,
            true => Tfuf::Underflow,
        }
    }
    #[doc = "No TX underflow"]
    #[inline(always)]
    pub fn is_no_underflow(&self) -> bool {
        *self == Tfuf::NoUnderflow
    }
    #[doc = "TX FIFO underflow has occurred"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        *self == Tfuf::Underflow
    }
}
#[doc = "Field `TFUF` writer - Transmit FIFO Underflow Flag"]
pub type TfufW<'a, REG> = crate::BitWriter1C<'a, REG, Tfuf>;
impl<'a, REG> TfufW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No TX underflow"]
    #[inline(always)]
    pub fn no_underflow(self) -> &'a mut crate::W<REG> {
        self.variant(Tfuf::NoUnderflow)
    }
    #[doc = "TX FIFO underflow has occurred"]
    #[inline(always)]
    pub fn underflow(self) -> &'a mut crate::W<REG> {
        self.variant(Tfuf::Underflow)
    }
}
#[doc = "End of Queue Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eoqf {
    #[doc = "0: EOQ bit not set in executing entry"]
    NotSet = 0,
    #[doc = "1: EOQ bit set in executing entry"]
    Set = 1,
}
impl From<Eoqf> for bool {
    #[inline(always)]
    fn from(variant: Eoqf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOQF` reader - End of Queue Flag"]
pub type EoqfR = crate::BitReader<Eoqf>;
impl EoqfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eoqf {
        match self.bits {
            false => Eoqf::NotSet,
            true => Eoqf::Set,
        }
    }
    #[doc = "EOQ bit not set in executing entry"]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == Eoqf::NotSet
    }
    #[doc = "EOQ bit set in executing entry"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Eoqf::Set
    }
}
#[doc = "Field `EOQF` writer - End of Queue Flag"]
pub type EoqfW<'a, REG> = crate::BitWriter1C<'a, REG, Eoqf>;
impl<'a, REG> EoqfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOQ bit not set in executing entry"]
    #[inline(always)]
    pub fn not_set(self) -> &'a mut crate::W<REG> {
        self.variant(Eoqf::NotSet)
    }
    #[doc = "EOQ bit set in executing entry"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Eoqf::Set)
    }
}
#[doc = "TX and RX Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txrxs {
    #[doc = "0: Transmit and receive operations disabled"]
    Stopped = 0,
    #[doc = "1: Transmit and receive operations enabled"]
    Started = 1,
}
impl From<Txrxs> for bool {
    #[inline(always)]
    fn from(variant: Txrxs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRXS` reader - TX and RX Status"]
pub type TxrxsR = crate::BitReader<Txrxs>;
impl TxrxsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txrxs {
        match self.bits {
            false => Txrxs::Stopped,
            true => Txrxs::Started,
        }
    }
    #[doc = "Transmit and receive operations disabled"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == Txrxs::Stopped
    }
    #[doc = "Transmit and receive operations enabled"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Txrxs::Started
    }
}
#[doc = "Field `TXRXS` writer - TX and RX Status"]
pub type TxrxsW<'a, REG> = crate::BitWriter<'a, REG, Txrxs>;
impl<'a, REG> TxrxsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit and receive operations disabled"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(Txrxs::Stopped)
    }
    #[doc = "Transmit and receive operations enabled"]
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(Txrxs::Started)
    }
}
#[doc = "Transfer Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcf {
    #[doc = "0: Transfer not complete"]
    NotComplete = 0,
    #[doc = "1: Transfer complete"]
    Complete = 1,
}
impl From<Tcf> for bool {
    #[inline(always)]
    fn from(variant: Tcf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCF` reader - Transfer Complete Flag"]
pub type TcfR = crate::BitReader<Tcf>;
impl TcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcf {
        match self.bits {
            false => Tcf::NotComplete,
            true => Tcf::Complete,
        }
    }
    #[doc = "Transfer not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == Tcf::NotComplete
    }
    #[doc = "Transfer complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == Tcf::Complete
    }
}
#[doc = "Field `TCF` writer - Transfer Complete Flag"]
pub type TcfW<'a, REG> = crate::BitWriter1C<'a, REG, Tcf>;
impl<'a, REG> TcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer not complete"]
    #[inline(always)]
    pub fn not_complete(self) -> &'a mut crate::W<REG> {
        self.variant(Tcf::NotComplete)
    }
    #[doc = "Transfer complete"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(Tcf::Complete)
    }
}
impl R {
    #[doc = "Bits 0:3 - Pop Next Pointer"]
    #[inline(always)]
    pub fn popnxtptr(&self) -> PopnxtptrR {
        PopnxtptrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RX FIFO Counter"]
    #[inline(always)]
    pub fn rxctr(&self) -> RxctrR {
        RxctrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Transmit Next Pointer"]
    #[inline(always)]
    pub fn txnxtptr(&self) -> TxnxtptrR {
        TxnxtptrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TX FIFO Counter"]
    #[inline(always)]
    pub fn txctr(&self) -> TxctrR {
        TxctrR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - Receive FIFO Drain Flag"]
    #[inline(always)]
    pub fn rfdf(&self) -> RfdfR {
        RfdfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Flag"]
    #[inline(always)]
    pub fn rfof(&self) -> RfofR {
        RfofR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Flag"]
    #[inline(always)]
    pub fn tfff(&self) -> TfffR {
        TfffR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Flag"]
    #[inline(always)]
    pub fn tfuf(&self) -> TfufR {
        TfufR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - End of Queue Flag"]
    #[inline(always)]
    pub fn eoqf(&self) -> EoqfR {
        EoqfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - TX and RX Status"]
    #[inline(always)]
    pub fn txrxs(&self) -> TxrxsR {
        TxrxsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transfer Complete Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TcfR {
        TcfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Receive FIFO Drain Flag"]
    #[inline(always)]
    pub fn rfdf(&mut self) -> RfdfW<'_, SrSpec> {
        RfdfW::new(self, 17)
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Flag"]
    #[inline(always)]
    pub fn rfof(&mut self) -> RfofW<'_, SrSpec> {
        RfofW::new(self, 19)
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Flag"]
    #[inline(always)]
    pub fn tfff(&mut self) -> TfffW<'_, SrSpec> {
        TfffW::new(self, 25)
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Flag"]
    #[inline(always)]
    pub fn tfuf(&mut self) -> TfufW<'_, SrSpec> {
        TfufW::new(self, 27)
    }
    #[doc = "Bit 28 - End of Queue Flag"]
    #[inline(always)]
    pub fn eoqf(&mut self) -> EoqfW<'_, SrSpec> {
        EoqfW::new(self, 28)
    }
    #[doc = "Bit 30 - TX and RX Status"]
    #[inline(always)]
    pub fn txrxs(&mut self) -> TxrxsW<'_, SrSpec> {
        TxrxsW::new(self, 30)
    }
    #[doc = "Bit 31 - Transfer Complete Flag"]
    #[inline(always)]
    pub fn tcf(&mut self) -> TcfW<'_, SrSpec> {
        TcfW::new(self, 31)
    }
}
#[doc = "DSPI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x9a0a_0000;
}
#[doc = "`reset()` method sets SR to value 0x0200_0000"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x0200_0000;
}

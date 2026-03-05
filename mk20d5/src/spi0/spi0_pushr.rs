#[doc = "Register `PUSHR` reader"]
pub type R = crate::R<Spi0PushrSpec>;
#[doc = "Register `PUSHR` writer"]
pub type W = crate::W<Spi0PushrSpec>;
#[doc = "Field `TXDATA` reader - Transmit Data"]
pub type TxdataR = crate::FieldReader<u16>;
#[doc = "Field `TXDATA` writer - Transmit Data"]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Select which PCS signals are to be asserted for the transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcs {
    #[doc = "0: Negate the PCS\\[x\\] signal"]
    Negate = 0,
    #[doc = "1: Assert the PCS\\[x\\] signal"]
    Assert = 1,
}
impl From<Pcs> for u8 {
    #[inline(always)]
    fn from(variant: Pcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcs {
    type Ux = u8;
}
impl crate::IsEnum for Pcs {}
#[doc = "Field `PCS` reader - Select which PCS signals are to be asserted for the transfer"]
pub type PcsR = crate::FieldReader<Pcs>;
impl PcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pcs> {
        match self.bits {
            0 => Some(Pcs::Negate),
            1 => Some(Pcs::Assert),
            _ => None,
        }
    }
    #[doc = "Negate the PCS\\[x\\] signal"]
    #[inline(always)]
    pub fn is_negate(&self) -> bool {
        *self == Pcs::Negate
    }
    #[doc = "Assert the PCS\\[x\\] signal"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == Pcs::Assert
    }
}
#[doc = "Field `PCS` writer - Select which PCS signals are to be asserted for the transfer"]
pub type PcsW<'a, REG> = crate::FieldWriter<'a, REG, 6, Pcs>;
impl<'a, REG> PcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Negate the PCS\\[x\\] signal"]
    #[inline(always)]
    pub fn negate(self) -> &'a mut crate::W<REG> {
        self.variant(Pcs::Negate)
    }
    #[doc = "Assert the PCS\\[x\\] signal"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(Pcs::Assert)
    }
}
#[doc = "Clear Transfer Counter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctcnt {
    #[doc = "0: Do not clear transfer counter"]
    Keep = 0,
    #[doc = "1: Clear transfer counter"]
    Clear = 1,
}
impl From<Ctcnt> for bool {
    #[inline(always)]
    fn from(variant: Ctcnt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCNT` reader - Clear Transfer Counter."]
pub type CtcntR = crate::BitReader<Ctcnt>;
impl CtcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctcnt {
        match self.bits {
            false => Ctcnt::Keep,
            true => Ctcnt::Clear,
        }
    }
    #[doc = "Do not clear transfer counter"]
    #[inline(always)]
    pub fn is_keep(&self) -> bool {
        *self == Ctcnt::Keep
    }
    #[doc = "Clear transfer counter"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Ctcnt::Clear
    }
}
#[doc = "Field `CTCNT` writer - Clear Transfer Counter."]
pub type CtcntW<'a, REG> = crate::BitWriter<'a, REG, Ctcnt>;
impl<'a, REG> CtcntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not clear transfer counter"]
    #[inline(always)]
    pub fn keep(self) -> &'a mut crate::W<REG> {
        self.variant(Ctcnt::Keep)
    }
    #[doc = "Clear transfer counter"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Ctcnt::Clear)
    }
}
#[doc = "End Of Queue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eoq {
    #[doc = "0: Not last data in queue"]
    NotLast = 0,
    #[doc = "1: Last data in queue"]
    Last = 1,
}
impl From<Eoq> for bool {
    #[inline(always)]
    fn from(variant: Eoq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOQ` reader - End Of Queue"]
pub type EoqR = crate::BitReader<Eoq>;
impl EoqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eoq {
        match self.bits {
            false => Eoq::NotLast,
            true => Eoq::Last,
        }
    }
    #[doc = "Not last data in queue"]
    #[inline(always)]
    pub fn is_not_last(&self) -> bool {
        *self == Eoq::NotLast
    }
    #[doc = "Last data in queue"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == Eoq::Last
    }
}
#[doc = "Field `EOQ` writer - End Of Queue"]
pub type EoqW<'a, REG> = crate::BitWriter<'a, REG, Eoq>;
impl<'a, REG> EoqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not last data in queue"]
    #[inline(always)]
    pub fn not_last(self) -> &'a mut crate::W<REG> {
        self.variant(Eoq::NotLast)
    }
    #[doc = "Last data in queue"]
    #[inline(always)]
    pub fn last(self) -> &'a mut crate::W<REG> {
        self.variant(Eoq::Last)
    }
}
#[doc = "Clock and Transfer Attributes Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctas {
    #[doc = "0: CTAR0"]
    Ctar0 = 0,
    #[doc = "1: CTAR1"]
    Ctar1 = 1,
}
impl From<Ctas> for u8 {
    #[inline(always)]
    fn from(variant: Ctas) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctas {
    type Ux = u8;
}
impl crate::IsEnum for Ctas {}
#[doc = "Field `CTAS` reader - Clock and Transfer Attributes Select."]
pub type CtasR = crate::FieldReader<Ctas>;
impl CtasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctas> {
        match self.bits {
            0 => Some(Ctas::Ctar0),
            1 => Some(Ctas::Ctar1),
            _ => None,
        }
    }
    #[doc = "CTAR0"]
    #[inline(always)]
    pub fn is_ctar0(&self) -> bool {
        *self == Ctas::Ctar0
    }
    #[doc = "CTAR1"]
    #[inline(always)]
    pub fn is_ctar1(&self) -> bool {
        *self == Ctas::Ctar1
    }
}
#[doc = "Field `CTAS` writer - Clock and Transfer Attributes Select."]
pub type CtasW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ctas>;
impl<'a, REG> CtasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CTAR0"]
    #[inline(always)]
    pub fn ctar0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctas::Ctar0)
    }
    #[doc = "CTAR1"]
    #[inline(always)]
    pub fn ctar1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctas::Ctar1)
    }
}
#[doc = "Continuous Peripheral Chip Select Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cont {
    #[doc = "0: Return PCS to inactive between transfers"]
    Deassert = 0,
    #[doc = "1: Keep PCS asserted between transfers"]
    Assert = 1,
}
impl From<Cont> for bool {
    #[inline(always)]
    fn from(variant: Cont) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT` reader - Continuous Peripheral Chip Select Enable"]
pub type ContR = crate::BitReader<Cont>;
impl ContR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cont {
        match self.bits {
            false => Cont::Deassert,
            true => Cont::Assert,
        }
    }
    #[doc = "Return PCS to inactive between transfers"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        *self == Cont::Deassert
    }
    #[doc = "Keep PCS asserted between transfers"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == Cont::Assert
    }
}
#[doc = "Field `CONT` writer - Continuous Peripheral Chip Select Enable"]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG, Cont>;
impl<'a, REG> ContW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Return PCS to inactive between transfers"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut crate::W<REG> {
        self.variant(Cont::Deassert)
    }
    #[doc = "Keep PCS asserted between transfers"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut crate::W<REG> {
        self.variant(Cont::Assert)
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn txdata(&self) -> TxdataR {
        TxdataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs(&self) -> PcsR {
        PcsR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - Clear Transfer Counter."]
    #[inline(always)]
    pub fn ctcnt(&self) -> CtcntR {
        CtcntR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - End Of Queue"]
    #[inline(always)]
    pub fn eoq(&self) -> EoqR {
        EoqR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Clock and Transfer Attributes Select."]
    #[inline(always)]
    pub fn ctas(&self) -> CtasR {
        CtasR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Continuous Peripheral Chip Select Enable"]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TxdataW<'_, Spi0PushrSpec> {
        TxdataW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PcsW<'_, Spi0PushrSpec> {
        PcsW::new(self, 16)
    }
    #[doc = "Bit 26 - Clear Transfer Counter."]
    #[inline(always)]
    pub fn ctcnt(&mut self) -> CtcntW<'_, Spi0PushrSpec> {
        CtcntW::new(self, 26)
    }
    #[doc = "Bit 27 - End Of Queue"]
    #[inline(always)]
    pub fn eoq(&mut self) -> EoqW<'_, Spi0PushrSpec> {
        EoqW::new(self, 27)
    }
    #[doc = "Bits 28:30 - Clock and Transfer Attributes Select."]
    #[inline(always)]
    pub fn ctas(&mut self) -> CtasW<'_, Spi0PushrSpec> {
        CtasW::new(self, 28)
    }
    #[doc = "Bit 31 - Continuous Peripheral Chip Select Enable"]
    #[inline(always)]
    pub fn cont(&mut self) -> ContW<'_, Spi0PushrSpec> {
        ContW::new(self, 31)
    }
}
#[doc = "DSPI PUSH TX FIFO Register In Master Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_pushr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_pushr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0PushrSpec;
impl crate::RegisterSpec for Spi0PushrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi0_pushr::R`](R) reader structure"]
impl crate::Readable for Spi0PushrSpec {}
#[doc = "`write(|w| ..)` method takes [`spi0_pushr::W`](W) writer structure"]
impl crate::Writable for Spi0PushrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PUSHR to value 0"]
impl crate::Resettable for Spi0PushrSpec {}

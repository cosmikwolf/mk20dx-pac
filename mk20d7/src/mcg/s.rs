#[doc = "Register `S` reader"]
pub type R = crate::R<SSpec>;
#[doc = "Register `S` writer"]
pub type W = crate::W<SSpec>;
#[doc = "Internal Reference Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ircst {
    #[doc = "0: Source of internal reference clock is the slow clock (32 kHz IRC)"]
    Slow = 0,
    #[doc = "1: Source of internal reference clock is the fast clock (4 MHz IRC)"]
    Fast = 1,
}
impl From<Ircst> for bool {
    #[inline(always)]
    fn from(variant: Ircst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRCST` reader - Internal Reference Clock Status"]
pub type IrcstR = crate::BitReader<Ircst>;
impl IrcstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ircst {
        match self.bits {
            false => Ircst::Slow,
            true => Ircst::Fast,
        }
    }
    #[doc = "Source of internal reference clock is the slow clock (32 kHz IRC)"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == Ircst::Slow
    }
    #[doc = "Source of internal reference clock is the fast clock (4 MHz IRC)"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == Ircst::Fast
    }
}
#[doc = "Field `OSCINIT0` reader - OSC Initialization"]
pub type Oscinit0R = crate::BitReader;
#[doc = "Clock Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkst {
    #[doc = "0: Output of the FLL is selected (reset default)"]
    Fll = 0,
    #[doc = "1: Internal reference clock is selected"]
    Internal = 1,
    #[doc = "2: External reference clock is selected"]
    External = 2,
    #[doc = "3: Output of the PLL is selected"]
    Pll = 3,
}
impl From<Clkst> for u8 {
    #[inline(always)]
    fn from(variant: Clkst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkst {
    type Ux = u8;
}
impl crate::IsEnum for Clkst {}
#[doc = "Field `CLKST` reader - Clock Mode Status"]
pub type ClkstR = crate::FieldReader<Clkst>;
impl ClkstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkst {
        match self.bits {
            0 => Clkst::Fll,
            1 => Clkst::Internal,
            2 => Clkst::External,
            3 => Clkst::Pll,
            _ => unreachable!(),
        }
    }
    #[doc = "Output of the FLL is selected (reset default)"]
    #[inline(always)]
    pub fn is_fll(&self) -> bool {
        *self == Clkst::Fll
    }
    #[doc = "Internal reference clock is selected"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Clkst::Internal
    }
    #[doc = "External reference clock is selected"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Clkst::External
    }
    #[doc = "Output of the PLL is selected"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == Clkst::Pll
    }
}
#[doc = "Internal Reference Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irefst {
    #[doc = "0: Source of FLL reference clock is the external reference clock"]
    External = 0,
    #[doc = "1: Source of FLL reference clock is the internal reference clock"]
    Internal = 1,
}
impl From<Irefst> for bool {
    #[inline(always)]
    fn from(variant: Irefst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREFST` reader - Internal Reference Status"]
pub type IrefstR = crate::BitReader<Irefst>;
impl IrefstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irefst {
        match self.bits {
            false => Irefst::External,
            true => Irefst::Internal,
        }
    }
    #[doc = "Source of FLL reference clock is the external reference clock"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Irefst::External
    }
    #[doc = "Source of FLL reference clock is the internal reference clock"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Irefst::Internal
    }
}
#[doc = "PLL Select Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pllst {
    #[doc = "0: Source of PLLS clock is FLL clock"]
    Fll = 0,
    #[doc = "1: Source of PLLS clock is PLL output clock"]
    Pll = 1,
}
impl From<Pllst> for bool {
    #[inline(always)]
    fn from(variant: Pllst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLST` reader - PLL Select Status"]
pub type PllstR = crate::BitReader<Pllst>;
impl PllstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pllst {
        match self.bits {
            false => Pllst::Fll,
            true => Pllst::Pll,
        }
    }
    #[doc = "Source of PLLS clock is FLL clock"]
    #[inline(always)]
    pub fn is_fll(&self) -> bool {
        *self == Pllst::Fll
    }
    #[doc = "Source of PLLS clock is PLL output clock"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == Pllst::Pll
    }
}
#[doc = "Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock0 {
    #[doc = "0: PLL is currently unlocked"]
    Unlocked = 0,
    #[doc = "1: PLL is currently locked"]
    Locked = 1,
}
impl From<Lock0> for bool {
    #[inline(always)]
    fn from(variant: Lock0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK0` reader - Lock Status"]
pub type Lock0R = crate::BitReader<Lock0>;
impl Lock0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock0 {
        match self.bits {
            false => Lock0::Unlocked,
            true => Lock0::Locked,
        }
    }
    #[doc = "PLL is currently unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock0::Unlocked
    }
    #[doc = "PLL is currently locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock0::Locked
    }
}
#[doc = "Loss of Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lols0 {
    #[doc = "0: PLL has not lost lock since LOLS0 was last cleared"]
    NoLoss = 0,
    #[doc = "1: PLL has lost lock since LOLS0 was last cleared"]
    LossDetected = 1,
}
impl From<Lols0> for bool {
    #[inline(always)]
    fn from(variant: Lols0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOLS0` reader - Loss of Lock Status"]
pub type Lols0R = crate::BitReader<Lols0>;
impl Lols0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lols0 {
        match self.bits {
            false => Lols0::NoLoss,
            true => Lols0::LossDetected,
        }
    }
    #[doc = "PLL has not lost lock since LOLS0 was last cleared"]
    #[inline(always)]
    pub fn is_no_loss(&self) -> bool {
        *self == Lols0::NoLoss
    }
    #[doc = "PLL has lost lock since LOLS0 was last cleared"]
    #[inline(always)]
    pub fn is_loss_detected(&self) -> bool {
        *self == Lols0::LossDetected
    }
}
#[doc = "Field `LOLS0` writer - Loss of Lock Status"]
pub type Lols0W<'a, REG> = crate::BitWriter<'a, REG, Lols0>;
impl<'a, REG> Lols0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL has not lost lock since LOLS0 was last cleared"]
    #[inline(always)]
    pub fn no_loss(self) -> &'a mut crate::W<REG> {
        self.variant(Lols0::NoLoss)
    }
    #[doc = "PLL has lost lock since LOLS0 was last cleared"]
    #[inline(always)]
    pub fn loss_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Lols0::LossDetected)
    }
}
impl R {
    #[doc = "Bit 0 - Internal Reference Clock Status"]
    #[inline(always)]
    pub fn ircst(&self) -> IrcstR {
        IrcstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OSC Initialization"]
    #[inline(always)]
    pub fn oscinit0(&self) -> Oscinit0R {
        Oscinit0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Clock Mode Status"]
    #[inline(always)]
    pub fn clkst(&self) -> ClkstR {
        ClkstR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Internal Reference Status"]
    #[inline(always)]
    pub fn irefst(&self) -> IrefstR {
        IrefstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL Select Status"]
    #[inline(always)]
    pub fn pllst(&self) -> PllstR {
        PllstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lock Status"]
    #[inline(always)]
    pub fn lock0(&self) -> Lock0R {
        Lock0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Loss of Lock Status"]
    #[inline(always)]
    pub fn lols0(&self) -> Lols0R {
        Lols0R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Loss of Lock Status"]
    #[inline(always)]
    pub fn lols0(&mut self) -> Lols0W<'_, SSpec> {
        Lols0W::new(self, 7)
    }
}
#[doc = "MCG Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSpec;
impl crate::RegisterSpec for SSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`s::R`](R) reader structure"]
impl crate::Readable for SSpec {}
#[doc = "`write(|w| ..)` method takes [`s::W`](W) writer structure"]
impl crate::Writable for SSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S to value 0x10"]
impl crate::Resettable for SSpec {
    const RESET_VALUE: u8 = 0x10;
}

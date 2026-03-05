#[doc = "Register `C2` reader"]
pub type R = crate::R<C2Spec>;
#[doc = "Register `C2` writer"]
pub type W = crate::W<C2Spec>;
#[doc = "Internal Reference Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ircs {
    #[doc = "0: Slow internal reference clock selected"]
    SlowIrc = 0,
    #[doc = "1: Fast internal reference clock selected"]
    FastIrc = 1,
}
impl From<Ircs> for bool {
    #[inline(always)]
    fn from(variant: Ircs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRCS` reader - Internal Reference Clock Select"]
pub type IrcsR = crate::BitReader<Ircs>;
impl IrcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ircs {
        match self.bits {
            false => Ircs::SlowIrc,
            true => Ircs::FastIrc,
        }
    }
    #[doc = "Slow internal reference clock selected"]
    #[inline(always)]
    pub fn is_slow_irc(&self) -> bool {
        *self == Ircs::SlowIrc
    }
    #[doc = "Fast internal reference clock selected"]
    #[inline(always)]
    pub fn is_fast_irc(&self) -> bool {
        *self == Ircs::FastIrc
    }
}
#[doc = "Field `IRCS` writer - Internal Reference Clock Select"]
pub type IrcsW<'a, REG> = crate::BitWriter<'a, REG, Ircs>;
impl<'a, REG> IrcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slow internal reference clock selected"]
    #[inline(always)]
    pub fn slow_irc(self) -> &'a mut crate::W<REG> {
        self.variant(Ircs::SlowIrc)
    }
    #[doc = "Fast internal reference clock selected"]
    #[inline(always)]
    pub fn fast_irc(self) -> &'a mut crate::W<REG> {
        self.variant(Ircs::FastIrc)
    }
}
#[doc = "Low Power Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lp {
    #[doc = "0: FLL or PLL is not disabled in bypass modes"]
    FllPllActive = 0,
    #[doc = "1: FLL or PLL is disabled in bypass modes"]
    FllPllDisabled = 1,
}
impl From<Lp> for bool {
    #[inline(always)]
    fn from(variant: Lp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LP` reader - Low Power Select"]
pub type LpR = crate::BitReader<Lp>;
impl LpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lp {
        match self.bits {
            false => Lp::FllPllActive,
            true => Lp::FllPllDisabled,
        }
    }
    #[doc = "FLL or PLL is not disabled in bypass modes"]
    #[inline(always)]
    pub fn is_fll_pll_active(&self) -> bool {
        *self == Lp::FllPllActive
    }
    #[doc = "FLL or PLL is disabled in bypass modes"]
    #[inline(always)]
    pub fn is_fll_pll_disabled(&self) -> bool {
        *self == Lp::FllPllDisabled
    }
}
#[doc = "Field `LP` writer - Low Power Select"]
pub type LpW<'a, REG> = crate::BitWriter<'a, REG, Lp>;
impl<'a, REG> LpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FLL or PLL is not disabled in bypass modes"]
    #[inline(always)]
    pub fn fll_pll_active(self) -> &'a mut crate::W<REG> {
        self.variant(Lp::FllPllActive)
    }
    #[doc = "FLL or PLL is disabled in bypass modes"]
    #[inline(always)]
    pub fn fll_pll_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lp::FllPllDisabled)
    }
}
#[doc = "External Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erefs0 {
    #[doc = "0: External clock requested"]
    ExternalClock = 0,
    #[doc = "1: Oscillator requested"]
    Oscillator = 1,
}
impl From<Erefs0> for bool {
    #[inline(always)]
    fn from(variant: Erefs0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EREFS0` reader - External Reference Select"]
pub type Erefs0R = crate::BitReader<Erefs0>;
impl Erefs0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erefs0 {
        match self.bits {
            false => Erefs0::ExternalClock,
            true => Erefs0::Oscillator,
        }
    }
    #[doc = "External clock requested"]
    #[inline(always)]
    pub fn is_external_clock(&self) -> bool {
        *self == Erefs0::ExternalClock
    }
    #[doc = "Oscillator requested"]
    #[inline(always)]
    pub fn is_oscillator(&self) -> bool {
        *self == Erefs0::Oscillator
    }
}
#[doc = "Field `EREFS0` writer - External Reference Select"]
pub type Erefs0W<'a, REG> = crate::BitWriter<'a, REG, Erefs0>;
impl<'a, REG> Erefs0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External clock requested"]
    #[inline(always)]
    pub fn external_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Erefs0::ExternalClock)
    }
    #[doc = "Oscillator requested"]
    #[inline(always)]
    pub fn oscillator(self) -> &'a mut crate::W<REG> {
        self.variant(Erefs0::Oscillator)
    }
}
#[doc = "High Gain Oscillator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hgo0 {
    #[doc = "0: Configure crystal oscillator for low-power operation"]
    LowPower = 0,
    #[doc = "1: Configure crystal oscillator for high-gain operation"]
    HighGain = 1,
}
impl From<Hgo0> for bool {
    #[inline(always)]
    fn from(variant: Hgo0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HGO0` reader - High Gain Oscillator Select"]
pub type Hgo0R = crate::BitReader<Hgo0>;
impl Hgo0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hgo0 {
        match self.bits {
            false => Hgo0::LowPower,
            true => Hgo0::HighGain,
        }
    }
    #[doc = "Configure crystal oscillator for low-power operation"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == Hgo0::LowPower
    }
    #[doc = "Configure crystal oscillator for high-gain operation"]
    #[inline(always)]
    pub fn is_high_gain(&self) -> bool {
        *self == Hgo0::HighGain
    }
}
#[doc = "Field `HGO0` writer - High Gain Oscillator Select"]
pub type Hgo0W<'a, REG> = crate::BitWriter<'a, REG, Hgo0>;
impl<'a, REG> Hgo0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configure crystal oscillator for low-power operation"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(Hgo0::LowPower)
    }
    #[doc = "Configure crystal oscillator for high-gain operation"]
    #[inline(always)]
    pub fn high_gain(self) -> &'a mut crate::W<REG> {
        self.variant(Hgo0::HighGain)
    }
}
#[doc = "Frequency Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Range0 {
    #[doc = "0: Low frequency range for the crystal oscillator"]
    Low = 0,
    #[doc = "1: High frequency range for the crystal oscillator"]
    High = 1,
}
impl From<Range0> for u8 {
    #[inline(always)]
    fn from(variant: Range0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Range0 {
    type Ux = u8;
}
impl crate::IsEnum for Range0 {}
#[doc = "Field `RANGE0` reader - Frequency Range Select"]
pub type Range0R = crate::FieldReader<Range0>;
impl Range0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Range0> {
        match self.bits {
            0 => Some(Range0::Low),
            1 => Some(Range0::High),
            _ => None,
        }
    }
    #[doc = "Low frequency range for the crystal oscillator"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Range0::Low
    }
    #[doc = "High frequency range for the crystal oscillator"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Range0::High
    }
}
#[doc = "Field `RANGE0` writer - Frequency Range Select"]
pub type Range0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Range0>;
impl<'a, REG> Range0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low frequency range for the crystal oscillator"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Range0::Low)
    }
    #[doc = "High frequency range for the crystal oscillator"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Range0::High)
    }
}
#[doc = "Loss of Clock Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locre0 {
    #[doc = "0: Interrupt request is generated on a loss of OSC0 external reference clock"]
    Interrupt = 0,
    #[doc = "1: Generate a reset request on a loss of OSC0 external reference clock"]
    Reset = 1,
}
impl From<Locre0> for bool {
    #[inline(always)]
    fn from(variant: Locre0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCRE0` reader - Loss of Clock Reset Enable"]
pub type Locre0R = crate::BitReader<Locre0>;
impl Locre0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Locre0 {
        match self.bits {
            false => Locre0::Interrupt,
            true => Locre0::Reset,
        }
    }
    #[doc = "Interrupt request is generated on a loss of OSC0 external reference clock"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Locre0::Interrupt
    }
    #[doc = "Generate a reset request on a loss of OSC0 external reference clock"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Locre0::Reset
    }
}
#[doc = "Field `LOCRE0` writer - Loss of Clock Reset Enable"]
pub type Locre0W<'a, REG> = crate::BitWriter<'a, REG, Locre0>;
impl<'a, REG> Locre0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt request is generated on a loss of OSC0 external reference clock"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Locre0::Interrupt)
    }
    #[doc = "Generate a reset request on a loss of OSC0 external reference clock"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Locre0::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Internal Reference Clock Select"]
    #[inline(always)]
    pub fn ircs(&self) -> IrcsR {
        IrcsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Power Select"]
    #[inline(always)]
    pub fn lp(&self) -> LpR {
        LpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Reference Select"]
    #[inline(always)]
    pub fn erefs0(&self) -> Erefs0R {
        Erefs0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline(always)]
    pub fn hgo0(&self) -> Hgo0R {
        Hgo0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Frequency Range Select"]
    #[inline(always)]
    pub fn range0(&self) -> Range0R {
        Range0R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    pub fn locre0(&self) -> Locre0R {
        Locre0R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Clock Select"]
    #[inline(always)]
    pub fn ircs(&mut self) -> IrcsW<'_, C2Spec> {
        IrcsW::new(self, 0)
    }
    #[doc = "Bit 1 - Low Power Select"]
    #[inline(always)]
    pub fn lp(&mut self) -> LpW<'_, C2Spec> {
        LpW::new(self, 1)
    }
    #[doc = "Bit 2 - External Reference Select"]
    #[inline(always)]
    pub fn erefs0(&mut self) -> Erefs0W<'_, C2Spec> {
        Erefs0W::new(self, 2)
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline(always)]
    pub fn hgo0(&mut self) -> Hgo0W<'_, C2Spec> {
        Hgo0W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Frequency Range Select"]
    #[inline(always)]
    pub fn range0(&mut self) -> Range0W<'_, C2Spec> {
        Range0W::new(self, 4)
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    pub fn locre0(&mut self) -> Locre0W<'_, C2Spec> {
        Locre0W::new(self, 7)
    }
}
#[doc = "MCG Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2Spec;
impl crate::RegisterSpec for C2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c2::R`](R) reader structure"]
impl crate::Readable for C2Spec {}
#[doc = "`write(|w| ..)` method takes [`c2::W`](W) writer structure"]
impl crate::Writable for C2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C2 to value 0x80"]
impl crate::Resettable for C2Spec {
    const RESET_VALUE: u8 = 0x80;
}

#[doc = "Register `PCR%s` reader"]
pub type R = crate::R<PcrSpec>;
#[doc = "Register `PCR%s` writer"]
pub type W = crate::W<PcrSpec>;
#[doc = "Pull Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ps {
    #[doc = "0: Internal pulldown resistor enabled"]
    PullDown = 0,
    #[doc = "1: Internal pullup resistor enabled"]
    PullUp = 1,
}
impl From<Ps> for bool {
    #[inline(always)]
    fn from(variant: Ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS` reader - Pull Select"]
pub type PsR = crate::BitReader<Ps>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ps {
        match self.bits {
            false => Ps::PullDown,
            true => Ps::PullUp,
        }
    }
    #[doc = "Internal pulldown resistor enabled"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Ps::PullDown
    }
    #[doc = "Internal pullup resistor enabled"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Ps::PullUp
    }
}
#[doc = "Field `PS` writer - Pull Select"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG, Ps>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal pulldown resistor enabled"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::PullDown)
    }
    #[doc = "Internal pullup resistor enabled"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::PullUp)
    }
}
#[doc = "Pull Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: Internal pull resistor disabled"]
    Disabled = 0,
    #[doc = "1: Internal pull resistor enabled"]
    Enabled = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Pull Enable"]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::Disabled,
            true => Pe::Enabled,
        }
    }
    #[doc = "Internal pull resistor disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pe::Disabled
    }
    #[doc = "Internal pull resistor enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pe::Enabled
    }
}
#[doc = "Field `PE` writer - Pull Enable"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG, Pe>;
impl<'a, REG> PeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal pull resistor disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::Disabled)
    }
    #[doc = "Internal pull resistor enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::Enabled)
    }
}
#[doc = "Slew Rate Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sre {
    #[doc = "0: Fast slew rate"]
    Fast = 0,
    #[doc = "1: Slow slew rate"]
    Slow = 1,
}
impl From<Sre> for bool {
    #[inline(always)]
    fn from(variant: Sre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRE` reader - Slew Rate Enable"]
pub type SreR = crate::BitReader<Sre>;
impl SreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sre {
        match self.bits {
            false => Sre::Fast,
            true => Sre::Slow,
        }
    }
    #[doc = "Fast slew rate"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == Sre::Fast
    }
    #[doc = "Slow slew rate"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == Sre::Slow
    }
}
#[doc = "Field `SRE` writer - Slew Rate Enable"]
pub type SreW<'a, REG> = crate::BitWriter<'a, REG, Sre>;
impl<'a, REG> SreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast slew rate"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(Sre::Fast)
    }
    #[doc = "Slow slew rate"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut crate::W<REG> {
        self.variant(Sre::Slow)
    }
}
#[doc = "Passive Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfe {
    #[doc = "0: Passive input filter disabled"]
    Disabled = 0,
    #[doc = "1: Passive input filter enabled"]
    Enabled = 1,
}
impl From<Pfe> for bool {
    #[inline(always)]
    fn from(variant: Pfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFE` reader - Passive Filter Enable"]
pub type PfeR = crate::BitReader<Pfe>;
impl PfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfe {
        match self.bits {
            false => Pfe::Disabled,
            true => Pfe::Enabled,
        }
    }
    #[doc = "Passive input filter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pfe::Disabled
    }
    #[doc = "Passive input filter enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pfe::Enabled
    }
}
#[doc = "Field `PFE` writer - Passive Filter Enable"]
pub type PfeW<'a, REG> = crate::BitWriter<'a, REG, Pfe>;
impl<'a, REG> PfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Passive input filter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pfe::Disabled)
    }
    #[doc = "Passive input filter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pfe::Enabled)
    }
}
#[doc = "Open Drain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ode {
    #[doc = "0: Open drain output disabled"]
    Disabled = 0,
    #[doc = "1: Open drain output enabled"]
    Enabled = 1,
}
impl From<Ode> for bool {
    #[inline(always)]
    fn from(variant: Ode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODE` reader - Open Drain Enable"]
pub type OdeR = crate::BitReader<Ode>;
impl OdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ode {
        match self.bits {
            false => Ode::Disabled,
            true => Ode::Enabled,
        }
    }
    #[doc = "Open drain output disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ode::Disabled
    }
    #[doc = "Open drain output enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ode::Enabled
    }
}
#[doc = "Field `ODE` writer - Open Drain Enable"]
pub type OdeW<'a, REG> = crate::BitWriter<'a, REG, Ode>;
impl<'a, REG> OdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Open drain output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ode::Disabled)
    }
    #[doc = "Open drain output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ode::Enabled)
    }
}
#[doc = "Drive Strength Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dse {
    #[doc = "0: Low drive strength"]
    Low = 0,
    #[doc = "1: High drive strength"]
    High = 1,
}
impl From<Dse> for bool {
    #[inline(always)]
    fn from(variant: Dse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSE` reader - Drive Strength Enable"]
pub type DseR = crate::BitReader<Dse>;
impl DseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dse {
        match self.bits {
            false => Dse::Low,
            true => Dse::High,
        }
    }
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Dse::Low
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Dse::High
    }
}
#[doc = "Field `DSE` writer - Drive Strength Enable"]
pub type DseW<'a, REG> = crate::BitWriter<'a, REG, Dse>;
impl<'a, REG> DseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low drive strength"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Dse::Low)
    }
    #[doc = "High drive strength"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Dse::High)
    }
}
#[doc = "Pin Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mux {
    #[doc = "0: Pin disabled (analog)"]
    Disabled = 0,
    #[doc = "1: Alternative 1 (GPIO)"]
    Gpio = 1,
    #[doc = "2: Alternative 2 (chip-specific)"]
    Alt2 = 2,
    #[doc = "3: Alternative 3 (chip-specific)"]
    Alt3 = 3,
    #[doc = "4: Alternative 4 (chip-specific)"]
    Alt4 = 4,
    #[doc = "5: Alternative 5 (chip-specific)"]
    Alt5 = 5,
    #[doc = "6: Alternative 6 (chip-specific)"]
    Alt6 = 6,
    #[doc = "7: Alternative 7 (chip-specific)"]
    Alt7 = 7,
}
impl From<Mux> for u8 {
    #[inline(always)]
    fn from(variant: Mux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mux {
    type Ux = u8;
}
impl crate::IsEnum for Mux {}
#[doc = "Field `MUX` reader - Pin Mux Control"]
pub type MuxR = crate::FieldReader<Mux>;
impl MuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mux {
        match self.bits {
            0 => Mux::Disabled,
            1 => Mux::Gpio,
            2 => Mux::Alt2,
            3 => Mux::Alt3,
            4 => Mux::Alt4,
            5 => Mux::Alt5,
            6 => Mux::Alt6,
            7 => Mux::Alt7,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin disabled (analog)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mux::Disabled
    }
    #[doc = "Alternative 1 (GPIO)"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Mux::Gpio
    }
    #[doc = "Alternative 2 (chip-specific)"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == Mux::Alt2
    }
    #[doc = "Alternative 3 (chip-specific)"]
    #[inline(always)]
    pub fn is_alt3(&self) -> bool {
        *self == Mux::Alt3
    }
    #[doc = "Alternative 4 (chip-specific)"]
    #[inline(always)]
    pub fn is_alt4(&self) -> bool {
        *self == Mux::Alt4
    }
    #[doc = "Alternative 5 (chip-specific)"]
    #[inline(always)]
    pub fn is_alt5(&self) -> bool {
        *self == Mux::Alt5
    }
    #[doc = "Alternative 6 (chip-specific)"]
    #[inline(always)]
    pub fn is_alt6(&self) -> bool {
        *self == Mux::Alt6
    }
    #[doc = "Alternative 7 (chip-specific)"]
    #[inline(always)]
    pub fn is_alt7(&self) -> bool {
        *self == Mux::Alt7
    }
}
#[doc = "Field `MUX` writer - Pin Mux Control"]
pub type MuxW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mux, crate::Safe>;
impl<'a, REG> MuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin disabled (analog)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Disabled)
    }
    #[doc = "Alternative 1 (GPIO)"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Gpio)
    }
    #[doc = "Alternative 2 (chip-specific)"]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Alt2)
    }
    #[doc = "Alternative 3 (chip-specific)"]
    #[inline(always)]
    pub fn alt3(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Alt3)
    }
    #[doc = "Alternative 4 (chip-specific)"]
    #[inline(always)]
    pub fn alt4(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Alt4)
    }
    #[doc = "Alternative 5 (chip-specific)"]
    #[inline(always)]
    pub fn alt5(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Alt5)
    }
    #[doc = "Alternative 6 (chip-specific)"]
    #[inline(always)]
    pub fn alt6(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Alt6)
    }
    #[doc = "Alternative 7 (chip-specific)"]
    #[inline(always)]
    pub fn alt7(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::Alt7)
    }
}
#[doc = "Lock Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lk {
    #[doc = "0: Pin control register is not locked"]
    Unlocked = 0,
    #[doc = "1: Pin control register is locked"]
    Locked = 1,
}
impl From<Lk> for bool {
    #[inline(always)]
    fn from(variant: Lk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LK` reader - Lock Register"]
pub type LkR = crate::BitReader<Lk>;
impl LkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lk {
        match self.bits {
            false => Lk::Unlocked,
            true => Lk::Locked,
        }
    }
    #[doc = "Pin control register is not locked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lk::Unlocked
    }
    #[doc = "Pin control register is locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lk::Locked
    }
}
#[doc = "Field `LK` writer - Lock Register"]
pub type LkW<'a, REG> = crate::BitWriter<'a, REG, Lk>;
impl<'a, REG> LkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin control register is not locked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Lk::Unlocked)
    }
    #[doc = "Pin control register is locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Lk::Locked)
    }
}
#[doc = "Interrupt Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irqc {
    #[doc = "0: Interrupt/DMA Request disabled."]
    _0000 = 0,
    #[doc = "1: DMA Request on rising edge."]
    _0001 = 1,
    #[doc = "2: DMA Request on falling edge."]
    _0010 = 2,
    #[doc = "3: DMA Request on either edge."]
    _0011 = 3,
    #[doc = "8: Interrupt when logic zero."]
    _1000 = 8,
    #[doc = "9: Interrupt on rising edge."]
    _1001 = 9,
    #[doc = "10: Interrupt on falling edge."]
    _1010 = 10,
    #[doc = "11: Interrupt on either edge."]
    _1011 = 11,
    #[doc = "12: Interrupt when logic one."]
    _1100 = 12,
}
impl From<Irqc> for u8 {
    #[inline(always)]
    fn from(variant: Irqc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irqc {
    type Ux = u8;
}
impl crate::IsEnum for Irqc {}
#[doc = "Field `IRQC` reader - Interrupt Configuration"]
pub type IrqcR = crate::FieldReader<Irqc>;
impl IrqcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Irqc> {
        match self.bits {
            0 => Some(Irqc::_0000),
            1 => Some(Irqc::_0001),
            2 => Some(Irqc::_0010),
            3 => Some(Irqc::_0011),
            8 => Some(Irqc::_1000),
            9 => Some(Irqc::_1001),
            10 => Some(Irqc::_1010),
            11 => Some(Irqc::_1011),
            12 => Some(Irqc::_1100),
            _ => None,
        }
    }
    #[doc = "Interrupt/DMA Request disabled."]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == Irqc::_0000
    }
    #[doc = "DMA Request on rising edge."]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == Irqc::_0001
    }
    #[doc = "DMA Request on falling edge."]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == Irqc::_0010
    }
    #[doc = "DMA Request on either edge."]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == Irqc::_0011
    }
    #[doc = "Interrupt when logic zero."]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Irqc::_1000
    }
    #[doc = "Interrupt on rising edge."]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == Irqc::_1001
    }
    #[doc = "Interrupt on falling edge."]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == Irqc::_1010
    }
    #[doc = "Interrupt on either edge."]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == Irqc::_1011
    }
    #[doc = "Interrupt when logic one."]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == Irqc::_1100
    }
}
#[doc = "Field `IRQC` writer - Interrupt Configuration"]
pub type IrqcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Irqc>;
impl<'a, REG> IrqcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt/DMA Request disabled."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_0000)
    }
    #[doc = "DMA Request on rising edge."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_0001)
    }
    #[doc = "DMA Request on falling edge."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_0010)
    }
    #[doc = "DMA Request on either edge."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_0011)
    }
    #[doc = "Interrupt when logic zero."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_1000)
    }
    #[doc = "Interrupt on rising edge."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_1001)
    }
    #[doc = "Interrupt on falling edge."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_1010)
    }
    #[doc = "Interrupt on either edge."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_1011)
    }
    #[doc = "Interrupt when logic one."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::_1100)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf {
    #[doc = "0: Configured interrupt has not been detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt has been detected. If pin is configured to generate a DMA request then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer, otherwise the flag remains set until a logic one is written to that flag. If configured for a level sensitive interrupt that remains asserted then flag will set again immediately."]
    _1 = 1,
}
impl From<Isf> for bool {
    #[inline(always)]
    fn from(variant: Isf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF` reader - Interrupt Status Flag"]
pub type IsfR = crate::BitReader<Isf>;
impl IsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf {
        match self.bits {
            false => Isf::_0,
            true => Isf::_1,
        }
    }
    #[doc = "Configured interrupt has not been detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Isf::_0
    }
    #[doc = "Configured interrupt has been detected. If pin is configured to generate a DMA request then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer, otherwise the flag remains set until a logic one is written to that flag. If configured for a level sensitive interrupt that remains asserted then flag will set again immediately."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Isf::_1
    }
}
#[doc = "Field `ISF` writer - Interrupt Status Flag"]
pub type IsfW<'a, REG> = crate::BitWriter<'a, REG, Isf>;
impl<'a, REG> IsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt has not been detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Isf::_0)
    }
    #[doc = "Configured interrupt has been detected. If pin is configured to generate a DMA request then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer, otherwise the flag remains set until a logic one is written to that flag. If configured for a level sensitive interrupt that remains asserted then flag will set again immediately."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Isf::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Pull Select"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pull Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slew Rate Enable"]
    #[inline(always)]
    pub fn sre(&self) -> SreR {
        SreR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Passive Filter Enable"]
    #[inline(always)]
    pub fn pfe(&self) -> PfeR {
        PfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Open Drain Enable"]
    #[inline(always)]
    pub fn ode(&self) -> OdeR {
        OdeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Drive Strength Enable"]
    #[inline(always)]
    pub fn dse(&self) -> DseR {
        DseR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Pin Mux Control"]
    #[inline(always)]
    pub fn mux(&self) -> MuxR {
        MuxR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Lock Register"]
    #[inline(always)]
    pub fn lk(&self) -> LkR {
        LkR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Interrupt Configuration"]
    #[inline(always)]
    pub fn irqc(&self) -> IrqcR {
        IrqcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf(&self) -> IsfR {
        IsfR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pull Select"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, PcrSpec> {
        PsW::new(self, 0)
    }
    #[doc = "Bit 1 - Pull Enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, PcrSpec> {
        PeW::new(self, 1)
    }
    #[doc = "Bit 2 - Slew Rate Enable"]
    #[inline(always)]
    pub fn sre(&mut self) -> SreW<'_, PcrSpec> {
        SreW::new(self, 2)
    }
    #[doc = "Bit 4 - Passive Filter Enable"]
    #[inline(always)]
    pub fn pfe(&mut self) -> PfeW<'_, PcrSpec> {
        PfeW::new(self, 4)
    }
    #[doc = "Bit 5 - Open Drain Enable"]
    #[inline(always)]
    pub fn ode(&mut self) -> OdeW<'_, PcrSpec> {
        OdeW::new(self, 5)
    }
    #[doc = "Bit 6 - Drive Strength Enable"]
    #[inline(always)]
    pub fn dse(&mut self) -> DseW<'_, PcrSpec> {
        DseW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Pin Mux Control"]
    #[inline(always)]
    pub fn mux(&mut self) -> MuxW<'_, PcrSpec> {
        MuxW::new(self, 8)
    }
    #[doc = "Bit 15 - Lock Register"]
    #[inline(always)]
    pub fn lk(&mut self) -> LkW<'_, PcrSpec> {
        LkW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Interrupt Configuration"]
    #[inline(always)]
    pub fn irqc(&mut self) -> IrqcW<'_, PcrSpec> {
        IrqcW::new(self, 16)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf(&mut self) -> IsfW<'_, PcrSpec> {
        IsfW::new(self, 24)
    }
}
#[doc = "Pin Control Register n\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcrSpec;
impl crate::RegisterSpec for PcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr::R`](R) reader structure"]
impl crate::Readable for PcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pcr::W`](W) writer structure"]
impl crate::Writable for PcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCR%s to value 0"]
impl crate::Resettable for PcrSpec {}

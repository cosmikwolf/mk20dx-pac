#[doc = "Register `SCGC5` reader"]
pub type R = crate::R<Scgc5Spec>;
#[doc = "Register `SCGC5` writer"]
pub type W = crate::W<Scgc5Spec>;
#[doc = "Low Power Timer Access Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lptimer {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Lptimer> for bool {
    #[inline(always)]
    fn from(variant: Lptimer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTIMER` reader - Low Power Timer Access Control"]
pub type LptimerR = crate::BitReader<Lptimer>;
impl LptimerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptimer {
        match self.bits {
            false => Lptimer::Disabled,
            true => Lptimer::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lptimer::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lptimer::Enabled
    }
}
#[doc = "Field `LPTIMER` writer - Low Power Timer Access Control"]
pub type LptimerW<'a, REG> = crate::BitWriter<'a, REG, Lptimer>;
impl<'a, REG> LptimerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lptimer::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lptimer::Enabled)
    }
}
#[doc = "TSI Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsi {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Tsi> for bool {
    #[inline(always)]
    fn from(variant: Tsi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSI` reader - TSI Clock Gate Control"]
pub type TsiR = crate::BitReader<Tsi>;
impl TsiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsi {
        match self.bits {
            false => Tsi::Disabled,
            true => Tsi::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tsi::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tsi::Enabled
    }
}
#[doc = "Field `TSI` writer - TSI Clock Gate Control"]
pub type TsiW<'a, REG> = crate::BitWriter<'a, REG, Tsi>;
impl<'a, REG> TsiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tsi::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tsi::Enabled)
    }
}
#[doc = "Port A Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porta {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Porta> for bool {
    #[inline(always)]
    fn from(variant: Porta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORTA` reader - Port A Clock Gate Control"]
pub type PortaR = crate::BitReader<Porta>;
impl PortaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Porta {
        match self.bits {
            false => Porta::Disabled,
            true => Porta::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Porta::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Porta::Enabled
    }
}
#[doc = "Field `PORTA` writer - Port A Clock Gate Control"]
pub type PortaW<'a, REG> = crate::BitWriter<'a, REG, Porta>;
impl<'a, REG> PortaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Porta::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Porta::Enabled)
    }
}
#[doc = "Port B Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Portb {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Portb> for bool {
    #[inline(always)]
    fn from(variant: Portb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORTB` reader - Port B Clock Gate Control"]
pub type PortbR = crate::BitReader<Portb>;
impl PortbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Portb {
        match self.bits {
            false => Portb::Disabled,
            true => Portb::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Portb::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Portb::Enabled
    }
}
#[doc = "Field `PORTB` writer - Port B Clock Gate Control"]
pub type PortbW<'a, REG> = crate::BitWriter<'a, REG, Portb>;
impl<'a, REG> PortbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Portb::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Portb::Enabled)
    }
}
#[doc = "Port C Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Portc {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Portc> for bool {
    #[inline(always)]
    fn from(variant: Portc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORTC` reader - Port C Clock Gate Control"]
pub type PortcR = crate::BitReader<Portc>;
impl PortcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Portc {
        match self.bits {
            false => Portc::Disabled,
            true => Portc::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Portc::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Portc::Enabled
    }
}
#[doc = "Field `PORTC` writer - Port C Clock Gate Control"]
pub type PortcW<'a, REG> = crate::BitWriter<'a, REG, Portc>;
impl<'a, REG> PortcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Portc::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Portc::Enabled)
    }
}
#[doc = "Port D Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Portd {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Portd> for bool {
    #[inline(always)]
    fn from(variant: Portd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORTD` reader - Port D Clock Gate Control"]
pub type PortdR = crate::BitReader<Portd>;
impl PortdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Portd {
        match self.bits {
            false => Portd::Disabled,
            true => Portd::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Portd::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Portd::Enabled
    }
}
#[doc = "Field `PORTD` writer - Port D Clock Gate Control"]
pub type PortdW<'a, REG> = crate::BitWriter<'a, REG, Portd>;
impl<'a, REG> PortdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Portd::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Portd::Enabled)
    }
}
#[doc = "Port E Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porte {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Porte> for bool {
    #[inline(always)]
    fn from(variant: Porte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORTE` reader - Port E Clock Gate Control"]
pub type PorteR = crate::BitReader<Porte>;
impl PorteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Porte {
        match self.bits {
            false => Porte::Disabled,
            true => Porte::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Porte::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Porte::Enabled
    }
}
#[doc = "Field `PORTE` writer - Port E Clock Gate Control"]
pub type PorteW<'a, REG> = crate::BitWriter<'a, REG, Porte>;
impl<'a, REG> PorteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Porte::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Porte::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Low Power Timer Access Control"]
    #[inline(always)]
    pub fn lptimer(&self) -> LptimerR {
        LptimerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - TSI Clock Gate Control"]
    #[inline(always)]
    pub fn tsi(&self) -> TsiR {
        TsiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Port A Clock Gate Control"]
    #[inline(always)]
    pub fn porta(&self) -> PortaR {
        PortaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port B Clock Gate Control"]
    #[inline(always)]
    pub fn portb(&self) -> PortbR {
        PortbR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port C Clock Gate Control"]
    #[inline(always)]
    pub fn portc(&self) -> PortcR {
        PortcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port D Clock Gate Control"]
    #[inline(always)]
    pub fn portd(&self) -> PortdR {
        PortdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port E Clock Gate Control"]
    #[inline(always)]
    pub fn porte(&self) -> PorteR {
        PorteR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power Timer Access Control"]
    #[inline(always)]
    pub fn lptimer(&mut self) -> LptimerW<'_, Scgc5Spec> {
        LptimerW::new(self, 0)
    }
    #[doc = "Bit 5 - TSI Clock Gate Control"]
    #[inline(always)]
    pub fn tsi(&mut self) -> TsiW<'_, Scgc5Spec> {
        TsiW::new(self, 5)
    }
    #[doc = "Bit 9 - Port A Clock Gate Control"]
    #[inline(always)]
    pub fn porta(&mut self) -> PortaW<'_, Scgc5Spec> {
        PortaW::new(self, 9)
    }
    #[doc = "Bit 10 - Port B Clock Gate Control"]
    #[inline(always)]
    pub fn portb(&mut self) -> PortbW<'_, Scgc5Spec> {
        PortbW::new(self, 10)
    }
    #[doc = "Bit 11 - Port C Clock Gate Control"]
    #[inline(always)]
    pub fn portc(&mut self) -> PortcW<'_, Scgc5Spec> {
        PortcW::new(self, 11)
    }
    #[doc = "Bit 12 - Port D Clock Gate Control"]
    #[inline(always)]
    pub fn portd(&mut self) -> PortdW<'_, Scgc5Spec> {
        PortdW::new(self, 12)
    }
    #[doc = "Bit 13 - Port E Clock Gate Control"]
    #[inline(always)]
    pub fn porte(&mut self) -> PorteW<'_, Scgc5Spec> {
        PorteW::new(self, 13)
    }
}
#[doc = "System Clock Gating Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scgc5Spec;
impl crate::RegisterSpec for Scgc5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgc5::R`](R) reader structure"]
impl crate::Readable for Scgc5Spec {}
#[doc = "`write(|w| ..)` method takes [`scgc5::W`](W) writer structure"]
impl crate::Writable for Scgc5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCGC5 to value 0x0004_0182"]
impl crate::Resettable for Scgc5Spec {
    const RESET_VALUE: u32 = 0x0004_0182;
}

#[doc = "Register `C3` reader"]
pub type R = crate::R<C3Spec>;
#[doc = "Register `C3` writer"]
pub type W = crate::W<C3Spec>;
#[doc = "Parity Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peie {
    #[doc = "0: PF interrupt disabled"]
    Disabled = 0,
    #[doc = "1: PF interrupt enabled"]
    Enabled = 1,
}
impl From<Peie> for bool {
    #[inline(always)]
    fn from(variant: Peie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEIE` reader - Parity Error Interrupt Enable"]
pub type PeieR = crate::BitReader<Peie>;
impl PeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peie {
        match self.bits {
            false => Peie::Disabled,
            true => Peie::Enabled,
        }
    }
    #[doc = "PF interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Peie::Disabled
    }
    #[doc = "PF interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Peie::Enabled
    }
}
#[doc = "Field `PEIE` writer - Parity Error Interrupt Enable"]
pub type PeieW<'a, REG> = crate::BitWriter<'a, REG, Peie>;
impl<'a, REG> PeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PF interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Peie::Disabled)
    }
    #[doc = "PF interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Peie::Enabled)
    }
}
#[doc = "Framing Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Feie {
    #[doc = "0: FE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: FE interrupt enabled"]
    Enabled = 1,
}
impl From<Feie> for bool {
    #[inline(always)]
    fn from(variant: Feie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIE` reader - Framing Error Interrupt Enable"]
pub type FeieR = crate::BitReader<Feie>;
impl FeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Feie {
        match self.bits {
            false => Feie::Disabled,
            true => Feie::Enabled,
        }
    }
    #[doc = "FE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Feie::Disabled
    }
    #[doc = "FE interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Feie::Enabled
    }
}
#[doc = "Field `FEIE` writer - Framing Error Interrupt Enable"]
pub type FeieW<'a, REG> = crate::BitWriter<'a, REG, Feie>;
impl<'a, REG> FeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Feie::Disabled)
    }
    #[doc = "FE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Feie::Enabled)
    }
}
#[doc = "Noise Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Neie {
    #[doc = "0: NF interrupt disabled"]
    Disabled = 0,
    #[doc = "1: NF interrupt enabled"]
    Enabled = 1,
}
impl From<Neie> for bool {
    #[inline(always)]
    fn from(variant: Neie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEIE` reader - Noise Error Interrupt Enable"]
pub type NeieR = crate::BitReader<Neie>;
impl NeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Neie {
        match self.bits {
            false => Neie::Disabled,
            true => Neie::Enabled,
        }
    }
    #[doc = "NF interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Neie::Disabled
    }
    #[doc = "NF interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Neie::Enabled
    }
}
#[doc = "Field `NEIE` writer - Noise Error Interrupt Enable"]
pub type NeieW<'a, REG> = crate::BitWriter<'a, REG, Neie>;
impl<'a, REG> NeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NF interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Neie::Disabled)
    }
    #[doc = "NF interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Neie::Enabled)
    }
}
#[doc = "Overrun Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orie {
    #[doc = "0: OR interrupt disabled"]
    Disabled = 0,
    #[doc = "1: OR interrupt enabled"]
    Enabled = 1,
}
impl From<Orie> for bool {
    #[inline(always)]
    fn from(variant: Orie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORIE` reader - Overrun Error Interrupt Enable"]
pub type OrieR = crate::BitReader<Orie>;
impl OrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Orie {
        match self.bits {
            false => Orie::Disabled,
            true => Orie::Enabled,
        }
    }
    #[doc = "OR interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Orie::Disabled
    }
    #[doc = "OR interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Orie::Enabled
    }
}
#[doc = "Field `ORIE` writer - Overrun Error Interrupt Enable"]
pub type OrieW<'a, REG> = crate::BitWriter<'a, REG, Orie>;
impl<'a, REG> OrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OR interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Orie::Disabled)
    }
    #[doc = "OR interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Orie::Enabled)
    }
}
#[doc = "Transmit Data Inversion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txinv {
    #[doc = "0: Transmit data not inverted"]
    Normal = 0,
    #[doc = "1: Transmit data inverted"]
    Inverted = 1,
}
impl From<Txinv> for bool {
    #[inline(always)]
    fn from(variant: Txinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINV` reader - Transmit Data Inversion."]
pub type TxinvR = crate::BitReader<Txinv>;
impl TxinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txinv {
        match self.bits {
            false => Txinv::Normal,
            true => Txinv::Inverted,
        }
    }
    #[doc = "Transmit data not inverted"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Txinv::Normal
    }
    #[doc = "Transmit data inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Txinv::Inverted
    }
}
#[doc = "Field `TXINV` writer - Transmit Data Inversion."]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG, Txinv>;
impl<'a, REG> TxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit data not inverted"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::Normal)
    }
    #[doc = "Transmit data inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::Inverted)
    }
}
#[doc = "Transmitter Pin Data Direction in Single-Wire mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdir {
    #[doc = "0: TXD pin is input in single-wire mode"]
    Input = 0,
    #[doc = "1: TXD pin is output in single-wire mode"]
    Output = 1,
}
impl From<Txdir> for bool {
    #[inline(always)]
    fn from(variant: Txdir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDIR` reader - Transmitter Pin Data Direction in Single-Wire mode"]
pub type TxdirR = crate::BitReader<Txdir>;
impl TxdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdir {
        match self.bits {
            false => Txdir::Input,
            true => Txdir::Output,
        }
    }
    #[doc = "TXD pin is input in single-wire mode"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == Txdir::Input
    }
    #[doc = "TXD pin is output in single-wire mode"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Txdir::Output
    }
}
#[doc = "Field `TXDIR` writer - Transmitter Pin Data Direction in Single-Wire mode"]
pub type TxdirW<'a, REG> = crate::BitWriter<'a, REG, Txdir>;
impl<'a, REG> TxdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXD pin is input in single-wire mode"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(Txdir::Input)
    }
    #[doc = "TXD pin is output in single-wire mode"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(Txdir::Output)
    }
}
#[doc = "Field `T8` reader - Transmit Bit 8"]
pub type T8R = crate::BitReader;
#[doc = "Field `T8` writer - Transmit Bit 8"]
pub type T8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R8` reader - Received Bit 8"]
pub type R8R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn peie(&self) -> PeieR {
        PeieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FeieR {
        FeieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise Error Interrupt Enable"]
    #[inline(always)]
    pub fn neie(&self) -> NeieR {
        NeieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn orie(&self) -> OrieR {
        OrieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Data Inversion."]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmitter Pin Data Direction in Single-Wire mode"]
    #[inline(always)]
    pub fn txdir(&self) -> TxdirR {
        TxdirR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Bit 8"]
    #[inline(always)]
    pub fn t8(&self) -> T8R {
        T8R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Received Bit 8"]
    #[inline(always)]
    pub fn r8(&self) -> R8R {
        R8R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn peie(&mut self) -> PeieW<'_, C3Spec> {
        PeieW::new(self, 0)
    }
    #[doc = "Bit 1 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FeieW<'_, C3Spec> {
        FeieW::new(self, 1)
    }
    #[doc = "Bit 2 - Noise Error Interrupt Enable"]
    #[inline(always)]
    pub fn neie(&mut self) -> NeieW<'_, C3Spec> {
        NeieW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn orie(&mut self) -> OrieW<'_, C3Spec> {
        OrieW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit Data Inversion."]
    #[inline(always)]
    pub fn txinv(&mut self) -> TxinvW<'_, C3Spec> {
        TxinvW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmitter Pin Data Direction in Single-Wire mode"]
    #[inline(always)]
    pub fn txdir(&mut self) -> TxdirW<'_, C3Spec> {
        TxdirW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit Bit 8"]
    #[inline(always)]
    pub fn t8(&mut self) -> T8W<'_, C3Spec> {
        T8W::new(self, 6)
    }
}
#[doc = "UART Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`c3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3Spec;
impl crate::RegisterSpec for C3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c3::R`](R) reader structure"]
impl crate::Readable for C3Spec {}
#[doc = "`write(|w| ..)` method takes [`c3::W`](W) writer structure"]
impl crate::Writable for C3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C3 to value 0"]
impl crate::Resettable for C3Spec {}

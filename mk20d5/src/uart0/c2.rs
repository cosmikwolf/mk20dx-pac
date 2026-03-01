#[doc = "Register `C2` reader"]
pub type R = crate::R<C2Spec>;
#[doc = "Register `C2` writer"]
pub type W = crate::W<C2Spec>;
#[doc = "Send Break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbk {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: Queue break character"]
    Break = 1,
}
impl From<Sbk> for bool {
    #[inline(always)]
    fn from(variant: Sbk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBK` reader - Send Break"]
pub type SbkR = crate::BitReader<Sbk>;
impl SbkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbk {
        match self.bits {
            false => Sbk::Normal,
            true => Sbk::Break,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Sbk::Normal
    }
    #[doc = "Queue break character"]
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == Sbk::Break
    }
}
#[doc = "Field `SBK` writer - Send Break"]
pub type SbkW<'a, REG> = crate::BitWriter<'a, REG, Sbk>;
impl<'a, REG> SbkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Sbk::Normal)
    }
    #[doc = "Queue break character"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut crate::W<REG> {
        self.variant(Sbk::Break)
    }
}
#[doc = "Receiver Wakeup Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwu {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: Receiver standby"]
    Standby = 1,
}
impl From<Rwu> for bool {
    #[inline(always)]
    fn from(variant: Rwu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWU` reader - Receiver Wakeup Control"]
pub type RwuR = crate::BitReader<Rwu>;
impl RwuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwu {
        match self.bits {
            false => Rwu::Normal,
            true => Rwu::Standby,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Rwu::Normal
    }
    #[doc = "Receiver standby"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == Rwu::Standby
    }
}
#[doc = "Field `RWU` writer - Receiver Wakeup Control"]
pub type RwuW<'a, REG> = crate::BitWriter<'a, REG, Rwu>;
impl<'a, REG> RwuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Rwu::Normal)
    }
    #[doc = "Receiver standby"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut crate::W<REG> {
        self.variant(Rwu::Standby)
    }
}
#[doc = "Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Re {
    #[doc = "0: Receiver disabled"]
    Disabled = 0,
    #[doc = "1: Receiver enabled"]
    Enabled = 1,
}
impl From<Re> for bool {
    #[inline(always)]
    fn from(variant: Re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE` reader - Receiver Enable"]
pub type ReR = crate::BitReader<Re>;
impl ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Re {
        match self.bits {
            false => Re::Disabled,
            true => Re::Enabled,
        }
    }
    #[doc = "Receiver disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Re::Disabled
    }
    #[doc = "Receiver enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Re::Enabled
    }
}
#[doc = "Field `RE` writer - Receiver Enable"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG, Re>;
impl<'a, REG> ReW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Re::Disabled)
    }
    #[doc = "Receiver enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Re::Enabled)
    }
}
#[doc = "Transmitter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te {
    #[doc = "0: Transmitter disabled"]
    Disabled = 0,
    #[doc = "1: Transmitter enabled"]
    Enabled = 1,
}
impl From<Te> for bool {
    #[inline(always)]
    fn from(variant: Te) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE` reader - Transmitter Enable"]
pub type TeR = crate::BitReader<Te>;
impl TeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te {
        match self.bits {
            false => Te::Disabled,
            true => Te::Enabled,
        }
    }
    #[doc = "Transmitter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Te::Disabled
    }
    #[doc = "Transmitter enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Te::Enabled
    }
}
#[doc = "Field `TE` writer - Transmitter Enable"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG, Te>;
impl<'a, REG> TeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Te::Disabled)
    }
    #[doc = "Transmitter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Te::Enabled)
    }
}
#[doc = "Idle Line Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilie {
    #[doc = "0: IDLE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: IDLE interrupt enabled"]
    Enabled = 1,
}
impl From<Ilie> for bool {
    #[inline(always)]
    fn from(variant: Ilie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIE` reader - Idle Line Interrupt Enable"]
pub type IlieR = crate::BitReader<Ilie>;
impl IlieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ilie {
        match self.bits {
            false => Ilie::Disabled,
            true => Ilie::Enabled,
        }
    }
    #[doc = "IDLE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ilie::Disabled
    }
    #[doc = "IDLE interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ilie::Enabled
    }
}
#[doc = "Field `ILIE` writer - Idle Line Interrupt Enable"]
pub type IlieW<'a, REG> = crate::BitWriter<'a, REG, Ilie>;
impl<'a, REG> IlieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IDLE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ilie::Disabled)
    }
    #[doc = "IDLE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ilie::Enabled)
    }
}
#[doc = "Receiver Full Interrupt or DMA Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rie {
    #[doc = "0: RDRF interrupt disabled"]
    Disabled = 0,
    #[doc = "1: RDRF interrupt enabled"]
    Enabled = 1,
}
impl From<Rie> for bool {
    #[inline(always)]
    fn from(variant: Rie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIE` reader - Receiver Full Interrupt or DMA Transfer Enable"]
pub type RieR = crate::BitReader<Rie>;
impl RieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rie {
        match self.bits {
            false => Rie::Disabled,
            true => Rie::Enabled,
        }
    }
    #[doc = "RDRF interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rie::Disabled
    }
    #[doc = "RDRF interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rie::Enabled
    }
}
#[doc = "Field `RIE` writer - Receiver Full Interrupt or DMA Transfer Enable"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG, Rie>;
impl<'a, REG> RieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RDRF interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::Disabled)
    }
    #[doc = "RDRF interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::Enabled)
    }
}
#[doc = "Transmission Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcie {
    #[doc = "0: TC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: TC interrupt enabled"]
    Enabled = 1,
}
impl From<Tcie> for bool {
    #[inline(always)]
    fn from(variant: Tcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transmission Complete Interrupt Enable"]
pub type TcieR = crate::BitReader<Tcie>;
impl TcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcie {
        match self.bits {
            false => Tcie::Disabled,
            true => Tcie::Enabled,
        }
    }
    #[doc = "TC interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tcie::Disabled
    }
    #[doc = "TC interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tcie::Enabled
    }
}
#[doc = "Field `TCIE` writer - Transmission Complete Interrupt Enable"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG, Tcie>;
impl<'a, REG> TcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::Disabled)
    }
    #[doc = "TC interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::Enabled)
    }
}
#[doc = "Transmitter Interrupt or DMA Transfer Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tie {
    #[doc = "0: TDRE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: TDRE interrupt enabled"]
    Enabled = 1,
}
impl From<Tie> for bool {
    #[inline(always)]
    fn from(variant: Tie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Transmitter Interrupt or DMA Transfer Enable."]
pub type TieR = crate::BitReader<Tie>;
impl TieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tie {
        match self.bits {
            false => Tie::Disabled,
            true => Tie::Enabled,
        }
    }
    #[doc = "TDRE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tie::Disabled
    }
    #[doc = "TDRE interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tie::Enabled
    }
}
#[doc = "Field `TIE` writer - Transmitter Interrupt or DMA Transfer Enable."]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG, Tie>;
impl<'a, REG> TieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TDRE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::Disabled)
    }
    #[doc = "TDRE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Send Break"]
    #[inline(always)]
    pub fn sbk(&self) -> SbkR {
        SbkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver Wakeup Control"]
    #[inline(always)]
    pub fn rwu(&self) -> RwuR {
        RwuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle Line Interrupt Enable"]
    #[inline(always)]
    pub fn ilie(&self) -> IlieR {
        IlieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receiver Full Interrupt or DMA Transfer Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmitter Interrupt or DMA Transfer Enable."]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send Break"]
    #[inline(always)]
    pub fn sbk(&mut self) -> SbkW<'_, C2Spec> {
        SbkW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver Wakeup Control"]
    #[inline(always)]
    pub fn rwu(&mut self) -> RwuW<'_, C2Spec> {
        RwuW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<'_, C2Spec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TeW<'_, C2Spec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - Idle Line Interrupt Enable"]
    #[inline(always)]
    pub fn ilie(&mut self) -> IlieW<'_, C2Spec> {
        IlieW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Full Interrupt or DMA Transfer Enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<'_, C2Spec> {
        RieW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<'_, C2Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmitter Interrupt or DMA Transfer Enable."]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<'_, C2Spec> {
        TieW::new(self, 7)
    }
}
#[doc = "UART Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets C2 to value 0"]
impl crate::Resettable for C2Spec {}

#[doc = "Register `S2` reader"]
pub type R = crate::R<S2Spec>;
#[doc = "Register `S2` writer"]
pub type W = crate::W<S2Spec>;
#[doc = "Receiver Active Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Raf {
    #[doc = "0: UART receiver idle"]
    Idle = 0,
    #[doc = "1: UART receiver active"]
    Active = 1,
}
impl From<Raf> for bool {
    #[inline(always)]
    fn from(variant: Raf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAF` reader - Receiver Active Flag"]
pub type RafR = crate::BitReader<Raf>;
impl RafR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Raf {
        match self.bits {
            false => Raf::Idle,
            true => Raf::Active,
        }
    }
    #[doc = "UART receiver idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Raf::Idle
    }
    #[doc = "UART receiver active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Raf::Active
    }
}
#[doc = "LIN Break Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbkde {
    #[doc = "0: Break detection disabled"]
    Disabled = 0,
    #[doc = "1: Break detection enabled"]
    Enabled = 1,
}
impl From<Lbkde> for bool {
    #[inline(always)]
    fn from(variant: Lbkde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBKDE` reader - LIN Break Detection Enable"]
pub type LbkdeR = crate::BitReader<Lbkde>;
impl LbkdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbkde {
        match self.bits {
            false => Lbkde::Disabled,
            true => Lbkde::Enabled,
        }
    }
    #[doc = "Break detection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lbkde::Disabled
    }
    #[doc = "Break detection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lbkde::Enabled
    }
}
#[doc = "Field `LBKDE` writer - LIN Break Detection Enable"]
pub type LbkdeW<'a, REG> = crate::BitWriter<'a, REG, Lbkde>;
impl<'a, REG> LbkdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lbkde::Disabled)
    }
    #[doc = "Break detection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lbkde::Enabled)
    }
}
#[doc = "Break Transmit Character Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brk13 {
    #[doc = "0: Break is 10-11-12 bit times"]
    Short = 0,
    #[doc = "1: Break is 13-14 bit times"]
    Long = 1,
}
impl From<Brk13> for bool {
    #[inline(always)]
    fn from(variant: Brk13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRK13` reader - Break Transmit Character Length"]
pub type Brk13R = crate::BitReader<Brk13>;
impl Brk13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brk13 {
        match self.bits {
            false => Brk13::Short,
            true => Brk13::Long,
        }
    }
    #[doc = "Break is 10-11-12 bit times"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == Brk13::Short
    }
    #[doc = "Break is 13-14 bit times"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == Brk13::Long
    }
}
#[doc = "Field `BRK13` writer - Break Transmit Character Length"]
pub type Brk13W<'a, REG> = crate::BitWriter<'a, REG, Brk13>;
impl<'a, REG> Brk13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break is 10-11-12 bit times"]
    #[inline(always)]
    pub fn short(self) -> &'a mut crate::W<REG> {
        self.variant(Brk13::Short)
    }
    #[doc = "Break is 13-14 bit times"]
    #[inline(always)]
    pub fn long(self) -> &'a mut crate::W<REG> {
        self.variant(Brk13::Long)
    }
}
#[doc = "Receive Wakeup Idle Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwuid {
    #[doc = "0: S1.IDLE not set on idle during standby"]
    Disabled = 0,
    #[doc = "1: S1.IDLE set on idle during standby"]
    Enabled = 1,
}
impl From<Rwuid> for bool {
    #[inline(always)]
    fn from(variant: Rwuid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWUID` reader - Receive Wakeup Idle Detect"]
pub type RwuidR = crate::BitReader<Rwuid>;
impl RwuidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwuid {
        match self.bits {
            false => Rwuid::Disabled,
            true => Rwuid::Enabled,
        }
    }
    #[doc = "S1.IDLE not set on idle during standby"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rwuid::Disabled
    }
    #[doc = "S1.IDLE set on idle during standby"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rwuid::Enabled
    }
}
#[doc = "Field `RWUID` writer - Receive Wakeup Idle Detect"]
pub type RwuidW<'a, REG> = crate::BitWriter<'a, REG, Rwuid>;
impl<'a, REG> RwuidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S1.IDLE not set on idle during standby"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rwuid::Disabled)
    }
    #[doc = "S1.IDLE set on idle during standby"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rwuid::Enabled)
    }
}
#[doc = "Receive Data Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxinv {
    #[doc = "0: Receive data not inverted"]
    Normal = 0,
    #[doc = "1: Receive data inverted"]
    Inverted = 1,
}
impl From<Rxinv> for bool {
    #[inline(always)]
    fn from(variant: Rxinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINV` reader - Receive Data Inversion"]
pub type RxinvR = crate::BitReader<Rxinv>;
impl RxinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxinv {
        match self.bits {
            false => Rxinv::Normal,
            true => Rxinv::Inverted,
        }
    }
    #[doc = "Receive data not inverted"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Rxinv::Normal
    }
    #[doc = "Receive data inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Rxinv::Inverted
    }
}
#[doc = "Field `RXINV` writer - Receive Data Inversion"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG, Rxinv>;
impl<'a, REG> RxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive data not inverted"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinv::Normal)
    }
    #[doc = "Receive data inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinv::Inverted)
    }
}
#[doc = "Most Significant Bit First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msbf {
    #[doc = "0: LSB first"]
    LsbFirst = 0,
    #[doc = "1: MSB first"]
    MsbFirst = 1,
}
impl From<Msbf> for bool {
    #[inline(always)]
    fn from(variant: Msbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub type MsbfR = crate::BitReader<Msbf>;
impl MsbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msbf {
        match self.bits {
            false => Msbf::LsbFirst,
            true => Msbf::MsbFirst,
        }
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == Msbf::LsbFirst
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        *self == Msbf::MsbFirst
    }
}
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub type MsbfW<'a, REG> = crate::BitWriter<'a, REG, Msbf>;
impl<'a, REG> MsbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut crate::W<REG> {
        self.variant(Msbf::LsbFirst)
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut crate::W<REG> {
        self.variant(Msbf::MsbFirst)
    }
}
#[doc = "RxD Pin Active Edge Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxedgif {
    #[doc = "0: No active edge on RXD"]
    NoEdge = 0,
    #[doc = "1: Active edge on RXD"]
    Edge = 1,
}
impl From<Rxedgif> for bool {
    #[inline(always)]
    fn from(variant: Rxedgif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEDGIF` reader - RxD Pin Active Edge Interrupt Flag"]
pub type RxedgifR = crate::BitReader<Rxedgif>;
impl RxedgifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxedgif {
        match self.bits {
            false => Rxedgif::NoEdge,
            true => Rxedgif::Edge,
        }
    }
    #[doc = "No active edge on RXD"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == Rxedgif::NoEdge
    }
    #[doc = "Active edge on RXD"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == Rxedgif::Edge
    }
}
#[doc = "Field `RXEDGIF` writer - RxD Pin Active Edge Interrupt Flag"]
pub type RxedgifW<'a, REG> = crate::BitWriter<'a, REG, Rxedgif>;
impl<'a, REG> RxedgifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No active edge on RXD"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Rxedgif::NoEdge)
    }
    #[doc = "Active edge on RXD"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(Rxedgif::Edge)
    }
}
#[doc = "LIN Break Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbkdif {
    #[doc = "0: No LIN break character has been detected."]
    _0 = 0,
    #[doc = "1: LIN break character has been detected."]
    _1 = 1,
}
impl From<Lbkdif> for bool {
    #[inline(always)]
    fn from(variant: Lbkdif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBKDIF` reader - LIN Break Detect Interrupt Flag"]
pub type LbkdifR = crate::BitReader<Lbkdif>;
impl LbkdifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbkdif {
        match self.bits {
            false => Lbkdif::_0,
            true => Lbkdif::_1,
        }
    }
    #[doc = "No LIN break character has been detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lbkdif::_0
    }
    #[doc = "LIN break character has been detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lbkdif::_1
    }
}
#[doc = "Field `LBKDIF` writer - LIN Break Detect Interrupt Flag"]
pub type LbkdifW<'a, REG> = crate::BitWriter<'a, REG, Lbkdif>;
impl<'a, REG> LbkdifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No LIN break character has been detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lbkdif::_0)
    }
    #[doc = "LIN break character has been detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lbkdif::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Active Flag"]
    #[inline(always)]
    pub fn raf(&self) -> RafR {
        RafR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LIN Break Detection Enable"]
    #[inline(always)]
    pub fn lbkde(&self) -> LbkdeR {
        LbkdeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Break Transmit Character Length"]
    #[inline(always)]
    pub fn brk13(&self) -> Brk13R {
        Brk13R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive Wakeup Idle Detect"]
    #[inline(always)]
    pub fn rwuid(&self) -> RwuidR {
        RwuidR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Data Inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MsbfR {
        MsbfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RxD Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    pub fn rxedgif(&self) -> RxedgifR {
        RxedgifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    pub fn lbkdif(&self) -> LbkdifR {
        LbkdifR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - LIN Break Detection Enable"]
    #[inline(always)]
    pub fn lbkde(&mut self) -> LbkdeW<'_, S2Spec> {
        LbkdeW::new(self, 1)
    }
    #[doc = "Bit 2 - Break Transmit Character Length"]
    #[inline(always)]
    pub fn brk13(&mut self) -> Brk13W<'_, S2Spec> {
        Brk13W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive Wakeup Idle Detect"]
    #[inline(always)]
    pub fn rwuid(&mut self) -> RwuidW<'_, S2Spec> {
        RwuidW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive Data Inversion"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RxinvW<'_, S2Spec> {
        RxinvW::new(self, 4)
    }
    #[doc = "Bit 5 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MsbfW<'_, S2Spec> {
        MsbfW::new(self, 5)
    }
    #[doc = "Bit 6 - RxD Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    pub fn rxedgif(&mut self) -> RxedgifW<'_, S2Spec> {
        RxedgifW::new(self, 6)
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    pub fn lbkdif(&mut self) -> LbkdifW<'_, S2Spec> {
        LbkdifW::new(self, 7)
    }
}
#[doc = "UART Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`s2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2Spec;
impl crate::RegisterSpec for S2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`s2::R`](R) reader structure"]
impl crate::Readable for S2Spec {}
#[doc = "`write(|w| ..)` method takes [`s2::W`](W) writer structure"]
impl crate::Writable for S2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S2 to value 0"]
impl crate::Resettable for S2Spec {}

#[doc = "Register `C1` reader"]
pub type R = crate::R<C1Spec>;
#[doc = "Register `C1` writer"]
pub type W = crate::W<C1Spec>;
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pt {
    #[doc = "0: Even parity"]
    Even = 0,
    #[doc = "1: Odd parity"]
    Odd = 1,
}
impl From<Pt> for bool {
    #[inline(always)]
    fn from(variant: Pt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PT` reader - Parity Type"]
pub type PtR = crate::BitReader<Pt>;
impl PtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pt {
        match self.bits {
            false => Pt::Even,
            true => Pt::Odd,
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Pt::Even
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Pt::Odd
    }
}
#[doc = "Field `PT` writer - Parity Type"]
pub type PtW<'a, REG> = crate::BitWriter<'a, REG, Pt>;
impl<'a, REG> PtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Pt::Even)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Pt::Odd)
    }
}
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: Parity disabled"]
    Disabled = 0,
    #[doc = "1: Parity enabled"]
    Enabled = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Parity Enable"]
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
    #[doc = "Parity disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pe::Disabled
    }
    #[doc = "Parity enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pe::Enabled
    }
}
#[doc = "Field `PE` writer - Parity Enable"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG, Pe>;
impl<'a, REG> PeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::Disabled)
    }
    #[doc = "Parity enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::Enabled)
    }
}
#[doc = "Idle Line Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilt {
    #[doc = "0: Idle after start bit"]
    AfterStart = 0,
    #[doc = "1: Idle after stop bit"]
    AfterStop = 1,
}
impl From<Ilt> for bool {
    #[inline(always)]
    fn from(variant: Ilt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILT` reader - Idle Line Type Select"]
pub type IltR = crate::BitReader<Ilt>;
impl IltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ilt {
        match self.bits {
            false => Ilt::AfterStart,
            true => Ilt::AfterStop,
        }
    }
    #[doc = "Idle after start bit"]
    #[inline(always)]
    pub fn is_after_start(&self) -> bool {
        *self == Ilt::AfterStart
    }
    #[doc = "Idle after stop bit"]
    #[inline(always)]
    pub fn is_after_stop(&self) -> bool {
        *self == Ilt::AfterStop
    }
}
#[doc = "Field `ILT` writer - Idle Line Type Select"]
pub type IltW<'a, REG> = crate::BitWriter<'a, REG, Ilt>;
impl<'a, REG> IltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Idle after start bit"]
    #[inline(always)]
    pub fn after_start(self) -> &'a mut crate::W<REG> {
        self.variant(Ilt::AfterStart)
    }
    #[doc = "Idle after stop bit"]
    #[inline(always)]
    pub fn after_stop(self) -> &'a mut crate::W<REG> {
        self.variant(Ilt::AfterStop)
    }
}
#[doc = "Receiver Wakeup Method Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wake {
    #[doc = "0: Idle-line wakeup"]
    IdleLine = 0,
    #[doc = "1: Address-mark wakeup"]
    AddressMark = 1,
}
impl From<Wake> for bool {
    #[inline(always)]
    fn from(variant: Wake) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKE` reader - Receiver Wakeup Method Select"]
pub type WakeR = crate::BitReader<Wake>;
impl WakeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wake {
        match self.bits {
            false => Wake::IdleLine,
            true => Wake::AddressMark,
        }
    }
    #[doc = "Idle-line wakeup"]
    #[inline(always)]
    pub fn is_idle_line(&self) -> bool {
        *self == Wake::IdleLine
    }
    #[doc = "Address-mark wakeup"]
    #[inline(always)]
    pub fn is_address_mark(&self) -> bool {
        *self == Wake::AddressMark
    }
}
#[doc = "Field `WAKE` writer - Receiver Wakeup Method Select"]
pub type WakeW<'a, REG> = crate::BitWriter<'a, REG, Wake>;
impl<'a, REG> WakeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Idle-line wakeup"]
    #[inline(always)]
    pub fn idle_line(self) -> &'a mut crate::W<REG> {
        self.variant(Wake::IdleLine)
    }
    #[doc = "Address-mark wakeup"]
    #[inline(always)]
    pub fn address_mark(self) -> &'a mut crate::W<REG> {
        self.variant(Wake::AddressMark)
    }
}
#[doc = "9-bit or 8-bit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M {
    #[doc = "0: 8-bit data"]
    Data8 = 0,
    #[doc = "1: 9-bit data"]
    Data9 = 1,
}
impl From<M> for bool {
    #[inline(always)]
    fn from(variant: M) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M` reader - 9-bit or 8-bit Mode Select"]
pub type MR = crate::BitReader<M>;
impl MR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M {
        match self.bits {
            false => M::Data8,
            true => M::Data9,
        }
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn is_data8(&self) -> bool {
        *self == M::Data8
    }
    #[doc = "9-bit data"]
    #[inline(always)]
    pub fn is_data9(&self) -> bool {
        *self == M::Data9
    }
}
#[doc = "Field `M` writer - 9-bit or 8-bit Mode Select"]
pub type MW<'a, REG> = crate::BitWriter<'a, REG, M>;
impl<'a, REG> MW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn data8(self) -> &'a mut crate::W<REG> {
        self.variant(M::Data8)
    }
    #[doc = "9-bit data"]
    #[inline(always)]
    pub fn data9(self) -> &'a mut crate::W<REG> {
        self.variant(M::Data9)
    }
}
#[doc = "Receiver Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsrc {
    #[doc = "0: Internal loopback"]
    Internal = 0,
    #[doc = "1: Single-wire mode from RXD pin"]
    External = 1,
}
impl From<Rsrc> for bool {
    #[inline(always)]
    fn from(variant: Rsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSRC` reader - Receiver Source Select"]
pub type RsrcR = crate::BitReader<Rsrc>;
impl RsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rsrc {
        match self.bits {
            false => Rsrc::Internal,
            true => Rsrc::External,
        }
    }
    #[doc = "Internal loopback"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Rsrc::Internal
    }
    #[doc = "Single-wire mode from RXD pin"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Rsrc::External
    }
}
#[doc = "Field `RSRC` writer - Receiver Source Select"]
pub type RsrcW<'a, REG> = crate::BitWriter<'a, REG, Rsrc>;
impl<'a, REG> RsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal loopback"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(Rsrc::Internal)
    }
    #[doc = "Single-wire mode from RXD pin"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(Rsrc::External)
    }
}
#[doc = "UART Stops in Wait Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uartswai {
    #[doc = "0: UART clocks continue in wait mode"]
    Running = 0,
    #[doc = "1: UART clocks freeze in wait mode"]
    Stopped = 1,
}
impl From<Uartswai> for bool {
    #[inline(always)]
    fn from(variant: Uartswai) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UARTSWAI` reader - UART Stops in Wait Mode"]
pub type UartswaiR = crate::BitReader<Uartswai>;
impl UartswaiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uartswai {
        match self.bits {
            false => Uartswai::Running,
            true => Uartswai::Stopped,
        }
    }
    #[doc = "UART clocks continue in wait mode"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == Uartswai::Running
    }
    #[doc = "UART clocks freeze in wait mode"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == Uartswai::Stopped
    }
}
#[doc = "Field `UARTSWAI` writer - UART Stops in Wait Mode"]
pub type UartswaiW<'a, REG> = crate::BitWriter<'a, REG, Uartswai>;
impl<'a, REG> UartswaiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UART clocks continue in wait mode"]
    #[inline(always)]
    pub fn running(self) -> &'a mut crate::W<REG> {
        self.variant(Uartswai::Running)
    }
    #[doc = "UART clocks freeze in wait mode"]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(Uartswai::Stopped)
    }
}
#[doc = "Loop Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loops {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: Loop mode"]
    Loopback = 1,
}
impl From<Loops> for bool {
    #[inline(always)]
    fn from(variant: Loops) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPS` reader - Loop Mode Select"]
pub type LoopsR = crate::BitReader<Loops>;
impl LoopsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loops {
        match self.bits {
            false => Loops::Normal,
            true => Loops::Loopback,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Loops::Normal
    }
    #[doc = "Loop mode"]
    #[inline(always)]
    pub fn is_loopback(&self) -> bool {
        *self == Loops::Loopback
    }
}
#[doc = "Field `LOOPS` writer - Loop Mode Select"]
pub type LoopsW<'a, REG> = crate::BitWriter<'a, REG, Loops>;
impl<'a, REG> LoopsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Loops::Normal)
    }
    #[doc = "Loop mode"]
    #[inline(always)]
    pub fn loopback(self) -> &'a mut crate::W<REG> {
        self.variant(Loops::Loopback)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Type"]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline(always)]
    pub fn ilt(&self) -> IltR {
        IltR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline(always)]
    pub fn wake(&self) -> WakeR {
        WakeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 9-bit or 8-bit Mode Select"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    pub fn rsrc(&self) -> RsrcR {
        RsrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Stops in Wait Mode"]
    #[inline(always)]
    pub fn uartswai(&self) -> UartswaiR {
        UartswaiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    pub fn loops(&self) -> LoopsR {
        LoopsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Type"]
    #[inline(always)]
    pub fn pt(&mut self) -> PtW<'_, C1Spec> {
        PtW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, C1Spec> {
        PeW::new(self, 1)
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline(always)]
    pub fn ilt(&mut self) -> IltW<'_, C1Spec> {
        IltW::new(self, 2)
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline(always)]
    pub fn wake(&mut self) -> WakeW<'_, C1Spec> {
        WakeW::new(self, 3)
    }
    #[doc = "Bit 4 - 9-bit or 8-bit Mode Select"]
    #[inline(always)]
    pub fn m(&mut self) -> MW<'_, C1Spec> {
        MW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    pub fn rsrc(&mut self) -> RsrcW<'_, C1Spec> {
        RsrcW::new(self, 5)
    }
    #[doc = "Bit 6 - UART Stops in Wait Mode"]
    #[inline(always)]
    pub fn uartswai(&mut self) -> UartswaiW<'_, C1Spec> {
        UartswaiW::new(self, 6)
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    pub fn loops(&mut self) -> LoopsW<'_, C1Spec> {
        LoopsW::new(self, 7)
    }
}
#[doc = "UART Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1Spec;
impl crate::RegisterSpec for C1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c1::R`](R) reader structure"]
impl crate::Readable for C1Spec {}
#[doc = "`write(|w| ..)` method takes [`c1::W`](W) writer structure"]
impl crate::Writable for C1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C1 to value 0"]
impl crate::Resettable for C1Spec {}

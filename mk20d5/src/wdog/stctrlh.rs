#[doc = "Register `STCTRLH` reader"]
pub type R = crate::R<StctrlhSpec>;
#[doc = "Register `STCTRLH` writer"]
pub type W = crate::W<StctrlhSpec>;
#[doc = "Enables or disables the WDOG's operation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdogen {
    #[doc = "0: WDOG is disabled"]
    Disabled = 0,
    #[doc = "1: WDOG is enabled"]
    Enabled = 1,
}
impl From<Wdogen> for bool {
    #[inline(always)]
    fn from(variant: Wdogen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDOGEN` reader - Enables or disables the WDOG's operation"]
pub type WdogenR = crate::BitReader<Wdogen>;
impl WdogenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdogen {
        match self.bits {
            false => Wdogen::Disabled,
            true => Wdogen::Enabled,
        }
    }
    #[doc = "WDOG is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wdogen::Disabled
    }
    #[doc = "WDOG is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Wdogen::Enabled
    }
}
#[doc = "Field `WDOGEN` writer - Enables or disables the WDOG's operation"]
pub type WdogenW<'a, REG> = crate::BitWriter<'a, REG, Wdogen>;
impl<'a, REG> WdogenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDOG is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wdogen::Disabled)
    }
    #[doc = "WDOG is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wdogen::Enabled)
    }
}
#[doc = "Selects clock source for the WDOG timer and other internal timing operations.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clksrc {
    #[doc = "0: Dedicated LPO clock source selected as WDOG clock"]
    LpoOscillator = 0,
    #[doc = "1: WDOG clock sourced from alternate clock source"]
    AlternateClock = 1,
}
impl From<Clksrc> for bool {
    #[inline(always)]
    fn from(variant: Clksrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSRC` reader - Selects clock source for the WDOG timer and other internal timing operations."]
pub type ClksrcR = crate::BitReader<Clksrc>;
impl ClksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksrc {
        match self.bits {
            false => Clksrc::LpoOscillator,
            true => Clksrc::AlternateClock,
        }
    }
    #[doc = "Dedicated LPO clock source selected as WDOG clock"]
    #[inline(always)]
    pub fn is_lpo_oscillator(&self) -> bool {
        *self == Clksrc::LpoOscillator
    }
    #[doc = "WDOG clock sourced from alternate clock source"]
    #[inline(always)]
    pub fn is_alternate_clock(&self) -> bool {
        *self == Clksrc::AlternateClock
    }
}
#[doc = "Field `CLKSRC` writer - Selects clock source for the WDOG timer and other internal timing operations."]
pub type ClksrcW<'a, REG> = crate::BitWriter<'a, REG, Clksrc>;
impl<'a, REG> ClksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dedicated LPO clock source selected as WDOG clock"]
    #[inline(always)]
    pub fn lpo_oscillator(self) -> &'a mut crate::W<REG> {
        self.variant(Clksrc::LpoOscillator)
    }
    #[doc = "WDOG clock sourced from alternate clock source"]
    #[inline(always)]
    pub fn alternate_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Clksrc::AlternateClock)
    }
}
#[doc = "Used to enable the debug breadcrumbs feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irqrsten {
    #[doc = "0: WDOG time-out generates reset only"]
    ResetOnly = 0,
    #[doc = "1: WDOG time-out initially generates an interrupt, then a reset after WCT time"]
    InterruptThenReset = 1,
}
impl From<Irqrsten> for bool {
    #[inline(always)]
    fn from(variant: Irqrsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQRSTEN` reader - Used to enable the debug breadcrumbs feature"]
pub type IrqrstenR = crate::BitReader<Irqrsten>;
impl IrqrstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irqrsten {
        match self.bits {
            false => Irqrsten::ResetOnly,
            true => Irqrsten::InterruptThenReset,
        }
    }
    #[doc = "WDOG time-out generates reset only"]
    #[inline(always)]
    pub fn is_reset_only(&self) -> bool {
        *self == Irqrsten::ResetOnly
    }
    #[doc = "WDOG time-out initially generates an interrupt, then a reset after WCT time"]
    #[inline(always)]
    pub fn is_interrupt_then_reset(&self) -> bool {
        *self == Irqrsten::InterruptThenReset
    }
}
#[doc = "Field `IRQRSTEN` writer - Used to enable the debug breadcrumbs feature"]
pub type IrqrstenW<'a, REG> = crate::BitWriter<'a, REG, Irqrsten>;
impl<'a, REG> IrqrstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDOG time-out generates reset only"]
    #[inline(always)]
    pub fn reset_only(self) -> &'a mut crate::W<REG> {
        self.variant(Irqrsten::ResetOnly)
    }
    #[doc = "WDOG time-out initially generates an interrupt, then a reset after WCT time"]
    #[inline(always)]
    pub fn interrupt_then_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Irqrsten::InterruptThenReset)
    }
}
#[doc = "Enable windowing mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Winen {
    #[doc = "0: Windowing mode is disabled"]
    Disabled = 0,
    #[doc = "1: Windowing mode is enabled"]
    Enabled = 1,
}
impl From<Winen> for bool {
    #[inline(always)]
    fn from(variant: Winen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINEN` reader - Enable windowing mode."]
pub type WinenR = crate::BitReader<Winen>;
impl WinenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Winen {
        match self.bits {
            false => Winen::Disabled,
            true => Winen::Enabled,
        }
    }
    #[doc = "Windowing mode is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Winen::Disabled
    }
    #[doc = "Windowing mode is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Winen::Enabled
    }
}
#[doc = "Field `WINEN` writer - Enable windowing mode."]
pub type WinenW<'a, REG> = crate::BitWriter<'a, REG, Winen>;
impl<'a, REG> WinenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Windowing mode is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Winen::Disabled)
    }
    #[doc = "Windowing mode is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Winen::Enabled)
    }
}
#[doc = "Enables updates to watchdog write once registers, after initial configuration window (WCT) closes, through unlock sequence\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allowupdate {
    #[doc = "0: No further updates allowed to WDOG write-once registers"]
    Locked = 0,
    #[doc = "1: WDOG write-once registers can be unlocked for updating"]
    Unlocked = 1,
}
impl From<Allowupdate> for bool {
    #[inline(always)]
    fn from(variant: Allowupdate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALLOWUPDATE` reader - Enables updates to watchdog write once registers, after initial configuration window (WCT) closes, through unlock sequence"]
pub type AllowupdateR = crate::BitReader<Allowupdate>;
impl AllowupdateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Allowupdate {
        match self.bits {
            false => Allowupdate::Locked,
            true => Allowupdate::Unlocked,
        }
    }
    #[doc = "No further updates allowed to WDOG write-once registers"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Allowupdate::Locked
    }
    #[doc = "WDOG write-once registers can be unlocked for updating"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Allowupdate::Unlocked
    }
}
#[doc = "Field `ALLOWUPDATE` writer - Enables updates to watchdog write once registers, after initial configuration window (WCT) closes, through unlock sequence"]
pub type AllowupdateW<'a, REG> = crate::BitWriter<'a, REG, Allowupdate>;
impl<'a, REG> AllowupdateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No further updates allowed to WDOG write-once registers"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Allowupdate::Locked)
    }
    #[doc = "WDOG write-once registers can be unlocked for updating"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Allowupdate::Unlocked)
    }
}
#[doc = "Enables or disables WDOG in Debug mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgen {
    #[doc = "0: WDOG is disabled in CPU debug mode"]
    Disabled = 0,
    #[doc = "1: WDOG is enabled in CPU debug mode"]
    Enabled = 1,
}
impl From<Dbgen> for bool {
    #[inline(always)]
    fn from(variant: Dbgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN` reader - Enables or disables WDOG in Debug mode."]
pub type DbgenR = crate::BitReader<Dbgen>;
impl DbgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgen {
        match self.bits {
            false => Dbgen::Disabled,
            true => Dbgen::Enabled,
        }
    }
    #[doc = "WDOG is disabled in CPU debug mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dbgen::Disabled
    }
    #[doc = "WDOG is enabled in CPU debug mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dbgen::Enabled
    }
}
#[doc = "Field `DBGEN` writer - Enables or disables WDOG in Debug mode."]
pub type DbgenW<'a, REG> = crate::BitWriter<'a, REG, Dbgen>;
impl<'a, REG> DbgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDOG is disabled in CPU debug mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::Disabled)
    }
    #[doc = "WDOG is enabled in CPU debug mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::Enabled)
    }
}
#[doc = "Enables or disables WDOG in stop mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopen {
    #[doc = "0: WDOG is disabled in CPU Stop mode"]
    Disabled = 0,
    #[doc = "1: WDOG is enabled in CPU Stop mode"]
    Enabled = 1,
}
impl From<Stopen> for bool {
    #[inline(always)]
    fn from(variant: Stopen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPEN` reader - Enables or disables WDOG in stop mode."]
pub type StopenR = crate::BitReader<Stopen>;
impl StopenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopen {
        match self.bits {
            false => Stopen::Disabled,
            true => Stopen::Enabled,
        }
    }
    #[doc = "WDOG is disabled in CPU Stop mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stopen::Disabled
    }
    #[doc = "WDOG is enabled in CPU Stop mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stopen::Enabled
    }
}
#[doc = "Field `STOPEN` writer - Enables or disables WDOG in stop mode."]
pub type StopenW<'a, REG> = crate::BitWriter<'a, REG, Stopen>;
impl<'a, REG> StopenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDOG is disabled in CPU Stop mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stopen::Disabled)
    }
    #[doc = "WDOG is enabled in CPU Stop mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stopen::Enabled)
    }
}
#[doc = "Enables or disables WDOG in wait mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Waiten {
    #[doc = "0: WDOG is disabled in CPU Wait mode"]
    Disabled = 0,
    #[doc = "1: WDOG is enabled in CPU Wait mode"]
    Enabled = 1,
}
impl From<Waiten> for bool {
    #[inline(always)]
    fn from(variant: Waiten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAITEN` reader - Enables or disables WDOG in wait mode."]
pub type WaitenR = crate::BitReader<Waiten>;
impl WaitenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Waiten {
        match self.bits {
            false => Waiten::Disabled,
            true => Waiten::Enabled,
        }
    }
    #[doc = "WDOG is disabled in CPU Wait mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Waiten::Disabled
    }
    #[doc = "WDOG is enabled in CPU Wait mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Waiten::Enabled
    }
}
#[doc = "Field `WAITEN` writer - Enables or disables WDOG in wait mode."]
pub type WaitenW<'a, REG> = crate::BitWriter<'a, REG, Waiten>;
impl<'a, REG> WaitenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDOG is disabled in CPU Wait mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Waiten::Disabled)
    }
    #[doc = "WDOG is enabled in CPU Wait mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Waiten::Enabled)
    }
}
#[doc = "Field `TESTWDOG` reader - Puts the watchdog in the functional test mode"]
pub type TestwdogR = crate::BitReader;
#[doc = "Field `TESTWDOG` writer - Puts the watchdog in the functional test mode"]
pub type TestwdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Selects the test to be run on the watchdog timer. Effective only if TESTWDOG is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Testsel {
    #[doc = "0: Quick test. The timer runs normally. After 16 clock cycles the counter is loaded with 0xFFFF to reduce the time taken for the counter to expire."]
    QuickTest = 0,
    #[doc = "1: Byte test. Puts the timer in the byte test mode where individual bytes of the counter are enabled for operation and are compared for time out against the TOVALL/TOVALH depending on BYTESEL\\[1:0\\]"]
    ByteTest = 1,
}
impl From<Testsel> for bool {
    #[inline(always)]
    fn from(variant: Testsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TESTSEL` reader - Selects the test to be run on the watchdog timer. Effective only if TESTWDOG is set."]
pub type TestselR = crate::BitReader<Testsel>;
impl TestselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Testsel {
        match self.bits {
            false => Testsel::QuickTest,
            true => Testsel::ByteTest,
        }
    }
    #[doc = "Quick test. The timer runs normally. After 16 clock cycles the counter is loaded with 0xFFFF to reduce the time taken for the counter to expire."]
    #[inline(always)]
    pub fn is_quick_test(&self) -> bool {
        *self == Testsel::QuickTest
    }
    #[doc = "Byte test. Puts the timer in the byte test mode where individual bytes of the counter are enabled for operation and are compared for time out against the TOVALL/TOVALH depending on BYTESEL\\[1:0\\]"]
    #[inline(always)]
    pub fn is_byte_test(&self) -> bool {
        *self == Testsel::ByteTest
    }
}
#[doc = "Field `TESTSEL` writer - Selects the test to be run on the watchdog timer. Effective only if TESTWDOG is set."]
pub type TestselW<'a, REG> = crate::BitWriter<'a, REG, Testsel>;
impl<'a, REG> TestselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Quick test. The timer runs normally. After 16 clock cycles the counter is loaded with 0xFFFF to reduce the time taken for the counter to expire."]
    #[inline(always)]
    pub fn quick_test(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::QuickTest)
    }
    #[doc = "Byte test. Puts the timer in the byte test mode where individual bytes of the counter are enabled for operation and are compared for time out against the TOVALL/TOVALH depending on BYTESEL\\[1:0\\]"]
    #[inline(always)]
    pub fn byte_test(self) -> &'a mut crate::W<REG> {
        self.variant(Testsel::ByteTest)
    }
}
#[doc = "This 2-bit field select the byte to be tested when the watchdog is in the byte test mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bytesel {
    #[doc = "0: Byte 0 selected"]
    Byte0 = 0,
    #[doc = "1: Byte 1 selected"]
    Byte1 = 1,
    #[doc = "2: Byte 2 selected"]
    Byte2 = 2,
    #[doc = "3: Byte 3 selected"]
    Byte3 = 3,
}
impl From<Bytesel> for u8 {
    #[inline(always)]
    fn from(variant: Bytesel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bytesel {
    type Ux = u8;
}
impl crate::IsEnum for Bytesel {}
#[doc = "Field `BYTESEL` reader - This 2-bit field select the byte to be tested when the watchdog is in the byte test mode."]
pub type ByteselR = crate::FieldReader<Bytesel>;
impl ByteselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bytesel {
        match self.bits {
            0 => Bytesel::Byte0,
            1 => Bytesel::Byte1,
            2 => Bytesel::Byte2,
            3 => Bytesel::Byte3,
            _ => unreachable!(),
        }
    }
    #[doc = "Byte 0 selected"]
    #[inline(always)]
    pub fn is_byte0(&self) -> bool {
        *self == Bytesel::Byte0
    }
    #[doc = "Byte 1 selected"]
    #[inline(always)]
    pub fn is_byte1(&self) -> bool {
        *self == Bytesel::Byte1
    }
    #[doc = "Byte 2 selected"]
    #[inline(always)]
    pub fn is_byte2(&self) -> bool {
        *self == Bytesel::Byte2
    }
    #[doc = "Byte 3 selected"]
    #[inline(always)]
    pub fn is_byte3(&self) -> bool {
        *self == Bytesel::Byte3
    }
}
#[doc = "Field `BYTESEL` writer - This 2-bit field select the byte to be tested when the watchdog is in the byte test mode."]
pub type ByteselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bytesel, crate::Safe>;
impl<'a, REG> ByteselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte 0 selected"]
    #[inline(always)]
    pub fn byte0(self) -> &'a mut crate::W<REG> {
        self.variant(Bytesel::Byte0)
    }
    #[doc = "Byte 1 selected"]
    #[inline(always)]
    pub fn byte1(self) -> &'a mut crate::W<REG> {
        self.variant(Bytesel::Byte1)
    }
    #[doc = "Byte 2 selected"]
    #[inline(always)]
    pub fn byte2(self) -> &'a mut crate::W<REG> {
        self.variant(Bytesel::Byte2)
    }
    #[doc = "Byte 3 selected"]
    #[inline(always)]
    pub fn byte3(self) -> &'a mut crate::W<REG> {
        self.variant(Bytesel::Byte3)
    }
}
#[doc = "Allows the WDOG's functional test mode to be disabled permanently\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Distestwdog {
    #[doc = "0: WDOG functional test mode is not disabled"]
    Allowed = 0,
    #[doc = "1: WDOG functional test mode is disabled permanently until reset"]
    Disabled = 1,
}
impl From<Distestwdog> for bool {
    #[inline(always)]
    fn from(variant: Distestwdog) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISTESTWDOG` reader - Allows the WDOG's functional test mode to be disabled permanently"]
pub type DistestwdogR = crate::BitReader<Distestwdog>;
impl DistestwdogR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Distestwdog {
        match self.bits {
            false => Distestwdog::Allowed,
            true => Distestwdog::Disabled,
        }
    }
    #[doc = "WDOG functional test mode is not disabled"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == Distestwdog::Allowed
    }
    #[doc = "WDOG functional test mode is disabled permanently until reset"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Distestwdog::Disabled
    }
}
#[doc = "Field `DISTESTWDOG` writer - Allows the WDOG's functional test mode to be disabled permanently"]
pub type DistestwdogW<'a, REG> = crate::BitWriter<'a, REG, Distestwdog>;
impl<'a, REG> DistestwdogW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WDOG functional test mode is not disabled"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut crate::W<REG> {
        self.variant(Distestwdog::Allowed)
    }
    #[doc = "WDOG functional test mode is disabled permanently until reset"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Distestwdog::Disabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enables or disables the WDOG's operation"]
    #[inline(always)]
    pub fn wdogen(&self) -> WdogenR {
        WdogenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects clock source for the WDOG timer and other internal timing operations."]
    #[inline(always)]
    pub fn clksrc(&self) -> ClksrcR {
        ClksrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Used to enable the debug breadcrumbs feature"]
    #[inline(always)]
    pub fn irqrsten(&self) -> IrqrstenR {
        IrqrstenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable windowing mode."]
    #[inline(always)]
    pub fn winen(&self) -> WinenR {
        WinenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables updates to watchdog write once registers, after initial configuration window (WCT) closes, through unlock sequence"]
    #[inline(always)]
    pub fn allowupdate(&self) -> AllowupdateR {
        AllowupdateR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables or disables WDOG in Debug mode."]
    #[inline(always)]
    pub fn dbgen(&self) -> DbgenR {
        DbgenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables or disables WDOG in stop mode."]
    #[inline(always)]
    pub fn stopen(&self) -> StopenR {
        StopenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables or disables WDOG in wait mode."]
    #[inline(always)]
    pub fn waiten(&self) -> WaitenR {
        WaitenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Puts the watchdog in the functional test mode"]
    #[inline(always)]
    pub fn testwdog(&self) -> TestwdogR {
        TestwdogR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects the test to be run on the watchdog timer. Effective only if TESTWDOG is set."]
    #[inline(always)]
    pub fn testsel(&self) -> TestselR {
        TestselR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - This 2-bit field select the byte to be tested when the watchdog is in the byte test mode."]
    #[inline(always)]
    pub fn bytesel(&self) -> ByteselR {
        ByteselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Allows the WDOG's functional test mode to be disabled permanently"]
    #[inline(always)]
    pub fn distestwdog(&self) -> DistestwdogR {
        DistestwdogR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables or disables the WDOG's operation"]
    #[inline(always)]
    pub fn wdogen(&mut self) -> WdogenW<'_, StctrlhSpec> {
        WdogenW::new(self, 0)
    }
    #[doc = "Bit 1 - Selects clock source for the WDOG timer and other internal timing operations."]
    #[inline(always)]
    pub fn clksrc(&mut self) -> ClksrcW<'_, StctrlhSpec> {
        ClksrcW::new(self, 1)
    }
    #[doc = "Bit 2 - Used to enable the debug breadcrumbs feature"]
    #[inline(always)]
    pub fn irqrsten(&mut self) -> IrqrstenW<'_, StctrlhSpec> {
        IrqrstenW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable windowing mode."]
    #[inline(always)]
    pub fn winen(&mut self) -> WinenW<'_, StctrlhSpec> {
        WinenW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables updates to watchdog write once registers, after initial configuration window (WCT) closes, through unlock sequence"]
    #[inline(always)]
    pub fn allowupdate(&mut self) -> AllowupdateW<'_, StctrlhSpec> {
        AllowupdateW::new(self, 4)
    }
    #[doc = "Bit 5 - Enables or disables WDOG in Debug mode."]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DbgenW<'_, StctrlhSpec> {
        DbgenW::new(self, 5)
    }
    #[doc = "Bit 6 - Enables or disables WDOG in stop mode."]
    #[inline(always)]
    pub fn stopen(&mut self) -> StopenW<'_, StctrlhSpec> {
        StopenW::new(self, 6)
    }
    #[doc = "Bit 7 - Enables or disables WDOG in wait mode."]
    #[inline(always)]
    pub fn waiten(&mut self) -> WaitenW<'_, StctrlhSpec> {
        WaitenW::new(self, 7)
    }
    #[doc = "Bit 10 - Puts the watchdog in the functional test mode"]
    #[inline(always)]
    pub fn testwdog(&mut self) -> TestwdogW<'_, StctrlhSpec> {
        TestwdogW::new(self, 10)
    }
    #[doc = "Bit 11 - Selects the test to be run on the watchdog timer. Effective only if TESTWDOG is set."]
    #[inline(always)]
    pub fn testsel(&mut self) -> TestselW<'_, StctrlhSpec> {
        TestselW::new(self, 11)
    }
    #[doc = "Bits 12:13 - This 2-bit field select the byte to be tested when the watchdog is in the byte test mode."]
    #[inline(always)]
    pub fn bytesel(&mut self) -> ByteselW<'_, StctrlhSpec> {
        ByteselW::new(self, 12)
    }
    #[doc = "Bit 14 - Allows the WDOG's functional test mode to be disabled permanently"]
    #[inline(always)]
    pub fn distestwdog(&mut self) -> DistestwdogW<'_, StctrlhSpec> {
        DistestwdogW::new(self, 14)
    }
}
#[doc = "Watchdog Status and Control Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`stctrlh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stctrlh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StctrlhSpec;
impl crate::RegisterSpec for StctrlhSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`stctrlh::R`](R) reader structure"]
impl crate::Readable for StctrlhSpec {}
#[doc = "`write(|w| ..)` method takes [`stctrlh::W`](W) writer structure"]
impl crate::Writable for StctrlhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STCTRLH to value 0x01d3"]
impl crate::Resettable for StctrlhSpec {
    const RESET_VALUE: u16 = 0x01d3;
}

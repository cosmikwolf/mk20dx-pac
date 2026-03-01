#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Halt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Halt {
    #[doc = "0: Start transfers"]
    Running = 0,
    #[doc = "1: Stop transfers"]
    Halted = 1,
}
impl From<Halt> for bool {
    #[inline(always)]
    fn from(variant: Halt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT` reader - Halt"]
pub type HaltR = crate::BitReader<Halt>;
impl HaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Halt {
        match self.bits {
            false => Halt::Running,
            true => Halt::Halted,
        }
    }
    #[doc = "Start transfers"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == Halt::Running
    }
    #[doc = "Stop transfers"]
    #[inline(always)]
    pub fn is_halted(&self) -> bool {
        *self == Halt::Halted
    }
}
#[doc = "Field `HALT` writer - Halt"]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG, Halt>;
impl<'a, REG> HaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start transfers"]
    #[inline(always)]
    pub fn running(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::Running)
    }
    #[doc = "Stop transfers"]
    #[inline(always)]
    pub fn halted(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::Halted)
    }
}
#[doc = "Sample Point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SmplPt {
    #[doc = "0: 0 system clocks between SCK edge and SIN sample"]
    _00 = 0,
    #[doc = "1: 1 system clock between SCK edge and SIN sample"]
    _01 = 1,
    #[doc = "2: 2 system clocks between SCK edge and SIN sample"]
    _10 = 2,
}
impl From<SmplPt> for u8 {
    #[inline(always)]
    fn from(variant: SmplPt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SmplPt {
    type Ux = u8;
}
impl crate::IsEnum for SmplPt {}
#[doc = "Field `SMPL_PT` reader - Sample Point"]
pub type SmplPtR = crate::FieldReader<SmplPt>;
impl SmplPtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SmplPt> {
        match self.bits {
            0 => Some(SmplPt::_00),
            1 => Some(SmplPt::_01),
            2 => Some(SmplPt::_10),
            _ => None,
        }
    }
    #[doc = "0 system clocks between SCK edge and SIN sample"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SmplPt::_00
    }
    #[doc = "1 system clock between SCK edge and SIN sample"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SmplPt::_01
    }
    #[doc = "2 system clocks between SCK edge and SIN sample"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SmplPt::_10
    }
}
#[doc = "Field `SMPL_PT` writer - Sample Point"]
pub type SmplPtW<'a, REG> = crate::FieldWriter<'a, REG, 2, SmplPt>;
impl<'a, REG> SmplPtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0 system clocks between SCK edge and SIN sample"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(SmplPt::_00)
    }
    #[doc = "1 system clock between SCK edge and SIN sample"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(SmplPt::_01)
    }
    #[doc = "2 system clocks between SCK edge and SIN sample"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(SmplPt::_10)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrRxf {
    #[doc = "0: Do not clear RX FIFO"]
    NoEffect = 0,
    #[doc = "1: Clear RX FIFO counter"]
    Clear = 1,
}
impl From<ClrRxf> for bool {
    #[inline(always)]
    fn from(variant: ClrRxf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_RXF` writer - no description available"]
pub type ClrRxfW<'a, REG> = crate::BitWriter<'a, REG, ClrRxf>;
impl<'a, REG> ClrRxfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not clear RX FIFO"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ClrRxf::NoEffect)
    }
    #[doc = "Clear RX FIFO counter"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ClrRxf::Clear)
    }
}
#[doc = "Clear TX FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClrTxf {
    #[doc = "0: Do not clear TX FIFO"]
    NoEffect = 0,
    #[doc = "1: Clear TX FIFO counter"]
    Clear = 1,
}
impl From<ClrTxf> for bool {
    #[inline(always)]
    fn from(variant: ClrTxf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_TXF` writer - Clear TX FIFO"]
pub type ClrTxfW<'a, REG> = crate::BitWriter<'a, REG, ClrTxf>;
impl<'a, REG> ClrTxfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not clear TX FIFO"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ClrTxf::NoEffect)
    }
    #[doc = "Clear TX FIFO counter"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ClrTxf::Clear)
    }
}
#[doc = "Disable Receive FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisRxf {
    #[doc = "0: RX FIFO enabled"]
    Enabled = 0,
    #[doc = "1: RX FIFO disabled"]
    Disabled = 1,
}
impl From<DisRxf> for bool {
    #[inline(always)]
    fn from(variant: DisRxf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS_RXF` reader - Disable Receive FIFO"]
pub type DisRxfR = crate::BitReader<DisRxf>;
impl DisRxfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisRxf {
        match self.bits {
            false => DisRxf::Enabled,
            true => DisRxf::Disabled,
        }
    }
    #[doc = "RX FIFO enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DisRxf::Enabled
    }
    #[doc = "RX FIFO disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DisRxf::Disabled
    }
}
#[doc = "Field `DIS_RXF` writer - Disable Receive FIFO"]
pub type DisRxfW<'a, REG> = crate::BitWriter<'a, REG, DisRxf>;
impl<'a, REG> DisRxfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX FIFO enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DisRxf::Enabled)
    }
    #[doc = "RX FIFO disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DisRxf::Disabled)
    }
}
#[doc = "Disable Transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisTxf {
    #[doc = "0: TX FIFO enabled"]
    Enabled = 0,
    #[doc = "1: TX FIFO disabled"]
    Disabled = 1,
}
impl From<DisTxf> for bool {
    #[inline(always)]
    fn from(variant: DisTxf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS_TXF` reader - Disable Transmit FIFO"]
pub type DisTxfR = crate::BitReader<DisTxf>;
impl DisTxfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisTxf {
        match self.bits {
            false => DisTxf::Enabled,
            true => DisTxf::Disabled,
        }
    }
    #[doc = "TX FIFO enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DisTxf::Enabled
    }
    #[doc = "TX FIFO disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DisTxf::Disabled
    }
}
#[doc = "Field `DIS_TXF` writer - Disable Transmit FIFO"]
pub type DisTxfW<'a, REG> = crate::BitWriter<'a, REG, DisTxf>;
impl<'a, REG> DisTxfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX FIFO enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DisTxf::Enabled)
    }
    #[doc = "TX FIFO disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DisTxf::Disabled)
    }
}
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdis {
    #[doc = "0: Enable DSPI clocks"]
    Enabled = 0,
    #[doc = "1: Allow external logic to disable DSPI clocks"]
    Disabled = 1,
}
impl From<Mdis> for bool {
    #[inline(always)]
    fn from(variant: Mdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDIS` reader - Module Disable"]
pub type MdisR = crate::BitReader<Mdis>;
impl MdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdis {
        match self.bits {
            false => Mdis::Enabled,
            true => Mdis::Disabled,
        }
    }
    #[doc = "Enable DSPI clocks"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mdis::Enabled
    }
    #[doc = "Allow external logic to disable DSPI clocks"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mdis::Disabled
    }
}
#[doc = "Field `MDIS` writer - Module Disable"]
pub type MdisW<'a, REG> = crate::BitWriter<'a, REG, Mdis>;
impl<'a, REG> MdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable DSPI clocks"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mdis::Enabled)
    }
    #[doc = "Allow external logic to disable DSPI clocks"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mdis::Disabled)
    }
}
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doze {
    #[doc = "0: Doze mode has no effect on DSPI"]
    NoEffect = 0,
    #[doc = "1: Doze mode disables DSPI"]
    Disabled = 1,
}
impl From<Doze> for bool {
    #[inline(always)]
    fn from(variant: Doze) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOZE` reader - Doze Enable"]
pub type DozeR = crate::BitReader<Doze>;
impl DozeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doze {
        match self.bits {
            false => Doze::NoEffect,
            true => Doze::Disabled,
        }
    }
    #[doc = "Doze mode has no effect on DSPI"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Doze::NoEffect
    }
    #[doc = "Doze mode disables DSPI"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Doze::Disabled
    }
}
#[doc = "Field `DOZE` writer - Doze Enable"]
pub type DozeW<'a, REG> = crate::BitWriter<'a, REG, Doze>;
impl<'a, REG> DozeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Doze mode has no effect on DSPI"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doze::NoEffect)
    }
    #[doc = "Doze mode disables DSPI"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Doze::Disabled)
    }
}
#[doc = "Peripheral Chip Select x Inactive State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcsis {
    #[doc = "0: The inactive state of PCSx is low."]
    _0 = 0,
    #[doc = "1: The inactive state of PCSx is high."]
    _1 = 1,
}
impl From<Pcsis> for u8 {
    #[inline(always)]
    fn from(variant: Pcsis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcsis {
    type Ux = u8;
}
impl crate::IsEnum for Pcsis {}
#[doc = "Field `PCSIS` reader - Peripheral Chip Select x Inactive State"]
pub type PcsisR = crate::FieldReader<Pcsis>;
impl PcsisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pcsis> {
        match self.bits {
            0 => Some(Pcsis::_0),
            1 => Some(Pcsis::_1),
            _ => None,
        }
    }
    #[doc = "The inactive state of PCSx is low."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pcsis::_0
    }
    #[doc = "The inactive state of PCSx is high."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pcsis::_1
    }
}
#[doc = "Field `PCSIS` writer - Peripheral Chip Select x Inactive State"]
pub type PcsisW<'a, REG> = crate::FieldWriter<'a, REG, 6, Pcsis>;
impl<'a, REG> PcsisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The inactive state of PCSx is low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pcsis::_0)
    }
    #[doc = "The inactive state of PCSx is high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcsis::_1)
    }
}
#[doc = "Receive FIFO Overflow Overwrite Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rooe {
    #[doc = "0: Incoming data is ignored on overflow"]
    Ignore = 0,
    #[doc = "1: Incoming data is shifted in on overflow"]
    ShiftIn = 1,
}
impl From<Rooe> for bool {
    #[inline(always)]
    fn from(variant: Rooe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROOE` reader - Receive FIFO Overflow Overwrite Enable"]
pub type RooeR = crate::BitReader<Rooe>;
impl RooeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rooe {
        match self.bits {
            false => Rooe::Ignore,
            true => Rooe::ShiftIn,
        }
    }
    #[doc = "Incoming data is ignored on overflow"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == Rooe::Ignore
    }
    #[doc = "Incoming data is shifted in on overflow"]
    #[inline(always)]
    pub fn is_shift_in(&self) -> bool {
        *self == Rooe::ShiftIn
    }
}
#[doc = "Field `ROOE` writer - Receive FIFO Overflow Overwrite Enable"]
pub type RooeW<'a, REG> = crate::BitWriter<'a, REG, Rooe>;
impl<'a, REG> RooeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Incoming data is ignored on overflow"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut crate::W<REG> {
        self.variant(Rooe::Ignore)
    }
    #[doc = "Incoming data is shifted in on overflow"]
    #[inline(always)]
    pub fn shift_in(self) -> &'a mut crate::W<REG> {
        self.variant(Rooe::ShiftIn)
    }
}
#[doc = "Peripheral Chip Select Strobe Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcsse {
    #[doc = "0: PCS5 used as chip select"]
    ChipSelect = 0,
    #[doc = "1: PCS5 used as active-low PCS strobe"]
    Strobe = 1,
}
impl From<Pcsse> for bool {
    #[inline(always)]
    fn from(variant: Pcsse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCSSE` reader - Peripheral Chip Select Strobe Enable"]
pub type PcsseR = crate::BitReader<Pcsse>;
impl PcsseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcsse {
        match self.bits {
            false => Pcsse::ChipSelect,
            true => Pcsse::Strobe,
        }
    }
    #[doc = "PCS5 used as chip select"]
    #[inline(always)]
    pub fn is_chip_select(&self) -> bool {
        *self == Pcsse::ChipSelect
    }
    #[doc = "PCS5 used as active-low PCS strobe"]
    #[inline(always)]
    pub fn is_strobe(&self) -> bool {
        *self == Pcsse::Strobe
    }
}
#[doc = "Field `PCSSE` writer - Peripheral Chip Select Strobe Enable"]
pub type PcsseW<'a, REG> = crate::BitWriter<'a, REG, Pcsse>;
impl<'a, REG> PcsseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PCS5 used as chip select"]
    #[inline(always)]
    pub fn chip_select(self) -> &'a mut crate::W<REG> {
        self.variant(Pcsse::ChipSelect)
    }
    #[doc = "PCS5 used as active-low PCS strobe"]
    #[inline(always)]
    pub fn strobe(self) -> &'a mut crate::W<REG> {
        self.variant(Pcsse::Strobe)
    }
}
#[doc = "Modified Timing Format Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtfe {
    #[doc = "0: Modified SPI transfer format disabled"]
    Disabled = 0,
    #[doc = "1: Modified SPI transfer format enabled"]
    Enabled = 1,
}
impl From<Mtfe> for bool {
    #[inline(always)]
    fn from(variant: Mtfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTFE` reader - Modified Timing Format Enable"]
pub type MtfeR = crate::BitReader<Mtfe>;
impl MtfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mtfe {
        match self.bits {
            false => Mtfe::Disabled,
            true => Mtfe::Enabled,
        }
    }
    #[doc = "Modified SPI transfer format disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mtfe::Disabled
    }
    #[doc = "Modified SPI transfer format enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mtfe::Enabled
    }
}
#[doc = "Field `MTFE` writer - Modified Timing Format Enable"]
pub type MtfeW<'a, REG> = crate::BitWriter<'a, REG, Mtfe>;
impl<'a, REG> MtfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Modified SPI transfer format disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mtfe::Disabled)
    }
    #[doc = "Modified SPI transfer format enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mtfe::Enabled)
    }
}
#[doc = "Freeze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frz {
    #[doc = "0: Do not halt in debug mode"]
    NoFreeze = 0,
    #[doc = "1: Halt transfers in debug mode"]
    Freeze = 1,
}
impl From<Frz> for bool {
    #[inline(always)]
    fn from(variant: Frz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRZ` reader - Freeze"]
pub type FrzR = crate::BitReader<Frz>;
impl FrzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frz {
        match self.bits {
            false => Frz::NoFreeze,
            true => Frz::Freeze,
        }
    }
    #[doc = "Do not halt in debug mode"]
    #[inline(always)]
    pub fn is_no_freeze(&self) -> bool {
        *self == Frz::NoFreeze
    }
    #[doc = "Halt transfers in debug mode"]
    #[inline(always)]
    pub fn is_freeze(&self) -> bool {
        *self == Frz::Freeze
    }
}
#[doc = "Field `FRZ` writer - Freeze"]
pub type FrzW<'a, REG> = crate::BitWriter<'a, REG, Frz>;
impl<'a, REG> FrzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not halt in debug mode"]
    #[inline(always)]
    pub fn no_freeze(self) -> &'a mut crate::W<REG> {
        self.variant(Frz::NoFreeze)
    }
    #[doc = "Halt transfers in debug mode"]
    #[inline(always)]
    pub fn freeze(self) -> &'a mut crate::W<REG> {
        self.variant(Frz::Freeze)
    }
}
#[doc = "DSPI Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dconf {
    #[doc = "0: SPI"]
    _00 = 0,
}
impl From<Dconf> for u8 {
    #[inline(always)]
    fn from(variant: Dconf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dconf {
    type Ux = u8;
}
impl crate::IsEnum for Dconf {}
#[doc = "Field `DCONF` reader - DSPI Configuration"]
pub type DconfR = crate::FieldReader<Dconf>;
impl DconfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dconf> {
        match self.bits {
            0 => Some(Dconf::_00),
            _ => None,
        }
    }
    #[doc = "SPI"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Dconf::_00
    }
}
#[doc = "Field `DCONF` writer - DSPI Configuration"]
pub type DconfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dconf>;
impl<'a, REG> DconfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Dconf::_00)
    }
}
#[doc = "Continuous SCK Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContScke {
    #[doc = "0: Continuous SCK disabled"]
    Disabled = 0,
    #[doc = "1: Continuous SCK enabled"]
    Enabled = 1,
}
impl From<ContScke> for bool {
    #[inline(always)]
    fn from(variant: ContScke) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONT_SCKE` reader - Continuous SCK Enable"]
pub type ContSckeR = crate::BitReader<ContScke>;
impl ContSckeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ContScke {
        match self.bits {
            false => ContScke::Disabled,
            true => ContScke::Enabled,
        }
    }
    #[doc = "Continuous SCK disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ContScke::Disabled
    }
    #[doc = "Continuous SCK enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ContScke::Enabled
    }
}
#[doc = "Field `CONT_SCKE` writer - Continuous SCK Enable"]
pub type ContSckeW<'a, REG> = crate::BitWriter<'a, REG, ContScke>;
impl<'a, REG> ContSckeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continuous SCK disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ContScke::Disabled)
    }
    #[doc = "Continuous SCK enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ContScke::Enabled)
    }
}
#[doc = "Master/Slave Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstr {
    #[doc = "0: DSPI in slave mode"]
    Slave = 0,
    #[doc = "1: DSPI in master mode"]
    Master = 1,
}
impl From<Mstr> for bool {
    #[inline(always)]
    fn from(variant: Mstr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTR` reader - Master/Slave Mode Select"]
pub type MstrR = crate::BitReader<Mstr>;
impl MstrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstr {
        match self.bits {
            false => Mstr::Slave,
            true => Mstr::Master,
        }
    }
    #[doc = "DSPI in slave mode"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Mstr::Slave
    }
    #[doc = "DSPI in master mode"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Mstr::Master
    }
}
#[doc = "Field `MSTR` writer - Master/Slave Mode Select"]
pub type MstrW<'a, REG> = crate::BitWriter<'a, REG, Mstr>;
impl<'a, REG> MstrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DSPI in slave mode"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(Mstr::Slave)
    }
    #[doc = "DSPI in master mode"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Mstr::Master)
    }
}
impl R {
    #[doc = "Bit 0 - Halt"]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Sample Point"]
    #[inline(always)]
    pub fn smpl_pt(&self) -> SmplPtR {
        SmplPtR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Disable Receive FIFO"]
    #[inline(always)]
    pub fn dis_rxf(&self) -> DisRxfR {
        DisRxfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable Transmit FIFO"]
    #[inline(always)]
    pub fn dis_txf(&self) -> DisTxfR {
        DisTxfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MdisR {
        MdisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Doze Enable"]
    #[inline(always)]
    pub fn doze(&self) -> DozeR {
        DozeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    pub fn pcsis(&self) -> PcsisR {
        PcsisR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Receive FIFO Overflow Overwrite Enable"]
    #[inline(always)]
    pub fn rooe(&self) -> RooeR {
        RooeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peripheral Chip Select Strobe Enable"]
    #[inline(always)]
    pub fn pcsse(&self) -> PcsseR {
        PcsseR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Modified Timing Format Enable"]
    #[inline(always)]
    pub fn mtfe(&self) -> MtfeR {
        MtfeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Freeze"]
    #[inline(always)]
    pub fn frz(&self) -> FrzR {
        FrzR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - DSPI Configuration"]
    #[inline(always)]
    pub fn dconf(&self) -> DconfR {
        DconfR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Continuous SCK Enable"]
    #[inline(always)]
    pub fn cont_scke(&self) -> ContSckeR {
        ContSckeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Master/Slave Mode Select"]
    #[inline(always)]
    pub fn mstr(&self) -> MstrR {
        MstrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt"]
    #[inline(always)]
    pub fn halt(&mut self) -> HaltW<'_, McrSpec> {
        HaltW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Sample Point"]
    #[inline(always)]
    pub fn smpl_pt(&mut self) -> SmplPtW<'_, McrSpec> {
        SmplPtW::new(self, 8)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn clr_rxf(&mut self) -> ClrRxfW<'_, McrSpec> {
        ClrRxfW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear TX FIFO"]
    #[inline(always)]
    pub fn clr_txf(&mut self) -> ClrTxfW<'_, McrSpec> {
        ClrTxfW::new(self, 11)
    }
    #[doc = "Bit 12 - Disable Receive FIFO"]
    #[inline(always)]
    pub fn dis_rxf(&mut self) -> DisRxfW<'_, McrSpec> {
        DisRxfW::new(self, 12)
    }
    #[doc = "Bit 13 - Disable Transmit FIFO"]
    #[inline(always)]
    pub fn dis_txf(&mut self) -> DisTxfW<'_, McrSpec> {
        DisTxfW::new(self, 13)
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&mut self) -> MdisW<'_, McrSpec> {
        MdisW::new(self, 14)
    }
    #[doc = "Bit 15 - Doze Enable"]
    #[inline(always)]
    pub fn doze(&mut self) -> DozeW<'_, McrSpec> {
        DozeW::new(self, 15)
    }
    #[doc = "Bits 16:21 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    pub fn pcsis(&mut self) -> PcsisW<'_, McrSpec> {
        PcsisW::new(self, 16)
    }
    #[doc = "Bit 24 - Receive FIFO Overflow Overwrite Enable"]
    #[inline(always)]
    pub fn rooe(&mut self) -> RooeW<'_, McrSpec> {
        RooeW::new(self, 24)
    }
    #[doc = "Bit 25 - Peripheral Chip Select Strobe Enable"]
    #[inline(always)]
    pub fn pcsse(&mut self) -> PcsseW<'_, McrSpec> {
        PcsseW::new(self, 25)
    }
    #[doc = "Bit 26 - Modified Timing Format Enable"]
    #[inline(always)]
    pub fn mtfe(&mut self) -> MtfeW<'_, McrSpec> {
        MtfeW::new(self, 26)
    }
    #[doc = "Bit 27 - Freeze"]
    #[inline(always)]
    pub fn frz(&mut self) -> FrzW<'_, McrSpec> {
        FrzW::new(self, 27)
    }
    #[doc = "Bits 28:29 - DSPI Configuration"]
    #[inline(always)]
    pub fn dconf(&mut self) -> DconfW<'_, McrSpec> {
        DconfW::new(self, 28)
    }
    #[doc = "Bit 30 - Continuous SCK Enable"]
    #[inline(always)]
    pub fn cont_scke(&mut self) -> ContSckeW<'_, McrSpec> {
        ContSckeW::new(self, 30)
    }
    #[doc = "Bit 31 - Master/Slave Mode Select"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MstrW<'_, McrSpec> {
        MstrW::new(self, 31)
    }
}
#[doc = "DSPI Module Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCR to value 0x4001"]
impl crate::Resettable for McrSpec {
    const RESET_VALUE: u32 = 0x4001;
}

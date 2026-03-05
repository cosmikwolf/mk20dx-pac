#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Channel Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: The channel is not explicitly started"]
    Inactive = 0,
    #[doc = "1: The channel is explicitly started via a software initiated service request"]
    Active = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Channel Start"]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::Inactive,
            true => Start::Active,
        }
    }
    #[doc = "The channel is not explicitly started"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Start::Inactive
    }
    #[doc = "The channel is explicitly started via a software initiated service request"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Start::Active
    }
}
#[doc = "Field `START` writer - Channel Start"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The channel is not explicitly started"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Inactive)
    }
    #[doc = "The channel is explicitly started via a software initiated service request"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Active)
    }
}
#[doc = "Enable an interrupt when major iteration count completes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intmajor {
    #[doc = "0: The end-of-major loop interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: The end-of-major loop interrupt is enabled"]
    Enabled = 1,
}
impl From<Intmajor> for bool {
    #[inline(always)]
    fn from(variant: Intmajor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTMAJOR` reader - Enable an interrupt when major iteration count completes"]
pub type IntmajorR = crate::BitReader<Intmajor>;
impl IntmajorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intmajor {
        match self.bits {
            false => Intmajor::Disabled,
            true => Intmajor::Enabled,
        }
    }
    #[doc = "The end-of-major loop interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Intmajor::Disabled
    }
    #[doc = "The end-of-major loop interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Intmajor::Enabled
    }
}
#[doc = "Field `INTMAJOR` writer - Enable an interrupt when major iteration count completes"]
pub type IntmajorW<'a, REG> = crate::BitWriter<'a, REG, Intmajor>;
impl<'a, REG> IntmajorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The end-of-major loop interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Intmajor::Disabled)
    }
    #[doc = "The end-of-major loop interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Intmajor::Enabled)
    }
}
#[doc = "Enable an interrupt when major counter is half complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inthalf {
    #[doc = "0: The half-point interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: The half-point interrupt is enabled"]
    Enabled = 1,
}
impl From<Inthalf> for bool {
    #[inline(always)]
    fn from(variant: Inthalf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTHALF` reader - Enable an interrupt when major counter is half complete."]
pub type InthalfR = crate::BitReader<Inthalf>;
impl InthalfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inthalf {
        match self.bits {
            false => Inthalf::Disabled,
            true => Inthalf::Enabled,
        }
    }
    #[doc = "The half-point interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Inthalf::Disabled
    }
    #[doc = "The half-point interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Inthalf::Enabled
    }
}
#[doc = "Field `INTHALF` writer - Enable an interrupt when major counter is half complete."]
pub type InthalfW<'a, REG> = crate::BitWriter<'a, REG, Inthalf>;
impl<'a, REG> InthalfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The half-point interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inthalf::Disabled)
    }
    #[doc = "The half-point interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inthalf::Enabled)
    }
}
#[doc = "Disable Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dreq {
    #[doc = "0: The channel's ERQ field is not affected"]
    NoDisable = 0,
    #[doc = "1: The channel's ERQ field is cleared when the major loop is complete"]
    DisableOnComplete = 1,
}
impl From<Dreq> for bool {
    #[inline(always)]
    fn from(variant: Dreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DREQ` reader - Disable Request"]
pub type DreqR = crate::BitReader<Dreq>;
impl DreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dreq {
        match self.bits {
            false => Dreq::NoDisable,
            true => Dreq::DisableOnComplete,
        }
    }
    #[doc = "The channel's ERQ field is not affected"]
    #[inline(always)]
    pub fn is_no_disable(&self) -> bool {
        *self == Dreq::NoDisable
    }
    #[doc = "The channel's ERQ field is cleared when the major loop is complete"]
    #[inline(always)]
    pub fn is_disable_on_complete(&self) -> bool {
        *self == Dreq::DisableOnComplete
    }
}
#[doc = "Field `DREQ` writer - Disable Request"]
pub type DreqW<'a, REG> = crate::BitWriter<'a, REG, Dreq>;
impl<'a, REG> DreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The channel's ERQ field is not affected"]
    #[inline(always)]
    pub fn no_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dreq::NoDisable)
    }
    #[doc = "The channel's ERQ field is cleared when the major loop is complete"]
    #[inline(always)]
    pub fn disable_on_complete(self) -> &'a mut crate::W<REG> {
        self.variant(Dreq::DisableOnComplete)
    }
}
#[doc = "Enable Scatter/Gather Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esg {
    #[doc = "0: The current channel's TCD is normal format"]
    Disabled = 0,
    #[doc = "1: The current channel's TCD specifies a scatter gather format"]
    Enabled = 1,
}
impl From<Esg> for bool {
    #[inline(always)]
    fn from(variant: Esg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESG` reader - Enable Scatter/Gather Processing"]
pub type EsgR = crate::BitReader<Esg>;
impl EsgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Esg {
        match self.bits {
            false => Esg::Disabled,
            true => Esg::Enabled,
        }
    }
    #[doc = "The current channel's TCD is normal format"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Esg::Disabled
    }
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Esg::Enabled
    }
}
#[doc = "Field `ESG` writer - Enable Scatter/Gather Processing"]
pub type EsgW<'a, REG> = crate::BitWriter<'a, REG, Esg>;
impl<'a, REG> EsgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The current channel's TCD is normal format"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Esg::Disabled)
    }
    #[doc = "The current channel's TCD specifies a scatter gather format"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Esg::Enabled)
    }
}
#[doc = "Enable channel-to-channel linking on major loop complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Majorelink {
    #[doc = "0: The channel-to-channel linking is disabled"]
    Disabled = 0,
    #[doc = "1: The channel-to-channel linking is enabled"]
    Enabled = 1,
}
impl From<Majorelink> for bool {
    #[inline(always)]
    fn from(variant: Majorelink) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAJORELINK` reader - Enable channel-to-channel linking on major loop complete"]
pub type MajorelinkR = crate::BitReader<Majorelink>;
impl MajorelinkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Majorelink {
        match self.bits {
            false => Majorelink::Disabled,
            true => Majorelink::Enabled,
        }
    }
    #[doc = "The channel-to-channel linking is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Majorelink::Disabled
    }
    #[doc = "The channel-to-channel linking is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Majorelink::Enabled
    }
}
#[doc = "Field `MAJORELINK` writer - Enable channel-to-channel linking on major loop complete"]
pub type MajorelinkW<'a, REG> = crate::BitWriter<'a, REG, Majorelink>;
impl<'a, REG> MajorelinkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The channel-to-channel linking is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Majorelink::Disabled)
    }
    #[doc = "The channel-to-channel linking is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Majorelink::Enabled)
    }
}
#[doc = "Field `ACTIVE` reader - Channel Active"]
pub type ActiveR = crate::BitReader;
#[doc = "Field `ACTIVE` writer - Channel Active"]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONE` reader - Channel Done"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - Channel Done"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAJORLINKCH` reader - Link Channel Number"]
pub type MajorlinkchR = crate::FieldReader;
#[doc = "Field `MAJORLINKCH` writer - Link Channel Number"]
pub type MajorlinkchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Bandwidth Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bwc {
    #[doc = "0: No eDMA engine stalls"]
    NoStall = 0,
    #[doc = "2: eDMA engine stalls for 4 cycles after each R/W"]
    Stall4cycles = 2,
    #[doc = "3: eDMA engine stalls for 8 cycles after each R/W"]
    Stall8cycles = 3,
}
impl From<Bwc> for u8 {
    #[inline(always)]
    fn from(variant: Bwc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bwc {
    type Ux = u8;
}
impl crate::IsEnum for Bwc {}
#[doc = "Field `BWC` reader - Bandwidth Control"]
pub type BwcR = crate::FieldReader<Bwc>;
impl BwcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bwc> {
        match self.bits {
            0 => Some(Bwc::NoStall),
            2 => Some(Bwc::Stall4cycles),
            3 => Some(Bwc::Stall8cycles),
            _ => None,
        }
    }
    #[doc = "No eDMA engine stalls"]
    #[inline(always)]
    pub fn is_no_stall(&self) -> bool {
        *self == Bwc::NoStall
    }
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    #[inline(always)]
    pub fn is_stall4cycles(&self) -> bool {
        *self == Bwc::Stall4cycles
    }
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    #[inline(always)]
    pub fn is_stall8cycles(&self) -> bool {
        *self == Bwc::Stall8cycles
    }
}
#[doc = "Field `BWC` writer - Bandwidth Control"]
pub type BwcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bwc>;
impl<'a, REG> BwcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No eDMA engine stalls"]
    #[inline(always)]
    pub fn no_stall(self) -> &'a mut crate::W<REG> {
        self.variant(Bwc::NoStall)
    }
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    #[inline(always)]
    pub fn stall4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Bwc::Stall4cycles)
    }
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    #[inline(always)]
    pub fn stall8cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Bwc::Stall8cycles)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Start"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes"]
    #[inline(always)]
    pub fn intmajor(&self) -> IntmajorR {
        IntmajorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub fn inthalf(&self) -> InthalfR {
        InthalfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline(always)]
    pub fn dreq(&self) -> DreqR {
        DreqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub fn esg(&self) -> EsgR {
        EsgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub fn majorelink(&self) -> MajorelinkR {
        MajorelinkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Active"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Link Channel Number"]
    #[inline(always)]
    pub fn majorlinkch(&self) -> MajorlinkchR {
        MajorlinkchR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline(always)]
    pub fn bwc(&self) -> BwcR {
        BwcR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Start"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, CsrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes"]
    #[inline(always)]
    pub fn intmajor(&mut self) -> IntmajorW<'_, CsrSpec> {
        IntmajorW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub fn inthalf(&mut self) -> InthalfW<'_, CsrSpec> {
        InthalfW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline(always)]
    pub fn dreq(&mut self) -> DreqW<'_, CsrSpec> {
        DreqW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub fn esg(&mut self) -> EsgW<'_, CsrSpec> {
        EsgW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub fn majorelink(&mut self) -> MajorelinkW<'_, CsrSpec> {
        MajorelinkW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel Active"]
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<'_, CsrSpec> {
        ActiveW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, CsrSpec> {
        DoneW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Link Channel Number"]
    #[inline(always)]
    pub fn majorlinkch(&mut self) -> MajorlinkchW<'_, CsrSpec> {
        MajorlinkchW::new(self, 8)
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline(always)]
    pub fn bwc(&mut self) -> BwcW<'_, CsrSpec> {
        BwcW::new(self, 14)
    }
}
#[doc = "TCD Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}

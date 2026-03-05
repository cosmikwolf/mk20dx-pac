#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Enable Debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edbg {
    #[doc = "0: When in debug mode, the DMA continues to operate"]
    ContinueOnDebug = 0,
    #[doc = "1: When in debug mode, the DMA stalls the start of a new channel"]
    HaltOnDebug = 1,
}
impl From<Edbg> for bool {
    #[inline(always)]
    fn from(variant: Edbg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDBG` reader - Enable Debug"]
pub type EdbgR = crate::BitReader<Edbg>;
impl EdbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edbg {
        match self.bits {
            false => Edbg::ContinueOnDebug,
            true => Edbg::HaltOnDebug,
        }
    }
    #[doc = "When in debug mode, the DMA continues to operate"]
    #[inline(always)]
    pub fn is_continue_on_debug(&self) -> bool {
        *self == Edbg::ContinueOnDebug
    }
    #[doc = "When in debug mode, the DMA stalls the start of a new channel"]
    #[inline(always)]
    pub fn is_halt_on_debug(&self) -> bool {
        *self == Edbg::HaltOnDebug
    }
}
#[doc = "Field `EDBG` writer - Enable Debug"]
pub type EdbgW<'a, REG> = crate::BitWriter<'a, REG, Edbg>;
impl<'a, REG> EdbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When in debug mode, the DMA continues to operate"]
    #[inline(always)]
    pub fn continue_on_debug(self) -> &'a mut crate::W<REG> {
        self.variant(Edbg::ContinueOnDebug)
    }
    #[doc = "When in debug mode, the DMA stalls the start of a new channel"]
    #[inline(always)]
    pub fn halt_on_debug(self) -> &'a mut crate::W<REG> {
        self.variant(Edbg::HaltOnDebug)
    }
}
#[doc = "Enable Round Robin Channel Arbitration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erca {
    #[doc = "0: Fixed priority arbitration is used for channel selection"]
    FixedPriority = 0,
    #[doc = "1: Round robin arbitration is used for channel selection"]
    RoundRobin = 1,
}
impl From<Erca> for bool {
    #[inline(always)]
    fn from(variant: Erca) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERCA` reader - Enable Round Robin Channel Arbitration"]
pub type ErcaR = crate::BitReader<Erca>;
impl ErcaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erca {
        match self.bits {
            false => Erca::FixedPriority,
            true => Erca::RoundRobin,
        }
    }
    #[doc = "Fixed priority arbitration is used for channel selection"]
    #[inline(always)]
    pub fn is_fixed_priority(&self) -> bool {
        *self == Erca::FixedPriority
    }
    #[doc = "Round robin arbitration is used for channel selection"]
    #[inline(always)]
    pub fn is_round_robin(&self) -> bool {
        *self == Erca::RoundRobin
    }
}
#[doc = "Field `ERCA` writer - Enable Round Robin Channel Arbitration"]
pub type ErcaW<'a, REG> = crate::BitWriter<'a, REG, Erca>;
impl<'a, REG> ErcaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fixed priority arbitration is used for channel selection"]
    #[inline(always)]
    pub fn fixed_priority(self) -> &'a mut crate::W<REG> {
        self.variant(Erca::FixedPriority)
    }
    #[doc = "Round robin arbitration is used for channel selection"]
    #[inline(always)]
    pub fn round_robin(self) -> &'a mut crate::W<REG> {
        self.variant(Erca::RoundRobin)
    }
}
#[doc = "Halt On Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hoe {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: Any error causes the HALT bit to set"]
    HaltOnError = 1,
}
impl From<Hoe> for bool {
    #[inline(always)]
    fn from(variant: Hoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOE` reader - Halt On Error"]
pub type HoeR = crate::BitReader<Hoe>;
impl HoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hoe {
        match self.bits {
            false => Hoe::Normal,
            true => Hoe::HaltOnError,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Hoe::Normal
    }
    #[doc = "Any error causes the HALT bit to set"]
    #[inline(always)]
    pub fn is_halt_on_error(&self) -> bool {
        *self == Hoe::HaltOnError
    }
}
#[doc = "Field `HOE` writer - Halt On Error"]
pub type HoeW<'a, REG> = crate::BitWriter<'a, REG, Hoe>;
impl<'a, REG> HoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Hoe::Normal)
    }
    #[doc = "Any error causes the HALT bit to set"]
    #[inline(always)]
    pub fn halt_on_error(self) -> &'a mut crate::W<REG> {
        self.variant(Hoe::HaltOnError)
    }
}
#[doc = "Halt DMA Operations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Halt {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: Stall the start of any new channels"]
    Halt = 1,
}
impl From<Halt> for bool {
    #[inline(always)]
    fn from(variant: Halt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT` reader - Halt DMA Operations"]
pub type HaltR = crate::BitReader<Halt>;
impl HaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Halt {
        match self.bits {
            false => Halt::Normal,
            true => Halt::Halt,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Halt::Normal
    }
    #[doc = "Stall the start of any new channels"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == Halt::Halt
    }
}
#[doc = "Field `HALT` writer - Halt DMA Operations"]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG, Halt>;
impl<'a, REG> HaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::Normal)
    }
    #[doc = "Stall the start of any new channels"]
    #[inline(always)]
    pub fn halt(self) -> &'a mut crate::W<REG> {
        self.variant(Halt::Halt)
    }
}
#[doc = "Continuous Link Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clm {
    #[doc = "0: A minor loop channel link made to itself goes through channel arbitration before being activated again"]
    Arbitrate = 0,
    #[doc = "1: A minor loop channel link made to itself does not go through channel arbitration before being activated again"]
    ContinuousMinorLink = 1,
}
impl From<Clm> for bool {
    #[inline(always)]
    fn from(variant: Clm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLM` reader - Continuous Link Mode"]
pub type ClmR = crate::BitReader<Clm>;
impl ClmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clm {
        match self.bits {
            false => Clm::Arbitrate,
            true => Clm::ContinuousMinorLink,
        }
    }
    #[doc = "A minor loop channel link made to itself goes through channel arbitration before being activated again"]
    #[inline(always)]
    pub fn is_arbitrate(&self) -> bool {
        *self == Clm::Arbitrate
    }
    #[doc = "A minor loop channel link made to itself does not go through channel arbitration before being activated again"]
    #[inline(always)]
    pub fn is_continuous_minor_link(&self) -> bool {
        *self == Clm::ContinuousMinorLink
    }
}
#[doc = "Field `CLM` writer - Continuous Link Mode"]
pub type ClmW<'a, REG> = crate::BitWriter<'a, REG, Clm>;
impl<'a, REG> ClmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A minor loop channel link made to itself goes through channel arbitration before being activated again"]
    #[inline(always)]
    pub fn arbitrate(self) -> &'a mut crate::W<REG> {
        self.variant(Clm::Arbitrate)
    }
    #[doc = "A minor loop channel link made to itself does not go through channel arbitration before being activated again"]
    #[inline(always)]
    pub fn continuous_minor_link(self) -> &'a mut crate::W<REG> {
        self.variant(Clm::ContinuousMinorLink)
    }
}
#[doc = "Enable Minor Loop Mapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emlm {
    #[doc = "0: Disabled. TCDn.word2 is defined as a 32-bit NBYTES field"]
    Disabled = 0,
    #[doc = "1: Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field"]
    Enabled = 1,
}
impl From<Emlm> for bool {
    #[inline(always)]
    fn from(variant: Emlm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMLM` reader - Enable Minor Loop Mapping"]
pub type EmlmR = crate::BitReader<Emlm>;
impl EmlmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emlm {
        match self.bits {
            false => Emlm::Disabled,
            true => Emlm::Enabled,
        }
    }
    #[doc = "Disabled. TCDn.word2 is defined as a 32-bit NBYTES field"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Emlm::Disabled
    }
    #[doc = "Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Emlm::Enabled
    }
}
#[doc = "Field `EMLM` writer - Enable Minor Loop Mapping"]
pub type EmlmW<'a, REG> = crate::BitWriter<'a, REG, Emlm>;
impl<'a, REG> EmlmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. TCDn.word2 is defined as a 32-bit NBYTES field"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Emlm::Disabled)
    }
    #[doc = "Enabled. TCDn.word2 is redefined to include individual enable fields, an offset field, and the NBYTES field"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Emlm::Enabled)
    }
}
#[doc = "Error Cancel Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecx {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: Cancel the remaining data transfer in the same fashion as the EBW bit"]
    Cancel = 1,
}
impl From<Ecx> for bool {
    #[inline(always)]
    fn from(variant: Ecx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECX` reader - Error Cancel Transfer"]
pub type EcxR = crate::BitReader<Ecx>;
impl EcxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecx {
        match self.bits {
            false => Ecx::Normal,
            true => Ecx::Cancel,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Ecx::Normal
    }
    #[doc = "Cancel the remaining data transfer in the same fashion as the EBW bit"]
    #[inline(always)]
    pub fn is_cancel(&self) -> bool {
        *self == Ecx::Cancel
    }
}
#[doc = "Field `ECX` writer - Error Cancel Transfer"]
pub type EcxW<'a, REG> = crate::BitWriter<'a, REG, Ecx>;
impl<'a, REG> EcxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Ecx::Normal)
    }
    #[doc = "Cancel the remaining data transfer in the same fashion as the EBW bit"]
    #[inline(always)]
    pub fn cancel(self) -> &'a mut crate::W<REG> {
        self.variant(Ecx::Cancel)
    }
}
#[doc = "Cancel Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cx {
    #[doc = "0: Normal operation"]
    Normal = 0,
    #[doc = "1: Cancel the remaining data transfer in the same fashion as the EBW bit"]
    Cancel = 1,
}
impl From<Cx> for bool {
    #[inline(always)]
    fn from(variant: Cx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CX` reader - Cancel Transfer"]
pub type CxR = crate::BitReader<Cx>;
impl CxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cx {
        match self.bits {
            false => Cx::Normal,
            true => Cx::Cancel,
        }
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Cx::Normal
    }
    #[doc = "Cancel the remaining data transfer in the same fashion as the EBW bit"]
    #[inline(always)]
    pub fn is_cancel(&self) -> bool {
        *self == Cx::Cancel
    }
}
#[doc = "Field `CX` writer - Cancel Transfer"]
pub type CxW<'a, REG> = crate::BitWriter<'a, REG, Cx>;
impl<'a, REG> CxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Cx::Normal)
    }
    #[doc = "Cancel the remaining data transfer in the same fashion as the EBW bit"]
    #[inline(always)]
    pub fn cancel(self) -> &'a mut crate::W<REG> {
        self.variant(Cx::Cancel)
    }
}
impl R {
    #[doc = "Bit 1 - Enable Debug"]
    #[inline(always)]
    pub fn edbg(&self) -> EdbgR {
        EdbgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Round Robin Channel Arbitration"]
    #[inline(always)]
    pub fn erca(&self) -> ErcaR {
        ErcaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Halt On Error"]
    #[inline(always)]
    pub fn hoe(&self) -> HoeR {
        HoeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Halt DMA Operations"]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Continuous Link Mode"]
    #[inline(always)]
    pub fn clm(&self) -> ClmR {
        ClmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Minor Loop Mapping"]
    #[inline(always)]
    pub fn emlm(&self) -> EmlmR {
        EmlmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Error Cancel Transfer"]
    #[inline(always)]
    pub fn ecx(&self) -> EcxR {
        EcxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Cancel Transfer"]
    #[inline(always)]
    pub fn cx(&self) -> CxR {
        CxR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Enable Debug"]
    #[inline(always)]
    pub fn edbg(&mut self) -> EdbgW<'_, CrSpec> {
        EdbgW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Round Robin Channel Arbitration"]
    #[inline(always)]
    pub fn erca(&mut self) -> ErcaW<'_, CrSpec> {
        ErcaW::new(self, 2)
    }
    #[doc = "Bit 4 - Halt On Error"]
    #[inline(always)]
    pub fn hoe(&mut self) -> HoeW<'_, CrSpec> {
        HoeW::new(self, 4)
    }
    #[doc = "Bit 5 - Halt DMA Operations"]
    #[inline(always)]
    pub fn halt(&mut self) -> HaltW<'_, CrSpec> {
        HaltW::new(self, 5)
    }
    #[doc = "Bit 6 - Continuous Link Mode"]
    #[inline(always)]
    pub fn clm(&mut self) -> ClmW<'_, CrSpec> {
        ClmW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Minor Loop Mapping"]
    #[inline(always)]
    pub fn emlm(&mut self) -> EmlmW<'_, CrSpec> {
        EmlmW::new(self, 7)
    }
    #[doc = "Bit 16 - Error Cancel Transfer"]
    #[inline(always)]
    pub fn ecx(&mut self) -> EcxW<'_, CrSpec> {
        EcxW::new(self, 16)
    }
    #[doc = "Bit 17 - Cancel Transfer"]
    #[inline(always)]
    pub fn cx(&mut self) -> CxW<'_, CrSpec> {
        CxW::new(self, 17)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}

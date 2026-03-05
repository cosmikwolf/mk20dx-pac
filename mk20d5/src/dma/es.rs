#[doc = "Register `ES` reader"]
pub type R = crate::R<EsSpec>;
#[doc = "Destination Bus Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbe {
    #[doc = "0: No destination bus error"]
    NoError = 0,
    #[doc = "1: The last recorded error was a bus error on a destination write"]
    Error = 1,
}
impl From<Dbe> for bool {
    #[inline(always)]
    fn from(variant: Dbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBE` reader - Destination Bus Error"]
pub type DbeR = crate::BitReader<Dbe>;
impl DbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbe {
        match self.bits {
            false => Dbe::NoError,
            true => Dbe::Error,
        }
    }
    #[doc = "No destination bus error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Dbe::NoError
    }
    #[doc = "The last recorded error was a bus error on a destination write"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Dbe::Error
    }
}
#[doc = "Source Bus Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbe {
    #[doc = "0: No source bus error"]
    NoError = 0,
    #[doc = "1: The last recorded error was a bus error on a source read"]
    Error = 1,
}
impl From<Sbe> for bool {
    #[inline(always)]
    fn from(variant: Sbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBE` reader - Source Bus Error"]
pub type SbeR = crate::BitReader<Sbe>;
impl SbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbe {
        match self.bits {
            false => Sbe::NoError,
            true => Sbe::Error,
        }
    }
    #[doc = "No source bus error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Sbe::NoError
    }
    #[doc = "The last recorded error was a bus error on a source read"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Sbe::Error
    }
}
#[doc = "Scatter/Gather Configuration Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sge {
    #[doc = "0: No scatter/gather configuration error"]
    NoError = 0,
    #[doc = "1: The last recorded error was a scatter/gather configuration error"]
    Error = 1,
}
impl From<Sge> for bool {
    #[inline(always)]
    fn from(variant: Sge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SGE` reader - Scatter/Gather Configuration Error"]
pub type SgeR = crate::BitReader<Sge>;
impl SgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sge {
        match self.bits {
            false => Sge::NoError,
            true => Sge::Error,
        }
    }
    #[doc = "No scatter/gather configuration error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Sge::NoError
    }
    #[doc = "The last recorded error was a scatter/gather configuration error"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Sge::Error
    }
}
#[doc = "NBYTES/CITER Configuration Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nce {
    #[doc = "0: No NBYTES/CITER configuration error"]
    NoError = 0,
    #[doc = "1: The last recorded error was a configuration error in the TCDn_NBYTES or TCDn_CITER fields"]
    Error = 1,
}
impl From<Nce> for bool {
    #[inline(always)]
    fn from(variant: Nce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCE` reader - NBYTES/CITER Configuration Error"]
pub type NceR = crate::BitReader<Nce>;
impl NceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nce {
        match self.bits {
            false => Nce::NoError,
            true => Nce::Error,
        }
    }
    #[doc = "No NBYTES/CITER configuration error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Nce::NoError
    }
    #[doc = "The last recorded error was a configuration error in the TCDn_NBYTES or TCDn_CITER fields"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Nce::Error
    }
}
#[doc = "Destination Offset Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe {
    #[doc = "0: No destination offset configuration error"]
    NoError = 0,
    #[doc = "1: The last recorded error was a configuration error in the TCDn_DOFF field"]
    Error = 1,
}
impl From<Doe> for bool {
    #[inline(always)]
    fn from(variant: Doe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE` reader - Destination Offset Error"]
pub type DoeR = crate::BitReader<Doe>;
impl DoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe {
        match self.bits {
            false => Doe::NoError,
            true => Doe::Error,
        }
    }
    #[doc = "No destination offset configuration error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Doe::NoError
    }
    #[doc = "The last recorded error was a configuration error in the TCDn_DOFF field"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Doe::Error
    }
}
#[doc = "Destination Address Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dae {
    #[doc = "0: No destination address configuration error"]
    NoError = 0,
    #[doc = "1: The last recorded error was a configuration error in the TCDn_DADDR field"]
    Error = 1,
}
impl From<Dae> for bool {
    #[inline(always)]
    fn from(variant: Dae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAE` reader - Destination Address Error"]
pub type DaeR = crate::BitReader<Dae>;
impl DaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dae {
        match self.bits {
            false => Dae::NoError,
            true => Dae::Error,
        }
    }
    #[doc = "No destination address configuration error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Dae::NoError
    }
    #[doc = "The last recorded error was a configuration error in the TCDn_DADDR field"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Dae::Error
    }
}
#[doc = "Source Offset Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Soe {
    #[doc = "0: No source offset configuration error"]
    NoError = 0,
    #[doc = "1: The last recorded error was a configuration error in the TCDn_SOFF field"]
    Error = 1,
}
impl From<Soe> for bool {
    #[inline(always)]
    fn from(variant: Soe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOE` reader - Source Offset Error"]
pub type SoeR = crate::BitReader<Soe>;
impl SoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Soe {
        match self.bits {
            false => Soe::NoError,
            true => Soe::Error,
        }
    }
    #[doc = "No source offset configuration error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Soe::NoError
    }
    #[doc = "The last recorded error was a configuration error in the TCDn_SOFF field"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Soe::Error
    }
}
#[doc = "Source Address Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sae {
    #[doc = "0: No source address configuration error"]
    NoError = 0,
    #[doc = "1: The last recorded error was a configuration error in the TCDn_SADDR field"]
    Error = 1,
}
impl From<Sae> for bool {
    #[inline(always)]
    fn from(variant: Sae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAE` reader - Source Address Error"]
pub type SaeR = crate::BitReader<Sae>;
impl SaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sae {
        match self.bits {
            false => Sae::NoError,
            true => Sae::Error,
        }
    }
    #[doc = "No source address configuration error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Sae::NoError
    }
    #[doc = "The last recorded error was a configuration error in the TCDn_SADDR field"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Sae::Error
    }
}
#[doc = "Field `ERRCHN` reader - Error Channel Number or Cancelled Channel Number"]
pub type ErrchnR = crate::FieldReader;
#[doc = "Channel Priority Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpe {
    #[doc = "0: No channel priority configuration error"]
    NoError = 0,
    #[doc = "1: The last recorded error was a configuration error in the channel priorities"]
    Error = 1,
}
impl From<Cpe> for bool {
    #[inline(always)]
    fn from(variant: Cpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPE` reader - Channel Priority Error"]
pub type CpeR = crate::BitReader<Cpe>;
impl CpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpe {
        match self.bits {
            false => Cpe::NoError,
            true => Cpe::Error,
        }
    }
    #[doc = "No channel priority configuration error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Cpe::NoError
    }
    #[doc = "The last recorded error was a configuration error in the channel priorities"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Cpe::Error
    }
}
#[doc = "Transfer Cancelled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecx {
    #[doc = "0: No cancelled transfers"]
    NotCancelled = 0,
    #[doc = "1: The last recorded entry was a cancelled transfer by the error cancel transfer input"]
    Cancelled = 1,
}
impl From<Ecx> for bool {
    #[inline(always)]
    fn from(variant: Ecx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECX` reader - Transfer Cancelled"]
pub type EcxR = crate::BitReader<Ecx>;
impl EcxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecx {
        match self.bits {
            false => Ecx::NotCancelled,
            true => Ecx::Cancelled,
        }
    }
    #[doc = "No cancelled transfers"]
    #[inline(always)]
    pub fn is_not_cancelled(&self) -> bool {
        *self == Ecx::NotCancelled
    }
    #[doc = "The last recorded entry was a cancelled transfer by the error cancel transfer input"]
    #[inline(always)]
    pub fn is_cancelled(&self) -> bool {
        *self == Ecx::Cancelled
    }
}
#[doc = "Logical OR of all ERR status bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vld {
    #[doc = "0: No ERR bits are set"]
    NoErrors = 0,
    #[doc = "1: At least one ERR bit is set indicating a valid error exists that has not been cleared"]
    Valid = 1,
}
impl From<Vld> for bool {
    #[inline(always)]
    fn from(variant: Vld) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLD` reader - Logical OR of all ERR status bits"]
pub type VldR = crate::BitReader<Vld>;
impl VldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vld {
        match self.bits {
            false => Vld::NoErrors,
            true => Vld::Valid,
        }
    }
    #[doc = "No ERR bits are set"]
    #[inline(always)]
    pub fn is_no_errors(&self) -> bool {
        *self == Vld::NoErrors
    }
    #[doc = "At least one ERR bit is set indicating a valid error exists that has not been cleared"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Vld::Valid
    }
}
impl R {
    #[doc = "Bit 0 - Destination Bus Error"]
    #[inline(always)]
    pub fn dbe(&self) -> DbeR {
        DbeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Source Bus Error"]
    #[inline(always)]
    pub fn sbe(&self) -> SbeR {
        SbeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scatter/Gather Configuration Error"]
    #[inline(always)]
    pub fn sge(&self) -> SgeR {
        SgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NBYTES/CITER Configuration Error"]
    #[inline(always)]
    pub fn nce(&self) -> NceR {
        NceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Destination Offset Error"]
    #[inline(always)]
    pub fn doe(&self) -> DoeR {
        DoeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Destination Address Error"]
    #[inline(always)]
    pub fn dae(&self) -> DaeR {
        DaeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source Offset Error"]
    #[inline(always)]
    pub fn soe(&self) -> SoeR {
        SoeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Source Address Error"]
    #[inline(always)]
    pub fn sae(&self) -> SaeR {
        SaeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Error Channel Number or Cancelled Channel Number"]
    #[inline(always)]
    pub fn errchn(&self) -> ErrchnR {
        ErrchnR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Channel Priority Error"]
    #[inline(always)]
    pub fn cpe(&self) -> CpeR {
        CpeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Transfer Cancelled"]
    #[inline(always)]
    pub fn ecx(&self) -> EcxR {
        EcxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Logical OR of all ERR status bits"]
    #[inline(always)]
    pub fn vld(&self) -> VldR {
        VldR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`es::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EsSpec;
impl crate::RegisterSpec for EsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`es::R`](R) reader structure"]
impl crate::Readable for EsSpec {}
#[doc = "`reset()` method sets ES to value 0"]
impl crate::Resettable for EsSpec {}

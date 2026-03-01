#[doc = "Register `S1` reader"]
pub type R = crate::R<S1Spec>;
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pf {
    #[doc = "0: No parity error"]
    NoError = 0,
    #[doc = "1: Parity error detected"]
    Error = 1,
}
impl From<Pf> for bool {
    #[inline(always)]
    fn from(variant: Pf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PF` reader - Parity Error Flag"]
pub type PfR = crate::BitReader<Pf>;
impl PfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pf {
        match self.bits {
            false => Pf::NoError,
            true => Pf::Error,
        }
    }
    #[doc = "No parity error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Pf::NoError
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Pf::Error
    }
}
#[doc = "Framing Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    #[doc = "0: No framing error"]
    NoError = 0,
    #[doc = "1: Framing error detected"]
    Error = 1,
}
impl From<Fe> for bool {
    #[inline(always)]
    fn from(variant: Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Framing Error Flag"]
pub type FeR = crate::BitReader<Fe>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fe {
        match self.bits {
            false => Fe::NoError,
            true => Fe::Error,
        }
    }
    #[doc = "No framing error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == Fe::NoError
    }
    #[doc = "Framing error detected"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == Fe::Error
    }
}
#[doc = "Noise Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nf {
    #[doc = "0: No noise detected"]
    NoNoise = 0,
    #[doc = "1: Noise detected"]
    Noise = 1,
}
impl From<Nf> for bool {
    #[inline(always)]
    fn from(variant: Nf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NF` reader - Noise Flag"]
pub type NfR = crate::BitReader<Nf>;
impl NfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nf {
        match self.bits {
            false => Nf::NoNoise,
            true => Nf::Noise,
        }
    }
    #[doc = "No noise detected"]
    #[inline(always)]
    pub fn is_no_noise(&self) -> bool {
        *self == Nf::NoNoise
    }
    #[doc = "Noise detected"]
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == Nf::Noise
    }
}
#[doc = "Receiver Overrun Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Or {
    #[doc = "0: No overrun"]
    NoOverrun = 0,
    #[doc = "1: Overrun detected"]
    Overrun = 1,
}
impl From<Or> for bool {
    #[inline(always)]
    fn from(variant: Or) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OR` reader - Receiver Overrun Flag"]
pub type OrR = crate::BitReader<Or>;
impl OrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Or {
        match self.bits {
            false => Or::NoOverrun,
            true => Or::Overrun,
        }
    }
    #[doc = "No overrun"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == Or::NoOverrun
    }
    #[doc = "Overrun detected"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == Or::Overrun
    }
}
#[doc = "Idle Line Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "0: Receiver active or not idle"]
    Active = 0,
    #[doc = "1: Receiver idle"]
    Idle = 1,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - Idle Line Flag"]
pub type IdleR = crate::BitReader<Idle>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idle {
        match self.bits {
            false => Idle::Active,
            true => Idle::Idle,
        }
    }
    #[doc = "Receiver active or not idle"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Idle::Active
    }
    #[doc = "Receiver idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Idle::Idle
    }
}
#[doc = "Receive Data Register Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdrf {
    #[doc = "0: Receive data register empty"]
    Empty = 0,
    #[doc = "1: Receive data register full"]
    Full = 1,
}
impl From<Rdrf> for bool {
    #[inline(always)]
    fn from(variant: Rdrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDRF` reader - Receive Data Register Full Flag"]
pub type RdrfR = crate::BitReader<Rdrf>;
impl RdrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdrf {
        match self.bits {
            false => Rdrf::Empty,
            true => Rdrf::Full,
        }
    }
    #[doc = "Receive data register empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Rdrf::Empty
    }
    #[doc = "Receive data register full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Rdrf::Full
    }
}
#[doc = "Transmit Complete Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc {
    #[doc = "0: Transmitter active"]
    Active = 0,
    #[doc = "1: Transmitter idle"]
    Complete = 1,
}
impl From<Tc> for bool {
    #[inline(always)]
    fn from(variant: Tc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC` reader - Transmit Complete Flag"]
pub type TcR = crate::BitReader<Tc>;
impl TcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc {
        match self.bits {
            false => Tc::Active,
            true => Tc::Complete,
        }
    }
    #[doc = "Transmitter active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Tc::Active
    }
    #[doc = "Transmitter idle"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == Tc::Complete
    }
}
#[doc = "Transmit Data Register Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdre {
    #[doc = "0: Transmit data register full"]
    Full = 0,
    #[doc = "1: Transmit data register empty"]
    Empty = 1,
}
impl From<Tdre> for bool {
    #[inline(always)]
    fn from(variant: Tdre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDRE` reader - Transmit Data Register Empty Flag"]
pub type TdreR = crate::BitReader<Tdre>;
impl TdreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdre {
        match self.bits {
            false => Tdre::Full,
            true => Tdre::Empty,
        }
    }
    #[doc = "Transmit data register full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Tdre::Full
    }
    #[doc = "Transmit data register empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Tdre::Empty
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Flag"]
    #[inline(always)]
    pub fn pf(&self) -> PfR {
        PfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing Error Flag"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise Flag"]
    #[inline(always)]
    pub fn nf(&self) -> NfR {
        NfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver Overrun Flag"]
    #[inline(always)]
    pub fn or(&self) -> OrR {
        OrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle Line Flag"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Data Register Full Flag"]
    #[inline(always)]
    pub fn rdrf(&self) -> RdrfR {
        RdrfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Complete Flag"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Data Register Empty Flag"]
    #[inline(always)]
    pub fn tdre(&self) -> TdreR {
        TdreR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "UART Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`s1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S1Spec;
impl crate::RegisterSpec for S1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`s1::R`](R) reader structure"]
impl crate::Readable for S1Spec {}
#[doc = "`reset()` method sets S1 to value 0xc0"]
impl crate::Resettable for S1Spec {
    const RESET_VALUE: u8 = 0xc0;
}

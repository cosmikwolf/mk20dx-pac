#[doc = "Register `SCGC6` reader"]
pub type R = crate::R<Scgc6Spec>;
#[doc = "Register `SCGC6` writer"]
pub type W = crate::W<Scgc6Spec>;
#[doc = "Flash Memory Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ftfl {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Ftfl> for bool {
    #[inline(always)]
    fn from(variant: Ftfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTFL` reader - Flash Memory Clock Gate Control"]
pub type FtflR = crate::BitReader<Ftfl>;
impl FtflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ftfl {
        match self.bits {
            false => Ftfl::Disabled,
            true => Ftfl::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ftfl::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ftfl::Enabled
    }
}
#[doc = "Field `FTFL` writer - Flash Memory Clock Gate Control"]
pub type FtflW<'a, REG> = crate::BitWriter<'a, REG, Ftfl>;
impl<'a, REG> FtflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ftfl::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ftfl::Enabled)
    }
}
#[doc = "DMA Mux Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmamux {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Dmamux> for bool {
    #[inline(always)]
    fn from(variant: Dmamux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMUX` reader - DMA Mux Clock Gate Control"]
pub type DmamuxR = crate::BitReader<Dmamux>;
impl DmamuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmamux {
        match self.bits {
            false => Dmamux::Disabled,
            true => Dmamux::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmamux::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmamux::Enabled
    }
}
#[doc = "Field `DMAMUX` writer - DMA Mux Clock Gate Control"]
pub type DmamuxW<'a, REG> = crate::BitWriter<'a, REG, Dmamux>;
impl<'a, REG> DmamuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamux::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamux::Enabled)
    }
}
#[doc = "SPI0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi0 {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Spi0> for bool {
    #[inline(always)]
    fn from(variant: Spi0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0` reader - SPI0 Clock Gate Control"]
pub type Spi0R = crate::BitReader<Spi0>;
impl Spi0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi0 {
        match self.bits {
            false => Spi0::Disabled,
            true => Spi0::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Spi0::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Spi0::Enabled
    }
}
#[doc = "Field `SPI0` writer - SPI0 Clock Gate Control"]
pub type Spi0W<'a, REG> = crate::BitWriter<'a, REG, Spi0>;
impl<'a, REG> Spi0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Spi0::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Spi0::Enabled)
    }
}
#[doc = "I2S Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2s {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<I2s> for bool {
    #[inline(always)]
    fn from(variant: I2s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2S` reader - I2S Clock Gate Control"]
pub type I2sR = crate::BitReader<I2s>;
impl I2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2s {
        match self.bits {
            false => I2s::Disabled,
            true => I2s::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2s::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2s::Enabled
    }
}
#[doc = "Field `I2S` writer - I2S Clock Gate Control"]
pub type I2sW<'a, REG> = crate::BitWriter<'a, REG, I2s>;
impl<'a, REG> I2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2s::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2s::Enabled)
    }
}
#[doc = "CRC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crc {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Crc> for bool {
    #[inline(always)]
    fn from(variant: Crc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC` reader - CRC Clock Gate Control"]
pub type CrcR = crate::BitReader<Crc>;
impl CrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crc {
        match self.bits {
            false => Crc::Disabled,
            true => Crc::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Crc::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Crc::Enabled
    }
}
#[doc = "Field `CRC` writer - CRC Clock Gate Control"]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG, Crc>;
impl<'a, REG> CrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Crc::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Crc::Enabled)
    }
}
#[doc = "USB DCD Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbdcd {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Usbdcd> for bool {
    #[inline(always)]
    fn from(variant: Usbdcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBDCD` reader - USB DCD Clock Gate Control"]
pub type UsbdcdR = crate::BitReader<Usbdcd>;
impl UsbdcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbdcd {
        match self.bits {
            false => Usbdcd::Disabled,
            true => Usbdcd::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Usbdcd::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Usbdcd::Enabled
    }
}
#[doc = "Field `USBDCD` writer - USB DCD Clock Gate Control"]
pub type UsbdcdW<'a, REG> = crate::BitWriter<'a, REG, Usbdcd>;
impl<'a, REG> UsbdcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Usbdcd::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Usbdcd::Enabled)
    }
}
#[doc = "PDB Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdb {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Pdb> for bool {
    #[inline(always)]
    fn from(variant: Pdb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDB` reader - PDB Clock Gate Control"]
pub type PdbR = crate::BitReader<Pdb>;
impl PdbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdb {
        match self.bits {
            false => Pdb::Disabled,
            true => Pdb::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pdb::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pdb::Enabled
    }
}
#[doc = "Field `PDB` writer - PDB Clock Gate Control"]
pub type PdbW<'a, REG> = crate::BitWriter<'a, REG, Pdb>;
impl<'a, REG> PdbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pdb::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pdb::Enabled)
    }
}
#[doc = "PIT Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pit {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Pit> for bool {
    #[inline(always)]
    fn from(variant: Pit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIT` reader - PIT Clock Gate Control"]
pub type PitR = crate::BitReader<Pit>;
impl PitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pit {
        match self.bits {
            false => Pit::Disabled,
            true => Pit::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pit::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pit::Enabled
    }
}
#[doc = "Field `PIT` writer - PIT Clock Gate Control"]
pub type PitW<'a, REG> = crate::BitWriter<'a, REG, Pit>;
impl<'a, REG> PitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pit::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pit::Enabled)
    }
}
#[doc = "FTM0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ftm0 {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Ftm0> for bool {
    #[inline(always)]
    fn from(variant: Ftm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM0` reader - FTM0 Clock Gate Control"]
pub type Ftm0R = crate::BitReader<Ftm0>;
impl Ftm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ftm0 {
        match self.bits {
            false => Ftm0::Disabled,
            true => Ftm0::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ftm0::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ftm0::Enabled
    }
}
#[doc = "Field `FTM0` writer - FTM0 Clock Gate Control"]
pub type Ftm0W<'a, REG> = crate::BitWriter<'a, REG, Ftm0>;
impl<'a, REG> Ftm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ftm0::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ftm0::Enabled)
    }
}
#[doc = "FTM1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ftm1 {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Ftm1> for bool {
    #[inline(always)]
    fn from(variant: Ftm1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM1` reader - FTM1 Clock Gate Control"]
pub type Ftm1R = crate::BitReader<Ftm1>;
impl Ftm1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ftm1 {
        match self.bits {
            false => Ftm1::Disabled,
            true => Ftm1::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ftm1::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ftm1::Enabled
    }
}
#[doc = "Field `FTM1` writer - FTM1 Clock Gate Control"]
pub type Ftm1W<'a, REG> = crate::BitWriter<'a, REG, Ftm1>;
impl<'a, REG> Ftm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ftm1::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ftm1::Enabled)
    }
}
#[doc = "ADC0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0 {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Adc0> for bool {
    #[inline(always)]
    fn from(variant: Adc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0` reader - ADC0 Clock Gate Control"]
pub type Adc0R = crate::BitReader<Adc0>;
impl Adc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc0 {
        match self.bits {
            false => Adc0::Disabled,
            true => Adc0::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Adc0::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Adc0::Enabled
    }
}
#[doc = "Field `ADC0` writer - ADC0 Clock Gate Control"]
pub type Adc0W<'a, REG> = crate::BitWriter<'a, REG, Adc0>;
impl<'a, REG> Adc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::Enabled)
    }
}
#[doc = "RTC Access Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtc {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Rtc> for bool {
    #[inline(always)]
    fn from(variant: Rtc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC` reader - RTC Access Control"]
pub type RtcR = crate::BitReader<Rtc>;
impl RtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtc {
        match self.bits {
            false => Rtc::Disabled,
            true => Rtc::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rtc::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rtc::Enabled
    }
}
#[doc = "Field `RTC` writer - RTC Access Control"]
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG, Rtc>;
impl<'a, REG> RtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Flash Memory Clock Gate Control"]
    #[inline(always)]
    pub fn ftfl(&self) -> FtflR {
        FtflR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Mux Clock Gate Control"]
    #[inline(always)]
    pub fn dmamux(&self) -> DmamuxR {
        DmamuxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 Clock Gate Control"]
    #[inline(always)]
    pub fn spi0(&self) -> Spi0R {
        Spi0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - I2S Clock Gate Control"]
    #[inline(always)]
    pub fn i2s(&self) -> I2sR {
        I2sR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - CRC Clock Gate Control"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - USB DCD Clock Gate Control"]
    #[inline(always)]
    pub fn usbdcd(&self) -> UsbdcdR {
        UsbdcdR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PDB Clock Gate Control"]
    #[inline(always)]
    pub fn pdb(&self) -> PdbR {
        PdbR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PIT Clock Gate Control"]
    #[inline(always)]
    pub fn pit(&self) -> PitR {
        PitR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - FTM0 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm0(&self) -> Ftm0R {
        Ftm0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FTM1 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm1(&self) -> Ftm1R {
        Ftm1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - ADC0 Clock Gate Control"]
    #[inline(always)]
    pub fn adc0(&self) -> Adc0R {
        Adc0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - RTC Access Control"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Memory Clock Gate Control"]
    #[inline(always)]
    pub fn ftfl(&mut self) -> FtflW<'_, Scgc6Spec> {
        FtflW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Mux Clock Gate Control"]
    #[inline(always)]
    pub fn dmamux(&mut self) -> DmamuxW<'_, Scgc6Spec> {
        DmamuxW::new(self, 1)
    }
    #[doc = "Bit 12 - SPI0 Clock Gate Control"]
    #[inline(always)]
    pub fn spi0(&mut self) -> Spi0W<'_, Scgc6Spec> {
        Spi0W::new(self, 12)
    }
    #[doc = "Bit 15 - I2S Clock Gate Control"]
    #[inline(always)]
    pub fn i2s(&mut self) -> I2sW<'_, Scgc6Spec> {
        I2sW::new(self, 15)
    }
    #[doc = "Bit 18 - CRC Clock Gate Control"]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<'_, Scgc6Spec> {
        CrcW::new(self, 18)
    }
    #[doc = "Bit 21 - USB DCD Clock Gate Control"]
    #[inline(always)]
    pub fn usbdcd(&mut self) -> UsbdcdW<'_, Scgc6Spec> {
        UsbdcdW::new(self, 21)
    }
    #[doc = "Bit 22 - PDB Clock Gate Control"]
    #[inline(always)]
    pub fn pdb(&mut self) -> PdbW<'_, Scgc6Spec> {
        PdbW::new(self, 22)
    }
    #[doc = "Bit 23 - PIT Clock Gate Control"]
    #[inline(always)]
    pub fn pit(&mut self) -> PitW<'_, Scgc6Spec> {
        PitW::new(self, 23)
    }
    #[doc = "Bit 24 - FTM0 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm0(&mut self) -> Ftm0W<'_, Scgc6Spec> {
        Ftm0W::new(self, 24)
    }
    #[doc = "Bit 25 - FTM1 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm1(&mut self) -> Ftm1W<'_, Scgc6Spec> {
        Ftm1W::new(self, 25)
    }
    #[doc = "Bit 27 - ADC0 Clock Gate Control"]
    #[inline(always)]
    pub fn adc0(&mut self) -> Adc0W<'_, Scgc6Spec> {
        Adc0W::new(self, 27)
    }
    #[doc = "Bit 29 - RTC Access Control"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<'_, Scgc6Spec> {
        RtcW::new(self, 29)
    }
}
#[doc = "System Clock Gating Control Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scgc6Spec;
impl crate::RegisterSpec for Scgc6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgc6::R`](R) reader structure"]
impl crate::Readable for Scgc6Spec {}
#[doc = "`write(|w| ..)` method takes [`scgc6::W`](W) writer structure"]
impl crate::Writable for Scgc6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCGC6 to value 0x4000_0001"]
impl crate::Resettable for Scgc6Spec {
    const RESET_VALUE: u32 = 0x4000_0001;
}

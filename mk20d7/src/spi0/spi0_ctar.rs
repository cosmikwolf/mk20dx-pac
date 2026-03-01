#[doc = "Register `CTAR%s` reader"]
pub type R = crate::R<Spi0CtarSpec>;
#[doc = "Register `CTAR%s` writer"]
pub type W = crate::W<Spi0CtarSpec>;
#[doc = "Field `BR` reader - Baud Rate Scaler"]
pub type BrR = crate::FieldReader;
#[doc = "Field `BR` writer - Baud Rate Scaler"]
pub type BrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DT` reader - Delay After Transfer Scaler"]
pub type DtR = crate::FieldReader;
#[doc = "Field `DT` writer - Delay After Transfer Scaler"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ASC` reader - After SCK Delay Scaler"]
pub type AscR = crate::FieldReader;
#[doc = "Field `ASC` writer - After SCK Delay Scaler"]
pub type AscW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSSCK` reader - PCS to SCK Delay Scaler"]
pub type CssckR = crate::FieldReader;
#[doc = "Field `CSSCK` writer - PCS to SCK Delay Scaler"]
pub type CssckW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Baud Rate Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pbr {
    #[doc = "0: Baud rate prescaler 2"]
    Div2 = 0,
    #[doc = "1: Baud rate prescaler 3"]
    Div3 = 1,
    #[doc = "2: Baud rate prescaler 5"]
    Div5 = 2,
    #[doc = "3: Baud rate prescaler 7"]
    Div7 = 3,
}
impl From<Pbr> for u8 {
    #[inline(always)]
    fn from(variant: Pbr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pbr {
    type Ux = u8;
}
impl crate::IsEnum for Pbr {}
#[doc = "Field `PBR` reader - Baud Rate Prescaler"]
pub type PbrR = crate::FieldReader<Pbr>;
impl PbrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbr {
        match self.bits {
            0 => Pbr::Div2,
            1 => Pbr::Div3,
            2 => Pbr::Div5,
            3 => Pbr::Div7,
            _ => unreachable!(),
        }
    }
    #[doc = "Baud rate prescaler 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Pbr::Div2
    }
    #[doc = "Baud rate prescaler 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Pbr::Div3
    }
    #[doc = "Baud rate prescaler 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Pbr::Div5
    }
    #[doc = "Baud rate prescaler 7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == Pbr::Div7
    }
}
#[doc = "Field `PBR` writer - Baud Rate Prescaler"]
pub type PbrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pbr, crate::Safe>;
impl<'a, REG> PbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Baud rate prescaler 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Pbr::Div2)
    }
    #[doc = "Baud rate prescaler 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Pbr::Div3)
    }
    #[doc = "Baud rate prescaler 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Pbr::Div5)
    }
    #[doc = "Baud rate prescaler 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(Pbr::Div7)
    }
}
#[doc = "Delay after Transfer Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pdt {
    #[doc = "0: Delay prescaler 1"]
    Scale1 = 0,
    #[doc = "1: Delay prescaler 3"]
    Scale3 = 1,
    #[doc = "2: Delay prescaler 5"]
    Scale5 = 2,
    #[doc = "3: Delay prescaler 7"]
    Scale7 = 3,
}
impl From<Pdt> for u8 {
    #[inline(always)]
    fn from(variant: Pdt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pdt {
    type Ux = u8;
}
impl crate::IsEnum for Pdt {}
#[doc = "Field `PDT` reader - Delay after Transfer Prescaler"]
pub type PdtR = crate::FieldReader<Pdt>;
impl PdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdt {
        match self.bits {
            0 => Pdt::Scale1,
            1 => Pdt::Scale3,
            2 => Pdt::Scale5,
            3 => Pdt::Scale7,
            _ => unreachable!(),
        }
    }
    #[doc = "Delay prescaler 1"]
    #[inline(always)]
    pub fn is_scale1(&self) -> bool {
        *self == Pdt::Scale1
    }
    #[doc = "Delay prescaler 3"]
    #[inline(always)]
    pub fn is_scale3(&self) -> bool {
        *self == Pdt::Scale3
    }
    #[doc = "Delay prescaler 5"]
    #[inline(always)]
    pub fn is_scale5(&self) -> bool {
        *self == Pdt::Scale5
    }
    #[doc = "Delay prescaler 7"]
    #[inline(always)]
    pub fn is_scale7(&self) -> bool {
        *self == Pdt::Scale7
    }
}
#[doc = "Field `PDT` writer - Delay after Transfer Prescaler"]
pub type PdtW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pdt, crate::Safe>;
impl<'a, REG> PdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Delay prescaler 1"]
    #[inline(always)]
    pub fn scale1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdt::Scale1)
    }
    #[doc = "Delay prescaler 3"]
    #[inline(always)]
    pub fn scale3(self) -> &'a mut crate::W<REG> {
        self.variant(Pdt::Scale3)
    }
    #[doc = "Delay prescaler 5"]
    #[inline(always)]
    pub fn scale5(self) -> &'a mut crate::W<REG> {
        self.variant(Pdt::Scale5)
    }
    #[doc = "Delay prescaler 7"]
    #[inline(always)]
    pub fn scale7(self) -> &'a mut crate::W<REG> {
        self.variant(Pdt::Scale7)
    }
}
#[doc = "After SCK Delay Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pasc {
    #[doc = "0: After SCK delay prescaler 1"]
    Scale1 = 0,
    #[doc = "1: After SCK delay prescaler 3"]
    Scale3 = 1,
    #[doc = "2: After SCK delay prescaler 5"]
    Scale5 = 2,
    #[doc = "3: After SCK delay prescaler 7"]
    Scale7 = 3,
}
impl From<Pasc> for u8 {
    #[inline(always)]
    fn from(variant: Pasc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pasc {
    type Ux = u8;
}
impl crate::IsEnum for Pasc {}
#[doc = "Field `PASC` reader - After SCK Delay Prescaler"]
pub type PascR = crate::FieldReader<Pasc>;
impl PascR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pasc {
        match self.bits {
            0 => Pasc::Scale1,
            1 => Pasc::Scale3,
            2 => Pasc::Scale5,
            3 => Pasc::Scale7,
            _ => unreachable!(),
        }
    }
    #[doc = "After SCK delay prescaler 1"]
    #[inline(always)]
    pub fn is_scale1(&self) -> bool {
        *self == Pasc::Scale1
    }
    #[doc = "After SCK delay prescaler 3"]
    #[inline(always)]
    pub fn is_scale3(&self) -> bool {
        *self == Pasc::Scale3
    }
    #[doc = "After SCK delay prescaler 5"]
    #[inline(always)]
    pub fn is_scale5(&self) -> bool {
        *self == Pasc::Scale5
    }
    #[doc = "After SCK delay prescaler 7"]
    #[inline(always)]
    pub fn is_scale7(&self) -> bool {
        *self == Pasc::Scale7
    }
}
#[doc = "Field `PASC` writer - After SCK Delay Prescaler"]
pub type PascW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pasc, crate::Safe>;
impl<'a, REG> PascW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "After SCK delay prescaler 1"]
    #[inline(always)]
    pub fn scale1(self) -> &'a mut crate::W<REG> {
        self.variant(Pasc::Scale1)
    }
    #[doc = "After SCK delay prescaler 3"]
    #[inline(always)]
    pub fn scale3(self) -> &'a mut crate::W<REG> {
        self.variant(Pasc::Scale3)
    }
    #[doc = "After SCK delay prescaler 5"]
    #[inline(always)]
    pub fn scale5(self) -> &'a mut crate::W<REG> {
        self.variant(Pasc::Scale5)
    }
    #[doc = "After SCK delay prescaler 7"]
    #[inline(always)]
    pub fn scale7(self) -> &'a mut crate::W<REG> {
        self.variant(Pasc::Scale7)
    }
}
#[doc = "PCS to SCK Delay Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcssck {
    #[doc = "0: PCS to SCK delay prescaler 1"]
    Scale1 = 0,
    #[doc = "1: PCS to SCK delay prescaler 3"]
    Scale3 = 1,
    #[doc = "2: PCS to SCK delay prescaler 5"]
    Scale5 = 2,
    #[doc = "3: PCS to SCK delay prescaler 7"]
    Scale7 = 3,
}
impl From<Pcssck> for u8 {
    #[inline(always)]
    fn from(variant: Pcssck) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcssck {
    type Ux = u8;
}
impl crate::IsEnum for Pcssck {}
#[doc = "Field `PCSSCK` reader - PCS to SCK Delay Prescaler"]
pub type PcssckR = crate::FieldReader<Pcssck>;
impl PcssckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcssck {
        match self.bits {
            0 => Pcssck::Scale1,
            1 => Pcssck::Scale3,
            2 => Pcssck::Scale5,
            3 => Pcssck::Scale7,
            _ => unreachable!(),
        }
    }
    #[doc = "PCS to SCK delay prescaler 1"]
    #[inline(always)]
    pub fn is_scale1(&self) -> bool {
        *self == Pcssck::Scale1
    }
    #[doc = "PCS to SCK delay prescaler 3"]
    #[inline(always)]
    pub fn is_scale3(&self) -> bool {
        *self == Pcssck::Scale3
    }
    #[doc = "PCS to SCK delay prescaler 5"]
    #[inline(always)]
    pub fn is_scale5(&self) -> bool {
        *self == Pcssck::Scale5
    }
    #[doc = "PCS to SCK delay prescaler 7"]
    #[inline(always)]
    pub fn is_scale7(&self) -> bool {
        *self == Pcssck::Scale7
    }
}
#[doc = "Field `PCSSCK` writer - PCS to SCK Delay Prescaler"]
pub type PcssckW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pcssck, crate::Safe>;
impl<'a, REG> PcssckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCS to SCK delay prescaler 1"]
    #[inline(always)]
    pub fn scale1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcssck::Scale1)
    }
    #[doc = "PCS to SCK delay prescaler 3"]
    #[inline(always)]
    pub fn scale3(self) -> &'a mut crate::W<REG> {
        self.variant(Pcssck::Scale3)
    }
    #[doc = "PCS to SCK delay prescaler 5"]
    #[inline(always)]
    pub fn scale5(self) -> &'a mut crate::W<REG> {
        self.variant(Pcssck::Scale5)
    }
    #[doc = "PCS to SCK delay prescaler 7"]
    #[inline(always)]
    pub fn scale7(self) -> &'a mut crate::W<REG> {
        self.variant(Pcssck::Scale7)
    }
}
#[doc = "LBS First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsbfe {
    #[doc = "0: Data transferred MSB first"]
    MsbFirst = 0,
    #[doc = "1: Data transferred LSB first"]
    LsbFirst = 1,
}
impl From<Lsbfe> for bool {
    #[inline(always)]
    fn from(variant: Lsbfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSBFE` reader - LBS First"]
pub type LsbfeR = crate::BitReader<Lsbfe>;
impl LsbfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsbfe {
        match self.bits {
            false => Lsbfe::MsbFirst,
            true => Lsbfe::LsbFirst,
        }
    }
    #[doc = "Data transferred MSB first"]
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        *self == Lsbfe::MsbFirst
    }
    #[doc = "Data transferred LSB first"]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == Lsbfe::LsbFirst
    }
}
#[doc = "Field `LSBFE` writer - LBS First"]
pub type LsbfeW<'a, REG> = crate::BitWriter<'a, REG, Lsbfe>;
impl<'a, REG> LsbfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data transferred MSB first"]
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbfe::MsbFirst)
    }
    #[doc = "Data transferred LSB first"]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbfe::LsbFirst)
    }
}
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpha {
    #[doc = "0: Data captured on leading edge, changed on following edge"]
    CaptureLeading = 0,
    #[doc = "1: Data changed on leading edge, captured on following edge"]
    CaptureFollowing = 1,
}
impl From<Cpha> for bool {
    #[inline(always)]
    fn from(variant: Cpha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock Phase"]
pub type CphaR = crate::BitReader<Cpha>;
impl CphaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpha {
        match self.bits {
            false => Cpha::CaptureLeading,
            true => Cpha::CaptureFollowing,
        }
    }
    #[doc = "Data captured on leading edge, changed on following edge"]
    #[inline(always)]
    pub fn is_capture_leading(&self) -> bool {
        *self == Cpha::CaptureLeading
    }
    #[doc = "Data changed on leading edge, captured on following edge"]
    #[inline(always)]
    pub fn is_capture_following(&self) -> bool {
        *self == Cpha::CaptureFollowing
    }
}
#[doc = "Field `CPHA` writer - Clock Phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG, Cpha>;
impl<'a, REG> CphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data captured on leading edge, changed on following edge"]
    #[inline(always)]
    pub fn capture_leading(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::CaptureLeading)
    }
    #[doc = "Data changed on leading edge, captured on following edge"]
    #[inline(always)]
    pub fn capture_following(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::CaptureFollowing)
    }
}
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpol {
    #[doc = "0: Inactive state of SCK is low"]
    IdleLow = 0,
    #[doc = "1: Inactive state of SCK is high"]
    IdleHigh = 1,
}
impl From<Cpol> for bool {
    #[inline(always)]
    fn from(variant: Cpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CpolR = crate::BitReader<Cpol>;
impl CpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpol {
        match self.bits {
            false => Cpol::IdleLow,
            true => Cpol::IdleHigh,
        }
    }
    #[doc = "Inactive state of SCK is low"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == Cpol::IdleLow
    }
    #[doc = "Inactive state of SCK is high"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == Cpol::IdleHigh
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, Cpol>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inactive state of SCK is low"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::IdleLow)
    }
    #[doc = "Inactive state of SCK is high"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::IdleHigh)
    }
}
#[doc = "Field `FMSZ` reader - Frame Size"]
pub type FmszR = crate::FieldReader;
#[doc = "Field `FMSZ` writer - Frame Size"]
pub type FmszW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Double Baud Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbr {
    #[doc = "0: Normal baud rate"]
    Normal = 0,
    #[doc = "1: Double baud rate"]
    Double = 1,
}
impl From<Dbr> for bool {
    #[inline(always)]
    fn from(variant: Dbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBR` reader - Double Baud Rate"]
pub type DbrR = crate::BitReader<Dbr>;
impl DbrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbr {
        match self.bits {
            false => Dbr::Normal,
            true => Dbr::Double,
        }
    }
    #[doc = "Normal baud rate"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Dbr::Normal
    }
    #[doc = "Double baud rate"]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == Dbr::Double
    }
}
#[doc = "Field `DBR` writer - Double Baud Rate"]
pub type DbrW<'a, REG> = crate::BitWriter<'a, REG, Dbr>;
impl<'a, REG> DbrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal baud rate"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Dbr::Normal)
    }
    #[doc = "Double baud rate"]
    #[inline(always)]
    pub fn double(self) -> &'a mut crate::W<REG> {
        self.variant(Dbr::Double)
    }
}
impl R {
    #[doc = "Bits 0:3 - Baud Rate Scaler"]
    #[inline(always)]
    pub fn br(&self) -> BrR {
        BrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Delay After Transfer Scaler"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - After SCK Delay Scaler"]
    #[inline(always)]
    pub fn asc(&self) -> AscR {
        AscR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - PCS to SCK Delay Scaler"]
    #[inline(always)]
    pub fn cssck(&self) -> CssckR {
        CssckR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn pbr(&self) -> PbrR {
        PbrR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Delay after Transfer Prescaler"]
    #[inline(always)]
    pub fn pdt(&self) -> PdtR {
        PdtR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - After SCK Delay Prescaler"]
    #[inline(always)]
    pub fn pasc(&self) -> PascR {
        PascR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PCS to SCK Delay Prescaler"]
    #[inline(always)]
    pub fn pcssck(&self) -> PcssckR {
        PcssckR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - LBS First"]
    #[inline(always)]
    pub fn lsbfe(&self) -> LsbfeR {
        LsbfeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:30 - Frame Size"]
    #[inline(always)]
    pub fn fmsz(&self) -> FmszR {
        FmszR::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Double Baud Rate"]
    #[inline(always)]
    pub fn dbr(&self) -> DbrR {
        DbrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Baud Rate Scaler"]
    #[inline(always)]
    pub fn br(&mut self) -> BrW<'_, Spi0CtarSpec> {
        BrW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Delay After Transfer Scaler"]
    #[inline(always)]
    pub fn dt(&mut self) -> DtW<'_, Spi0CtarSpec> {
        DtW::new(self, 4)
    }
    #[doc = "Bits 8:11 - After SCK Delay Scaler"]
    #[inline(always)]
    pub fn asc(&mut self) -> AscW<'_, Spi0CtarSpec> {
        AscW::new(self, 8)
    }
    #[doc = "Bits 12:15 - PCS to SCK Delay Scaler"]
    #[inline(always)]
    pub fn cssck(&mut self) -> CssckW<'_, Spi0CtarSpec> {
        CssckW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn pbr(&mut self) -> PbrW<'_, Spi0CtarSpec> {
        PbrW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Delay after Transfer Prescaler"]
    #[inline(always)]
    pub fn pdt(&mut self) -> PdtW<'_, Spi0CtarSpec> {
        PdtW::new(self, 18)
    }
    #[doc = "Bits 20:21 - After SCK Delay Prescaler"]
    #[inline(always)]
    pub fn pasc(&mut self) -> PascW<'_, Spi0CtarSpec> {
        PascW::new(self, 20)
    }
    #[doc = "Bits 22:23 - PCS to SCK Delay Prescaler"]
    #[inline(always)]
    pub fn pcssck(&mut self) -> PcssckW<'_, Spi0CtarSpec> {
        PcssckW::new(self, 22)
    }
    #[doc = "Bit 24 - LBS First"]
    #[inline(always)]
    pub fn lsbfe(&mut self) -> LsbfeW<'_, Spi0CtarSpec> {
        LsbfeW::new(self, 24)
    }
    #[doc = "Bit 25 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, Spi0CtarSpec> {
        CphaW::new(self, 25)
    }
    #[doc = "Bit 26 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, Spi0CtarSpec> {
        CpolW::new(self, 26)
    }
    #[doc = "Bits 27:30 - Frame Size"]
    #[inline(always)]
    pub fn fmsz(&mut self) -> FmszW<'_, Spi0CtarSpec> {
        FmszW::new(self, 27)
    }
    #[doc = "Bit 31 - Double Baud Rate"]
    #[inline(always)]
    pub fn dbr(&mut self) -> DbrW<'_, Spi0CtarSpec> {
        DbrW::new(self, 31)
    }
}
#[doc = "DSPI Clock and Transfer Attributes Register (In Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0_ctar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0_ctar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0CtarSpec;
impl crate::RegisterSpec for Spi0CtarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi0_ctar::R`](R) reader structure"]
impl crate::Readable for Spi0CtarSpec {}
#[doc = "`write(|w| ..)` method takes [`spi0_ctar::W`](W) writer structure"]
impl crate::Writable for Spi0CtarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTAR%s to value 0x7800_0000"]
impl crate::Resettable for Spi0CtarSpec {
    const RESET_VALUE: u32 = 0x7800_0000;
}

#[doc = "Register `C1` reader"]
pub type R = crate::R<C1Spec>;
#[doc = "Register `C1` writer"]
pub type W = crate::W<C1Spec>;
#[doc = "Internal Reference Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irefsten {
    #[doc = "0: Internal reference clock is disabled in Stop mode"]
    Disabled = 0,
    #[doc = "1: Internal reference clock stays enabled in Stop mode"]
    Enabled = 1,
}
impl From<Irefsten> for bool {
    #[inline(always)]
    fn from(variant: Irefsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREFSTEN` reader - Internal Reference Stop Enable"]
pub type IrefstenR = crate::BitReader<Irefsten>;
impl IrefstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irefsten {
        match self.bits {
            false => Irefsten::Disabled,
            true => Irefsten::Enabled,
        }
    }
    #[doc = "Internal reference clock is disabled in Stop mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Irefsten::Disabled
    }
    #[doc = "Internal reference clock stays enabled in Stop mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Irefsten::Enabled
    }
}
#[doc = "Field `IREFSTEN` writer - Internal Reference Stop Enable"]
pub type IrefstenW<'a, REG> = crate::BitWriter<'a, REG, Irefsten>;
impl<'a, REG> IrefstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal reference clock is disabled in Stop mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Irefsten::Disabled)
    }
    #[doc = "Internal reference clock stays enabled in Stop mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Irefsten::Enabled)
    }
}
#[doc = "Internal Reference Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irclken {
    #[doc = "0: MCGIRCLK inactive"]
    Inactive = 0,
    #[doc = "1: MCGIRCLK active"]
    Active = 1,
}
impl From<Irclken> for bool {
    #[inline(always)]
    fn from(variant: Irclken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRCLKEN` reader - Internal Reference Clock Enable"]
pub type IrclkenR = crate::BitReader<Irclken>;
impl IrclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irclken {
        match self.bits {
            false => Irclken::Inactive,
            true => Irclken::Active,
        }
    }
    #[doc = "MCGIRCLK inactive"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Irclken::Inactive
    }
    #[doc = "MCGIRCLK active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Irclken::Active
    }
}
#[doc = "Field `IRCLKEN` writer - Internal Reference Clock Enable"]
pub type IrclkenW<'a, REG> = crate::BitWriter<'a, REG, Irclken>;
impl<'a, REG> IrclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCGIRCLK inactive"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Irclken::Inactive)
    }
    #[doc = "MCGIRCLK active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Irclken::Active)
    }
}
#[doc = "Internal Reference Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irefs {
    #[doc = "0: External reference clock is selected"]
    External = 0,
    #[doc = "1: The slow internal reference clock is selected"]
    Internal = 1,
}
impl From<Irefs> for bool {
    #[inline(always)]
    fn from(variant: Irefs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREFS` reader - Internal Reference Select"]
pub type IrefsR = crate::BitReader<Irefs>;
impl IrefsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irefs {
        match self.bits {
            false => Irefs::External,
            true => Irefs::Internal,
        }
    }
    #[doc = "External reference clock is selected"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Irefs::External
    }
    #[doc = "The slow internal reference clock is selected"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Irefs::Internal
    }
}
#[doc = "Field `IREFS` writer - Internal Reference Select"]
pub type IrefsW<'a, REG> = crate::BitWriter<'a, REG, Irefs>;
impl<'a, REG> IrefsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External reference clock is selected"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(Irefs::External)
    }
    #[doc = "The slow internal reference clock is selected"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(Irefs::Internal)
    }
}
#[doc = "FLL External Reference Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frdiv {
    #[doc = "0: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 1; for all other RANGE 0 values, Divide Factor is 32."]
    _000 = 0,
    #[doc = "1: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 2; for all other RANGE 0 values, Divide Factor is 64."]
    _001 = 1,
    #[doc = "2: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 4; for all other RANGE 0 values, Divide Factor is 128."]
    _010 = 2,
    #[doc = "3: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 8; for all other RANGE 0 values, Divide Factor is 256."]
    _011 = 3,
    #[doc = "4: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 16; for all other RANGE 0 values, Divide Factor is 512."]
    _100 = 4,
    #[doc = "5: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 32; for all other RANGE 0 values, Divide Factor is 1024."]
    _101 = 5,
    #[doc = "6: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 64; for all other RANGE 0 values, Divide Factor is 1280 ."]
    _110 = 6,
    #[doc = "7: If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 128; for all other RANGE 0 values, Divide Factor is 1536 ."]
    _111 = 7,
}
impl From<Frdiv> for u8 {
    #[inline(always)]
    fn from(variant: Frdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frdiv {
    type Ux = u8;
}
impl crate::IsEnum for Frdiv {}
#[doc = "Field `FRDIV` reader - FLL External Reference Divider"]
pub type FrdivR = crate::FieldReader<Frdiv>;
impl FrdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frdiv {
        match self.bits {
            0 => Frdiv::_000,
            1 => Frdiv::_001,
            2 => Frdiv::_010,
            3 => Frdiv::_011,
            4 => Frdiv::_100,
            5 => Frdiv::_101,
            6 => Frdiv::_110,
            7 => Frdiv::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 1; for all other RANGE 0 values, Divide Factor is 32."]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == Frdiv::_000
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 2; for all other RANGE 0 values, Divide Factor is 64."]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == Frdiv::_001
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 4; for all other RANGE 0 values, Divide Factor is 128."]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == Frdiv::_010
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 8; for all other RANGE 0 values, Divide Factor is 256."]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == Frdiv::_011
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 16; for all other RANGE 0 values, Divide Factor is 512."]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Frdiv::_100
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 32; for all other RANGE 0 values, Divide Factor is 1024."]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == Frdiv::_101
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 64; for all other RANGE 0 values, Divide Factor is 1280 ."]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == Frdiv::_110
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 128; for all other RANGE 0 values, Divide Factor is 1536 ."]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == Frdiv::_111
    }
}
#[doc = "Field `FRDIV` writer - FLL External Reference Divider"]
pub type FrdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Frdiv, crate::Safe>;
impl<'a, REG> FrdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 1; for all other RANGE 0 values, Divide Factor is 32."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_000)
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 2; for all other RANGE 0 values, Divide Factor is 64."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_001)
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 4; for all other RANGE 0 values, Divide Factor is 128."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_010)
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 8; for all other RANGE 0 values, Divide Factor is 256."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_011)
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 16; for all other RANGE 0 values, Divide Factor is 512."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_100)
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 32; for all other RANGE 0 values, Divide Factor is 1024."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_101)
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 64; for all other RANGE 0 values, Divide Factor is 1280 ."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_110)
    }
    #[doc = "If RANGE 0 = 0 or OSCSEL=1 , Divide Factor is 128; for all other RANGE 0 values, Divide Factor is 1536 ."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(Frdiv::_111)
    }
}
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clks {
    #[doc = "0: Output of FLL or PLL (depends on PLLS)"]
    FllPll = 0,
    #[doc = "1: Internal reference clock"]
    Internal = 1,
    #[doc = "2: External reference clock"]
    External = 2,
}
impl From<Clks> for u8 {
    #[inline(always)]
    fn from(variant: Clks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clks {
    type Ux = u8;
}
impl crate::IsEnum for Clks {}
#[doc = "Field `CLKS` reader - Clock Source Select"]
pub type ClksR = crate::FieldReader<Clks>;
impl ClksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clks {
        match self.bits {
            0 => Clks::FllPll,
            1 => Clks::Internal,
            2 => Clks::External,
            _ => unreachable!(),
        }
    }
    #[doc = "Output of FLL or PLL (depends on PLLS)"]
    #[inline(always)]
    pub fn is_fll_pll(&self) -> bool {
        *self == Clks::FllPll
    }
    #[doc = "Internal reference clock"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Clks::Internal
    }
    #[doc = "External reference clock"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Clks::External
    }
}
#[doc = "Field `CLKS` writer - Clock Source Select"]
pub type ClksW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clks>;
impl<'a, REG> ClksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output of FLL or PLL (depends on PLLS)"]
    #[inline(always)]
    pub fn fll_pll(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::FllPll)
    }
    #[doc = "Internal reference clock"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::Internal)
    }
    #[doc = "External reference clock"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::External)
    }
}
impl R {
    #[doc = "Bit 0 - Internal Reference Stop Enable"]
    #[inline(always)]
    pub fn irefsten(&self) -> IrefstenR {
        IrefstenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Reference Clock Enable"]
    #[inline(always)]
    pub fn irclken(&self) -> IrclkenR {
        IrclkenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Internal Reference Select"]
    #[inline(always)]
    pub fn irefs(&self) -> IrefsR {
        IrefsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FLL External Reference Divider"]
    #[inline(always)]
    pub fn frdiv(&self) -> FrdivR {
        FrdivR::new((self.bits >> 3) & 7)
    }
    #[doc = "Bits 6:7 - Clock Source Select"]
    #[inline(always)]
    pub fn clks(&self) -> ClksR {
        ClksR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Stop Enable"]
    #[inline(always)]
    pub fn irefsten(&mut self) -> IrefstenW<'_, C1Spec> {
        IrefstenW::new(self, 0)
    }
    #[doc = "Bit 1 - Internal Reference Clock Enable"]
    #[inline(always)]
    pub fn irclken(&mut self) -> IrclkenW<'_, C1Spec> {
        IrclkenW::new(self, 1)
    }
    #[doc = "Bit 2 - Internal Reference Select"]
    #[inline(always)]
    pub fn irefs(&mut self) -> IrefsW<'_, C1Spec> {
        IrefsW::new(self, 2)
    }
    #[doc = "Bits 3:5 - FLL External Reference Divider"]
    #[inline(always)]
    pub fn frdiv(&mut self) -> FrdivW<'_, C1Spec> {
        FrdivW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Clock Source Select"]
    #[inline(always)]
    pub fn clks(&mut self) -> ClksW<'_, C1Spec> {
        ClksW::new(self, 6)
    }
}
#[doc = "MCG Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets C1 to value 0x04"]
impl crate::Resettable for C1Spec {
    const RESET_VALUE: u8 = 0x04;
}

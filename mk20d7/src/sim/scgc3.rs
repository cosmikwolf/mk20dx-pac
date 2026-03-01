#[doc = "Register `SCGC3` reader"]
pub type R = crate::R<Scgc3Spec>;
#[doc = "Register `SCGC3` writer"]
pub type W = crate::W<Scgc3Spec>;
#[doc = "FTM2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ftm2 {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Ftm2> for bool {
    #[inline(always)]
    fn from(variant: Ftm2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM2` reader - FTM2 Clock Gate Control"]
pub type Ftm2R = crate::BitReader<Ftm2>;
impl Ftm2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ftm2 {
        match self.bits {
            false => Ftm2::Disabled,
            true => Ftm2::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ftm2::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ftm2::Enabled
    }
}
#[doc = "Field `FTM2` writer - FTM2 Clock Gate Control"]
pub type Ftm2W<'a, REG> = crate::BitWriter<'a, REG, Ftm2>;
impl<'a, REG> Ftm2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ftm2::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ftm2::Enabled)
    }
}
#[doc = "ADC1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc1 {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Adc1> for bool {
    #[inline(always)]
    fn from(variant: Adc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC1` reader - ADC1 Clock Gate Control"]
pub type Adc1R = crate::BitReader<Adc1>;
impl Adc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc1 {
        match self.bits {
            false => Adc1::Disabled,
            true => Adc1::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Adc1::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Adc1::Enabled
    }
}
#[doc = "Field `ADC1` writer - ADC1 Clock Gate Control"]
pub type Adc1W<'a, REG> = crate::BitWriter<'a, REG, Adc1>;
impl<'a, REG> Adc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adc1::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Adc1::Enabled)
    }
}
impl R {
    #[doc = "Bit 24 - FTM2 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm2(&self) -> Ftm2R {
        Ftm2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - ADC1 Clock Gate Control"]
    #[inline(always)]
    pub fn adc1(&self) -> Adc1R {
        Adc1R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - FTM2 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm2(&mut self) -> Ftm2W<'_, Scgc3Spec> {
        Ftm2W::new(self, 24)
    }
    #[doc = "Bit 27 - ADC1 Clock Gate Control"]
    #[inline(always)]
    pub fn adc1(&mut self) -> Adc1W<'_, Scgc3Spec> {
        Adc1W::new(self, 27)
    }
}
#[doc = "System Clock Gating Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scgc3Spec;
impl crate::RegisterSpec for Scgc3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgc3::R`](R) reader structure"]
impl crate::Readable for Scgc3Spec {}
#[doc = "`write(|w| ..)` method takes [`scgc3::W`](W) writer structure"]
impl crate::Writable for Scgc3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCGC3 to value 0"]
impl crate::Resettable for Scgc3Spec {}

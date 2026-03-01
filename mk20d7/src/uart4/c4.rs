#[doc = "Register `C4` reader"]
pub type R = crate::R<C4Spec>;
#[doc = "Register `C4` writer"]
pub type W = crate::W<C4Spec>;
#[doc = "Field `BRFA` reader - Baud Rate Fine Adjust"]
pub type BrfaR = crate::FieldReader;
#[doc = "Field `BRFA` writer - Baud Rate Fine Adjust"]
pub type BrfaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "10-bit Mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M10 {
    #[doc = "0: 10-bit mode disabled"]
    Disabled = 0,
    #[doc = "1: 10-bit mode enabled"]
    Enabled = 1,
}
impl From<M10> for bool {
    #[inline(always)]
    fn from(variant: M10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M10` reader - 10-bit Mode select"]
pub type M10R = crate::BitReader<M10>;
impl M10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M10 {
        match self.bits {
            false => M10::Disabled,
            true => M10::Enabled,
        }
    }
    #[doc = "10-bit mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == M10::Disabled
    }
    #[doc = "10-bit mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == M10::Enabled
    }
}
#[doc = "Field `M10` writer - 10-bit Mode select"]
pub type M10W<'a, REG> = crate::BitWriter<'a, REG, M10>;
impl<'a, REG> M10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10-bit mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(M10::Disabled)
    }
    #[doc = "10-bit mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(M10::Enabled)
    }
}
#[doc = "Match Address Mode Enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Maen2 {
    #[doc = "0: Match address 2 disabled"]
    Disabled = 0,
    #[doc = "1: Match address 2 enabled"]
    Enabled = 1,
}
impl From<Maen2> for bool {
    #[inline(always)]
    fn from(variant: Maen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAEN2` reader - Match Address Mode Enable 2"]
pub type Maen2R = crate::BitReader<Maen2>;
impl Maen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Maen2 {
        match self.bits {
            false => Maen2::Disabled,
            true => Maen2::Enabled,
        }
    }
    #[doc = "Match address 2 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Maen2::Disabled
    }
    #[doc = "Match address 2 enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Maen2::Enabled
    }
}
#[doc = "Field `MAEN2` writer - Match Address Mode Enable 2"]
pub type Maen2W<'a, REG> = crate::BitWriter<'a, REG, Maen2>;
impl<'a, REG> Maen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match address 2 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Maen2::Disabled)
    }
    #[doc = "Match address 2 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Maen2::Enabled)
    }
}
#[doc = "Match Address Mode Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Maen1 {
    #[doc = "0: Match address 1 disabled"]
    Disabled = 0,
    #[doc = "1: Match address 1 enabled"]
    Enabled = 1,
}
impl From<Maen1> for bool {
    #[inline(always)]
    fn from(variant: Maen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAEN1` reader - Match Address Mode Enable 1"]
pub type Maen1R = crate::BitReader<Maen1>;
impl Maen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Maen1 {
        match self.bits {
            false => Maen1::Disabled,
            true => Maen1::Enabled,
        }
    }
    #[doc = "Match address 1 disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Maen1::Disabled
    }
    #[doc = "Match address 1 enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Maen1::Enabled
    }
}
#[doc = "Field `MAEN1` writer - Match Address Mode Enable 1"]
pub type Maen1W<'a, REG> = crate::BitWriter<'a, REG, Maen1>;
impl<'a, REG> Maen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Match address 1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Maen1::Disabled)
    }
    #[doc = "Match address 1 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Maen1::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:4 - Baud Rate Fine Adjust"]
    #[inline(always)]
    pub fn brfa(&self) -> BrfaR {
        BrfaR::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - 10-bit Mode select"]
    #[inline(always)]
    pub fn m10(&self) -> M10R {
        M10R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Match Address Mode Enable 2"]
    #[inline(always)]
    pub fn maen2(&self) -> Maen2R {
        Maen2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Match Address Mode Enable 1"]
    #[inline(always)]
    pub fn maen1(&self) -> Maen1R {
        Maen1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Baud Rate Fine Adjust"]
    #[inline(always)]
    pub fn brfa(&mut self) -> BrfaW<'_, C4Spec> {
        BrfaW::new(self, 0)
    }
    #[doc = "Bit 5 - 10-bit Mode select"]
    #[inline(always)]
    pub fn m10(&mut self) -> M10W<'_, C4Spec> {
        M10W::new(self, 5)
    }
    #[doc = "Bit 6 - Match Address Mode Enable 2"]
    #[inline(always)]
    pub fn maen2(&mut self) -> Maen2W<'_, C4Spec> {
        Maen2W::new(self, 6)
    }
    #[doc = "Bit 7 - Match Address Mode Enable 1"]
    #[inline(always)]
    pub fn maen1(&mut self) -> Maen1W<'_, C4Spec> {
        Maen1W::new(self, 7)
    }
}
#[doc = "UART Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C4Spec;
impl crate::RegisterSpec for C4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c4::R`](R) reader structure"]
impl crate::Readable for C4Spec {}
#[doc = "`write(|w| ..)` method takes [`c4::W`](W) writer structure"]
impl crate::Writable for C4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C4 to value 0"]
impl crate::Resettable for C4Spec {}

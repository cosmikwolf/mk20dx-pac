#[doc = "Register `INVCTRL` reader"]
pub type R = crate::R<InvctrlSpec>;
#[doc = "Register `INVCTRL` writer"]
pub type W = crate::W<InvctrlSpec>;
#[doc = "Pair Channels 0 Inverting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv0en {
    #[doc = "0: Pair output not inverted"]
    Disabled = 0,
    #[doc = "1: Pair output inverted"]
    Enabled = 1,
}
impl From<Inv0en> for bool {
    #[inline(always)]
    fn from(variant: Inv0en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV0EN` reader - Pair Channels 0 Inverting Enable"]
pub type Inv0enR = crate::BitReader<Inv0en>;
impl Inv0enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inv0en {
        match self.bits {
            false => Inv0en::Disabled,
            true => Inv0en::Enabled,
        }
    }
    #[doc = "Pair output not inverted"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Inv0en::Disabled
    }
    #[doc = "Pair output inverted"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Inv0en::Enabled
    }
}
#[doc = "Field `INV0EN` writer - Pair Channels 0 Inverting Enable"]
pub type Inv0enW<'a, REG> = crate::BitWriter<'a, REG, Inv0en>;
impl<'a, REG> Inv0enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pair output not inverted"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inv0en::Disabled)
    }
    #[doc = "Pair output inverted"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inv0en::Enabled)
    }
}
#[doc = "Pair Channels 1 Inverting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv1en {
    #[doc = "0: Pair output not inverted"]
    Disabled = 0,
    #[doc = "1: Pair output inverted"]
    Enabled = 1,
}
impl From<Inv1en> for bool {
    #[inline(always)]
    fn from(variant: Inv1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV1EN` reader - Pair Channels 1 Inverting Enable"]
pub type Inv1enR = crate::BitReader<Inv1en>;
impl Inv1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inv1en {
        match self.bits {
            false => Inv1en::Disabled,
            true => Inv1en::Enabled,
        }
    }
    #[doc = "Pair output not inverted"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Inv1en::Disabled
    }
    #[doc = "Pair output inverted"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Inv1en::Enabled
    }
}
#[doc = "Field `INV1EN` writer - Pair Channels 1 Inverting Enable"]
pub type Inv1enW<'a, REG> = crate::BitWriter<'a, REG, Inv1en>;
impl<'a, REG> Inv1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pair output not inverted"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inv1en::Disabled)
    }
    #[doc = "Pair output inverted"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inv1en::Enabled)
    }
}
#[doc = "Pair Channels 2 Inverting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv2en {
    #[doc = "0: Pair output not inverted"]
    Disabled = 0,
    #[doc = "1: Pair output inverted"]
    Enabled = 1,
}
impl From<Inv2en> for bool {
    #[inline(always)]
    fn from(variant: Inv2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV2EN` reader - Pair Channels 2 Inverting Enable"]
pub type Inv2enR = crate::BitReader<Inv2en>;
impl Inv2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inv2en {
        match self.bits {
            false => Inv2en::Disabled,
            true => Inv2en::Enabled,
        }
    }
    #[doc = "Pair output not inverted"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Inv2en::Disabled
    }
    #[doc = "Pair output inverted"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Inv2en::Enabled
    }
}
#[doc = "Field `INV2EN` writer - Pair Channels 2 Inverting Enable"]
pub type Inv2enW<'a, REG> = crate::BitWriter<'a, REG, Inv2en>;
impl<'a, REG> Inv2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pair output not inverted"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inv2en::Disabled)
    }
    #[doc = "Pair output inverted"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inv2en::Enabled)
    }
}
#[doc = "Pair Channels 3 Inverting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv3en {
    #[doc = "0: Pair output not inverted"]
    Disabled = 0,
    #[doc = "1: Pair output inverted"]
    Enabled = 1,
}
impl From<Inv3en> for bool {
    #[inline(always)]
    fn from(variant: Inv3en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV3EN` reader - Pair Channels 3 Inverting Enable"]
pub type Inv3enR = crate::BitReader<Inv3en>;
impl Inv3enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inv3en {
        match self.bits {
            false => Inv3en::Disabled,
            true => Inv3en::Enabled,
        }
    }
    #[doc = "Pair output not inverted"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Inv3en::Disabled
    }
    #[doc = "Pair output inverted"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Inv3en::Enabled
    }
}
#[doc = "Field `INV3EN` writer - Pair Channels 3 Inverting Enable"]
pub type Inv3enW<'a, REG> = crate::BitWriter<'a, REG, Inv3en>;
impl<'a, REG> Inv3enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pair output not inverted"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inv3en::Disabled)
    }
    #[doc = "Pair output inverted"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inv3en::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Pair Channels 0 Inverting Enable"]
    #[inline(always)]
    pub fn inv0en(&self) -> Inv0enR {
        Inv0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pair Channels 1 Inverting Enable"]
    #[inline(always)]
    pub fn inv1en(&self) -> Inv1enR {
        Inv1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pair Channels 2 Inverting Enable"]
    #[inline(always)]
    pub fn inv2en(&self) -> Inv2enR {
        Inv2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pair Channels 3 Inverting Enable"]
    #[inline(always)]
    pub fn inv3en(&self) -> Inv3enR {
        Inv3enR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pair Channels 0 Inverting Enable"]
    #[inline(always)]
    pub fn inv0en(&mut self) -> Inv0enW<'_, InvctrlSpec> {
        Inv0enW::new(self, 0)
    }
    #[doc = "Bit 1 - Pair Channels 1 Inverting Enable"]
    #[inline(always)]
    pub fn inv1en(&mut self) -> Inv1enW<'_, InvctrlSpec> {
        Inv1enW::new(self, 1)
    }
    #[doc = "Bit 2 - Pair Channels 2 Inverting Enable"]
    #[inline(always)]
    pub fn inv2en(&mut self) -> Inv2enW<'_, InvctrlSpec> {
        Inv2enW::new(self, 2)
    }
    #[doc = "Bit 3 - Pair Channels 3 Inverting Enable"]
    #[inline(always)]
    pub fn inv3en(&mut self) -> Inv3enW<'_, InvctrlSpec> {
        Inv3enW::new(self, 3)
    }
}
#[doc = "FTM Inverting Control\n\nYou can [`read`](crate::Reg::read) this register and get [`invctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`invctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InvctrlSpec;
impl crate::RegisterSpec for InvctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`invctrl::R`](R) reader structure"]
impl crate::Readable for InvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`invctrl::W`](W) writer structure"]
impl crate::Writable for InvctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INVCTRL to value 0"]
impl crate::Resettable for InvctrlSpec {}

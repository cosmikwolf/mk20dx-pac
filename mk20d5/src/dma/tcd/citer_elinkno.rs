#[doc = "Register `CITER_ELINKNO` reader"]
pub type R = crate::R<CiterElinknoSpec>;
#[doc = "Register `CITER_ELINKNO` writer"]
pub type W = crate::W<CiterElinknoSpec>;
#[doc = "Field `CITER` reader - Current Major Iteration Count"]
pub type CiterR = crate::FieldReader<u16>;
#[doc = "Field `CITER` writer - Current Major Iteration Count"]
pub type CiterW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Enable channel-to-channel linking on minor-loop complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elink {
    #[doc = "0: The channel-to-channel linking is disabled"]
    Disabled = 0,
    #[doc = "1: The channel-to-channel linking is enabled"]
    Enabled = 1,
}
impl From<Elink> for bool {
    #[inline(always)]
    fn from(variant: Elink) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELINK` reader - Enable channel-to-channel linking on minor-loop complete"]
pub type ElinkR = crate::BitReader<Elink>;
impl ElinkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Elink {
        match self.bits {
            false => Elink::Disabled,
            true => Elink::Enabled,
        }
    }
    #[doc = "The channel-to-channel linking is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Elink::Disabled
    }
    #[doc = "The channel-to-channel linking is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Elink::Enabled
    }
}
#[doc = "Field `ELINK` writer - Enable channel-to-channel linking on minor-loop complete"]
pub type ElinkW<'a, REG> = crate::BitWriter<'a, REG, Elink>;
impl<'a, REG> ElinkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The channel-to-channel linking is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Elink::Disabled)
    }
    #[doc = "The channel-to-channel linking is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Elink::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:14 - Current Major Iteration Count"]
    #[inline(always)]
    pub fn citer(&self) -> CiterR {
        CiterR::new(self.bits & 0x7fff)
    }
    #[doc = "Bit 15 - Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub fn elink(&self) -> ElinkR {
        ElinkR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Current Major Iteration Count"]
    #[inline(always)]
    pub fn citer(&mut self) -> CiterW<'_, CiterElinknoSpec> {
        CiterW::new(self, 0)
    }
    #[doc = "Bit 15 - Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub fn elink(&mut self) -> ElinkW<'_, CiterElinknoSpec> {
        ElinkW::new(self, 15)
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nYou can [`read`](crate::Reg::read) this register and get [`citer_elinkno::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`citer_elinkno::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CiterElinknoSpec;
impl crate::RegisterSpec for CiterElinknoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`citer_elinkno::R`](R) reader structure"]
impl crate::Readable for CiterElinknoSpec {}
#[doc = "`write(|w| ..)` method takes [`citer_elinkno::W`](W) writer structure"]
impl crate::Writable for CiterElinknoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CITER_ELINKNO to value 0"]
impl crate::Resettable for CiterElinknoSpec {}

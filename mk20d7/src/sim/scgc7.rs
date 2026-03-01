#[doc = "Register `SCGC7` reader"]
pub type R = crate::R<Scgc7Spec>;
#[doc = "Register `SCGC7` writer"]
pub type W = crate::W<Scgc7Spec>;
#[doc = "FlexBus Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexbus {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Flexbus> for bool {
    #[inline(always)]
    fn from(variant: Flexbus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXBUS` reader - FlexBus Clock Gate Control"]
pub type FlexbusR = crate::BitReader<Flexbus>;
impl FlexbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexbus {
        match self.bits {
            false => Flexbus::Disabled,
            true => Flexbus::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Flexbus::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Flexbus::Enabled
    }
}
#[doc = "Field `FLEXBUS` writer - FlexBus Clock Gate Control"]
pub type FlexbusW<'a, REG> = crate::BitWriter<'a, REG, Flexbus>;
impl<'a, REG> FlexbusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexbus::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Flexbus::Enabled)
    }
}
#[doc = "DMA Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<Dma> for bool {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - DMA Clock Gate Control"]
pub type DmaR = crate::BitReader<Dma>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma {
        match self.bits {
            false => Dma::Disabled,
            true => Dma::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dma::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dma::Enabled
    }
}
#[doc = "Field `DMA` writer - DMA Clock Gate Control"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG, Dma>;
impl<'a, REG> DmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - FlexBus Clock Gate Control"]
    #[inline(always)]
    pub fn flexbus(&self) -> FlexbusR {
        FlexbusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Clock Gate Control"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FlexBus Clock Gate Control"]
    #[inline(always)]
    pub fn flexbus(&mut self) -> FlexbusW<'_, Scgc7Spec> {
        FlexbusW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Clock Gate Control"]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, Scgc7Spec> {
        DmaW::new(self, 1)
    }
}
#[doc = "System Clock Gating Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scgc7Spec;
impl crate::RegisterSpec for Scgc7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgc7::R`](R) reader structure"]
impl crate::Readable for Scgc7Spec {}
#[doc = "`write(|w| ..)` method takes [`scgc7::W`](W) writer structure"]
impl crate::Writable for Scgc7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCGC7 to value 0x07"]
impl crate::Resettable for Scgc7Spec {
    const RESET_VALUE: u32 = 0x07;
}

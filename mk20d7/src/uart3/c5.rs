#[doc = "Register `C5` reader"]
pub type R = crate::R<C5Spec>;
#[doc = "Register `C5` writer"]
pub type W = crate::W<C5Spec>;
#[doc = "Receiver Full DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdmas {
    #[doc = "0: RDRF generates interrupt"]
    Interrupt = 0,
    #[doc = "1: RDRF generates DMA request"]
    Dma = 1,
}
impl From<Rdmas> for bool {
    #[inline(always)]
    fn from(variant: Rdmas) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDMAS` reader - Receiver Full DMA Select"]
pub type RdmasR = crate::BitReader<Rdmas>;
impl RdmasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdmas {
        match self.bits {
            false => Rdmas::Interrupt,
            true => Rdmas::Dma,
        }
    }
    #[doc = "RDRF generates interrupt"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Rdmas::Interrupt
    }
    #[doc = "RDRF generates DMA request"]
    #[inline(always)]
    pub fn is_dma(&self) -> bool {
        *self == Rdmas::Dma
    }
}
#[doc = "Field `RDMAS` writer - Receiver Full DMA Select"]
pub type RdmasW<'a, REG> = crate::BitWriter<'a, REG, Rdmas>;
impl<'a, REG> RdmasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RDRF generates interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Rdmas::Interrupt)
    }
    #[doc = "RDRF generates DMA request"]
    #[inline(always)]
    pub fn dma(self) -> &'a mut crate::W<REG> {
        self.variant(Rdmas::Dma)
    }
}
#[doc = "Transmitter DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdmas {
    #[doc = "0: TDRE generates interrupt"]
    Interrupt = 0,
    #[doc = "1: TDRE generates DMA request"]
    Dma = 1,
}
impl From<Tdmas> for bool {
    #[inline(always)]
    fn from(variant: Tdmas) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDMAS` reader - Transmitter DMA Select"]
pub type TdmasR = crate::BitReader<Tdmas>;
impl TdmasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdmas {
        match self.bits {
            false => Tdmas::Interrupt,
            true => Tdmas::Dma,
        }
    }
    #[doc = "TDRE generates interrupt"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Tdmas::Interrupt
    }
    #[doc = "TDRE generates DMA request"]
    #[inline(always)]
    pub fn is_dma(&self) -> bool {
        *self == Tdmas::Dma
    }
}
#[doc = "Field `TDMAS` writer - Transmitter DMA Select"]
pub type TdmasW<'a, REG> = crate::BitWriter<'a, REG, Tdmas>;
impl<'a, REG> TdmasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TDRE generates interrupt"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmas::Interrupt)
    }
    #[doc = "TDRE generates DMA request"]
    #[inline(always)]
    pub fn dma(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmas::Dma)
    }
}
impl R {
    #[doc = "Bit 5 - Receiver Full DMA Select"]
    #[inline(always)]
    pub fn rdmas(&self) -> RdmasR {
        RdmasR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmitter DMA Select"]
    #[inline(always)]
    pub fn tdmas(&self) -> TdmasR {
        TdmasR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Receiver Full DMA Select"]
    #[inline(always)]
    pub fn rdmas(&mut self) -> RdmasW<'_, C5Spec> {
        RdmasW::new(self, 5)
    }
    #[doc = "Bit 7 - Transmitter DMA Select"]
    #[inline(always)]
    pub fn tdmas(&mut self) -> TdmasW<'_, C5Spec> {
        TdmasW::new(self, 7)
    }
}
#[doc = "UART Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`c5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C5Spec;
impl crate::RegisterSpec for C5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c5::R`](R) reader structure"]
impl crate::Readable for C5Spec {}
#[doc = "`write(|w| ..)` method takes [`c5::W`](W) writer structure"]
impl crate::Writable for C5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C5 to value 0"]
impl crate::Resettable for C5Spec {}

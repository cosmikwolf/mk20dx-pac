#[doc = "Register `TDR%s` writer"]
pub type W = crate::W<TdrSpec>;
#[doc = "Field `TDR` writer - Transmit data register"]
pub type TdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Transmit data register"]
    #[inline(always)]
    pub fn tdr(&mut self) -> TdrW<'_, TdrSpec> {
        TdrW::new(self, 0)
    }
}
#[doc = "SAI Transmit Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdrSpec;
impl crate::RegisterSpec for TdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDR%s to value 0"]
impl crate::Resettable for TdrSpec {}

#[doc = "Register `RSER` reader"]
pub type R = crate::R<RserSpec>;
#[doc = "Register `RSER` writer"]
pub type W = crate::W<RserSpec>;
#[doc = "Receive FIFO Drain DMA or Interrupt Request Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RfdfDirs {
    #[doc = "0: RFDF flag generates interrupt request"]
    Irq = 0,
    #[doc = "1: RFDF flag generates DMA request"]
    Dma = 1,
}
impl From<RfdfDirs> for bool {
    #[inline(always)]
    fn from(variant: RfdfDirs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFDF_DIRS` reader - Receive FIFO Drain DMA or Interrupt Request Select."]
pub type RfdfDirsR = crate::BitReader<RfdfDirs>;
impl RfdfDirsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RfdfDirs {
        match self.bits {
            false => RfdfDirs::Irq,
            true => RfdfDirs::Dma,
        }
    }
    #[doc = "RFDF flag generates interrupt request"]
    #[inline(always)]
    pub fn is_irq(&self) -> bool {
        *self == RfdfDirs::Irq
    }
    #[doc = "RFDF flag generates DMA request"]
    #[inline(always)]
    pub fn is_dma(&self) -> bool {
        *self == RfdfDirs::Dma
    }
}
#[doc = "Field `RFDF_DIRS` writer - Receive FIFO Drain DMA or Interrupt Request Select."]
pub type RfdfDirsW<'a, REG> = crate::BitWriter<'a, REG, RfdfDirs>;
impl<'a, REG> RfdfDirsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RFDF flag generates interrupt request"]
    #[inline(always)]
    pub fn irq(self) -> &'a mut crate::W<REG> {
        self.variant(RfdfDirs::Irq)
    }
    #[doc = "RFDF flag generates DMA request"]
    #[inline(always)]
    pub fn dma(self) -> &'a mut crate::W<REG> {
        self.variant(RfdfDirs::Dma)
    }
}
#[doc = "Receive FIFO Drain Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RfdfRe {
    #[doc = "0: RFDF interrupt or DMA request disabled"]
    Disabled = 0,
    #[doc = "1: RFDF interrupt or DMA request enabled"]
    Enabled = 1,
}
impl From<RfdfRe> for bool {
    #[inline(always)]
    fn from(variant: RfdfRe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFDF_RE` reader - Receive FIFO Drain Request Enable"]
pub type RfdfReR = crate::BitReader<RfdfRe>;
impl RfdfReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RfdfRe {
        match self.bits {
            false => RfdfRe::Disabled,
            true => RfdfRe::Enabled,
        }
    }
    #[doc = "RFDF interrupt or DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RfdfRe::Disabled
    }
    #[doc = "RFDF interrupt or DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RfdfRe::Enabled
    }
}
#[doc = "Field `RFDF_RE` writer - Receive FIFO Drain Request Enable"]
pub type RfdfReW<'a, REG> = crate::BitWriter<'a, REG, RfdfRe>;
impl<'a, REG> RfdfReW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RFDF interrupt or DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RfdfRe::Disabled)
    }
    #[doc = "RFDF interrupt or DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RfdfRe::Enabled)
    }
}
#[doc = "Receive FIFO Overflow Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RfofRe {
    #[doc = "0: RFOF interrupt request disabled"]
    Disabled = 0,
    #[doc = "1: RFOF interrupt request enabled"]
    Enabled = 1,
}
impl From<RfofRe> for bool {
    #[inline(always)]
    fn from(variant: RfofRe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFOF_RE` reader - Receive FIFO Overflow Request Enable"]
pub type RfofReR = crate::BitReader<RfofRe>;
impl RfofReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RfofRe {
        match self.bits {
            false => RfofRe::Disabled,
            true => RfofRe::Enabled,
        }
    }
    #[doc = "RFOF interrupt request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RfofRe::Disabled
    }
    #[doc = "RFOF interrupt request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RfofRe::Enabled
    }
}
#[doc = "Field `RFOF_RE` writer - Receive FIFO Overflow Request Enable"]
pub type RfofReW<'a, REG> = crate::BitWriter<'a, REG, RfofRe>;
impl<'a, REG> RfofReW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RFOF interrupt request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RfofRe::Disabled)
    }
    #[doc = "RFOF interrupt request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RfofRe::Enabled)
    }
}
#[doc = "Transmit FIFO Fill DMA or Interrupt Request Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TfffDirs {
    #[doc = "0: TFFF flag generates interrupt request"]
    Irq = 0,
    #[doc = "1: TFFF flag generates DMA request"]
    Dma = 1,
}
impl From<TfffDirs> for bool {
    #[inline(always)]
    fn from(variant: TfffDirs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFFF_DIRS` reader - Transmit FIFO Fill DMA or Interrupt Request Select"]
pub type TfffDirsR = crate::BitReader<TfffDirs>;
impl TfffDirsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TfffDirs {
        match self.bits {
            false => TfffDirs::Irq,
            true => TfffDirs::Dma,
        }
    }
    #[doc = "TFFF flag generates interrupt request"]
    #[inline(always)]
    pub fn is_irq(&self) -> bool {
        *self == TfffDirs::Irq
    }
    #[doc = "TFFF flag generates DMA request"]
    #[inline(always)]
    pub fn is_dma(&self) -> bool {
        *self == TfffDirs::Dma
    }
}
#[doc = "Field `TFFF_DIRS` writer - Transmit FIFO Fill DMA or Interrupt Request Select"]
pub type TfffDirsW<'a, REG> = crate::BitWriter<'a, REG, TfffDirs>;
impl<'a, REG> TfffDirsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TFFF flag generates interrupt request"]
    #[inline(always)]
    pub fn irq(self) -> &'a mut crate::W<REG> {
        self.variant(TfffDirs::Irq)
    }
    #[doc = "TFFF flag generates DMA request"]
    #[inline(always)]
    pub fn dma(self) -> &'a mut crate::W<REG> {
        self.variant(TfffDirs::Dma)
    }
}
#[doc = "Transmit FIFO Fill Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TfffRe {
    #[doc = "0: TFFF interrupt or DMA request disabled"]
    Disabled = 0,
    #[doc = "1: TFFF interrupt or DMA request enabled"]
    Enabled = 1,
}
impl From<TfffRe> for bool {
    #[inline(always)]
    fn from(variant: TfffRe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFFF_RE` reader - Transmit FIFO Fill Request Enable"]
pub type TfffReR = crate::BitReader<TfffRe>;
impl TfffReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TfffRe {
        match self.bits {
            false => TfffRe::Disabled,
            true => TfffRe::Enabled,
        }
    }
    #[doc = "TFFF interrupt or DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TfffRe::Disabled
    }
    #[doc = "TFFF interrupt or DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TfffRe::Enabled
    }
}
#[doc = "Field `TFFF_RE` writer - Transmit FIFO Fill Request Enable"]
pub type TfffReW<'a, REG> = crate::BitWriter<'a, REG, TfffRe>;
impl<'a, REG> TfffReW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TFFF interrupt or DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TfffRe::Disabled)
    }
    #[doc = "TFFF interrupt or DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TfffRe::Enabled)
    }
}
#[doc = "Transmit FIFO Underflow Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TfufRe {
    #[doc = "0: TFUF interrupt request disabled"]
    Disabled = 0,
    #[doc = "1: TFUF interrupt request enabled"]
    Enabled = 1,
}
impl From<TfufRe> for bool {
    #[inline(always)]
    fn from(variant: TfufRe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFUF_RE` reader - Transmit FIFO Underflow Request Enable"]
pub type TfufReR = crate::BitReader<TfufRe>;
impl TfufReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TfufRe {
        match self.bits {
            false => TfufRe::Disabled,
            true => TfufRe::Enabled,
        }
    }
    #[doc = "TFUF interrupt request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TfufRe::Disabled
    }
    #[doc = "TFUF interrupt request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TfufRe::Enabled
    }
}
#[doc = "Field `TFUF_RE` writer - Transmit FIFO Underflow Request Enable"]
pub type TfufReW<'a, REG> = crate::BitWriter<'a, REG, TfufRe>;
impl<'a, REG> TfufReW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TFUF interrupt request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TfufRe::Disabled)
    }
    #[doc = "TFUF interrupt request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TfufRe::Enabled)
    }
}
#[doc = "DSPI Finished Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EoqfRe {
    #[doc = "0: EOQF interrupt request disabled"]
    Disabled = 0,
    #[doc = "1: EOQF interrupt request enabled"]
    Enabled = 1,
}
impl From<EoqfRe> for bool {
    #[inline(always)]
    fn from(variant: EoqfRe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOQF_RE` reader - DSPI Finished Request Enable"]
pub type EoqfReR = crate::BitReader<EoqfRe>;
impl EoqfReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EoqfRe {
        match self.bits {
            false => EoqfRe::Disabled,
            true => EoqfRe::Enabled,
        }
    }
    #[doc = "EOQF interrupt request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EoqfRe::Disabled
    }
    #[doc = "EOQF interrupt request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EoqfRe::Enabled
    }
}
#[doc = "Field `EOQF_RE` writer - DSPI Finished Request Enable"]
pub type EoqfReW<'a, REG> = crate::BitWriter<'a, REG, EoqfRe>;
impl<'a, REG> EoqfReW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EOQF interrupt request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EoqfRe::Disabled)
    }
    #[doc = "EOQF interrupt request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EoqfRe::Enabled)
    }
}
#[doc = "Transmission Complete Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TcfRe {
    #[doc = "0: TCF interrupt request disabled"]
    Disabled = 0,
    #[doc = "1: TCF interrupt request enabled"]
    Enabled = 1,
}
impl From<TcfRe> for bool {
    #[inline(always)]
    fn from(variant: TcfRe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCF_RE` reader - Transmission Complete Request Enable"]
pub type TcfReR = crate::BitReader<TcfRe>;
impl TcfReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TcfRe {
        match self.bits {
            false => TcfRe::Disabled,
            true => TcfRe::Enabled,
        }
    }
    #[doc = "TCF interrupt request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TcfRe::Disabled
    }
    #[doc = "TCF interrupt request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TcfRe::Enabled
    }
}
#[doc = "Field `TCF_RE` writer - Transmission Complete Request Enable"]
pub type TcfReW<'a, REG> = crate::BitWriter<'a, REG, TcfRe>;
impl<'a, REG> TcfReW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TCF interrupt request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TcfRe::Disabled)
    }
    #[doc = "TCF interrupt request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TcfRe::Enabled)
    }
}
impl R {
    #[doc = "Bit 16 - Receive FIFO Drain DMA or Interrupt Request Select."]
    #[inline(always)]
    pub fn rfdf_dirs(&self) -> RfdfDirsR {
        RfdfDirsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO Drain Request Enable"]
    #[inline(always)]
    pub fn rfdf_re(&self) -> RfdfReR {
        RfdfReR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Request Enable"]
    #[inline(always)]
    pub fn rfof_re(&self) -> RfofReR {
        RfofReR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmit FIFO Fill DMA or Interrupt Request Select"]
    #[inline(always)]
    pub fn tfff_dirs(&self) -> TfffDirsR {
        TfffDirsR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Request Enable"]
    #[inline(always)]
    pub fn tfff_re(&self) -> TfffReR {
        TfffReR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Request Enable"]
    #[inline(always)]
    pub fn tfuf_re(&self) -> TfufReR {
        TfufReR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DSPI Finished Request Enable"]
    #[inline(always)]
    pub fn eoqf_re(&self) -> EoqfReR {
        EoqfReR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmission Complete Request Enable"]
    #[inline(always)]
    pub fn tcf_re(&self) -> TcfReR {
        TcfReR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Receive FIFO Drain DMA or Interrupt Request Select."]
    #[inline(always)]
    pub fn rfdf_dirs(&mut self) -> RfdfDirsW<'_, RserSpec> {
        RfdfDirsW::new(self, 16)
    }
    #[doc = "Bit 17 - Receive FIFO Drain Request Enable"]
    #[inline(always)]
    pub fn rfdf_re(&mut self) -> RfdfReW<'_, RserSpec> {
        RfdfReW::new(self, 17)
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Request Enable"]
    #[inline(always)]
    pub fn rfof_re(&mut self) -> RfofReW<'_, RserSpec> {
        RfofReW::new(self, 19)
    }
    #[doc = "Bit 24 - Transmit FIFO Fill DMA or Interrupt Request Select"]
    #[inline(always)]
    pub fn tfff_dirs(&mut self) -> TfffDirsW<'_, RserSpec> {
        TfffDirsW::new(self, 24)
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Request Enable"]
    #[inline(always)]
    pub fn tfff_re(&mut self) -> TfffReW<'_, RserSpec> {
        TfffReW::new(self, 25)
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Request Enable"]
    #[inline(always)]
    pub fn tfuf_re(&mut self) -> TfufReW<'_, RserSpec> {
        TfufReW::new(self, 27)
    }
    #[doc = "Bit 28 - DSPI Finished Request Enable"]
    #[inline(always)]
    pub fn eoqf_re(&mut self) -> EoqfReW<'_, RserSpec> {
        EoqfReW::new(self, 28)
    }
    #[doc = "Bit 31 - Transmission Complete Request Enable"]
    #[inline(always)]
    pub fn tcf_re(&mut self) -> TcfReW<'_, RserSpec> {
        TcfReW::new(self, 31)
    }
}
#[doc = "DSPI DMA/Interrupt Request Select and Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RserSpec;
impl crate::RegisterSpec for RserSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rser::R`](R) reader structure"]
impl crate::Readable for RserSpec {}
#[doc = "`write(|w| ..)` method takes [`rser::W`](W) writer structure"]
impl crate::Writable for RserSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSER to value 0"]
impl crate::Resettable for RserSpec {}

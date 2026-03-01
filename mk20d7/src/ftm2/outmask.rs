#[doc = "Register `OUTMASK` reader"]
pub type R = crate::R<OutmaskSpec>;
#[doc = "Register `OUTMASK` writer"]
pub type W = crate::W<OutmaskSpec>;
#[doc = "Channel 0 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0om {
    #[doc = "0: Channel output not masked"]
    Unmasked = 0,
    #[doc = "1: Channel output masked"]
    Masked = 1,
}
impl From<Ch0om> for bool {
    #[inline(always)]
    fn from(variant: Ch0om) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0OM` reader - Channel 0 Output Mask"]
pub type Ch0omR = crate::BitReader<Ch0om>;
impl Ch0omR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0om {
        match self.bits {
            false => Ch0om::Unmasked,
            true => Ch0om::Masked,
        }
    }
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Ch0om::Unmasked
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Ch0om::Masked
    }
}
#[doc = "Field `CH0OM` writer - Channel 0 Output Mask"]
pub type Ch0omW<'a, REG> = crate::BitWriter<'a, REG, Ch0om>;
impl<'a, REG> Ch0omW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0om::Unmasked)
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0om::Masked)
    }
}
#[doc = "Channel 1 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1om {
    #[doc = "0: Channel output not masked"]
    Unmasked = 0,
    #[doc = "1: Channel output masked"]
    Masked = 1,
}
impl From<Ch1om> for bool {
    #[inline(always)]
    fn from(variant: Ch1om) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1OM` reader - Channel 1 Output Mask"]
pub type Ch1omR = crate::BitReader<Ch1om>;
impl Ch1omR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1om {
        match self.bits {
            false => Ch1om::Unmasked,
            true => Ch1om::Masked,
        }
    }
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Ch1om::Unmasked
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Ch1om::Masked
    }
}
#[doc = "Field `CH1OM` writer - Channel 1 Output Mask"]
pub type Ch1omW<'a, REG> = crate::BitWriter<'a, REG, Ch1om>;
impl<'a, REG> Ch1omW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1om::Unmasked)
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1om::Masked)
    }
}
#[doc = "Channel 2 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2om {
    #[doc = "0: Channel output not masked"]
    Unmasked = 0,
    #[doc = "1: Channel output masked"]
    Masked = 1,
}
impl From<Ch2om> for bool {
    #[inline(always)]
    fn from(variant: Ch2om) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2OM` reader - Channel 2 Output Mask"]
pub type Ch2omR = crate::BitReader<Ch2om>;
impl Ch2omR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2om {
        match self.bits {
            false => Ch2om::Unmasked,
            true => Ch2om::Masked,
        }
    }
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Ch2om::Unmasked
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Ch2om::Masked
    }
}
#[doc = "Field `CH2OM` writer - Channel 2 Output Mask"]
pub type Ch2omW<'a, REG> = crate::BitWriter<'a, REG, Ch2om>;
impl<'a, REG> Ch2omW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2om::Unmasked)
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2om::Masked)
    }
}
#[doc = "Channel 3 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3om {
    #[doc = "0: Channel output not masked"]
    Unmasked = 0,
    #[doc = "1: Channel output masked"]
    Masked = 1,
}
impl From<Ch3om> for bool {
    #[inline(always)]
    fn from(variant: Ch3om) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3OM` reader - Channel 3 Output Mask"]
pub type Ch3omR = crate::BitReader<Ch3om>;
impl Ch3omR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3om {
        match self.bits {
            false => Ch3om::Unmasked,
            true => Ch3om::Masked,
        }
    }
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Ch3om::Unmasked
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Ch3om::Masked
    }
}
#[doc = "Field `CH3OM` writer - Channel 3 Output Mask"]
pub type Ch3omW<'a, REG> = crate::BitWriter<'a, REG, Ch3om>;
impl<'a, REG> Ch3omW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3om::Unmasked)
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3om::Masked)
    }
}
#[doc = "Channel 4 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4om {
    #[doc = "0: Channel output not masked"]
    Unmasked = 0,
    #[doc = "1: Channel output masked"]
    Masked = 1,
}
impl From<Ch4om> for bool {
    #[inline(always)]
    fn from(variant: Ch4om) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4OM` reader - Channel 4 Output Mask"]
pub type Ch4omR = crate::BitReader<Ch4om>;
impl Ch4omR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch4om {
        match self.bits {
            false => Ch4om::Unmasked,
            true => Ch4om::Masked,
        }
    }
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Ch4om::Unmasked
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Ch4om::Masked
    }
}
#[doc = "Field `CH4OM` writer - Channel 4 Output Mask"]
pub type Ch4omW<'a, REG> = crate::BitWriter<'a, REG, Ch4om>;
impl<'a, REG> Ch4omW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4om::Unmasked)
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4om::Masked)
    }
}
#[doc = "Channel 5 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5om {
    #[doc = "0: Channel output not masked"]
    Unmasked = 0,
    #[doc = "1: Channel output masked"]
    Masked = 1,
}
impl From<Ch5om> for bool {
    #[inline(always)]
    fn from(variant: Ch5om) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5OM` reader - Channel 5 Output Mask"]
pub type Ch5omR = crate::BitReader<Ch5om>;
impl Ch5omR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch5om {
        match self.bits {
            false => Ch5om::Unmasked,
            true => Ch5om::Masked,
        }
    }
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Ch5om::Unmasked
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Ch5om::Masked
    }
}
#[doc = "Field `CH5OM` writer - Channel 5 Output Mask"]
pub type Ch5omW<'a, REG> = crate::BitWriter<'a, REG, Ch5om>;
impl<'a, REG> Ch5omW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5om::Unmasked)
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5om::Masked)
    }
}
#[doc = "Channel 6 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6om {
    #[doc = "0: Channel output not masked"]
    Unmasked = 0,
    #[doc = "1: Channel output masked"]
    Masked = 1,
}
impl From<Ch6om> for bool {
    #[inline(always)]
    fn from(variant: Ch6om) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6OM` reader - Channel 6 Output Mask"]
pub type Ch6omR = crate::BitReader<Ch6om>;
impl Ch6omR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch6om {
        match self.bits {
            false => Ch6om::Unmasked,
            true => Ch6om::Masked,
        }
    }
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Ch6om::Unmasked
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Ch6om::Masked
    }
}
#[doc = "Field `CH6OM` writer - Channel 6 Output Mask"]
pub type Ch6omW<'a, REG> = crate::BitWriter<'a, REG, Ch6om>;
impl<'a, REG> Ch6omW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6om::Unmasked)
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6om::Masked)
    }
}
#[doc = "Channel 7 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7om {
    #[doc = "0: Channel output not masked"]
    Unmasked = 0,
    #[doc = "1: Channel output masked"]
    Masked = 1,
}
impl From<Ch7om> for bool {
    #[inline(always)]
    fn from(variant: Ch7om) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7OM` reader - Channel 7 Output Mask"]
pub type Ch7omR = crate::BitReader<Ch7om>;
impl Ch7omR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch7om {
        match self.bits {
            false => Ch7om::Unmasked,
            true => Ch7om::Masked,
        }
    }
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == Ch7om::Unmasked
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == Ch7om::Masked
    }
}
#[doc = "Field `CH7OM` writer - Channel 7 Output Mask"]
pub type Ch7omW<'a, REG> = crate::BitWriter<'a, REG, Ch7om>;
impl<'a, REG> Ch7omW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel output not masked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7om::Unmasked)
    }
    #[doc = "Channel output masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7om::Masked)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Output Mask"]
    #[inline(always)]
    pub fn ch0om(&self) -> Ch0omR {
        Ch0omR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Output Mask"]
    #[inline(always)]
    pub fn ch1om(&self) -> Ch1omR {
        Ch1omR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Output Mask"]
    #[inline(always)]
    pub fn ch2om(&self) -> Ch2omR {
        Ch2omR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Output Mask"]
    #[inline(always)]
    pub fn ch3om(&self) -> Ch3omR {
        Ch3omR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Output Mask"]
    #[inline(always)]
    pub fn ch4om(&self) -> Ch4omR {
        Ch4omR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Output Mask"]
    #[inline(always)]
    pub fn ch5om(&self) -> Ch5omR {
        Ch5omR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Output Mask"]
    #[inline(always)]
    pub fn ch6om(&self) -> Ch6omR {
        Ch6omR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Output Mask"]
    #[inline(always)]
    pub fn ch7om(&self) -> Ch7omR {
        Ch7omR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Output Mask"]
    #[inline(always)]
    pub fn ch0om(&mut self) -> Ch0omW<'_, OutmaskSpec> {
        Ch0omW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Output Mask"]
    #[inline(always)]
    pub fn ch1om(&mut self) -> Ch1omW<'_, OutmaskSpec> {
        Ch1omW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Output Mask"]
    #[inline(always)]
    pub fn ch2om(&mut self) -> Ch2omW<'_, OutmaskSpec> {
        Ch2omW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Output Mask"]
    #[inline(always)]
    pub fn ch3om(&mut self) -> Ch3omW<'_, OutmaskSpec> {
        Ch3omW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Output Mask"]
    #[inline(always)]
    pub fn ch4om(&mut self) -> Ch4omW<'_, OutmaskSpec> {
        Ch4omW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Output Mask"]
    #[inline(always)]
    pub fn ch5om(&mut self) -> Ch5omW<'_, OutmaskSpec> {
        Ch5omW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Output Mask"]
    #[inline(always)]
    pub fn ch6om(&mut self) -> Ch6omW<'_, OutmaskSpec> {
        Ch6omW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Output Mask"]
    #[inline(always)]
    pub fn ch7om(&mut self) -> Ch7omW<'_, OutmaskSpec> {
        Ch7omW::new(self, 7)
    }
}
#[doc = "Output Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`outmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutmaskSpec;
impl crate::RegisterSpec for OutmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outmask::R`](R) reader structure"]
impl crate::Readable for OutmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`outmask::W`](W) writer structure"]
impl crate::Writable for OutmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTMASK to value 0"]
impl crate::Resettable for OutmaskSpec {}

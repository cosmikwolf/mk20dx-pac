#[doc = "Register `OUTINIT` reader"]
pub type R = crate::R<OutinitSpec>;
#[doc = "Register `OUTINIT` writer"]
pub type W = crate::W<OutinitSpec>;
#[doc = "Channel 0 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0oi {
    #[doc = "0: Initial value is low"]
    Low = 0,
    #[doc = "1: Initial value is high"]
    High = 1,
}
impl From<Ch0oi> for bool {
    #[inline(always)]
    fn from(variant: Ch0oi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0OI` reader - Channel 0 Output Initialization Value"]
pub type Ch0oiR = crate::BitReader<Ch0oi>;
impl Ch0oiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0oi {
        match self.bits {
            false => Ch0oi::Low,
            true => Ch0oi::High,
        }
    }
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch0oi::Low
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch0oi::High
    }
}
#[doc = "Field `CH0OI` writer - Channel 0 Output Initialization Value"]
pub type Ch0oiW<'a, REG> = crate::BitWriter<'a, REG, Ch0oi>;
impl<'a, REG> Ch0oiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0oi::Low)
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0oi::High)
    }
}
#[doc = "Channel 1 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1oi {
    #[doc = "0: Initial value is low"]
    Low = 0,
    #[doc = "1: Initial value is high"]
    High = 1,
}
impl From<Ch1oi> for bool {
    #[inline(always)]
    fn from(variant: Ch1oi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1OI` reader - Channel 1 Output Initialization Value"]
pub type Ch1oiR = crate::BitReader<Ch1oi>;
impl Ch1oiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1oi {
        match self.bits {
            false => Ch1oi::Low,
            true => Ch1oi::High,
        }
    }
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch1oi::Low
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch1oi::High
    }
}
#[doc = "Field `CH1OI` writer - Channel 1 Output Initialization Value"]
pub type Ch1oiW<'a, REG> = crate::BitWriter<'a, REG, Ch1oi>;
impl<'a, REG> Ch1oiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1oi::Low)
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1oi::High)
    }
}
#[doc = "Channel 2 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2oi {
    #[doc = "0: Initial value is low"]
    Low = 0,
    #[doc = "1: Initial value is high"]
    High = 1,
}
impl From<Ch2oi> for bool {
    #[inline(always)]
    fn from(variant: Ch2oi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2OI` reader - Channel 2 Output Initialization Value"]
pub type Ch2oiR = crate::BitReader<Ch2oi>;
impl Ch2oiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2oi {
        match self.bits {
            false => Ch2oi::Low,
            true => Ch2oi::High,
        }
    }
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch2oi::Low
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch2oi::High
    }
}
#[doc = "Field `CH2OI` writer - Channel 2 Output Initialization Value"]
pub type Ch2oiW<'a, REG> = crate::BitWriter<'a, REG, Ch2oi>;
impl<'a, REG> Ch2oiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2oi::Low)
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2oi::High)
    }
}
#[doc = "Channel 3 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3oi {
    #[doc = "0: Initial value is low"]
    Low = 0,
    #[doc = "1: Initial value is high"]
    High = 1,
}
impl From<Ch3oi> for bool {
    #[inline(always)]
    fn from(variant: Ch3oi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3OI` reader - Channel 3 Output Initialization Value"]
pub type Ch3oiR = crate::BitReader<Ch3oi>;
impl Ch3oiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3oi {
        match self.bits {
            false => Ch3oi::Low,
            true => Ch3oi::High,
        }
    }
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch3oi::Low
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch3oi::High
    }
}
#[doc = "Field `CH3OI` writer - Channel 3 Output Initialization Value"]
pub type Ch3oiW<'a, REG> = crate::BitWriter<'a, REG, Ch3oi>;
impl<'a, REG> Ch3oiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3oi::Low)
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3oi::High)
    }
}
#[doc = "Channel 4 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4oi {
    #[doc = "0: Initial value is low"]
    Low = 0,
    #[doc = "1: Initial value is high"]
    High = 1,
}
impl From<Ch4oi> for bool {
    #[inline(always)]
    fn from(variant: Ch4oi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4OI` reader - Channel 4 Output Initialization Value"]
pub type Ch4oiR = crate::BitReader<Ch4oi>;
impl Ch4oiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch4oi {
        match self.bits {
            false => Ch4oi::Low,
            true => Ch4oi::High,
        }
    }
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch4oi::Low
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch4oi::High
    }
}
#[doc = "Field `CH4OI` writer - Channel 4 Output Initialization Value"]
pub type Ch4oiW<'a, REG> = crate::BitWriter<'a, REG, Ch4oi>;
impl<'a, REG> Ch4oiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4oi::Low)
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4oi::High)
    }
}
#[doc = "Channel 5 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5oi {
    #[doc = "0: Initial value is low"]
    Low = 0,
    #[doc = "1: Initial value is high"]
    High = 1,
}
impl From<Ch5oi> for bool {
    #[inline(always)]
    fn from(variant: Ch5oi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5OI` reader - Channel 5 Output Initialization Value"]
pub type Ch5oiR = crate::BitReader<Ch5oi>;
impl Ch5oiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch5oi {
        match self.bits {
            false => Ch5oi::Low,
            true => Ch5oi::High,
        }
    }
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch5oi::Low
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch5oi::High
    }
}
#[doc = "Field `CH5OI` writer - Channel 5 Output Initialization Value"]
pub type Ch5oiW<'a, REG> = crate::BitWriter<'a, REG, Ch5oi>;
impl<'a, REG> Ch5oiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5oi::Low)
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5oi::High)
    }
}
#[doc = "Channel 6 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6oi {
    #[doc = "0: Initial value is low"]
    Low = 0,
    #[doc = "1: Initial value is high"]
    High = 1,
}
impl From<Ch6oi> for bool {
    #[inline(always)]
    fn from(variant: Ch6oi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6OI` reader - Channel 6 Output Initialization Value"]
pub type Ch6oiR = crate::BitReader<Ch6oi>;
impl Ch6oiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch6oi {
        match self.bits {
            false => Ch6oi::Low,
            true => Ch6oi::High,
        }
    }
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch6oi::Low
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch6oi::High
    }
}
#[doc = "Field `CH6OI` writer - Channel 6 Output Initialization Value"]
pub type Ch6oiW<'a, REG> = crate::BitWriter<'a, REG, Ch6oi>;
impl<'a, REG> Ch6oiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6oi::Low)
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6oi::High)
    }
}
#[doc = "Channel 7 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7oi {
    #[doc = "0: Initial value is low"]
    Low = 0,
    #[doc = "1: Initial value is high"]
    High = 1,
}
impl From<Ch7oi> for bool {
    #[inline(always)]
    fn from(variant: Ch7oi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7OI` reader - Channel 7 Output Initialization Value"]
pub type Ch7oiR = crate::BitReader<Ch7oi>;
impl Ch7oiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch7oi {
        match self.bits {
            false => Ch7oi::Low,
            true => Ch7oi::High,
        }
    }
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ch7oi::Low
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ch7oi::High
    }
}
#[doc = "Field `CH7OI` writer - Channel 7 Output Initialization Value"]
pub type Ch7oiW<'a, REG> = crate::BitWriter<'a, REG, Ch7oi>;
impl<'a, REG> Ch7oiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Initial value is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7oi::Low)
    }
    #[doc = "Initial value is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7oi::High)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Output Initialization Value"]
    #[inline(always)]
    pub fn ch0oi(&self) -> Ch0oiR {
        Ch0oiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Output Initialization Value"]
    #[inline(always)]
    pub fn ch1oi(&self) -> Ch1oiR {
        Ch1oiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Output Initialization Value"]
    #[inline(always)]
    pub fn ch2oi(&self) -> Ch2oiR {
        Ch2oiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Output Initialization Value"]
    #[inline(always)]
    pub fn ch3oi(&self) -> Ch3oiR {
        Ch3oiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Output Initialization Value"]
    #[inline(always)]
    pub fn ch4oi(&self) -> Ch4oiR {
        Ch4oiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Output Initialization Value"]
    #[inline(always)]
    pub fn ch5oi(&self) -> Ch5oiR {
        Ch5oiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Output Initialization Value"]
    #[inline(always)]
    pub fn ch6oi(&self) -> Ch6oiR {
        Ch6oiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Output Initialization Value"]
    #[inline(always)]
    pub fn ch7oi(&self) -> Ch7oiR {
        Ch7oiR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Output Initialization Value"]
    #[inline(always)]
    pub fn ch0oi(&mut self) -> Ch0oiW<'_, OutinitSpec> {
        Ch0oiW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Output Initialization Value"]
    #[inline(always)]
    pub fn ch1oi(&mut self) -> Ch1oiW<'_, OutinitSpec> {
        Ch1oiW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Output Initialization Value"]
    #[inline(always)]
    pub fn ch2oi(&mut self) -> Ch2oiW<'_, OutinitSpec> {
        Ch2oiW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Output Initialization Value"]
    #[inline(always)]
    pub fn ch3oi(&mut self) -> Ch3oiW<'_, OutinitSpec> {
        Ch3oiW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Output Initialization Value"]
    #[inline(always)]
    pub fn ch4oi(&mut self) -> Ch4oiW<'_, OutinitSpec> {
        Ch4oiW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Output Initialization Value"]
    #[inline(always)]
    pub fn ch5oi(&mut self) -> Ch5oiW<'_, OutinitSpec> {
        Ch5oiW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Output Initialization Value"]
    #[inline(always)]
    pub fn ch6oi(&mut self) -> Ch6oiW<'_, OutinitSpec> {
        Ch6oiW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Output Initialization Value"]
    #[inline(always)]
    pub fn ch7oi(&mut self) -> Ch7oiW<'_, OutinitSpec> {
        Ch7oiW::new(self, 7)
    }
}
#[doc = "Initial State for Channels Output\n\nYou can [`read`](crate::Reg::read) this register and get [`outinit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outinit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutinitSpec;
impl crate::RegisterSpec for OutinitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outinit::R`](R) reader structure"]
impl crate::Readable for OutinitSpec {}
#[doc = "`write(|w| ..)` method takes [`outinit::W`](W) writer structure"]
impl crate::Writable for OutinitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTINIT to value 0"]
impl crate::Resettable for OutinitSpec {}

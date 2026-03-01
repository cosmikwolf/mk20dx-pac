#[doc = "Register `SWOCTRL` reader"]
pub type R = crate::R<SwoctrlSpec>;
#[doc = "Register `SWOCTRL` writer"]
pub type W = crate::W<SwoctrlSpec>;
#[doc = "Channel 0 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0oc {
    #[doc = "0: Software output control disabled"]
    Disabled = 0,
    #[doc = "1: Software output control enabled"]
    Enabled = 1,
}
impl From<Ch0oc> for bool {
    #[inline(always)]
    fn from(variant: Ch0oc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0OC` reader - Channel 0 Software Output Control Enable"]
pub type Ch0ocR = crate::BitReader<Ch0oc>;
impl Ch0ocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0oc {
        match self.bits {
            false => Ch0oc::Disabled,
            true => Ch0oc::Enabled,
        }
    }
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch0oc::Disabled
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch0oc::Enabled
    }
}
#[doc = "Field `CH0OC` writer - Channel 0 Software Output Control Enable"]
pub type Ch0ocW<'a, REG> = crate::BitWriter<'a, REG, Ch0oc>;
impl<'a, REG> Ch0ocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0oc::Disabled)
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0oc::Enabled)
    }
}
#[doc = "Channel 1 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1oc {
    #[doc = "0: Software output control disabled"]
    Disabled = 0,
    #[doc = "1: Software output control enabled"]
    Enabled = 1,
}
impl From<Ch1oc> for bool {
    #[inline(always)]
    fn from(variant: Ch1oc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1OC` reader - Channel 1 Software Output Control Enable"]
pub type Ch1ocR = crate::BitReader<Ch1oc>;
impl Ch1ocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1oc {
        match self.bits {
            false => Ch1oc::Disabled,
            true => Ch1oc::Enabled,
        }
    }
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch1oc::Disabled
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch1oc::Enabled
    }
}
#[doc = "Field `CH1OC` writer - Channel 1 Software Output Control Enable"]
pub type Ch1ocW<'a, REG> = crate::BitWriter<'a, REG, Ch1oc>;
impl<'a, REG> Ch1ocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1oc::Disabled)
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1oc::Enabled)
    }
}
#[doc = "Channel 2 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2oc {
    #[doc = "0: Software output control disabled"]
    Disabled = 0,
    #[doc = "1: Software output control enabled"]
    Enabled = 1,
}
impl From<Ch2oc> for bool {
    #[inline(always)]
    fn from(variant: Ch2oc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2OC` reader - Channel 2 Software Output Control Enable"]
pub type Ch2ocR = crate::BitReader<Ch2oc>;
impl Ch2ocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2oc {
        match self.bits {
            false => Ch2oc::Disabled,
            true => Ch2oc::Enabled,
        }
    }
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch2oc::Disabled
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch2oc::Enabled
    }
}
#[doc = "Field `CH2OC` writer - Channel 2 Software Output Control Enable"]
pub type Ch2ocW<'a, REG> = crate::BitWriter<'a, REG, Ch2oc>;
impl<'a, REG> Ch2ocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2oc::Disabled)
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2oc::Enabled)
    }
}
#[doc = "Channel 3 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3oc {
    #[doc = "0: Software output control disabled"]
    Disabled = 0,
    #[doc = "1: Software output control enabled"]
    Enabled = 1,
}
impl From<Ch3oc> for bool {
    #[inline(always)]
    fn from(variant: Ch3oc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3OC` reader - Channel 3 Software Output Control Enable"]
pub type Ch3ocR = crate::BitReader<Ch3oc>;
impl Ch3ocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3oc {
        match self.bits {
            false => Ch3oc::Disabled,
            true => Ch3oc::Enabled,
        }
    }
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch3oc::Disabled
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch3oc::Enabled
    }
}
#[doc = "Field `CH3OC` writer - Channel 3 Software Output Control Enable"]
pub type Ch3ocW<'a, REG> = crate::BitWriter<'a, REG, Ch3oc>;
impl<'a, REG> Ch3ocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3oc::Disabled)
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3oc::Enabled)
    }
}
#[doc = "Channel 4 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4oc {
    #[doc = "0: Software output control disabled"]
    Disabled = 0,
    #[doc = "1: Software output control enabled"]
    Enabled = 1,
}
impl From<Ch4oc> for bool {
    #[inline(always)]
    fn from(variant: Ch4oc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4OC` reader - Channel 4 Software Output Control Enable"]
pub type Ch4ocR = crate::BitReader<Ch4oc>;
impl Ch4ocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch4oc {
        match self.bits {
            false => Ch4oc::Disabled,
            true => Ch4oc::Enabled,
        }
    }
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch4oc::Disabled
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch4oc::Enabled
    }
}
#[doc = "Field `CH4OC` writer - Channel 4 Software Output Control Enable"]
pub type Ch4ocW<'a, REG> = crate::BitWriter<'a, REG, Ch4oc>;
impl<'a, REG> Ch4ocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4oc::Disabled)
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4oc::Enabled)
    }
}
#[doc = "Channel 5 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5oc {
    #[doc = "0: Software output control disabled"]
    Disabled = 0,
    #[doc = "1: Software output control enabled"]
    Enabled = 1,
}
impl From<Ch5oc> for bool {
    #[inline(always)]
    fn from(variant: Ch5oc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5OC` reader - Channel 5 Software Output Control Enable"]
pub type Ch5ocR = crate::BitReader<Ch5oc>;
impl Ch5ocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch5oc {
        match self.bits {
            false => Ch5oc::Disabled,
            true => Ch5oc::Enabled,
        }
    }
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch5oc::Disabled
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch5oc::Enabled
    }
}
#[doc = "Field `CH5OC` writer - Channel 5 Software Output Control Enable"]
pub type Ch5ocW<'a, REG> = crate::BitWriter<'a, REG, Ch5oc>;
impl<'a, REG> Ch5ocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5oc::Disabled)
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5oc::Enabled)
    }
}
#[doc = "Channel 6 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6oc {
    #[doc = "0: Software output control disabled"]
    Disabled = 0,
    #[doc = "1: Software output control enabled"]
    Enabled = 1,
}
impl From<Ch6oc> for bool {
    #[inline(always)]
    fn from(variant: Ch6oc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6OC` reader - Channel 6 Software Output Control Enable"]
pub type Ch6ocR = crate::BitReader<Ch6oc>;
impl Ch6ocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch6oc {
        match self.bits {
            false => Ch6oc::Disabled,
            true => Ch6oc::Enabled,
        }
    }
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch6oc::Disabled
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch6oc::Enabled
    }
}
#[doc = "Field `CH6OC` writer - Channel 6 Software Output Control Enable"]
pub type Ch6ocW<'a, REG> = crate::BitWriter<'a, REG, Ch6oc>;
impl<'a, REG> Ch6ocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6oc::Disabled)
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6oc::Enabled)
    }
}
#[doc = "Channel 7 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7oc {
    #[doc = "0: Software output control disabled"]
    Disabled = 0,
    #[doc = "1: Software output control enabled"]
    Enabled = 1,
}
impl From<Ch7oc> for bool {
    #[inline(always)]
    fn from(variant: Ch7oc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7OC` reader - Channel 7 Software Output Control Enable"]
pub type Ch7ocR = crate::BitReader<Ch7oc>;
impl Ch7ocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch7oc {
        match self.bits {
            false => Ch7oc::Disabled,
            true => Ch7oc::Enabled,
        }
    }
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch7oc::Disabled
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ch7oc::Enabled
    }
}
#[doc = "Field `CH7OC` writer - Channel 7 Software Output Control Enable"]
pub type Ch7ocW<'a, REG> = crate::BitWriter<'a, REG, Ch7oc>;
impl<'a, REG> Ch7ocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software output control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7oc::Disabled)
    }
    #[doc = "Software output control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7oc::Enabled)
    }
}
#[doc = "Channel 0 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0ocv {
    #[doc = "0: Force channel output low"]
    ForceLow = 0,
    #[doc = "1: Force channel output high"]
    ForceHigh = 1,
}
impl From<Ch0ocv> for bool {
    #[inline(always)]
    fn from(variant: Ch0ocv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0OCV` reader - Channel 0 Software Output Control Value"]
pub type Ch0ocvR = crate::BitReader<Ch0ocv>;
impl Ch0ocvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0ocv {
        match self.bits {
            false => Ch0ocv::ForceLow,
            true => Ch0ocv::ForceHigh,
        }
    }
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn is_force_low(&self) -> bool {
        *self == Ch0ocv::ForceLow
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn is_force_high(&self) -> bool {
        *self == Ch0ocv::ForceHigh
    }
}
#[doc = "Field `CH0OCV` writer - Channel 0 Software Output Control Value"]
pub type Ch0ocvW<'a, REG> = crate::BitWriter<'a, REG, Ch0ocv>;
impl<'a, REG> Ch0ocvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn force_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0ocv::ForceLow)
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn force_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0ocv::ForceHigh)
    }
}
#[doc = "Channel 1 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1ocv {
    #[doc = "0: Force channel output low"]
    ForceLow = 0,
    #[doc = "1: Force channel output high"]
    ForceHigh = 1,
}
impl From<Ch1ocv> for bool {
    #[inline(always)]
    fn from(variant: Ch1ocv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1OCV` reader - Channel 1 Software Output Control Value"]
pub type Ch1ocvR = crate::BitReader<Ch1ocv>;
impl Ch1ocvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1ocv {
        match self.bits {
            false => Ch1ocv::ForceLow,
            true => Ch1ocv::ForceHigh,
        }
    }
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn is_force_low(&self) -> bool {
        *self == Ch1ocv::ForceLow
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn is_force_high(&self) -> bool {
        *self == Ch1ocv::ForceHigh
    }
}
#[doc = "Field `CH1OCV` writer - Channel 1 Software Output Control Value"]
pub type Ch1ocvW<'a, REG> = crate::BitWriter<'a, REG, Ch1ocv>;
impl<'a, REG> Ch1ocvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn force_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1ocv::ForceLow)
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn force_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1ocv::ForceHigh)
    }
}
#[doc = "Channel 2 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2ocv {
    #[doc = "0: Force channel output low"]
    ForceLow = 0,
    #[doc = "1: Force channel output high"]
    ForceHigh = 1,
}
impl From<Ch2ocv> for bool {
    #[inline(always)]
    fn from(variant: Ch2ocv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2OCV` reader - Channel 2 Software Output Control Value"]
pub type Ch2ocvR = crate::BitReader<Ch2ocv>;
impl Ch2ocvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2ocv {
        match self.bits {
            false => Ch2ocv::ForceLow,
            true => Ch2ocv::ForceHigh,
        }
    }
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn is_force_low(&self) -> bool {
        *self == Ch2ocv::ForceLow
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn is_force_high(&self) -> bool {
        *self == Ch2ocv::ForceHigh
    }
}
#[doc = "Field `CH2OCV` writer - Channel 2 Software Output Control Value"]
pub type Ch2ocvW<'a, REG> = crate::BitWriter<'a, REG, Ch2ocv>;
impl<'a, REG> Ch2ocvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn force_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2ocv::ForceLow)
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn force_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2ocv::ForceHigh)
    }
}
#[doc = "Channel 3 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3ocv {
    #[doc = "0: Force channel output low"]
    ForceLow = 0,
    #[doc = "1: Force channel output high"]
    ForceHigh = 1,
}
impl From<Ch3ocv> for bool {
    #[inline(always)]
    fn from(variant: Ch3ocv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3OCV` reader - Channel 3 Software Output Control Value"]
pub type Ch3ocvR = crate::BitReader<Ch3ocv>;
impl Ch3ocvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3ocv {
        match self.bits {
            false => Ch3ocv::ForceLow,
            true => Ch3ocv::ForceHigh,
        }
    }
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn is_force_low(&self) -> bool {
        *self == Ch3ocv::ForceLow
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn is_force_high(&self) -> bool {
        *self == Ch3ocv::ForceHigh
    }
}
#[doc = "Field `CH3OCV` writer - Channel 3 Software Output Control Value"]
pub type Ch3ocvW<'a, REG> = crate::BitWriter<'a, REG, Ch3ocv>;
impl<'a, REG> Ch3ocvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn force_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3ocv::ForceLow)
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn force_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3ocv::ForceHigh)
    }
}
#[doc = "Channel 4 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4ocv {
    #[doc = "0: Force channel output low"]
    ForceLow = 0,
    #[doc = "1: Force channel output high"]
    ForceHigh = 1,
}
impl From<Ch4ocv> for bool {
    #[inline(always)]
    fn from(variant: Ch4ocv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4OCV` reader - Channel 4 Software Output Control Value"]
pub type Ch4ocvR = crate::BitReader<Ch4ocv>;
impl Ch4ocvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch4ocv {
        match self.bits {
            false => Ch4ocv::ForceLow,
            true => Ch4ocv::ForceHigh,
        }
    }
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn is_force_low(&self) -> bool {
        *self == Ch4ocv::ForceLow
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn is_force_high(&self) -> bool {
        *self == Ch4ocv::ForceHigh
    }
}
#[doc = "Field `CH4OCV` writer - Channel 4 Software Output Control Value"]
pub type Ch4ocvW<'a, REG> = crate::BitWriter<'a, REG, Ch4ocv>;
impl<'a, REG> Ch4ocvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn force_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4ocv::ForceLow)
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn force_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4ocv::ForceHigh)
    }
}
#[doc = "Channel 5 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5ocv {
    #[doc = "0: Force channel output low"]
    ForceLow = 0,
    #[doc = "1: Force channel output high"]
    ForceHigh = 1,
}
impl From<Ch5ocv> for bool {
    #[inline(always)]
    fn from(variant: Ch5ocv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5OCV` reader - Channel 5 Software Output Control Value"]
pub type Ch5ocvR = crate::BitReader<Ch5ocv>;
impl Ch5ocvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch5ocv {
        match self.bits {
            false => Ch5ocv::ForceLow,
            true => Ch5ocv::ForceHigh,
        }
    }
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn is_force_low(&self) -> bool {
        *self == Ch5ocv::ForceLow
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn is_force_high(&self) -> bool {
        *self == Ch5ocv::ForceHigh
    }
}
#[doc = "Field `CH5OCV` writer - Channel 5 Software Output Control Value"]
pub type Ch5ocvW<'a, REG> = crate::BitWriter<'a, REG, Ch5ocv>;
impl<'a, REG> Ch5ocvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn force_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5ocv::ForceLow)
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn force_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5ocv::ForceHigh)
    }
}
#[doc = "Channel 6 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch6ocv {
    #[doc = "0: Force channel output low"]
    ForceLow = 0,
    #[doc = "1: Force channel output high"]
    ForceHigh = 1,
}
impl From<Ch6ocv> for bool {
    #[inline(always)]
    fn from(variant: Ch6ocv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6OCV` reader - Channel 6 Software Output Control Value"]
pub type Ch6ocvR = crate::BitReader<Ch6ocv>;
impl Ch6ocvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch6ocv {
        match self.bits {
            false => Ch6ocv::ForceLow,
            true => Ch6ocv::ForceHigh,
        }
    }
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn is_force_low(&self) -> bool {
        *self == Ch6ocv::ForceLow
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn is_force_high(&self) -> bool {
        *self == Ch6ocv::ForceHigh
    }
}
#[doc = "Field `CH6OCV` writer - Channel 6 Software Output Control Value"]
pub type Ch6ocvW<'a, REG> = crate::BitWriter<'a, REG, Ch6ocv>;
impl<'a, REG> Ch6ocvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn force_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6ocv::ForceLow)
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn force_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch6ocv::ForceHigh)
    }
}
#[doc = "Channel 7 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch7ocv {
    #[doc = "0: Force channel output low"]
    ForceLow = 0,
    #[doc = "1: Force channel output high"]
    ForceHigh = 1,
}
impl From<Ch7ocv> for bool {
    #[inline(always)]
    fn from(variant: Ch7ocv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7OCV` reader - Channel 7 Software Output Control Value"]
pub type Ch7ocvR = crate::BitReader<Ch7ocv>;
impl Ch7ocvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch7ocv {
        match self.bits {
            false => Ch7ocv::ForceLow,
            true => Ch7ocv::ForceHigh,
        }
    }
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn is_force_low(&self) -> bool {
        *self == Ch7ocv::ForceLow
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn is_force_high(&self) -> bool {
        *self == Ch7ocv::ForceHigh
    }
}
#[doc = "Field `CH7OCV` writer - Channel 7 Software Output Control Value"]
pub type Ch7ocvW<'a, REG> = crate::BitWriter<'a, REG, Ch7ocv>;
impl<'a, REG> Ch7ocvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force channel output low"]
    #[inline(always)]
    pub fn force_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7ocv::ForceLow)
    }
    #[doc = "Force channel output high"]
    #[inline(always)]
    pub fn force_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ch7ocv::ForceHigh)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch0oc(&self) -> Ch0ocR {
        Ch0ocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch1oc(&self) -> Ch1ocR {
        Ch1ocR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch2oc(&self) -> Ch2ocR {
        Ch2ocR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch3oc(&self) -> Ch3ocR {
        Ch3ocR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch4oc(&self) -> Ch4ocR {
        Ch4ocR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch5oc(&self) -> Ch5ocR {
        Ch5ocR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch6oc(&self) -> Ch6ocR {
        Ch6ocR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch7oc(&self) -> Ch7ocR {
        Ch7ocR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Software Output Control Value"]
    #[inline(always)]
    pub fn ch0ocv(&self) -> Ch0ocvR {
        Ch0ocvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Software Output Control Value"]
    #[inline(always)]
    pub fn ch1ocv(&self) -> Ch1ocvR {
        Ch1ocvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Software Output Control Value"]
    #[inline(always)]
    pub fn ch2ocv(&self) -> Ch2ocvR {
        Ch2ocvR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Software Output Control Value"]
    #[inline(always)]
    pub fn ch3ocv(&self) -> Ch3ocvR {
        Ch3ocvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Software Output Control Value"]
    #[inline(always)]
    pub fn ch4ocv(&self) -> Ch4ocvR {
        Ch4ocvR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Software Output Control Value"]
    #[inline(always)]
    pub fn ch5ocv(&self) -> Ch5ocvR {
        Ch5ocvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 6 Software Output Control Value"]
    #[inline(always)]
    pub fn ch6ocv(&self) -> Ch6ocvR {
        Ch6ocvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 7 Software Output Control Value"]
    #[inline(always)]
    pub fn ch7ocv(&self) -> Ch7ocvR {
        Ch7ocvR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch0oc(&mut self) -> Ch0ocW<'_, SwoctrlSpec> {
        Ch0ocW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch1oc(&mut self) -> Ch1ocW<'_, SwoctrlSpec> {
        Ch1ocW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch2oc(&mut self) -> Ch2ocW<'_, SwoctrlSpec> {
        Ch2ocW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch3oc(&mut self) -> Ch3ocW<'_, SwoctrlSpec> {
        Ch3ocW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch4oc(&mut self) -> Ch4ocW<'_, SwoctrlSpec> {
        Ch4ocW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch5oc(&mut self) -> Ch5ocW<'_, SwoctrlSpec> {
        Ch5ocW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 6 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch6oc(&mut self) -> Ch6ocW<'_, SwoctrlSpec> {
        Ch6ocW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 7 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch7oc(&mut self) -> Ch7ocW<'_, SwoctrlSpec> {
        Ch7ocW::new(self, 7)
    }
    #[doc = "Bit 8 - Channel 0 Software Output Control Value"]
    #[inline(always)]
    pub fn ch0ocv(&mut self) -> Ch0ocvW<'_, SwoctrlSpec> {
        Ch0ocvW::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 1 Software Output Control Value"]
    #[inline(always)]
    pub fn ch1ocv(&mut self) -> Ch1ocvW<'_, SwoctrlSpec> {
        Ch1ocvW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 Software Output Control Value"]
    #[inline(always)]
    pub fn ch2ocv(&mut self) -> Ch2ocvW<'_, SwoctrlSpec> {
        Ch2ocvW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 3 Software Output Control Value"]
    #[inline(always)]
    pub fn ch3ocv(&mut self) -> Ch3ocvW<'_, SwoctrlSpec> {
        Ch3ocvW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 4 Software Output Control Value"]
    #[inline(always)]
    pub fn ch4ocv(&mut self) -> Ch4ocvW<'_, SwoctrlSpec> {
        Ch4ocvW::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 5 Software Output Control Value"]
    #[inline(always)]
    pub fn ch5ocv(&mut self) -> Ch5ocvW<'_, SwoctrlSpec> {
        Ch5ocvW::new(self, 13)
    }
    #[doc = "Bit 14 - Channel 6 Software Output Control Value"]
    #[inline(always)]
    pub fn ch6ocv(&mut self) -> Ch6ocvW<'_, SwoctrlSpec> {
        Ch6ocvW::new(self, 14)
    }
    #[doc = "Bit 15 - Channel 7 Software Output Control Value"]
    #[inline(always)]
    pub fn ch7ocv(&mut self) -> Ch7ocvW<'_, SwoctrlSpec> {
        Ch7ocvW::new(self, 15)
    }
}
#[doc = "FTM Software Output Control\n\nYou can [`read`](crate::Reg::read) this register and get [`swoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwoctrlSpec;
impl crate::RegisterSpec for SwoctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swoctrl::R`](R) reader structure"]
impl crate::Readable for SwoctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`swoctrl::W`](W) writer structure"]
impl crate::Writable for SwoctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWOCTRL to value 0"]
impl crate::Resettable for SwoctrlSpec {}

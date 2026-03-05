#[doc = "Register `CLKDIV1` reader"]
pub type R = crate::R<Clkdiv1Spec>;
#[doc = "Register `CLKDIV1` writer"]
pub type W = crate::W<Clkdiv1Spec>;
#[doc = "Clock 4 output divider value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outdiv4 {
    #[doc = "0: Divide-by-1"]
    Div1 = 0,
    #[doc = "1: Divide-by-2"]
    Div2 = 1,
    #[doc = "2: Divide-by-3"]
    Div3 = 2,
    #[doc = "3: Divide-by-4"]
    Div4 = 3,
    #[doc = "4: Divide-by-5"]
    Div5 = 4,
    #[doc = "5: Divide-by-6"]
    Div6 = 5,
    #[doc = "6: Divide-by-7"]
    Div7 = 6,
    #[doc = "7: Divide-by-8"]
    Div8 = 7,
    #[doc = "8: Divide-by-9"]
    Div9 = 8,
    #[doc = "9: Divide-by-10"]
    Div10 = 9,
    #[doc = "10: Divide-by-11"]
    Div11 = 10,
    #[doc = "11: Divide-by-12"]
    Div12 = 11,
    #[doc = "12: Divide-by-13"]
    Div13 = 12,
    #[doc = "13: Divide-by-14"]
    Div14 = 13,
    #[doc = "14: Divide-by-15"]
    Div15 = 14,
    #[doc = "15: Divide-by-16"]
    Div16 = 15,
}
impl From<Outdiv4> for u8 {
    #[inline(always)]
    fn from(variant: Outdiv4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outdiv4 {
    type Ux = u8;
}
impl crate::IsEnum for Outdiv4 {}
#[doc = "Field `OUTDIV4` reader - Clock 4 output divider value"]
pub type Outdiv4R = crate::FieldReader<Outdiv4>;
impl Outdiv4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outdiv4 {
        match self.bits {
            0 => Outdiv4::Div1,
            1 => Outdiv4::Div2,
            2 => Outdiv4::Div3,
            3 => Outdiv4::Div4,
            4 => Outdiv4::Div5,
            5 => Outdiv4::Div6,
            6 => Outdiv4::Div7,
            7 => Outdiv4::Div8,
            8 => Outdiv4::Div9,
            9 => Outdiv4::Div10,
            10 => Outdiv4::Div11,
            11 => Outdiv4::Div12,
            12 => Outdiv4::Div13,
            13 => Outdiv4::Div14,
            14 => Outdiv4::Div15,
            15 => Outdiv4::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide-by-1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Outdiv4::Div1
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Outdiv4::Div2
    }
    #[doc = "Divide-by-3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Outdiv4::Div3
    }
    #[doc = "Divide-by-4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Outdiv4::Div4
    }
    #[doc = "Divide-by-5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Outdiv4::Div5
    }
    #[doc = "Divide-by-6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Outdiv4::Div6
    }
    #[doc = "Divide-by-7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == Outdiv4::Div7
    }
    #[doc = "Divide-by-8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Outdiv4::Div8
    }
    #[doc = "Divide-by-9"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == Outdiv4::Div9
    }
    #[doc = "Divide-by-10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Outdiv4::Div10
    }
    #[doc = "Divide-by-11"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == Outdiv4::Div11
    }
    #[doc = "Divide-by-12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == Outdiv4::Div12
    }
    #[doc = "Divide-by-13"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == Outdiv4::Div13
    }
    #[doc = "Divide-by-14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == Outdiv4::Div14
    }
    #[doc = "Divide-by-15"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == Outdiv4::Div15
    }
    #[doc = "Divide-by-16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Outdiv4::Div16
    }
}
#[doc = "Field `OUTDIV4` writer - Clock 4 output divider value"]
pub type Outdiv4W<'a, REG> = crate::FieldWriter<'a, REG, 4, Outdiv4, crate::Safe>;
impl<'a, REG> Outdiv4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide-by-1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div1)
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div2)
    }
    #[doc = "Divide-by-3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div3)
    }
    #[doc = "Divide-by-4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div4)
    }
    #[doc = "Divide-by-5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div5)
    }
    #[doc = "Divide-by-6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div6)
    }
    #[doc = "Divide-by-7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div7)
    }
    #[doc = "Divide-by-8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div8)
    }
    #[doc = "Divide-by-9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div9)
    }
    #[doc = "Divide-by-10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div10)
    }
    #[doc = "Divide-by-11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div11)
    }
    #[doc = "Divide-by-12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div12)
    }
    #[doc = "Divide-by-13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div13)
    }
    #[doc = "Divide-by-14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div14)
    }
    #[doc = "Divide-by-15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div15)
    }
    #[doc = "Divide-by-16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::Div16)
    }
}
#[doc = "Clock 2 output divider value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outdiv2 {
    #[doc = "0: Divide-by-1"]
    Div1 = 0,
    #[doc = "1: Divide-by-2"]
    Div2 = 1,
    #[doc = "2: Divide-by-3"]
    Div3 = 2,
    #[doc = "3: Divide-by-4"]
    Div4 = 3,
    #[doc = "4: Divide-by-5"]
    Div5 = 4,
    #[doc = "5: Divide-by-6"]
    Div6 = 5,
    #[doc = "6: Divide-by-7"]
    Div7 = 6,
    #[doc = "7: Divide-by-8"]
    Div8 = 7,
    #[doc = "8: Divide-by-9"]
    Div9 = 8,
    #[doc = "9: Divide-by-10"]
    Div10 = 9,
    #[doc = "10: Divide-by-11"]
    Div11 = 10,
    #[doc = "11: Divide-by-12"]
    Div12 = 11,
    #[doc = "12: Divide-by-13"]
    Div13 = 12,
    #[doc = "13: Divide-by-14"]
    Div14 = 13,
    #[doc = "14: Divide-by-15"]
    Div15 = 14,
    #[doc = "15: Divide-by-16"]
    Div16 = 15,
}
impl From<Outdiv2> for u8 {
    #[inline(always)]
    fn from(variant: Outdiv2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outdiv2 {
    type Ux = u8;
}
impl crate::IsEnum for Outdiv2 {}
#[doc = "Field `OUTDIV2` reader - Clock 2 output divider value"]
pub type Outdiv2R = crate::FieldReader<Outdiv2>;
impl Outdiv2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outdiv2 {
        match self.bits {
            0 => Outdiv2::Div1,
            1 => Outdiv2::Div2,
            2 => Outdiv2::Div3,
            3 => Outdiv2::Div4,
            4 => Outdiv2::Div5,
            5 => Outdiv2::Div6,
            6 => Outdiv2::Div7,
            7 => Outdiv2::Div8,
            8 => Outdiv2::Div9,
            9 => Outdiv2::Div10,
            10 => Outdiv2::Div11,
            11 => Outdiv2::Div12,
            12 => Outdiv2::Div13,
            13 => Outdiv2::Div14,
            14 => Outdiv2::Div15,
            15 => Outdiv2::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide-by-1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Outdiv2::Div1
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Outdiv2::Div2
    }
    #[doc = "Divide-by-3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Outdiv2::Div3
    }
    #[doc = "Divide-by-4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Outdiv2::Div4
    }
    #[doc = "Divide-by-5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Outdiv2::Div5
    }
    #[doc = "Divide-by-6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Outdiv2::Div6
    }
    #[doc = "Divide-by-7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == Outdiv2::Div7
    }
    #[doc = "Divide-by-8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Outdiv2::Div8
    }
    #[doc = "Divide-by-9"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == Outdiv2::Div9
    }
    #[doc = "Divide-by-10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Outdiv2::Div10
    }
    #[doc = "Divide-by-11"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == Outdiv2::Div11
    }
    #[doc = "Divide-by-12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == Outdiv2::Div12
    }
    #[doc = "Divide-by-13"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == Outdiv2::Div13
    }
    #[doc = "Divide-by-14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == Outdiv2::Div14
    }
    #[doc = "Divide-by-15"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == Outdiv2::Div15
    }
    #[doc = "Divide-by-16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Outdiv2::Div16
    }
}
#[doc = "Field `OUTDIV2` writer - Clock 2 output divider value"]
pub type Outdiv2W<'a, REG> = crate::FieldWriter<'a, REG, 4, Outdiv2, crate::Safe>;
impl<'a, REG> Outdiv2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide-by-1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div1)
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div2)
    }
    #[doc = "Divide-by-3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div3)
    }
    #[doc = "Divide-by-4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div4)
    }
    #[doc = "Divide-by-5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div5)
    }
    #[doc = "Divide-by-6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div6)
    }
    #[doc = "Divide-by-7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div7)
    }
    #[doc = "Divide-by-8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div8)
    }
    #[doc = "Divide-by-9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div9)
    }
    #[doc = "Divide-by-10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div10)
    }
    #[doc = "Divide-by-11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div11)
    }
    #[doc = "Divide-by-12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div12)
    }
    #[doc = "Divide-by-13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div13)
    }
    #[doc = "Divide-by-14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div14)
    }
    #[doc = "Divide-by-15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div15)
    }
    #[doc = "Divide-by-16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv2::Div16)
    }
}
#[doc = "Clock 1 output divider value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outdiv1 {
    #[doc = "0: Divide-by-1"]
    Div1 = 0,
    #[doc = "1: Divide-by-2"]
    Div2 = 1,
    #[doc = "2: Divide-by-3"]
    Div3 = 2,
    #[doc = "3: Divide-by-4"]
    Div4 = 3,
    #[doc = "4: Divide-by-5"]
    Div5 = 4,
    #[doc = "5: Divide-by-6"]
    Div6 = 5,
    #[doc = "6: Divide-by-7"]
    Div7 = 6,
    #[doc = "7: Divide-by-8"]
    Div8 = 7,
    #[doc = "8: Divide-by-9"]
    Div9 = 8,
    #[doc = "9: Divide-by-10"]
    Div10 = 9,
    #[doc = "10: Divide-by-11"]
    Div11 = 10,
    #[doc = "11: Divide-by-12"]
    Div12 = 11,
    #[doc = "12: Divide-by-13"]
    Div13 = 12,
    #[doc = "13: Divide-by-14"]
    Div14 = 13,
    #[doc = "14: Divide-by-15"]
    Div15 = 14,
    #[doc = "15: Divide-by-16"]
    Div16 = 15,
}
impl From<Outdiv1> for u8 {
    #[inline(always)]
    fn from(variant: Outdiv1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outdiv1 {
    type Ux = u8;
}
impl crate::IsEnum for Outdiv1 {}
#[doc = "Field `OUTDIV1` reader - Clock 1 output divider value"]
pub type Outdiv1R = crate::FieldReader<Outdiv1>;
impl Outdiv1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outdiv1 {
        match self.bits {
            0 => Outdiv1::Div1,
            1 => Outdiv1::Div2,
            2 => Outdiv1::Div3,
            3 => Outdiv1::Div4,
            4 => Outdiv1::Div5,
            5 => Outdiv1::Div6,
            6 => Outdiv1::Div7,
            7 => Outdiv1::Div8,
            8 => Outdiv1::Div9,
            9 => Outdiv1::Div10,
            10 => Outdiv1::Div11,
            11 => Outdiv1::Div12,
            12 => Outdiv1::Div13,
            13 => Outdiv1::Div14,
            14 => Outdiv1::Div15,
            15 => Outdiv1::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide-by-1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Outdiv1::Div1
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Outdiv1::Div2
    }
    #[doc = "Divide-by-3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Outdiv1::Div3
    }
    #[doc = "Divide-by-4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Outdiv1::Div4
    }
    #[doc = "Divide-by-5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Outdiv1::Div5
    }
    #[doc = "Divide-by-6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Outdiv1::Div6
    }
    #[doc = "Divide-by-7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == Outdiv1::Div7
    }
    #[doc = "Divide-by-8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Outdiv1::Div8
    }
    #[doc = "Divide-by-9"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == Outdiv1::Div9
    }
    #[doc = "Divide-by-10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Outdiv1::Div10
    }
    #[doc = "Divide-by-11"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == Outdiv1::Div11
    }
    #[doc = "Divide-by-12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == Outdiv1::Div12
    }
    #[doc = "Divide-by-13"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == Outdiv1::Div13
    }
    #[doc = "Divide-by-14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == Outdiv1::Div14
    }
    #[doc = "Divide-by-15"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == Outdiv1::Div15
    }
    #[doc = "Divide-by-16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Outdiv1::Div16
    }
}
#[doc = "Field `OUTDIV1` writer - Clock 1 output divider value"]
pub type Outdiv1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Outdiv1, crate::Safe>;
impl<'a, REG> Outdiv1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide-by-1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div1)
    }
    #[doc = "Divide-by-2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div2)
    }
    #[doc = "Divide-by-3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div3)
    }
    #[doc = "Divide-by-4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div4)
    }
    #[doc = "Divide-by-5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div5)
    }
    #[doc = "Divide-by-6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div6)
    }
    #[doc = "Divide-by-7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div7)
    }
    #[doc = "Divide-by-8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div8)
    }
    #[doc = "Divide-by-9"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div9)
    }
    #[doc = "Divide-by-10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div10)
    }
    #[doc = "Divide-by-11"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div11)
    }
    #[doc = "Divide-by-12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div12)
    }
    #[doc = "Divide-by-13"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div13)
    }
    #[doc = "Divide-by-14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div14)
    }
    #[doc = "Divide-by-15"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div15)
    }
    #[doc = "Divide-by-16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::Div16)
    }
}
impl R {
    #[doc = "Bits 16:19 - Clock 4 output divider value"]
    #[inline(always)]
    pub fn outdiv4(&self) -> Outdiv4R {
        Outdiv4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Clock 2 output divider value"]
    #[inline(always)]
    pub fn outdiv2(&self) -> Outdiv2R {
        Outdiv2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Clock 1 output divider value"]
    #[inline(always)]
    pub fn outdiv1(&self) -> Outdiv1R {
        Outdiv1R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Clock 4 output divider value"]
    #[inline(always)]
    pub fn outdiv4(&mut self) -> Outdiv4W<'_, Clkdiv1Spec> {
        Outdiv4W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Clock 2 output divider value"]
    #[inline(always)]
    pub fn outdiv2(&mut self) -> Outdiv2W<'_, Clkdiv1Spec> {
        Outdiv2W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Clock 1 output divider value"]
    #[inline(always)]
    pub fn outdiv1(&mut self) -> Outdiv1W<'_, Clkdiv1Spec> {
        Outdiv1W::new(self, 28)
    }
}
#[doc = "System Clock Divider Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkdiv1Spec;
impl crate::RegisterSpec for Clkdiv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv1::R`](R) reader structure"]
impl crate::Readable for Clkdiv1Spec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv1::W`](W) writer structure"]
impl crate::Writable for Clkdiv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKDIV1 to value 0"]
impl crate::Resettable for Clkdiv1Spec {}

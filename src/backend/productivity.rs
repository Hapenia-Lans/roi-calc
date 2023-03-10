// Copyright 2023 Hapenia Lans
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    collections::{hash_map::Iter, HashMap},
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use super::{money, recipe::Item};

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
pub struct Speed(f64);

impl Speed {
    pub fn monthly(&self) -> f64 {
        self.0 * 30.0
    }
}

impl From<f64> for Speed {
    fn from(value: f64) -> Self {
        Speed(value)
    }
}

impl Add for Speed {
    type Output = Speed;

    fn add(self, rhs: Self) -> Self::Output {
        Speed(self.0 + rhs.0)
    }
}

impl Sub for Speed {
    type Output = Speed;

    fn sub(self, rhs: Self) -> Self::Output {
        Speed(self.0 - rhs.0)
    }
}

impl Mul<f64> for Speed {
    type Output = Speed;

    fn mul(self, rhs: f64) -> Self::Output {
        Speed(self.0 * rhs)
    }
}

impl AddAssign for Speed {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl SubAssign for Speed {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl MulAssign<f64> for Speed {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs
    }
}

#[derive(Debug)]
pub struct Productivity {
    inner: HashMap<Item, Speed>,
}

impl Productivity {
    pub fn new(inner: HashMap<Item, Speed>) -> Self {
        Productivity { inner }
    }

    pub fn iter(&self) -> Iter<Item, Speed> {
        self.inner.iter()
    }

    pub fn estimated_monthly_sales(&self) -> money::Money {
        let val = self
            .inner
            .iter()
            .filter(|(_, speed)| speed.0 > 0.0)
            .map(|(item, speed)| item.price().value() as f64 * speed.monthly())
            .sum();
        money::Money::from(f64::round(val) as i64)
    }
    pub fn estimated_monthly_material_cost(&self) -> money::Money {
        let val = self
            .iter()
            .filter(|(_, speed)| speed.0 < 0.0)
            .map(|(item, speed)| item.price().value() as f64 * -speed.monthly())
            .sum();
        money::Money::from(f64::round(val) as i64)
    }
}

impl Sum for Productivity {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Productivity::new(HashMap::new()), |a, b| a + b)
    }
}

// ??? Productivity ??????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????

impl Add for Productivity {
    type Output = Productivity;

    /// ???????????????????????????????????? `Productivity` ??????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        rhs.inner.into_iter().for_each(|(item, productivity)| {
            result
                .inner
                .entry(item)
                .and_modify(|prod| *prod += productivity)
                .or_insert(productivity);
        });
        result
    }
}

impl AddAssign for Productivity {
    fn add_assign(&mut self, rhs: Self) {
        rhs.inner.into_iter().for_each(|(item, productivity)| {
            self.inner
                .entry(item)
                .and_modify(|prod| *prod += productivity)
                .or_insert(productivity);
        });
    }
}

impl Sub for Productivity {
    type Output = Productivity;
    /// ?????????????????? `a + (b * -1)`?????????????????????
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        rhs.inner.into_iter().for_each(|(item, productivity)| {
            result
                .inner
                .entry(item)
                .and_modify(|prod| *prod -= productivity)
                .or_insert(productivity);
        });
        result
    }
}

impl Mul<f64> for Productivity {
    type Output = Productivity;

    /// ????????????????????????????????????????????????????????? `rhs`???
    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = self;
        result.inner.values_mut().for_each(|x| *x *= rhs);
        result
    }
}

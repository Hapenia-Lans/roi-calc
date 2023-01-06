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
    collections::HashMap,
    iter::Sum,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use super::{money, recipe::Item};

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
pub struct Speed(f64);

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

pub struct Productivity {
    inner: HashMap<Item, Speed>,
}

impl Productivity {
    pub fn new(inner: HashMap<Item, Speed>) -> Self {
        Productivity { inner }
    }

    pub fn earning(&self) -> money::Money {
        todo!()
    }
}

impl Sum for Productivity {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Productivity::new(HashMap::new()), |a, b| a + b)
    }
}

// 为 Productivity 实现这些运算虽然会造成额外开销（例如多次加减本可以在一次循环内解决），但由于数据量较小，为了方便，这样做值得

impl Add for Productivity {
    type Output = Productivity;

    /// 将两个产能相加。对于两个 `Productivity` 共有的物品，速率相加；不共有的物品，插入新键值对。注意：如果同一物品产能相加结果为零，仍然不会删除这个物品！
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
    /// 在结果上等于 `a + (b * -1)`，但比它更快。
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

    /// 简单数乘。将所有生产速率乘以给定浮点数 `rhs`。
    fn mul(self, rhs: f64) -> Self::Output {
        let mut result = self;
        result.inner.values_mut().for_each(|x| *x *= rhs);
        result
    }
}

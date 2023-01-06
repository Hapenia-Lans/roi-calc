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

use std::ops::{Add, AddAssign, Mul, Sub};

use serde_derive::Deserialize;

pub enum Unit {
    One,
    Kilo,
    Million,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deserialize, Debug)]
pub struct Money(u64);

impl ToString for Money {
    fn to_string(&self) -> String {
        let unit = match self.0 {
            0..=1_000 => Unit::One,
            1_001..=1_000_000 => Unit::Kilo,
            _ => Unit::Million,
        };
        match unit {
            Unit::One => format!("${}", self.0),
            Unit::Kilo => format!("${}K", self.0 / 1_000),
            Unit::Million => todo!("${}M", self.0 / 1_000_000),
        }
    }
}

impl From<u64> for Money {
    fn from(value: u64) -> Self {
        Money(value)
    }
}

impl Money {
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Add<Money> for Money {
    type Output = Money;

    fn add(self, rhs: Money) -> Self::Output {
        Money(self.0 + rhs.0)
    }
}

impl AddAssign<Money> for Money {
    fn add_assign(&mut self, rhs: Money) {
        self.0 += rhs.0;
    }
}

impl Sub<Money> for Money {
    type Output = Money;

    fn sub(self, rhs: Money) -> Self::Output {
        Money(self.0 - rhs.0)
    }
}

impl Mul<Money> for u64 {
    type Output = Money;

    fn mul(self, rhs: Money) -> Self::Output {
        Money(self * rhs.0)
    }
}

impl Mul<u64> for Money {
    type Output = Money;

    fn mul(self, rhs: u64) -> Self::Output {
        Money(self.0 * rhs)
    }
}

impl Mul<Money> for f64 {
    type Output = Money;

    fn mul(self, rhs: Money) -> Self::Output {
        Money(f64::round(self) as u64 * rhs.0)
    }
}

impl Mul<f64> for Money {
    type Output = Money;

    fn mul(self, rhs: f64) -> Self::Output {
        Money(f64::round(rhs) as u64 * self.0)
    }
}

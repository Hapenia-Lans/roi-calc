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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Deserialize, Debug)]
pub struct Money(u64);

impl ToString for Money {
    fn to_string(&self) -> String {
        match self.0 {
            0..=999 => format!("${}", self.0),
            1_000..=999_999 => format!("${:.2}K", self.0 as f64 / 1_000 as f64),
            _ => format!("${:.2}M", self.0 as f64 / 1_000_000 as f64),
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

#[test]
fn test_money_to_string() {
    let m_one = Money(394);
    let m_kilo = Money(1024);
    let m_million = Money(1_900_030);
    assert_eq!(m_one.to_string(), "$394");
    assert_eq!(m_kilo.to_string(), "$1.02K");
    assert_eq!(m_million.to_string(), "$1.90M");
}

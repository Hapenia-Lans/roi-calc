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
pub struct Money(i64);

impl Money {
    pub fn zero() -> Money {
        Money(0)
    }
}

impl ToString for Money {
    fn to_string(&self) -> String {
        match self.0 {
            i64::MIN..=-1_000_000 => format!("-${:.2}M", -self.0 as f64 / 1_000_000 as f64),
            -999_999..=-1_000 => format!("-${:.2}K", -self.0 as f64 / 1_000 as f64),
            -999..=-1 => format!("-${}", -self.0),
            0..=999 => format!("${}", self.0),
            1_000..=999_999 => format!("${:.2}K", self.0 as f64 / 1_000 as f64),
            1_000_000..=i64::MAX => format!("${:.2}M", self.0 as f64 / 1_000_000 as f64),
        }
    }
}

impl From<i64> for Money {
    fn from(value: i64) -> Self {
        Money(value)
    }
}

impl Money {
    pub fn value(&self) -> i64 {
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

impl Mul<Money> for i64 {
    type Output = Money;

    fn mul(self, rhs: Money) -> Self::Output {
        Money(self * rhs.0)
    }
}

impl Mul<i64> for Money {
    type Output = Money;

    fn mul(self, rhs: i64) -> Self::Output {
        Money(self.0 * rhs)
    }
}

impl Mul<Money> for f64 {
    type Output = Money;

    fn mul(self, rhs: Money) -> Self::Output {
        Money(f64::round(rhs.0 as f64 * self) as i64)
    }
}

impl Mul<f64> for Money {
    type Output = Money;

    fn mul(self, rhs: f64) -> Self::Output {
        Money(f64::round(self.0 as f64 * rhs) as i64)
    }
}

#[test]
fn test_money_to_string() {
    let m_mo = Money(-231);
    let m_mk = Money(-2_131);
    let m_mm = Money(-12_931_904);
    let m_one = Money(394);
    let m_kilo = Money(1_024);
    let m_million = Money(1_900_030);
    assert_eq!(m_mo.to_string(), "-$231");
    assert_eq!(m_mk.to_string(), "-$2.13K");
    assert_eq!(m_mm.to_string(), "-$12.93M");
    assert_eq!(m_one.to_string(), "$394");
    assert_eq!(m_kilo.to_string(), "$1.02K");
    assert_eq!(m_million.to_string(), "$1.90M");
}

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
    ops::{Add, Mul, Sub},
};

pub mod money;

pub mod building;

pub mod recipe;

use recipe::{Item, Recipe};

pub enum Error {
    SimulatorCreationFailed(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<building::Error> for Error {
    fn from(value: building::Error) -> Self {
        match value {
            building::Error::InfoNotFoundError => {
                Self::SimulatorCreationFailed(String::from("Building infomation not found."))
            }
        }
    }
}

#[derive(Clone)]
pub enum Condition {
    Collector {
        building_type: building::Type,
        recipe: &'static Recipe, // 对全局的 RECIPES 的引用
        worker_wage: building::WorkerWage,
        collector_amount: building::OutbuildingAmount,
    },
    Farm {
        building_type: building::Type,
        recipe: &'static Recipe, // 对全局的 RECIPES 的引用
        worker_wage: building::WorkerWage,
        field_amount: building::OutbuildingAmount,
    },
    Factory {
        building_type: building::Type,
        recipe: &'static Recipe, // 对全局的 RECIPES 的引用
        worker_wage: building::WorkerWage,
    },
}

// 模拟报告。
pub struct Report {}

pub struct Simulator {
    buildings: Vec<Box<dyn building::Building>>,
}

impl Simulator {
    pub fn from_conditions(conditions: &[Condition]) -> Result<Self> {
        for cond in conditions {
            match cond {
                Condition::Collector {
                    building_type,
                    recipe,
                    worker_wage,
                    collector_amount,
                } => {
                    building::CollectorPlant::create(
                        *building_type,
                        *collector_amount,
                        recipe,
                        *worker_wage,
                    )?;
                }
                Condition::Farm {
                    building_type,
                    recipe,
                    worker_wage,
                    field_amount,
                } => todo!(),
                Condition::Factory {
                    building_type,
                    recipe,
                    worker_wage,
                } => todo!(),
            }
        }
        todo!()
    }
}

pub struct Productivity {
    inner: HashMap<Item, f64>,
}

impl Productivity {
    pub fn new(inner: HashMap<Item, f64>) -> Self {
        Productivity { inner }
    }
}

// 为 Productivity 实现这些运算虽然会造成额外开销（例如多次加减本可以在一次循环内解决），但由于数据量较小，为了方便，这样做值得

impl Add for Productivity {
    type Output = Productivity;

    /// 将两个产能相加。对于两个 `Productivity` 共有的物品，速率相加；不共有的物品，插入新键值对。注意：如果同一物品产能相加结果为零，仍然不会删除这个物品！
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        rhs.inner
            .into_iter()
            .for_each(|(item, productivity)| match result.inner.get_mut(&item) {
                Some(value) => {
                    *value += productivity;
                }
                None => {
                    result.inner.insert(item, productivity);
                }
            });
        result
    }
}

impl Sub for Productivity {
    type Output = Productivity;
    /// 在结果上等于 `a + (b * -1)`，但比它更快。
    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self;
        rhs.inner
            .into_iter()
            .for_each(|(item, productivity)| match result.inner.get_mut(&item) {
                Some(value) => {
                    *value -= productivity;
                }
                None => {
                    result.inner.insert(item, -productivity);
                }
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

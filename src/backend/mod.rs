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

pub mod money;

pub mod building;

pub mod recipe;

pub mod productivity;

use std::collections::HashMap;

use building::types;

use recipe::Id;

#[derive(Debug)]
pub struct Error(Box<ErrorImpl>);

#[derive(Debug)]
pub enum ErrorImpl {
    SimulatorCreationFailed(building::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<building::Error> for Error {
    fn from(value: building::Error) -> Self {
        Self(Box::new(ErrorImpl::SimulatorCreationFailed(value)))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {:?}", self.0)
    }
}

impl std::error::Error for Error {}

#[derive(Clone)]
pub enum Condition {
    Collector {
        building_type: types::Collector,
        recipe_id: Id,
        worker_wage: building::WorkerWage,
        collector_amount: building::OutbuildingAmount,
        amount: u8,
    },
    Farm {
        building_type: types::Farm,
        recipe_id: Id,
        worker_wage: building::WorkerWage,
        field_amount: building::OutbuildingAmount,
        amount: u8,
    },
    Factory {
        building_type: types::Factory,
        recipe_id: Id,
        worker_wage: building::WorkerWage,
        amount: u8,
    },
}

#[derive(Debug)]
// 模拟报告。
pub struct Report {
    productivity: productivity::Productivity,
    total_buildings: HashMap<types::Type, u32>,
    total_price: money::Money,                     // 建筑总价格
    estimated_monthly_upkeep: money::Money,        // 预计每月维护费
    estimated_monthly_material_cost: money::Money, // 预计购买每月原料价格
    estimated_monthly_sales: money::Money,         // 预计月销售额
}

impl Report {
    pub fn total_price(&self) -> money::Money {
        self.total_price
    }
    pub fn productivity(&self) -> &productivity::Productivity {
        &self.productivity
    }
    pub fn total_buildings(&self) -> &HashMap<types::Type, u32> {
        &self.total_buildings
    }
    pub fn monthly_sales(&self) -> money::Money {
        self.estimated_monthly_sales
    }
    pub fn monthly_upkeep(&self) -> money::Money {
        self.estimated_monthly_upkeep
    }
    pub fn monthly_profit(&self) -> money::Money {
        self.estimated_monthly_sales
            - self.estimated_monthly_upkeep
            - self.estimated_monthly_material_cost
    }
    pub fn monthly_material_cost(&self) -> money::Money {
        self.estimated_monthly_material_cost
    }
    pub fn profit_rate(&self) -> f64 {
        self.monthly_profit().value() as f64 / self.estimated_monthly_sales.value() as f64
    }
}

pub struct Simulator {
    buildings: Vec<Box<dyn building::Building>>,
}

impl Simulator {
    pub fn from_conditions(conditions: &[Condition]) -> Result<Self> {
        let mut buildings: Vec<Box<dyn building::Building>> = vec![];
        for cond in conditions {
            match cond {
                Condition::Collector {
                    building_type,
                    recipe_id,
                    worker_wage,
                    collector_amount,
                    amount,
                } => {
                    for _ in 0..*amount {
                        buildings.push(Box::new(building::CollectorPlant::create(
                            *building_type,
                            *collector_amount,
                            recipe::get(recipe_id),
                            *worker_wage,
                        )?));
                    }
                }
                Condition::Farm {
                    building_type,
                    recipe_id,
                    worker_wage,
                    field_amount,
                    amount,
                } => {
                    for _ in 0..*amount {
                        buildings.push(Box::new(building::Farm::create(
                            *building_type,
                            *field_amount,
                            recipe::get(recipe_id),
                            *worker_wage,
                        )?))
                    }
                }
                Condition::Factory {
                    building_type,
                    recipe_id,
                    worker_wage,
                    amount,
                } => {
                    for _ in 0..*amount {
                        buildings.push(Box::new(building::Factory::create(
                            *building_type,
                            recipe::get(recipe_id),
                            *worker_wage,
                        )?))
                    }
                }
            }
        }
        // buildings.push(building);
        Ok(Simulator { buildings })
    }

    pub fn simulate(&self) -> Report {
        let mut productivity: productivity::Productivity =
            productivity::Productivity::new(HashMap::new());
        let mut total_buildings: HashMap<types::Type, u32> = HashMap::new();
        let mut total_price = money::Money::zero();
        let mut estimated_monthly_upkeep = money::Money::zero();
        for i in self.buildings.iter() {
            let plant_type = i.plant_type();
            let prod = i.productivity();
            total_price += i.price();
            estimated_monthly_upkeep += i.upkeep();
            productivity += prod;
            total_buildings
                .entry(plant_type)
                .and_modify(|amount| *amount += 1)
                .or_insert(1);
        }
        let estimated_monthly_sales = productivity.estimated_monthly_sales();
        let estimated_monthly_material_cost = productivity.estimated_monthly_material_cost();
        Report {
            productivity,
            total_buildings,
            total_price,
            estimated_monthly_upkeep,
            estimated_monthly_sales,
            estimated_monthly_material_cost,
        }
    }
}

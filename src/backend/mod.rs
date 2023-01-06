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

use recipe::Recipe;

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
pub struct Report {
    productivity: productivity::Productivity,
    total_buildings: HashMap<building::Type, u32>,
    total_price: money::Money,
    total_upkeep: money::Money,
}

impl Report {
    pub fn productivity(&self) -> &productivity::Productivity {
        &self.productivity
    }
    pub fn total_buildings(&self) -> &HashMap<building::Type, u32> {
        &self.total_buildings
    }
    pub fn total_price(&self) -> money::Money {
        self.total_price
    }
    pub fn total_upkeep(&self) -> money::Money {
        self.total_upkeep
    }
}

pub struct Simulator {
    buildings: Vec<Box<dyn building::Building>>,
}

impl Simulator {
    pub fn from_conditions(conditions: &[Condition]) -> Result<Self> {
        let mut buildings: Vec<Box<dyn building::Building>> = vec![];
        for cond in conditions {
            let building: Box<dyn building::Building> = match cond {
                Condition::Collector {
                    building_type,
                    recipe,
                    worker_wage,
                    collector_amount,
                } => Box::new(building::CollectorPlant::create(
                    *building_type,
                    *collector_amount,
                    recipe,
                    *worker_wage,
                )?),
                Condition::Farm {
                    building_type,
                    recipe,
                    worker_wage,
                    field_amount,
                } => Box::new(building::Farm::create(
                    *building_type,
                    *field_amount,
                    recipe,
                    *worker_wage,
                )?),
                Condition::Factory {
                    building_type,
                    recipe,
                    worker_wage,
                } => Box::new(building::Factory::create(
                    *building_type,
                    recipe,
                    *worker_wage,
                )?),
            };
            buildings.push(building);
        }
        Ok(Simulator { buildings })
    }

    pub fn simulate(&self) -> Report {
        let mut productivity: productivity::Productivity =
            productivity::Productivity::new(HashMap::new());
        let mut total_buildings: HashMap<building::Type, u32> = HashMap::new();
        let mut total_price = money::Money::from(0);
        let mut total_upkeep = money::Money::from(0);
        for i in self.buildings.iter() {
            let plant_type = i.plant_type();
            let prod = i.productivity();
            total_price += i.price();
            total_upkeep += i.upkeep();
            productivity += prod;
            total_buildings
                .entry(plant_type)
                .and_modify(|amount| *amount += 1)
                .or_insert(1);
        }
        Report {
            productivity,
            total_buildings,
            total_price,
            total_upkeep,
        }
    }
}

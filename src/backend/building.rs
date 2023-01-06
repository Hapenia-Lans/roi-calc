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

use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde_derive::Deserialize;

use super::{
    money,
    productivity::Productivity,
    recipe::{Id, Recipe},
};

pub enum Error {
    InfoNotFoundError,
}

pub static INFOS: Lazy<HashMap<Type, Info>> = Lazy::new(|| {
    const INFO_RAW: &'static str = include_str!("../../data/building_info.yaml");
    serde_yaml::from_str(INFO_RAW).unwrap()
});

#[derive(Debug, Deserialize)]
pub enum Info {
    Collector(CollectorInfo),
    Farm(FarmInfo),
    Factory(FactoryInfo),
}

#[derive(Debug, Deserialize)]
pub struct CollectorInfo {
    pub recipes: Vec<Id>,
    pub price: money::Money,
    pub upkeep: money::Money,
    pub collector_price: money::Money,
    pub collector_upkeep: money::Money,
}

#[derive(Debug, Deserialize)]
pub struct FarmInfo {
    pub recipes: Vec<Id>,
    pub price: money::Money,
    pub upkeep: money::Money,
    pub field_price: money::Money,
    pub field_upkeep: money::Money,
}

#[derive(Debug, Deserialize)]
pub struct FactoryInfo {
    pub recipes: Vec<Id>,
    pub price: money::Money,
    pub upkeep: money::Money,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize)]
pub enum Type {
    // Collectors,
    Lumberyard,       // 伐木场
    IronMine,         // 铁矿
    CoalMine,         // 煤矿
    CopperMine,       // 铜矿
    WaterSiphon,      // 水厂
    FishermansPier,   // 渔民码头
    GasPump,          // 天然气厂
    OilDrill,         // 石油钻井
    SandCollector,    // 采沙场
    WaterWell,        // 水井
    OffShoreOilDrill, // 海上石油钻井平台
    // Farms
    CropFarm,      // 农场
    LiveStockFarm, // 牧场
    Orchard,       // 果园
    Plantation,    // 种植园
    // Light Industry
    DrinksFactory,        // 饮料厂
    PreservationFactory,  // 预制食品厂
    FoodFactory,          // 食品厂
    BreweryAndDistillery, // 酒厂
    PaperMill,            // 造纸厂
    ToyFactory,           // 玩具厂
    TextileFactory,       // 纺织厂
    CarpentryCenter,      // 木工房
    // Heavy Industry,
    AutomotiveFactory,    // 汽车厂
    HomeGoodsFactory,     // 家具厂
    GlassworksAndSmelter, // 玻璃厂
    PetrochemicalPlant,   // 化工厂
    // Prototype Factories
    ComputerMegaFactory,   // 计算机大型工厂
    MealMegaFactory,       // 食品大型工厂
    AutomobileMegaFactory, // 汽车大型工厂
}

/// 附属建筑数量。
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum OutbuildingAmount {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum WorkerWage {
    Percent25,
    Percent50,
    Percent75,
    Percent100,
    Percent125,
    Percent150,
    Percent200,
}

impl WorkerWage {
    pub fn value(&self) -> f64 {
        match self {
            WorkerWage::Percent25 => 0.25,
            WorkerWage::Percent50 => 0.50,
            WorkerWage::Percent75 => 0.75,
            WorkerWage::Percent100 => 1.00,
            WorkerWage::Percent125 => 1.25,
            WorkerWage::Percent150 => 1.50,
            WorkerWage::Percent200 => 2.00,
        }
    }
}

pub trait Building {
    fn plant_type(&self) -> Type;

    /// 计算本建筑的产能。
    fn productivity(&self) -> Productivity;

    fn price(&self) -> money::Money;

    fn upkeep(&self) -> money::Money;
}

pub struct CollectorPlant {
    plant_type: Type, // 类别，这里的前缀仅仅是为了避免与保留字重名。对于不重名的情况，按照风格指南是不应该有前缀的。
    collector_amount: OutbuildingAmount, // 收集器数量。
    recipe: &'static Recipe, // 配方
    worker_wage: WorkerWage, // 工人工资
    info: &'static CollectorInfo,
}

impl CollectorPlant {
    pub fn create(
        plant_type: Type,
        collector_amount: OutbuildingAmount,
        recipe: &'static Recipe,
        worker_wage: WorkerWage,
    ) -> Result<Self, Error> {
        let Some(Info::Collector(info)) = INFOS.get(&plant_type) else {
                return Err(Error::InfoNotFoundError);
            };
        Ok(CollectorPlant {
            plant_type,
            collector_amount,
            recipe,
            worker_wage,
            info,
        })
    }
}

impl Building for CollectorPlant {
    fn productivity(&self) -> Productivity {
        self.recipe.productivity()
            * (self.worker_wage.value() * (self.collector_amount as u8) as f64)
    }

    fn price(&self) -> money::Money {
        self.info.price + self.info.collector_price * self.collector_amount as u64
    }

    fn upkeep(&self) -> money::Money {
        self.info.upkeep
            + self.info.collector_upkeep * self.collector_amount as u64 * self.worker_wage.value()
    }

    fn plant_type(&self) -> Type {
        self.plant_type
    }
}

pub struct Farm {
    plant_type: Type,
    field_amount: OutbuildingAmount,
    recipe: &'static Recipe, // 配方
    worker_wage: WorkerWage, // 工人工资
    info: &'static FarmInfo,
}

impl Farm {
    pub fn create(
        plant_type: Type,
        field_amount: OutbuildingAmount,
        recipe: &'static Recipe,
        worker_wage: WorkerWage,
    ) -> Result<Self, Error> {
        let Some(Info::Farm(info)) = INFOS.get(&plant_type) else {
                return Err(Error::InfoNotFoundError);
            };
        Ok(Farm {
            plant_type,
            field_amount,
            recipe,
            worker_wage,
            info,
        })
    }
}

impl Building for Farm {
    fn plant_type(&self) -> Type {
        self.plant_type
    }

    fn productivity(&self) -> Productivity {
        self.recipe.productivity() * (self.field_amount as u8 as f64 * self.worker_wage.value())
    }

    fn price(&self) -> money::Money {
        self.info.price + (self.field_amount as u64 * self.info.field_price)
    }

    fn upkeep(&self) -> money::Money {
        self.info.upkeep
            + (self.field_amount as u64 * self.info.field_upkeep) * self.worker_wage.value()
    }
}

pub struct Factory {
    plant_type: Type,
    recipe: &'static Recipe, // 配方
    worker_wage: WorkerWage, // 工人工资
    info: &'static FactoryInfo,
}

impl Factory {
    pub fn create(
        plant_type: Type,
        recipe: &'static Recipe,
        worker_wage: WorkerWage,
    ) -> Result<Self, Error> {
        let Some(Info::Factory(info)) = INFOS.get(&plant_type) else {
                return Err(Error::InfoNotFoundError);
            };
        Ok(Factory {
            plant_type,
            recipe,
            worker_wage,
            info,
        })
    }
}

impl Building for Factory {
    fn plant_type(&self) -> Type {
        self.plant_type
    }

    fn productivity(&self) -> Productivity {
        self.recipe.productivity() * self.worker_wage.value()
    }

    fn price(&self) -> money::Money {
        self.info.price
    }

    fn upkeep(&self) -> money::Money {
        self.info.upkeep
    }
}

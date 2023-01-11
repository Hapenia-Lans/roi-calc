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

use std::fmt::Display;

use super::{money, productivity::Productivity, recipe::Recipe};

use enum_iterator_derive::Sequence;

#[derive(Debug)]
pub enum Error {
    InfoNotFoundError,
}

pub mod info {
    use super::super::recipe::Id;

    use super::types;
    use super::{money, types::Type};
    use std::collections::HashMap;

    use once_cell::sync::Lazy;
    use serde_derive::Deserialize;

    pub static INFOS: Lazy<HashMap<Type, Info>> = Lazy::new(|| {
        lazy_static_include_str! {
            INFO_RAW => "data/building_info.yaml",
        }
        serde_yaml::from_str(&INFO_RAW).unwrap()
    });

    #[derive(Debug, Deserialize)]
    pub enum Info {
        Collector(Collector),
        Farm(Farm),
        Factory(Factory),
    }

    impl From<Collector> for Info {
        fn from(value: Collector) -> Self {
            Info::Collector(value)
        }
    }

    impl From<Farm> for Info {
        fn from(value: Farm) -> Self {
            Info::Farm(value)
        }
    }

    impl From<Factory> for Info {
        fn from(value: Factory) -> Self {
            Info::Factory(value)
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Collector {
        pub recipes: Vec<Id>,
        pub price: money::Money,
        pub upkeep: money::Money,
        pub collector_price: money::Money,
        pub collector_upkeep: money::Money,
    }

    #[derive(Debug, Deserialize)]
    pub struct Farm {
        pub recipes: Vec<Id>,
        pub price: money::Money,
        pub upkeep: money::Money,
        pub field_price: money::Money,
        pub field_upkeep: money::Money,
    }

    #[derive(Debug, Deserialize)]
    pub struct Factory {
        pub recipes: Vec<Id>,
        pub price: money::Money,
        pub upkeep: money::Money,
    }

    pub fn get(building_type: Type) -> &'static Info {
        let Some(info) = INFOS.get(&building_type) else {
            unreachable!()
        };
        info
    }

    pub fn get_collector_info(collector_type: types::Collector) -> &'static Collector {
        let Info::Collector(x) = get(Type::Collector(collector_type)) else {unreachable!()};
        x
    }

    pub fn get_factory_info(factory_type: types::Factory) -> &'static Factory {
        let Info::Factory(x) = get(Type::Factory(factory_type)) else {unreachable!()};
        x
    }

    pub fn get_farm_info(farm_type: types::Farm) -> &'static Farm {
        let Info::Farm(x) = get(Type::Farm(farm_type)) else {unreachable!()};
        x
    }

    #[test]
    fn info_test() {
        let info = get(Type::Collector(types::Collector::Lumberyard));
        println!("{:?}", info);
    }
}

use info::Info;

pub mod types {

    use enum_iterator_derive::Sequence;
    use serde_derive::Deserialize;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, PartialOrd, Ord)]
    pub enum Type {
        Collector(Collector),
        Farm(Farm),
        Factory(Factory),
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Sequence, PartialOrd, Ord)]
    pub enum Collector {
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
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Sequence, PartialOrd, Ord)]
    pub enum Farm {
        CropFarm,      // 农场
        LiveStockFarm, // 牧场
        Orchard,       // 果园
        Plantation,    // 种植园}
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize, Sequence, PartialOrd, Ord)]
    pub enum Factory {
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

    // impl From<Collector> for Type {
    //     fn from(value: Collector) -> Self {
    //         Type::Collector(value)
    //     }
    // }
    // impl From<Farm> for Type {
    //     fn from(value: Farm) -> Self {
    //         Type::Farm(value)
    //     }
    // }
    // impl From<Factory> for Type {
    //     fn from(value: Factory) -> Self {
    //         Type::Factory(value)
    //     }
    // }
}

use types::Type;

/// 附属建筑数量。
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Sequence)]
pub enum OutbuildingAmount {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Sequence)]
pub enum WorkerWage {
    Percent25,
    Percent50,
    Percent75,
    Percent100,
    Percent125,
    Percent150,
    Percent200,
}

impl Display for WorkerWage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WorkerWage::Percent25 => "25%",
                WorkerWage::Percent50 => "50%",
                WorkerWage::Percent75 => "75%",
                WorkerWage::Percent100 => "100%",
                WorkerWage::Percent125 => "125%",
                WorkerWage::Percent150 => "150%",
                WorkerWage::Percent200 => "200%",
            }
        )
    }
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
    fn plant_type(&self) -> types::Type;

    /// 计算本建筑的产能。
    fn productivity(&self) -> Productivity;

    fn price(&self) -> money::Money;

    fn upkeep(&self) -> money::Money;
}

pub struct CollectorPlant {
    plant_type: types::Collector, // 类别，这里的前缀仅仅是为了避免与保留字重名。对于不重名的情况，按照风格指南是不应该有前缀的。
    collector_amount: OutbuildingAmount, // 收集器数量。
    recipe: &'static Recipe,      // 配方
    worker_wage: WorkerWage,      // 工人工资
    info: &'static info::Collector,
}

impl CollectorPlant {
    pub fn create(
        plant_type: types::Collector,
        collector_amount: OutbuildingAmount,
        recipe: &'static Recipe,
        worker_wage: WorkerWage,
    ) -> Result<Self, Error> {
        let Some(Info::Collector(info)) = info::INFOS.get(&Type::Collector(plant_type)) else {
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
        self.info.price + self.info.collector_price * self.collector_amount as i64
    }

    fn upkeep(&self) -> money::Money {
        (self.info.upkeep + self.info.collector_upkeep * self.collector_amount as i64)
            * self.worker_wage.value()
    }

    fn plant_type(&self) -> types::Type {
        types::Type::Collector(self.plant_type)
    }
}

pub struct Farm {
    plant_type: types::Farm,
    field_amount: OutbuildingAmount,
    recipe: &'static Recipe, // 配方
    worker_wage: WorkerWage, // 工人工资
    info: &'static info::Farm,
}

impl Farm {
    pub fn create(
        plant_type: types::Farm,
        field_amount: OutbuildingAmount,
        recipe: &'static Recipe,
        worker_wage: WorkerWage,
    ) -> Result<Self, Error> {
        let Some(Info::Farm(info)) = info::INFOS.get(&Type::Farm(plant_type)) else {
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
    fn plant_type(&self) -> types::Type {
        Type::Farm(self.plant_type)
    }

    fn productivity(&self) -> Productivity {
        self.recipe.productivity() * (self.field_amount as u8 as f64 * self.worker_wage.value())
    }

    fn price(&self) -> money::Money {
        self.info.price + (self.field_amount as i64 * self.info.field_price)
    }

    fn upkeep(&self) -> money::Money {
        (self.info.upkeep + (self.field_amount as i64 * self.info.field_upkeep))
            * self.worker_wage.value()
    }
}

pub struct Factory {
    plant_type: types::Factory,
    recipe: &'static Recipe, // 配方
    worker_wage: WorkerWage, // 工人工资
    info: &'static info::Factory,
}

impl Factory {
    pub fn create(
        plant_type: types::Factory,
        recipe: &'static Recipe,
        worker_wage: WorkerWage,
    ) -> Result<Self, Error> {
        let Some(Info::Factory(info)) = info::INFOS.get(&Type::Factory(plant_type)) else {
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
        Type::Factory(self.plant_type)
    }

    fn productivity(&self) -> Productivity {
        self.recipe.productivity() * self.worker_wage.value()
    }

    fn price(&self) -> money::Money {
        self.info.price
    }

    fn upkeep(&self) -> money::Money {
        self.info.upkeep * self.worker_wage.value()
    }
}

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

use enum_iterator_derive::Sequence;

use super::{
    money::Money,
    productivity::{Productivity, Speed},
};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct Id(String);

impl ToString for Id {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub static RECIPES: Lazy<HashMap<Id, Recipe>> = Lazy::new(|| {
    const RECIPE_RAW: &'static str = include_str!("../../data/recipes.yaml");
    match serde_yaml::from_str(RECIPE_RAW) {
        Ok(x) => x,
        Err(e) => unreachable!("{}", e),
    }
});

static PRICES: Lazy<HashMap<Item, Money>> = Lazy::new(|| {
    const PRICES_RAW: &'static str = include_str!("../../data/global_market_prices.yaml");
    match serde_yaml::from_str(PRICES_RAW) {
        Ok(x) => x,
        Err(e) => unreachable!("{}", e),
    }
});

#[derive(PartialEq, Debug, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize, Sequence)]
pub enum Item {
    Water,              // 水
    Sand,               // 沙子
    Wood,               // 木材
    Fish,               // 鱼
    IronOre,            // 铁矿
    Coal,               // 煤炭
    Copper,             // 铜矿
    Gas,                // 天然气
    Oil,                // 石油
    Oranges,            // 橘子
    Apples,             // 苹果
    Grapes,             // 葡萄
    RawRubber,          // 生橡胶
    Olives,             // 橄榄
    Wheat,              // 小麦
    Hops,               // 啤酒花
    Potato,             // 土豆
    Vegetables,         // 蔬菜
    Berries,            // 浆果
    Cotton,             // 棉花
    Sugar,              // 糖
    Cocoa,              // 可可豆
    ChickenMeat,        // 鸡肉
    Eggs,               // 鸡蛋
    Beef,               // 牛肉
    Leather,            // 皮革
    Milk,               // 牛奶
    Mutton,             // 羊肉
    Wool,               // 羊毛
    WoodenPlanks,       // 木板
    HardCider,          // 苹果酒
    Wine,               // 红酒
    Yeast,              // 酵母
    AppleSmoothie,      // 苹果冰沙
    BerrySmoothie,      // 浆果冰沙
    GrapeJuice,         // 葡萄汁
    OrangeJuice,        // 橘子汁
    SodaWater,          // 苏打水
    ChocolateBar,       // 巧克力板
    Flour,              // 面粉
    OliveOil,           // 橄榄油
    CopperTubing,       // 铜管
    CopperWire,         // 铜线
    Glass,              // 玻璃
    Steel,              // 钢铁
    HeavyPulp,          // 浓纸浆
    Ink,                // 墨水
    Parchment,          // 纸卷
    Chemicals,          // 化学品
    Plastic,            // 塑料
    RefinedOil,         // 成品油
    Rubber,             // 橡胶
    Soup,               // 汤
    Dye,                // 染料
    Fibers,             // 纤维
    LargeFurnitureBase, // 家具底座（大）
    SmallFurnitureBase, // 家具底座（小）
    WoodenBarrels,      // 木桶
    Beer,               // 啤酒
    Biofuel,            // 生物燃料
    Vodka,              // 伏特加
    BeefStew,           // 炖牛肉
    Cheese,             // 奶酪
    ChocolateCake,      // 巧克力蛋糕
    Dough,              // 面团
    Bottles,            // 瓶子
    Cans,               // 罐头
    Ceramic,            // 陶瓷
    SteelFrame,         // 钢架
    GlassTubes,         // 玻璃管
    SteelBarrels,       // 钢桶
    Diodes,             // 二极管
    LightBulb,          // 灯泡
    Radiator,           // 散热器
    Refrigerator,       // 冰箱
    Stovetop,           // 炉子
    Cardboard,          // 硬纸板
    PrintedPaper,       // 打印纸
    Adhesive,           // 胶水
    Paints,             // 油漆
    RubberTubes,        // 橡胶管
    Tire,               // 轮胎
    Buttons,            // 按钮
    PlasticCutlery,     // 塑料餐具
    BagOfChips,         // 薯片
    CannedFish,         // 鱼罐头
    CannedMutton,       // 羊肉罐头
    ChickenSoup,        // 鸡肉汤
    HeavyFabric,        // 厚面料
    LightFabric,        // 轻薄面料
    CarSeat,            // 汽车座椅
    LeatherFurniture,   // 皮革家具
    OfficeFurniture,    // 办公座椅
    PlasticFurniture,   // 塑料家具
    Brandy,             // 白兰地
    BarleyWhiskey,      // 大麦威士忌
    OrangeSoda,         // 橘子苏打
    Waffles,            // 华夫饼
    BerryPie,           // 浆果派
    Burgers,            // 汉堡包
    Pizza,              // 披萨
    Capacitors,         // 电容器
    EngineBlock,        // 发动机
    Headlights,         // 头灯
    Oven,               // 烤箱
    RadioReceiver,      // 收音机
    Telephones,         // 电话
    Books,              // 书
    DeluxeBooks,        // 精装书
    Newspapers,         // 报纸
    PunchCards,         // 打孔卡
    ThinCardboard,      // 薄纸板
    InteriorLining,     // 里衬
    SummerClothes,      // 夏装
    WinterClothes,      // 冬装
    Napkins,            // 餐巾
    ExteriorBody,       // 汽车外壳
    InteriorBody,       // 汽车内饰
    BodyChassis,        // 汽车底盘
    Axles,              // 汽车轴承
    CombustionEngine,   // 内燃机
    RollingChassis,     // 驱动轮
    Interface,          // 接口
    BinarySwitcher,     // 二进制开关
    Processor,          // 处理器
    ComputerMemory,     // 计算机内存
    FiredChicken,       // 炒鸡肉
    CookedVegetables,   // 熟蔬菜
    ChickenDinner,      // 鸡肉晚餐
    DinnerContainer,    // 晚餐容器
    Car,                // 汽车
    FirstComputer,      // 第一台电脑
    PremadeDinner,      // 预制晚餐
}

impl Item {
    pub fn price(&self) -> Money {
        match PRICES.get(&self) {
            Some(x) => *x,
            None => unreachable!(),
        }
    }
}

pub fn get_recipe(id: &Id) -> &'static Recipe {
    match RECIPES.get(id) {
        Some(x) => x,
        None => unreachable!(),
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Slot(Item, u64);

#[derive(Debug, Deserialize)]
pub struct Recipe {
    inputs: [Option<Slot>; 3],
    outputs: [Option<Slot>; 3],
    day_to_gen: u16,
}

impl Recipe {
    pub fn day_to_gen(&self) -> u16 {
        self.day_to_gen
    }

    pub fn productivity(&self) -> Productivity {
        let inputs_productivity = Productivity::new(
            self.inputs
                .iter()
                .filter_map(|&slot| slot)
                .map(|Slot(item, amount)| (item, Speed::from(amount as f64)))
                .collect(),
        );
        let outputs_productivity = Productivity::new(
            self.outputs
                .iter()
                .filter_map(|&slot| slot)
                .map(|Slot(item, amount)| (item, Speed::from(amount as f64)))
                .collect(),
        );
        (outputs_productivity - inputs_productivity) * (1.0 / self.day_to_gen as f64)
    }
}

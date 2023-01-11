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

use crate::backend::{
    self,
    building::{
        self,
        info::{self, Info},
        types::{self, Type},
    },
    productivity, recipe, Condition,
};
use egui_extras::{Column, TableBuilder};
use itertools::Itertools;

const CROSS: &str = "🗙";

#[derive(Default)]
pub struct App {
    simulation_conditions: Vec<backend::Condition>,
    simulation_report: Option<backend::Report>,
    counter: u128,
}

impl App {
    pub fn new(_: &eframe::CreationContext) -> Self {
        Self::default()
    }

    pub fn simulate(&mut self) {
        let simulator = backend::Simulator::from_conditions(&self.simulation_conditions);
        if let Ok(sim) = simulator {
            self.simulation_report = Some(sim.simulate());
        } else {
            todo!();
        }
    }

    pub fn add_collector(&mut self, building_type: types::Collector) {
        let recipe_id = building::info::get_collector_info(building_type).recipes[0].clone();
        self.simulation_conditions.push(Condition::Collector {
            building_type,
            recipe_id,
            worker_wage: building::WorkerWage::Percent100,
            collector_amount: building::OutbuildingAmount::Five,
            amount: 1,
        });
    }

    pub fn add_farm(&mut self, building_type: types::Farm) {
        let recipe_id = building::info::get_farm_info(building_type).recipes[0].clone();
        self.simulation_conditions.push(Condition::Farm {
            building_type,
            recipe_id,
            worker_wage: building::WorkerWage::Percent100,
            field_amount: building::OutbuildingAmount::Five,
            amount: 1,
        })
    }

    pub fn add_factory(&mut self, building_type: types::Factory) {
        let recipe_id = building::info::get_factory_info(building_type).recipes[0].clone();
        self.simulation_conditions.push(Condition::Factory {
            building_type,
            recipe_id,
            worker_wage: building::WorkerWage::Percent100,
            amount: 1,
        })
    }
}

trait View {
    type App: eframe::App;
    fn show(&mut self, app: &mut App, ctx: &egui::Context) {
        Self::surrounding_panels_id()
            .into_iter()
            .enumerate()
            .for_each(|(i, idx)| {
                let Some(id) = idx else { return };
                match i {
                    0 => {
                        egui::TopBottomPanel::bottom(id.clone()).show(ctx, |ui| {
                            self.show_bottom_panel(app, ui);
                        });
                    }
                    1 => {
                        egui::SidePanel::left(id.clone()).show(ctx, |ui| {
                            self.show_left_panel(app, ui);
                        });
                    }
                    2 => {
                        egui::SidePanel::right(id.clone()).show(ctx, |ui| {
                            self.show_right_panel(app, ui);
                        });
                    }
                    3 => {
                        egui::TopBottomPanel::top(id.clone()).show(ctx, |ui| {
                            self.show_top_panel(app, ui);
                        });
                    }
                    _ => unreachable!(),
                }
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_central_panel(app, ui);
        });
    }
    /// ["Bottom", "Left", "Right", Top]
    fn surrounding_panels_id() -> [Option<String>; 4] {
        [None, None, None, None]
    }
    fn show_central_panel(&mut self, _app: &mut App, _ui: &mut egui::Ui);
    fn show_top_panel(&mut self, _app: &mut App, _ui: &mut egui::Ui) {}
    fn show_bottom_panel(&mut self, _app: &mut App, _ui: &mut egui::Ui) {}
    fn show_left_panel(&mut self, _app: &mut App, _ui: &mut egui::Ui) {}
    fn show_right_panel(&mut self, _app: &mut App, _ui: &mut egui::Ui) {}
}

pub struct ProductivityView {
    mark_as_delete: Option<usize>,
}

impl ProductivityView {
    fn new() -> ProductivityView {
        ProductivityView {
            mark_as_delete: None,
        }
    }
    fn show_body_content(
        &mut self,
        body: &mut egui_extras::TableBody,
        i: usize,
        cond: &mut backend::Condition,
    ) {
        body.row(25.0, |row| match cond {
            backend::Condition::Collector {
                building_type,
                recipe_id,
                worker_wage,
                collector_amount,
                amount,
            } => {
                self.show_collector_cond(
                    i,
                    row,
                    building_type,
                    recipe_id,
                    worker_wage,
                    collector_amount,
                    amount,
                );
            }
            backend::Condition::Farm {
                building_type,
                recipe_id,
                worker_wage,
                field_amount,
                amount,
            } => {
                self.show_farm_cond(
                    i,
                    row,
                    building_type,
                    recipe_id,
                    worker_wage,
                    field_amount,
                    amount,
                );
            }
            backend::Condition::Factory {
                building_type,
                recipe_id,
                worker_wage,
                amount,
            } => {
                self.show_factory_cond(i, row, building_type, recipe_id, worker_wage, amount);
            }
        });
    }

    fn show_collector_cond(
        &mut self,
        i: usize,
        mut row: egui_extras::TableRow,
        building_type: &mut types::Collector,
        recipe_id: &mut recipe::Id,
        worker_wage: &mut building::WorkerWage,
        collector_amount: &mut building::OutbuildingAmount,
        amount: &mut u8,
    ) {
        row.col(|ui| {
            ui.label("Collector");
        });
        row.col(|ui| {
            ui.label(format!("{:?}", building_type));
        });
        row.col(|ui| {
            Self::show_recipe_combobox(
                ui,
                info::get(Type::Collector(*building_type)),
                recipe_id,
                i,
            );
        });
        row.col(|ui| {
            Self::show_worker_wage_combobox(ui, worker_wage, i);
        });
        row.col(|ui| {
            Self::show_outbuilding_amount_combobox(ui, collector_amount, i);
        });
        row.col(|ui| {
            ui.add(egui::DragValue::new(amount).clamp_range(1..=255));
        });
        row.col(|ui| {
            if ui.button(CROSS).clicked() {
                self.mark_as_delete = Some(i);
            }
        });
    }

    fn show_farm_cond(
        &mut self,
        i: usize,
        mut row: egui_extras::TableRow,
        building_type: &mut types::Farm,
        recipe_id: &mut recipe::Id,
        worker_wage: &mut building::WorkerWage,
        field_amount: &mut building::OutbuildingAmount,
        amount: &mut u8,
    ) {
        row.col(|ui| {
            ui.label("Farm");
        });
        row.col(|ui| {
            ui.label(format!("{:?}", building_type));
        });
        row.col(|ui| {
            Self::show_recipe_combobox(
                ui,
                building::info::get(Type::Farm(*building_type)),
                recipe_id,
                i,
            );
        });
        row.col(|ui| {
            Self::show_worker_wage_combobox(ui, worker_wage, i);
        });
        row.col(|ui| {
            Self::show_outbuilding_amount_combobox(ui, field_amount, i);
        });
        row.col(|ui| {
            ui.add(egui::DragValue::new(amount).clamp_range(1..=255));
        });
        row.col(|ui| {
            if ui.button(CROSS).clicked() {
                self.mark_as_delete = Some(i);
            }
        });
    }

    fn show_factory_cond(
        &mut self,
        i: usize,
        mut row: egui_extras::TableRow,
        building_type: &mut types::Factory,
        recipe_id: &mut recipe::Id,
        worker_wage: &mut building::WorkerWage,
        amount: &mut u8,
    ) {
        row.col(|ui| {
            ui.label("Factory");
        });
        row.col(|ui| {
            ui.label(format!("{:?}", building_type));
        });
        row.col(|ui| {
            Self::show_recipe_combobox(
                ui,
                building::info::get(Type::Factory(*building_type)),
                recipe_id,
                i,
            );
        });
        row.col(|ui| {
            Self::show_worker_wage_combobox(ui, worker_wage, i);
        });
        row.col(|ui| {
            ui.label("N/A");
        });
        row.col(|ui| {
            ui.add(egui::DragValue::new(amount).clamp_range(1..=255));
        });
        row.col(|ui| {
            if ui.button(CROSS).clicked() {
                self.mark_as_delete = Some(i);
            }
        });
    }

    fn show_worker_wage_combobox(
        ui: &mut egui::Ui,
        worker_wage: &mut building::WorkerWage,
        idx: usize,
    ) {
        eframe::egui::ComboBox::from_id_source(format!("worker_wage:{}", idx))
            .selected_text(format!("{}", worker_wage))
            .show_ui(ui, |ui| {
                for wage in enum_iterator::all::<building::WorkerWage>() {
                    ui.selectable_value(worker_wage, wage, format!("{}", wage));
                }
            });
    }

    fn show_recipe_combobox(
        ui: &mut egui::Ui,
        info: &Info,
        recipe_id: &mut recipe::Id,
        idx: usize,
    ) {
        eframe::egui::ComboBox::from_id_source(format!("recipe:{}", idx))
            .selected_text(recipe_id.to_string())
            .show_ui(ui, |ui| match info {
                Info::Collector(info) => {
                    for id in &info.recipes {
                        let response = ui
                            .selectable_label(recipe_id == id, id.to_string())
                            .clicked();
                        if response {
                            *recipe_id = id.clone();
                        }
                    }
                }
                Info::Farm(info) => {
                    for id in &info.recipes {
                        let response = ui
                            .selectable_label(recipe_id == id, id.to_string())
                            .clicked();
                        if response {
                            *recipe_id = id.clone();
                        }
                    }
                }
                Info::Factory(info) => {
                    for id in &info.recipes {
                        let response = ui
                            .selectable_label(recipe_id == id, id.to_string())
                            .clicked();
                        if response {
                            *recipe_id = id.clone();
                        }
                    }
                }
            });
    }

    fn show_outbuilding_amount_combobox(
        ui: &mut egui::Ui,
        outbuilding_amount: &mut building::OutbuildingAmount,
        idx: usize,
    ) {
        let text = format!("{}", *outbuilding_amount as u8);
        eframe::egui::ComboBox::from_id_source(format!("outbuilding:{}", idx))
            .selected_text(&text)
            .show_ui(ui, |ui| {
                for amount in enum_iterator::all::<building::OutbuildingAmount>() {
                    ui.selectable_value(outbuilding_amount, amount, format!("{}", amount as u8));
                }
            });
    }
}

impl View for ProductivityView {
    type App = crate::app::App;

    fn surrounding_panels_id() -> [Option<String>; 4] {
        [
            Some(String::from("Condition table")),
            Some(String::from("Simulation Result")),
            None,
            None,
        ]
    }

    fn show_bottom_panel(&mut self, app: &mut App, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            const CHUNK_SIZE: usize = 6;
            let types = enum_iterator::all::<types::Collector>().collect::<Vec<_>>();
            for chunk in &types.into_iter().chunks(CHUNK_SIZE) {
                ui.vertical(|ui| {
                    for building_type in chunk {
                        if ui.small_button(format!("{:?}", building_type)).clicked() {
                            app.add_collector(building_type);
                        }
                    }
                });
            }
            let types = enum_iterator::all::<types::Farm>().collect::<Vec<_>>();
            for chunk in &types.into_iter().chunks(CHUNK_SIZE) {
                ui.vertical(|ui| {
                    for building_type in chunk {
                        if ui.small_button(format!("{:?}", building_type)).clicked() {
                            app.add_farm(building_type);
                        }
                    }
                });
            }
            let types = enum_iterator::all::<types::Factory>().collect::<Vec<_>>();
            for chunk in &types.into_iter().chunks(CHUNK_SIZE) {
                ui.vertical(|ui| {
                    for building_type in chunk {
                        if ui.small_button(format!("{:?}", building_type)).clicked() {
                            app.add_factory(building_type);
                        }
                    }
                });
            }
        });
    }

    fn show_left_panel(&mut self, app: &mut App, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let Some(report) = &app.simulation_report else { return };
            ui.heading("Simulation Report");
            ui.separator();
            // 生产力汇报
            let mut prodpair = report
                .productivity()
                .iter()
                .collect::<Vec<(&recipe::Item, &productivity::Speed)>>();
            prodpair.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
            prodpair.iter().for_each(|(&k, &v)| {
                ui.label(format!("{:?}: {:.2} per month", k, v.monthly()));
            });

            ui.separator();
            ui.strong("total buildings:");
            ui.separator();
            let mut prodpair = report
                .total_buildings()
                .iter()
                .collect::<Vec<(&Type, &u32)>>();
            prodpair.sort_by(|a, b| b.1.cmp(a.1));
            prodpair.iter().for_each(|(&k, &v)| {
                ui.label(format!("  {:?}: {}", k, v));
            });
            ui.heading("Economics");
            ui.separator();
            ui.strong(format!("total price: {}", report.total_price()));
            ui.label(format!("monthly upkeep: {}", report.monthly_upkeep()));
            ui.label(format!(
                "monthly material cost: {}",
                report.monthly_material_cost()
            ));
            ui.label(format!("monthly sales: {}", report.monthly_sales()));
            ui.strong(format!("monthly profit: {}", report.monthly_profit()));
            ui.strong(format!("profit rate: {:.2}%", report.profit_rate() * 100.0));
        });
    }

    fn show_central_panel(&mut self, app: &mut App, ui: &mut egui::Ui) {
        const HEADERS: [&'static str; 7] = [
            "Type",
            "Name",
            "Recipe",
            "Worker Wage",
            "Outbuilding Amount",
            "Amount",
            "",
        ];
        ui.vertical_centered(|ui| {
            TableBuilder::new(ui)
                .auto_shrink([false, false])
                .columns(Column::auto().at_least(80.0), 2)
                .column(Column::auto().at_least(150.0))
                .columns(Column::auto().at_least(100.0), 2)
                .striped(true)
                .columns(Column::auto(), 2)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .header(50.0, |mut header| {
                    for i in HEADERS {
                        header.col(|ui| {
                            ui.strong(i);
                        });
                    }
                })
                .body(|mut body| {
                    self.mark_as_delete = None;
                    for (i, cond) in app.simulation_conditions.iter_mut().enumerate() {
                        self.show_body_content(&mut body, i, cond);
                    }
                    if let Some(idx) = self.mark_as_delete {
                        app.simulation_conditions.remove(idx);
                    }
                });
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let mut view = ProductivityView::new();
        view.show(self, ctx);
        if self.counter % 3 == 0 {
            self.simulate();
            self.counter = 0;
            // println!("{:#?}", self.simulation_report);
        }
        self.counter += 1;
    }
}

// backend::recipe::RECIPES.iter().for_each(|(name, recipe)| {
//     println!("name: {:?}, recipe: {:?}", name, recipe);
// });
// backend::building::info::INFOS.iter().for_each(|(b_type, inf)| {
//     println!("type: {:?}, info: {:?}", b_type, inf);
// });

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

    pub fn add_collector(&mut self) {
        self.simulation_conditions.push(Condition::Collector {
            building_type: types::Collector::WaterWell,
            recipe_id: recipe::Id("Water-Well".to_string()),
            worker_wage: building::WorkerWage::Percent100,
            collector_amount: building::OutbuildingAmount::Five,
        });
    }

    pub fn add_farm(&mut self) {
        self.simulation_conditions.push(Condition::Farm {
            building_type: types::Farm::CropFarm,
            recipe_id: recipe::Id("Wheat".to_string()),
            worker_wage: building::WorkerWage::Percent100,
            field_amount: building::OutbuildingAmount::Five,
        })
    }

    pub fn add_factory(&mut self) {
        todo!()
    }
}

trait View {
    fn show(app: &mut App, ui: &mut egui::Ui);
}

pub struct ProductivityView;

impl ProductivityView {
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
    fn show(app: &mut App, ui: &mut egui::Ui) {
        const HEADERS: [&'static str; 6] = [
            "Type",
            "Name",
            "Recipe",
            "Worker Wage",
            "Outbuilding Amount",
            "",
        ];
        ui.allocate_ui_with_layout(
            egui::Vec2 { x: 600.0, y: 400.0 },
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                TableBuilder::new(ui)
                    .columns(Column::auto().at_least(80.0), 2)
                    .columns(Column::auto().at_least(100.0), 3)
                    .striped(true)
                    .column(Column::auto())
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .header(50.0, |mut header| {
                        for i in HEADERS {
                            header.col(|ui| {
                                ui.strong(i);
                            });
                        }
                    })
                    .body(|mut body| {
                        let mut rm_content = None;
                        for (i, cond) in app.simulation_conditions.iter_mut().enumerate() {
                            body.row(25.0, |mut row| match cond {
                                backend::Condition::Collector {
                                    building_type,
                                    recipe_id,
                                    worker_wage,
                                    collector_amount,
                                } => {
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
                                        Self::show_outbuilding_amount_combobox(
                                            ui,
                                            collector_amount,
                                            i,
                                        );
                                    });
                                    row.col(|ui| {
                                        if ui.button("🗙").clicked() {
                                            rm_content = Some(i);
                                        }
                                    });
                                }
                                backend::Condition::Farm {
                                    building_type,
                                    recipe_id,
                                    worker_wage,
                                    field_amount,
                                } => {
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
                                        if ui.button("🗙").clicked() {
                                            rm_content = Some(i);
                                        }
                                    });
                                }
                                backend::Condition::Factory {
                                    building_type,
                                    recipe_id,
                                    worker_wage,
                                } => {
                                    row.col(|ui| {
                                        ui.label("Farm");
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
                                        if ui.button("🗙").clicked() {
                                            rm_content = Some(i);
                                        }
                                    });
                                }
                            });
                        }
                        if let Some(idx) = rm_content {
                            app.simulation_conditions.remove(idx);
                        }
                    });
            },
        );
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                if ui.button("Add Collector").clicked() {
                    app.add_collector();
                }
                if ui.button("Add Farm").clicked() {
                    app.add_farm();
                }
                if ui.button("Add Factory").clicked() {
                    app.add_factory();
                }
            });
            ui.with_layout(
                egui::Layout::with_main_justify(
                    egui::Layout::top_down_justified(egui::Align::TOP),
                    true,
                ),
                |ui| {
                    if let Some(report) = &app.simulation_report {
                        ui.heading("Simulation Report");
                        ui.separator();
                        // 生产力汇报
                        let mut prodpair =
                            report
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
                        ui.label(format!(
                            "monthly upkeep: {}",
                            report.monthly_upkeep()
                        ));
                        ui.label(format!(
                            "monthly material cost: {}",
                            report.monthly_material_cost()
                        ));
                        ui.label(format!(
                            "monthly sales: {}",
                            report.monthly_sales()
                        ));
                        ui.strong(format!(
                            "monthly profit: {}",
                            report.monthly_profit()
                        ));
                        ui.strong(format!("profit rate: {:.2}%", report.profit_rate() * 100.0));
                    }
                },
            );
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ProductivityView::show(self, ui);
            if self.counter % 3 == 0 {
                self.simulate();
                self.counter = 0;
                // println!("{:#?}", self.simulation_report);
            }
            self.counter += 1;
        });
    }
}

// backend::recipe::RECIPES.iter().for_each(|(name, recipe)| {
//     println!("name: {:?}, recipe: {:?}", name, recipe);
// });
// backend::building::info::INFOS.iter().for_each(|(b_type, inf)| {
//     println!("type: {:?}, info: {:?}", b_type, inf);
// });

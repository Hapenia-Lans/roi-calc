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

use crate::backend;
use crate::backend::{building, recipe};
use egui_extras::{Column, TableBuilder};

#[derive(Default)]
pub struct App {
    simulation_conditions: Vec<backend::Condition>,
    simulation_report: Option<backend::Report>,
}

impl App {
    pub fn new(_: &eframe::CreationContext) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        eframe::egui::Window::new("Simulation Conditions").show(ctx, |ui| {
            TableBuilder::new(ui)
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .header(50.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Type");
                    });
                    header.col(|ui| {
                        ui.strong("Name");
                    });
                    header.col(|ui| {
                        ui.strong("Recipe");
                    });
                    header.col(|ui| {
                        ui.strong("Worker Wage");
                    });
                    header.col(|ui| {
                        ui.strong("Outbuilding Amount");
                    });
                    header.col(|ui| {
                        ui.strong("🗙");
                    });
                })
                .body(|mut body| {
                    for cond in &mut self.simulation_conditions {
                        body.row(25.0, |mut row| match cond {
                            backend::Condition::Collector {
                                building_type,
                                recipe_id,
                                worker_wage,
                                collector_amount,
                            } => {
                                row.col(|mut ui| {
                                    ui.label("Collector");
                                });
                                row.col(|mut ui| {
                                    ui.label(format!("{:?}", building_type));
                                });
                                row.col(|mut ui| {
                                    eframe::egui::ComboBox::from_label("")
                                        .selected_text(recipe_id.to_string())
                                        .show_ui(ui, |ui| {
                                            let info = building::info::INFOS.get(
                                                &building::types::Type::Collector(*building_type),
                                            );
                                            match info {
                                                Some(building::info::Info::Collector(info)) => {
                                                    for id in &info.recipes {
                                                        if ui
                                                            .selectable_label(
                                                                recipe_id == id,
                                                                id.to_string(),
                                                            )
                                                            .clicked()
                                                        {
                                                            *recipe_id = id.clone();
                                                        }
                                                    }
                                                }
                                                _ => unreachable!(),
                                            }
                                        });
                                });
                                row.col(|ui| {
                                    eframe::egui::ComboBox::from_label("")
                                        .selected_text(format!("{}", worker_wage))
                                        .show_ui(ui, |ui| {
                                            for wage in enum_iterator::all::<building::WorkerWage>() {
                                                ui.selectable_value(
                                                    worker_wage,
                                                    wage,
                                                    format!("{}", wage),
                                                );
                                            }
                                        });
                                });
                                row.col(|ui| {
                                    // TODO: 完成 Collector Amount
                                });
                            }
                            backend::Condition::Farm {
                                building_type,
                                recipe_id,
                                worker_wage,
                                field_amount,
                            } => todo!(),
                            backend::Condition::Factory {
                                building_type,
                                recipe_id,
                                worker_wage,
                            } => todo!(),
                        });
                    }
                });
        });
    }
}

// backend::recipe::RECIPES.iter().for_each(|(name, recipe)| {
//     println!("name: {:?}, recipe: {:?}", name, recipe);
// });
// backend::building::info::INFOS.iter().for_each(|(b_type, inf)| {
//     println!("type: {:?}, info: {:?}", b_type, inf);
// });

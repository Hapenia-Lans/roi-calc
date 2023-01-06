

mod backend;
mod app;

const APP_NAME: &'static str = "ROI Calculator";

fn main() {
    let options = eframe::NativeOptions {default_theme: eframe::Theme::Dark, ..Default::default()};
    eframe::run_native(APP_NAME, options, Box::new(|cc| Box::new(app::App::new(cc))));
}

#[macro_use]
extern crate lazy_static_include;

mod app;
mod backend;

const APP_NAME: &'static str = "ROI Calculator";

fn main() {
    let options = eframe::NativeOptions {
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| {
            let mut app = app::App::new(cc);
            app.simulate();
            Box::new(app)
        }),
    );
}

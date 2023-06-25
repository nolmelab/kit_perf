use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    init_logger();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "kit-perf", 
        options, 
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {

}

impl Default for MyApp {
    fn default() -> Self {
        Self {

        }
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Practice egui and master it");


        });

    }
}


fn init_logger() {
    let mut builder = env_logger::Builder::from_default_env();

    builder
    .filter(None, log::LevelFilter::Info)
    .init(); 


}
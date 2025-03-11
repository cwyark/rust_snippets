use eframe::egui;

// reference: https://blog.csdn.net/qq_33446100/article/details/132792406

struct MyApp {}

impl MyApp {
    fn new(cc: &eframe::CreationContext) -> Self {
        MyApp {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("TopPanel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Node Graph Example");

            // paint a node
            let node =
                egui::Rect::from_center_size(egui::pos2(10.0, 100.0), egui::vec2(10.0, 10.0));
        });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "Node Graph",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

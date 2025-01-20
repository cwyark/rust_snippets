use eframe::egui;

struct MyApp {
    label: String,
    value: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            label: "hello!".to_string(),
            value: 2.7,
        }
    }
}

impl MyApp {
    fn new(cc: &eframe::CreationContext) -> Self {
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("panel example");
            ui.horizontal(|ui| {
                ui.label("writing something");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));

            if ui.button("increment").clicked() {
                self.value += 1.0
            }

            ui.separator();
        });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    eframe::run_native(
        "gui panel example",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

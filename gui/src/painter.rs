use eframe::egui::{self, Color32};

// reference: https://blog.csdn.net/qq_33446100/article/details/132778862

struct MyApp {}

impl MyApp {
    fn new(cc: &eframe::CreationContext) -> Self {
        MyApp {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("test");
            // get a available space from ui
            let response = ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());

            let shape = ui.painter().add(egui::Shape::Noop);

            let rounding = egui::Rounding::same(0.4);

            let body_rect =
                egui::Rect::from_min_size(egui::pos2(10.0, 100.0), egui::vec2(100.0, 200.0));

            let body = egui::Shape::Rect(egui::epaint::RectShape::filled(
                body_rect,
                rounding,
                Color32::BLACK,
            ));

            ui.painter().set(shape, body);
        });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "Painter",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

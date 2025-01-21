use eframe::egui::{self, Color32};

// reference: https://blog.csdn.net/qq_33446100/article/details/132778862

struct MyApp {
    rect_pos: egui::Pos2,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext) -> Self {
        MyApp {
            rect_pos: egui::pos2(10.0, 100.0),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("test");

            let shape = ui.painter().add(egui::Shape::Noop);

            let rounding = egui::Rounding::same(0.4);

            let mut body_rect = egui::Rect::from_min_size(self.rect_pos, egui::vec2(100.0, 200.0));

            let window_response = ui.interact(
                body_rect,
                egui::Id::new((1, "windows")),
                egui::Sense::click_and_drag(),
            );

            let drag_delta = window_response.drag_delta();

            if drag_delta.length_sq() > 0.0 {
                // move the body_rect
                body_rect = body_rect.translate(drag_delta);
                self.rect_pos += drag_delta;
            }

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

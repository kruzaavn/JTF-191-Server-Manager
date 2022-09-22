use eframe::egui;
use egui_extras::RetainedImage;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native(
        "JTF-191 Server Manager Installer",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    path: String,
    image: RetainedImage
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            path: "".to_owned(),
            image: RetainedImage::from_image_bytes(
                "JTF191d.png",
                include_bytes!("JTF191d.png"),
            )
            .unwrap(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            self.image.show_scaled(ui, 0.1);

            ui.horizontal(|ui| {
                ui.label("DCS Install Path: ");
                ui.text_edit_singleline(&mut self.path);
            });
            if ui.button("Install Package").clicked() {
            }
        });
    }
}
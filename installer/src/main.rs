use eframe::egui;
use std::path::Path;
use egui_extras::RetainedImage;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 400.0)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "JTF-191 Server Manager Installer",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

fn get_default_dcs_install_path() -> Option<String> {
    return None
}

fn get_default_dcs_saved_games_path() -> Option<String> {
    return None
}

struct MyApp {
    install_path: Option<String>,
    saved_games_path: Option<String>,
    image: RetainedImage
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            install_path: get_default_dcs_install_path(),
            saved_games_path: get_default_dcs_saved_games_path(),
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

                if ui.button("DCS Install Path").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.install_path = Some(path.display().to_string());
                    }
                };
            });
            ui.horizontal(|ui| {
                if ui.button("DCS Saved Games Path").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.saved_games_path = Some(path.display().to_string());
                    }
                };
            });
            if ui.button("Install Package").clicked() {

                if self.install_path.is_some()  && self.saved_games_path.is_some() {
                    println!("I've been clicked! ")
                }
            }
        });
    }
}
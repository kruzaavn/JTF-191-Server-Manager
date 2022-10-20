use eframe::egui;
use egui_extras::RetainedImage;
use dirs::home_dir;
use std::path::PathBuf;
use std::path::Path;
use reqwest::blocking::{Client};
use reqwest;
use serde::Deserialize;
use std::io;
use std::fs::File;
use serde_json::{Result, Value};


static SPACING: f32 = 20.0;

fn main() {

    let options = eframe::NativeOptions {
        // initial_window_size: Some(egui::vec2(300.0, 400.0)),

        ..Default::default()
    };
    eframe::run_native(
        "JTF-191 Server Manager Installer",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

fn get_default_path(install: bool) -> Option<String> {

    let full_path: PathBuf;

    if install {

        full_path = Path::new(r"C:\Program Files\Eagle Dynamics\DCS.openbeta").to_path_buf();

    } else {

        full_path = home_dir()?.join("Saved Games/DCS.openbeta");

    }

    return if full_path.exists() {

        Some(full_path.to_str()?.to_string())

    } else {

        None

    }

}

fn get_client() -> Client {

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", reqwest::header::HeaderValue::from_static("kruzaavn"));


    let client = Client::builder().default_headers(headers).build().expect("Unable to create client");



    return client

}

fn get_json(client: &Client, url: &str) -> Value {
        let json_string = client.get(url).send().expect("request failed").text().expect("no message text");

        let json: Value = serde_json::from_str(&json_string).unwrap();

        return json

    }

struct MyApp {
    install_path: Option<String>,
    saved_games_path: Option<String>,
    image: RetainedImage,
    client: Client
}


impl Default for MyApp {
    fn default() -> Self {
        Self {
            install_path: get_default_path(true),
            saved_games_path: get_default_path(false),
            image: RetainedImage::from_image_bytes(
                "JTF191d.png",
                include_bytes!("JTF191d.png"),
            )
            .unwrap(),
            client: get_client()
        }
    }
}



impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            self.image.show_scaled(ui, 0.1);

            ui.add_space(SPACING);

            if ui.button("DCS Install Path").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.install_path = Some(path.display().to_string());
                }
            };

            if let Some(picked_path) = &self.install_path {
                ui.horizontal(|ui| {
                    ui.label("Install Path:");
                    ui.monospace(picked_path);

                });
            } else {
                ui.horizontal(|ui| {
                    ui.label("Select DCS Install Path") ;
                });
            }

            ui.add_space(SPACING);

            if ui.button("DCS Saved Games Path").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.saved_games_path = Some(path.display().to_string());
                    }
                };

            if let Some(picked_path) = &self.saved_games_path {
                ui.horizontal(|ui| {
                    ui.label("Saved Games Path:");
                    ui.monospace(picked_path);

                });
            } else {
                ui.horizontal(|ui| {
                    ui.label("Select DCS Saved Games Path") ;
                }); }

            ui.add_space(SPACING);

            ui.separator();

            let install_button = ui.add_enabled(self.install_path.is_some()  && self.saved_games_path.is_some(), egui::Button::new("Install Package"));


            if install_button.clicked() {

                if self.install_path.is_some()  && self.saved_games_path.is_some() {

                    let package_url = "https://api.github.com/repos/kruzaavn/JTF-191-Server-Manager/releases/latest";
                    let moose_url = "https://api.github.com/repos/FlightControl-Master/MOOSE/releases/latest";
                    let test_url = "https://kruzaavn.github.io/data/minecraft/ops.json";

                    let json = get_json(&self.client, package_url);

                    println!("{}", json["url"]);

                    // let resp = reqwest::blocking::get("https://github.com/kruzaavn/JTF-191-Server-Manager/archive/refs/tags/v0.7.zip").expect("request failed");
                    // let body = resp.text().expect("body invalid");
                    // let mut out = File::create("file_downloaded.lua").expect("failed to create file");
                    // io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");

                }
            }
        });
    }


}


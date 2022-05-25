#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rand::prelude::*;
use eframe::{epi, egui};

#[derive(Default)]
struct MyEguiApp {
    length: i32,
    pwd: String
}

impl epi::App for MyEguiApp {
    fn name(&self) -> &str {
        "Password Generator"
    }

    fn setup(
        &mut self, 
        _ctx: &egui::Context,
        frame: &epi::Frame, 
        _storage: Option<&dyn epi::Storage>
    ) {
        frame.set_window_size(egui::Vec2{x:610.0,y:200.0});
        self.length = 2;
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Password Generator");
            ui.add(egui::Slider::new(&mut self.length, 2..=128).text("Length"));
            if ui.button("Generate").clicked() {
                self.pwd = pwdgen(self.length);
            }
            ui.text_edit_singleline(&mut self.pwd);
        });
    }
}

fn main() {
let app = MyEguiApp::default();
let native_options = eframe::NativeOptions::default();
eframe::run_native(Box::new(app), native_options);
}

fn pwdgen(length: i32) -> String {
    let list = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", 
    "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x",
    "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
    "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1",
    "2", "3", "4", "5", "6", "7", "8", "9", "!", "@", "#", "$", "%", "^", "&",
    "*", "(", ")", "-", "_", "+", "=", ",", ".", "?", ";", ":", "'",];
    let mut rng = rand::thread_rng();
    let mut letter = "".to_string();
    for _ in 0..length {
        let index: usize = rng.gen_range(0..82);
        letter.push_str(list[index]);
    }
    return letter;
}

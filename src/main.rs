#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rand::prelude::*;
use eframe::{epi, egui};

#[derive(Default)]
struct PasswordGenerator {
    length: i32,
    lowercase: bool,
    uppercase: bool,
    numbers: bool,
    symbols: bool,
    pwd: String
}

impl epi::App for PasswordGenerator {
    fn name(&self) -> &str {
        "Password Generator"
    }

    fn setup(
        &mut self, 
        _ctx: &egui::Context,
        frame: &epi::Frame, 
        _storage: Option<&dyn epi::Storage>
    ) {
        frame.set_window_size(egui::Vec2{x:610.0,y:275.0});
        self.length = 16;
        self.lowercase = true;
        self.uppercase = true;
        self.numbers = true;
        self.symbols = true;
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Password Generator");
            ui.add(egui::Slider::new(&mut self.length, 2..=128).text("Length"));
            ui.horizontal(|ui| {
                ui.add(egui::Checkbox::new(&mut self.lowercase, "Lowercase"));
                ui.add(egui::Checkbox::new(&mut self.uppercase, "Uppercase"));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Checkbox::new(&mut self.numbers, "Numbers"));
                ui.add(egui::Checkbox::new(&mut self.symbols, "Symbols"));
            });
            if ui.button("Generate").clicked() {
                if self.lowercase == false && self.uppercase == false && self.numbers == false && self.symbols == false {
                    self.pwd = " ".to_string();
                } else {
                    self.pwd = pwdgen(self.lowercase, self.uppercase, self.numbers, self.symbols, self.length);
                }
            }
            ui.text_edit_singleline(&mut self.pwd);
        });
    }
}

fn main() {
let app = PasswordGenerator::default();
let native_options = eframe::NativeOptions::default();
eframe::run_native(Box::new(app), native_options);
}

fn pwdgen(lowercase: bool, uppercase: bool, numbers: bool, symbols: bool, length: i32) -> String {
    let mut list: Vec<&str> = vec![];
    if lowercase == true {
        list.append(&mut vec!["a", "b", "c", "d", "e", "f",
        "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
        "t", "u", "v", "w", "x", "y", "z"]);
    }
    if uppercase == true {
        list.append(&mut vec!["A", "B", "C", "D", "E", "F",
        "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
        "T", "U", "V", "W", "X", "Y", "Z"]);
    }
    if numbers == true {
        list.append(&mut vec!["0", "1", "2", "3", "4", "5", "6",
        "7", "8", "9"]);
    }
    if symbols == true {
        list.append(&mut vec!["!", "@", "#", "$", "%", "^", "&",
        "*", "(", ")", "-", "_", "+", "=", ",", ".", "?", ";", ":", "'", "\""]);
    }
    let mut rng = rand::thread_rng();
    let mut letter = "".to_string();
    for _ in 0..length {
        let index: usize = rng.gen_range(0..list.len());
        letter.push_str(list[index]);
    }
    return letter;
}

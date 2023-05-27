use std::todo;

use eframe::egui;

// fn main() -> Result<(), eframe::Error> {

//     let options = eframe::NativeOptions {
//         initial_window_size: Some(egui::vec2(320.0, 240.0)),
//         ..Default::default()
//     };

//     // Our application state:
//     let mut duration = 50;
//     let timer = Timer{duration: 10};

//     eframe::run_simple_native("Timer app", options, move |ctx, _frame| {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("Timer app");
//             ui.add(egui::Slider::new(&mut duration, 0..=90).text("duration"));
//             if ui.button("+").clicked() {
//                 duration += 1;
//             }
//             if ui.button("-").clicked() {
//                 duration += 1;
//             }
//             if ui.button("x").clicked() {
//                 timer.start_timer()
//             }
//         });
//     })
// }

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Rust timer app",
        options,
        Box::new(|_cc| Box::<Timer>::default()),
    )
}

#[derive(Debug)]
struct Timer {
    duration: i32,
}

impl Timer {
    fn start_timer(self) {
        todo!()
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self { duration: 50 }
    }
}

impl eframe::App for Timer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Timer");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

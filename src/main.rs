use std::todo;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    // Our application state:
    let mut duration = 50;
    let timer = Timer{duration: 10};

    eframe::run_simple_native("Timer app", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Timer app");
            ui.add(egui::Slider::new(&mut duration, 0..=90).text("duration"));
            if ui.button("+").clicked() {
                duration += 1;
            }
            if ui.button("-").clicked() {
                duration += 1;
            }
            if ui.button("x").clicked() {
                timer.start_timer()
            }
        });
    })
}

#[derive(Debug)]
struct Timer {
    duration: i32
}

impl Timer {
    fn start_timer(self){
        todo!()
    }
}


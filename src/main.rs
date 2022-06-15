#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui::{self, Ui, Frame, Pos2, Stroke, Color32, Sense, emath, Rect};


fn main() {
    let options = eframe::NativeOptions::default();
    let app = Painting::default();

    eframe::run_native(
        "Painting Demo",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}

pub struct Painting {
    /// in 0-1 normalized coordinates
    lines: Vec<Vec<Pos2>>,
    stroke: Stroke,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
        }
    }
}

impl Painting {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            egui::stroke_ui(ui, &mut self.stroke, "Stroke");
            ui.separator();
            if ui.button("Clear Painting").clicked() {
                self.lines.clear();
            }
        })
        .response
    }

    pub fn ui_content(&mut self, ui: &mut Ui) -> egui::Response {
        let (mut response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

        let current_line = self.lines.last_mut().unwrap();

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = from_screen * pointer_pos;
            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
                response.mark_changed();
            }
        } else if !current_line.is_empty() {
            self.lines.push(vec![]);
            response.mark_changed();
        }

        let mut shapes = vec![];
        for line in &self.lines {
            if line.len() >= 2 {
                let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                shapes.push(egui::Shape::line(points, self.stroke));
            }
        }
        painter.extend(shapes);

        response
    }
}


impl eframe::App for Painting {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
        .show(ctx, |ui| {
            ui.ctx().set_visuals(egui::Visuals::dark());

            self.ui_control(ui);
            ui.label("Paint with your mouse/touch!");
            Frame::canvas(ui.style()).show(ui, |ui| {
                self.ui_content(ui);
            });
             
        });
    }
}



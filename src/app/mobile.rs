use crate::app::entries;
use egui::{Color32, Context, FontId, TextStyle, Vec2, Visuals};

pub fn ui(ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let frame = egui::Frame::new()
                .fill(ui.visuals().window_fill)
                .corner_radius(8)
                .inner_margin(6)
                .stroke(ui.visuals().widgets.noninteractive.bg_stroke);

            frame.clone().show(ui, entries::pola);
            ui.add_space(6.0);
            frame.clone().show(ui, entries::swirly);
            ui.add_space(6.0);
            frame.clone().show(ui, entries::poaching);
            ui.add_space(6.0);
            frame.clone().show(ui, entries::scalie);
            ui.add_space(6.0);
            frame.clone().show(ui, entries::affiliates);
        });

        //ui.vertical(entries::pola);
    });
}

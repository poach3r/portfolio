use crate::app::entries;
use egui::Context;

pub fn ui(ctx: &Context) {
    egui::SidePanel::right("About Me").show(ctx, |ui| {
        ui.heading("About Me");
        ui.separator();

        ui.label("My name is Henry M. I'm a young programmer and Eagle Scout majoring in Computer Science at West Chester University. In my free time I enjoy gaming, listening to music, and recreationally programming.");
        ui.add_space(8.0);
        ui.label("This website displays some of the projects that I've worked on, I hope you enjoy it!");

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            ui.hyperlink_to("Github", "https://github.com/poach3r");
            ui.separator();
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });
    });

    egui::Window::new("Affiliates").show(ctx, entries::affiliates);

    egui::Window::new("Swirly").show(ctx, entries::swirly);

    egui::Window::new("Pola").show(ctx, entries::pola);

    egui::Window::new("Scalie").show(ctx, entries::scalie);

    egui::Window::new("Poaching").show(ctx, entries::poaching);
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

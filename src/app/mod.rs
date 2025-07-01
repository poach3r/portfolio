mod desktop;
mod entries;
mod mobile;

use egui::{Color32, FontId, TextStyle, Vec2, Visuals};

pub struct TemplateApp {}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {}
    }
}

fn custom_visuals() -> Visuals {
    let mut visuals = Visuals::dark();
    visuals.window_fill = Color32::from_hex("#262626").unwrap();
    visuals.panel_fill = Color32::from_hex("#161616").unwrap();
    visuals.widgets.noninteractive.fg_stroke.color = Color32::from_hex("#dde1e6").unwrap();
    visuals.hyperlink_color = Color32::from_hex("#78a9ff").unwrap();
    visuals.widgets.noninteractive.bg_stroke.color = Color32::from_hex("#393939").unwrap();
    visuals
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.all_styles_mut(|style| {
            style.text_styles.insert(
                TextStyle::Heading,
                FontId::new(20.0, egui::FontFamily::Proportional),
            );
            style.text_styles.insert(
                TextStyle::Body,
                FontId::new(14.0, egui::FontFamily::Proportional),
            );
        });
        cc.egui_ctx.set_visuals(custom_visuals());

        egui_extras::install_image_loaders(&cc.egui_ctx);

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let is_web = cfg!(target_arch = "wasm32");
        if !is_web {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);

                    egui::widgets::global_theme_preference_buttons(ui);
                });
            });
        }

        let screen = ctx.screen_rect();
        if screen.aspect_ratio() > 1.0 {
            desktop::ui(ctx);
        } else {
            mobile::ui(ctx);
        }
    }
}

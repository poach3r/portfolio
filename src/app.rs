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

        egui::Window::new("Affiliates").show(ctx, |ui| {
            ui.label("These are some cool people.");
            ui.add_space(2.0);
            ui.centered_and_justified(|ui| {
                egui::Grid::new("affiliated_projects").spacing(Vec2::new(8.0, 8.0))
                    .show(ui, |ui| {
                        if ui
                            .add_sized(
                                Vec2::new(128.0, 128.0),
                                egui::ImageButton::new(egui::include_image!("../assets/pita.webp"))
                                    .corner_radius(4),
                            )
                            .clicked()
                        {
                            open_url("https://pita.im/");
                        }

                        if ui
                            .add_sized(
                                Vec2::new(128.0, 128.0),
                                egui::ImageButton::new("https://avatars.githubusercontent.com/u/208179266?v=4")
                                    .corner_radius(4),
                            )
                            .clicked()
                        {
                            open_url("https://draco.is-a.dev//");
                        }

                        ui.end_row();

                        if ui
                            .add_sized(
                                Vec2::new(128.0, 128.0),
                                egui::ImageButton::new(egui::include_image!("../assets/icon-256.png"))
                                    .corner_radius(4),
                            )
                            .clicked()
                        {
                        }

                        if ui
                            .add_sized(
                                Vec2::new(128.0, 128.0),
                                egui::ImageButton::new(egui::include_image!("../assets/icon-256.png"))
                                    .corner_radius(4),
                            )
                            .clicked()
                        {
                        }
                    });
            });
        });

        egui::Window::new("Swirly").show(ctx, |ui| {
            ui.label("Swirly is a desktop shell for the Sway and Wayfire compositors. The project is written in Rust and uses Relm4 for the rendering. I currently daily drive it and its the project that I'm the most proud of.");
            ui.add_space(8.0);
            ui.image("https://raw.githubusercontent.com/poach3r/swirly/refs/heads/master/demo.png");

            ui.separator();
            ui.hyperlink_to("Source Code", "https://github.com/poach3r/swirly");
        });

        egui::Window::new("Pola").show(ctx, |ui| {
            ui.label("Pola is a simple to understand programming language written in Kotlin. It supports many features, such as classes, first-class functions, filesystem access, GUIs, and error handling.");
            ui.add_space(8.0);
            // This is really fucking ugly. The egui code block example uses a function
            // to remove leading indentation on all lines but I'd prefer not to use that.
            // https://github.com/emilk/egui/blob/main/crates/egui_demo_lib/src/demo/code_example.rs#L152
            ui.code_editor(&mut
r#"import("pola/io")

fun fib(n) {
    if(n <= 1)
        return n

    return fib(n - 1) + fib(n - 2)
}

io.println(fib(25))"#);
            ui.add_space(8.0);
            ui.image("https://raw.githubusercontent.com/poach3r/Pola/master/assets/logo.png");

            ui.separator();
            ui.hyperlink_to("Source Code", "https://github.com/poach3r/pola");
        });

        egui::Window::new("Scalie").show(ctx, |ui| {
            ui.label("Scalie is a Linux shell written in Scala (compiled using Scala Native) which takes inspiration from functional programming languages.");
            ui.add_space(8.0);
            ui.code_editor(&mut
r#"/home/poacher
> echo 5 * 32.5
162.5

/home/poacher/Pictures
> screp["screenshot" $ls]
screenshot-10_1_25.png
screenshot-4_1_25.png
screenshot-28_12_24.png"#);
            ui.separator();
            ui.hyperlink_to("Source Code", "https://github.com/poach3r/scalie");
        });

        egui::Window::new("Poaching").show(ctx, |ui| {
            ui.label("Poaching is a hunger games simulator written in Rust which uses Relm4 for rendering. This game was my first major Rust project and was made for me and a couple of my friends to have fun with.");
            ui.add_space(8.0);
            ui.image("https://raw.githubusercontent.com/poach3r/poaching/refs/heads/master/examples/demo-dark.png");
            ui.separator();
            ui.hyperlink_to("Source Code", "https://github.com/poach3r/poaching");
        });
    }
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

#[cfg(not(target_arch = "wasm32"))]
fn open_url(url: &str) {
    if let Err(e) = webbrowser::open(url) {
        println!("Failed to open {url}: {e}") // TODO improve logging
    };
}

#[cfg(target_arch = "wasm32")]
fn open_url(url: &str) {
    if let Some(win) = web_sys::window() {
        if let Err(_e) = win.open_with_url(url) {
            println!("Failed to open {url}") // TODO improve logging
        };
    }
}

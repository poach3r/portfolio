use egui::Ui;
use egui::{Color32, Context, FontId, TextStyle, Vec2, Visuals};

pub fn swirly(ui: &mut Ui) {
    ui.label("Swirly is a desktop shell for the Sway and Wayfire compositors. The project is written in Rust and uses Relm4 for the rendering. I currently daily drive it and its the project that I'm the most proud of.");
    ui.add_space(8.0);
    ui.image("https://raw.githubusercontent.com/poach3r/swirly/refs/heads/master/demo.png");

    ui.separator();
    ui.hyperlink_to("Source Code", "https://github.com/poach3r/swirly");
}

pub fn pola(ui: &mut Ui) {
    ui.label("Pola is a simple to understand programming language written in Kotlin. It supports many features, such as classes, first-class functions, filesystem access, GUIs, and error handling.");
    ui.add_space(8.0);
    // This is really fucking ugly. The egui code block example uses a function
    // to remove leading indentation on all lines but I'd prefer not to use that.
    // https://github.com/emilk/egui/blob/main/crates/egui_demo_lib/src/demo/code_example.rs#L152
    ui.code_editor(
        &mut r#"import("pola/io")

fun fib(n) {
if(n <= 1)
    return n

return fib(n - 1) + fib(n - 2)
}

io.println(fib(25))"#,
    );
    ui.add_space(8.0);
    ui.image("https://raw.githubusercontent.com/poach3r/Pola/master/assets/logo.png");

    ui.separator();
    ui.hyperlink_to("Source Code", "https://github.com/poach3r/pola");
}

pub fn scalie(ui: &mut Ui) {
    ui.label("Scalie is a Linux shell written in Scala (compiled using Scala Native) which takes inspiration from functional programming languages.");
    ui.add_space(8.0);
    ui.code_editor(
        &mut r#"/home/poacher
> echo 5 * 32.5
162.5

/home/poacher/Pictures
> screp["screenshot" $ls]
screenshot-10_1_25.png
screenshot-4_1_25.png
screenshot-28_12_24.png"#,
    );
    ui.separator();
    ui.hyperlink_to("Source Code", "https://github.com/poach3r/scalie");
}

pub fn poaching(ui: &mut Ui) {
    ui.label("Poaching is a hunger games simulator written in Rust which uses Relm4 for rendering. This game was my first major Rust project and was made for me and a couple of my friends to have fun with.");
    ui.add_space(8.0);
    ui.image("https://raw.githubusercontent.com/poach3r/poaching/refs/heads/master/examples/demo-dark.png");
    ui.separator();
    ui.hyperlink_to("Source Code", "https://github.com/poach3r/poaching");
}

pub fn affiliates(ui: &mut Ui) {
    ui.label("These are some cool people.");
    ui.add_space(2.0);
    ui.centered_and_justified(|ui| {
        egui::Grid::new("affiliated_projects")
            .spacing(Vec2::new(8.0, 8.0))
            .show(ui, |ui| {
                if ui
                    .add_sized(
                        Vec2::new(128.0, 128.0),
                        egui::ImageButton::new(
                            "https://avatars.githubusercontent.com/u/208179266?v=4",
                        )
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
                        egui::ImageButton::new(egui::include_image!("../../assets/icon-256.png"))
                            .corner_radius(4),
                    )
                    .clicked()
                {}

                if ui
                    .add_sized(
                        Vec2::new(128.0, 128.0),
                        egui::ImageButton::new(egui::include_image!("../../assets/icon-256.png"))
                            .corner_radius(4),
                    )
                    .clicked()
                {}

                if ui
                    .add_sized(
                        Vec2::new(128.0, 128.0),
                        egui::ImageButton::new(egui::include_image!("../../assets/icon-256.png"))
                            .corner_radius(4),
                    )
                    .clicked()
                {}
            });
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

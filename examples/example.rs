#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::{self, Ui};

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
enum Page {
    Page1,
    Page2,
    Page3,
}

struct MyApp {
    name: String,
    age: u32,
    page: Page,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            page: Page::Page1,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.style_mut(|style| {
            style.animation_time = 0.3;
        });

        let mut state = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.page, Page::Page1, "Home");
                ui.selectable_value(&mut self.page, Page::Page2, "Page 2");
                ui.selectable_value(&mut self.page, Page::Page3, "Page 3");
            });

            let state_s = egui_transition::animated_pager(
                ui,
                self.page.clone(),
                egui_transition::TransitionType::HorizontalMove,
                egui::Id::new("pager"),
                |ui: &mut Ui, page| match page {
                    Page::Page1 => {
                        ui.heading("Page 1");
                        ui.horizontal(|ui| {
                            let name_label = ui.label("Your name: ");
                            ui.text_edit_singleline(&mut self.name)
                                .labelled_by(name_label.id);
                        });
                        ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
                        if ui.button("Increment").clicked() {
                            self.age += 1;
                        }
                        ui.label(format!("Hello '{}', age {}", self.name, self.age));
                    }
                    Page::Page2 => {
                        ui.label("Hello from the second page");
                    }
                    Page::Page3 => {
                        ui.label("Hello from the third page");
                    }
                },
            );
            state = Some(state_s);
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            state.unwrap().show("bottom_panel_state", ui);
        });
    }
}

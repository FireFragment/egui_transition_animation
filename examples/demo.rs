#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::{self, Ui};
use egui::{Layout, Vec2};
use egui_page_transition::prelude::*;

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([512.0, 256.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Egui page transition demo",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone)]
enum Page {
    About,
    Configure,
    Example,
}

struct MyApp {
    name: String,
    age: u32,

    transition_type: TransitionType,
    animation_time: f32,
    page: Page,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,

            transition_type: TransitionType::HorizontalMove,
            animation_time: 0.3,
            page: Page::About,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.style_mut(|style| {
            style.animation_time = self.animation_time;
        });

        let mut state = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                match self.transition_type {
                    TransitionType::HorizontalMove => Layout::top_down(egui::Align::Min),
                    TransitionType::VerticalMove => Layout::left_to_right(egui::Align::Min),
                },
                |ui| {
                    ui.with_layout(
                        match self.transition_type {
                            TransitionType::HorizontalMove => {
                                Layout::left_to_right(egui::Align::Min)
                            }
                            TransitionType::VerticalMove => {
                                Layout::top_down_justified(egui::Align::Min)
                            }
                        },
                        |ui| {
                            if self.transition_type == TransitionType::VerticalMove {
                                ui.set_max_width(128.0);
                            }

                            ui.selectable_value(&mut self.page, Page::About, "ℹ About");
                            ui.selectable_value(&mut self.page, Page::Configure, "⛭ Configure");
                            ui.selectable_value(&mut self.page, Page::Example, "☺ Example tab");
                        },
                    );
                    ui.vertical(|ui| {
                        let state_s = animated_pager(
                            ui,
                            self.page.clone(),
                            &TransitionStyle::new_with_type(ui, self.transition_type.clone()),
                            egui::Id::new("pager"),
                            |ui: &mut Ui, page| match page {
                                Page::Example => {
                                    ui.heading("Example page");

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
                                Page::Configure => {
                                    ui.heading("Configure");

                                    ui.label("Animation type:");
                                    ui.indent((), |ui| {
                                        ui.radio_value(
                                            &mut self.transition_type,
                                            TransitionType::HorizontalMove,
                                            "Horizontal",
                                        );
                                        ui.radio_value(
                                            &mut self.transition_type,
                                            TransitionType::VerticalMove,
                                            "Vertical",
                                        );
                                    });

                                    ui.add(
                                        egui::Slider::new(&mut self.animation_time, 0.0..=2.0)
                                            .text("Animation time")
                                            .suffix("s"),
                                    );
                                }
                                Page::About => {
                                    ui.heading("About");
                                    ui.horizontal_wrapped(|ui| {
                                        ui.style_mut().spacing.item_spacing = Vec2::new(0.0, 0.0);

                                        ui.label("Welcome to the ");
                                        ui.monospace(env!("CARGO_PKG_NAME"));
                                        ui.label(" demo. See the transitions by switching tabs in the bar above or click this button to ");
                                        if ui.small_button("Go to the Example tab").clicked() {
                                            self.page = Page::Example;
                                        };
                                        ui.label(". Animation speed has been decreased to let you see the transitions better. You can customize it in ");
                                        if ui.link("the Configure tab").clicked() {
                                            self.page = Page::Configure;
                                        }
                                    });

                                    ui.add_space(8.0);

                                    ui.strong("Learn more: ");
                                    if ui.link(" GitHub").clicked() {
                                        ctx.open_url(egui::OpenUrl {
                                            url: String::from("https://github.com/FireFragment/egui_page_transition"),
                                            new_tab: true
                                        });
                                    }
                                }
                            },
                        );
                        state = Some(state_s);
                    });
                },
            );
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            state.unwrap().show("bottom_panel_state", ui);
        });
    }
}

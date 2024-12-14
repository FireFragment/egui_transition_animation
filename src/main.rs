#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::time::Duration;

use eframe::{
    egui::{self, Context, Ui},
    emath::easing,
};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    name: String,
    age: u32,
    page: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            page: false,
        }
    }
}

fn combine_easings<'a>(
    ease_in: (impl Fn(f32) -> f32 + 'a),
    ease_out: (impl Fn(f32) -> f32 + 'a),
) -> impl Fn(f32) -> f32 + 'a {
    move |t: f32| {
        if t > 0.5 {
            ease_in(t / 2.0)
        } else {
            ease_out(0.5 + t / 2.0)
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let fps = 60;
        let time = ctx.cumulative_pass_nr() * (120 / fps);

        ctx.style_mut(|style| {
            style.animation_time = 0.3;
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            //let page_2 = ;
            let page = self.page;
            page_transition(
                ui,
                ctx.animate_bool_with_easing(egui::Id::new("page"), page, easing::circular_in_out),
                |ui: &mut Ui, page| match page {
                    false => {
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

                        if ui.button("Switch to page 2").clicked() {
                            self.page = true;
                        }
                    }
                    true => {
                        ui.label("Hello from the second page");
                        if ui.button("Switch to page 1").clicked() {
                            self.page = false;
                        }
                    }
                },
            );

            //std::thread::sleep(Duration::from_millis(1000 / fps));
        });

        egui::TopBottomPanel::bottom("b").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Frame ");
                ui.label(time.to_string());
            });
        });
    }
}

/// <https://easings.net/#easeOutCirc>
///
/// Modeled after shifted quadrant II of unit circle
#[inline]
pub fn circular_out(t: f32) -> f32 {
    (1. - (t - 1.).powi(2)).sqrt() // 2.0
}

fn page_transition(ui: &mut Ui, t: f32, mut add_contents: impl FnMut(&mut Ui, bool)) {
    let dist = 16.0;
    if t <= 0.5 {
        let space = -dist * t * 2.;
        ui.add_space(space);
        add_contents(ui, false);
    } else {
        let tf = 2. * t - 1.;
        let space = dist + -dist * tf;
        ui.add_space(space);
        add_contents(ui, true);
    }
}

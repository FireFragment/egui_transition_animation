use std::{
    fmt::Display,
    hash::Hash,
    time::{Duration, Instant},
};

use egui::{
    self,
    emath::{easing, TSTransform},
    Ui, Vec2,
};

#[non_exhaustive]
pub enum TransitionAnimationType {
    HorizontalMove,
    VerticalMove,
}

impl TransitionAnimationType {
    fn into_tstransform(&self, amount: f32, origin: Vec2) -> TSTransform {
        match self {
            Self::HorizontalMove => TSTransform::from_translation(Vec2::new(amount, 0.)),
            Self::VerticalMove => TSTransform::from_translation(Vec2::new(0., amount)),
        }
    }
}

pub fn page_transition<T>(
    ui: &mut Ui,
    t: f32,
    easing: impl Fn(f32) -> f32,
    animation_type: TransitionAnimationType,
    invert_direction: bool,
    add_contents: impl FnOnce(&mut Ui, bool) -> T,
) -> T {
    let dist = 16.0;
    let anim_state = easing(t);
    let first_stage = anim_state <= 0.5;

    let offset_size = if first_stage {
        -dist * anim_state * 2.
    } else {
        dist + -dist * (2. * anim_state - 1.)
    } * if invert_direction { 1. } else { -1. };

    ui.with_visual_transform(
        animation_type.into_tstransform(offset_size, Vec2::new(32.0, 32.)),
        |ui| add_contents(ui, !first_stage),
    )
    .inner
}

pub struct PagerRet<Page, Ret> {
    pub real_page: Page,
    pub ui_ret: Ret,
    pub animation_running: bool,
}

impl<Page: Display, Ret> PagerRet<Page, Ret> {
    /// Omits the `ui_ret` field
    pub fn show(&self, id: impl Hash, ui: &mut Ui) {
        egui::Grid::new(id).num_columns(2).show(ui, |ui| {
            self.show_in_grid(ui);
        });
    }
    /// Omits the `ui_ret` field
    pub fn show_in_grid(&self, ui: &mut Ui) {
        ui.strong("Real page: ");
        ui.monospace(self.real_page.to_string());
        ui.end_row();

        ui.strong("Animation running: ");
        ui.monospace(self.animation_running.to_string());
        ui.end_row();
    }
}

pub fn animated_pager<Page: Default + Sync + Send + Clone + 'static + Eq + PartialOrd, Ret>(
    ui: &mut Ui,
    target_page: Page,
    animation_type: TransitionAnimationType,
    id: egui::Id,
    add_contents: impl FnMut(&mut Ui, Page) -> Ret,
) -> PagerRet<Page, Ret> {
    animated_pager_advanced(
        ui,
        target_page,
        easing::circular_in_out,
        animation_type,
        id,
        |original_page, new_page| original_page < new_page,
        add_contents,
    )
}

pub fn animated_pager_advanced<Page: Default + Sync + Send + Clone + 'static + Eq, Ret>(
    ui: &mut Ui,
    target_page: Page,
    easing: impl Fn(f32) -> f32,
    animation_type: TransitionAnimationType,
    id: egui::Id,
    invert_direction: impl FnOnce(&Page, &Page) -> bool,
    add_contents: impl FnOnce(&mut Ui, Page) -> Ret,
) -> PagerRet<Page, Ret> {
    let animation_length = ui.style().animation_time;

    let prev_page = ui.ctx().memory_mut(|mem| {
        mem.data
            .get_persisted_mut_or_insert_with(id.with("pager_current_page"), move || {
                Page::default()
            })
            .to_owned()
    });
    let animation_end: Option<Instant> = ui
        .ctx()
        .memory(|mem| mem.data.get_temp(id.with("pager_animation_end")));

    // If animation is running...
    if let Some(animation_end) = animation_end {
        let now = Instant::now();

        // 0 means we are at the beggining of animation, 1 means we are at the end, .5 means we are at the middle etc.
        let current_animation_state = 1. - ((animation_end - now).as_secs_f32() / animation_length);

        // If the animation is done, finish it by setting memory values and display the target page
        if current_animation_state >= 1. {
            ui.ctx().memory_mut(|mem| {
                mem.data
                    .insert_persisted(id.with("pager_current_page"), target_page.clone());
                mem.data.remove::<Instant>(id.with("pager_animation_end"));
            });

            let ui_ret = add_contents(ui, target_page.clone());
            return PagerRet {
                real_page: target_page,
                ui_ret,
                animation_running: false,
            };
        }

        ui.ctx().request_repaint();

        return page_transition(
            ui,
            current_animation_state,
            easing,
            animation_type,
            invert_direction(&prev_page, &target_page),
            |ui, show_second_page| {
                let show_page = if show_second_page {
                    target_page.clone()
                } else {
                    prev_page.clone()
                };
                let ui_ret = add_contents(ui, show_page);
                PagerRet {
                    real_page: if show_second_page {
                        target_page.clone()
                    } else {
                        prev_page.clone()
                    },
                    ui_ret,
                    animation_running: true,
                }
            },
        );
    };

    // If pages have changed, but animation isn't running...
    if prev_page != target_page {
        // ...start the animation
        ui.ctx().memory_mut(|mem| {
            mem.data.insert_temp(
                id.with("pager_animation_end"),
                Instant::now() + Duration::from_millis((animation_length * 1000.0) as u64),
            )
        });

        let ui_ret = add_contents(ui, prev_page.clone());
        ui.ctx().request_repaint();

        return PagerRet {
            real_page: prev_page,
            ui_ret,
            animation_running: true,
        };
    }

    // If nothing happens right now, just show the page
    // It doesn't matter whether we show `target_page` or `prev_page`, because they are the same.
    let ui_ret = add_contents(ui, target_page);
    PagerRet {
        real_page: prev_page,
        ui_ret,
        animation_running: false,
    }
}

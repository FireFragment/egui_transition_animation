use std::{
    default,
    fmt::{self, Display},
    hash::Hash,
    time::{Duration, Instant},
};

use egui::{
    self,
    emath::{easing, TSTransform},
    Ui, Vec2,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum TransitionType {
    #[default]
    HorizontalMove,
    VerticalMove,
}

impl TransitionType {
    fn generate_tstransform(&self, amount: f32, origin: Vec2) -> TSTransform {
        match self {
            Self::HorizontalMove => TSTransform::from_translation(Vec2::new(amount, 0.)),
            Self::VerticalMove => TSTransform::from_translation(Vec2::new(0., amount)),
        }
    }
}

pub fn page_transition<T>(
    ui: &mut Ui,
    t: f32,
    style: &TransitionStyle,
    invert_direction: bool,
    add_contents: impl FnOnce(&mut Ui, bool) -> T,
) -> T {
    let dist = 16.0;
    let anim_state = (style.easing)(t);
    let first_stage = anim_state <= 0.5;

    let offset_size = if first_stage {
        -dist * anim_state * 2.
    } else {
        dist + -dist * (2. * anim_state - 1.)
    } * if invert_direction { 1. } else { -1. };

    ui.with_visual_transform(
        style
            .t_type
            .generate_tstransform(offset_size, Vec2::new(32., 32.)),
        |ui| add_contents(ui, !first_stage),
    )
    .inner
}

pub struct PagerRet<Page, Ret> {
    pub real_page: Page,
    pub ui_ret: Ret,
    pub animation_running: bool,
}

impl<Page: fmt::Debug, Ret> PagerRet<Page, Ret> {
    /// Omits the `ui_ret` field
    pub fn show(&self, id: impl Hash, ui: &mut Ui) {
        egui::Grid::new(id).num_columns(2).show(ui, |ui| {
            self.show_in_grid(ui);
        });
    }
    /// Omits the `ui_ret` field
    pub fn show_in_grid(&self, ui: &mut Ui) {
        ui.strong("Real page: ").on_hover_text("The page that is currently actually shown. May be different from the \"target\" page when there's animation running");
        ui.monospace(format!("{:?}", self.real_page));
        ui.end_row();

        ui.strong("Animation running: ");
        ui.monospace(self.animation_running.to_string());
        ui.end_row();
    }
}

pub struct TransitionStyle {
    /// This easing _can_ return values lower than 0 or larget than 1
    pub easing: fn(f32) -> f32,
    pub duration: f32,
    pub t_type: TransitionType,
}

/// # Constructors
impl TransitionStyle {
    /// Create a new [`TransitionStyle`] with default settings and given [type](TransitionType) mostly based on ui [style](egui::Ui::style),
    /// but values of some fields (eg. [easing](TransitionStyle::easing)) are opinionated and may change slightly
    /// between versions.
    pub fn new_with_type(ui: &Ui, t_type: TransitionType) -> Self {
        TransitionStyle {
            t_type,
            ..Self::new(ui)
        }
    }
    /// Create a new [`TransitionStyle`] animated by shifting horizontally
    ///
    /// It uses default settings mostly based on the provided ui's [style](egui::Ui::style),
    /// but values of some fields (eg. [easing](TransitionStyle::easing)) are opinionated and may change slightly
    /// between versions.
    pub fn horizontal(ui: &Ui) -> Self {
        Self::new_with_type(ui, TransitionType::HorizontalMove)
    }
    /// Create a new [`TransitionStyle`] animated by shifting vertically
    ///
    /// It uses default settings mostly based on the provided ui's [style](egui::Ui::style),
    /// but values of some fields (eg. [easing](TransitionStyle::easing)) are opinionated and may change slightly
    /// between versions.
    pub fn vertical(ui: &Ui) -> Self {
        Self::new_with_type(ui, TransitionType::VerticalMove)
    }

    /// Create a new [`TransitionStyle`] with default settings mostly based on ui [style](egui::Ui::style),
    /// but values of some fields (eg. [easing](TransitionStyle::easing)) are opinionated and may change slightly
    /// between versions.
    ///
    /// You will mostly want to manually specify [transition type](TransitionStyle::t_type), so it's recommended
    /// to use the [`horizontal`](TransitionStyle::horizontal) or [`vertical`](TransitionStyle::vertical)function instead.
    pub fn new(ui: &Ui) -> Self {
        Self::new_with_type(ui, TransitionType::default())
    }
}
/// Shows one of several possible pages with transition animation between them. The animation goes _forward_.
///
/// # Parameters
///  - `target_page`: Page to show. When changed, it will take some time for the pager to play animation
///    before actually showing this page
pub fn animated_pager_forward<Page: Sync + Send + Clone + 'static + Eq + PartialOrd, Ret>(
    ui: &mut Ui,
    target_page: Page,
    style: &TransitionStyle,
    id: egui::Id,
    add_contents: impl FnMut(&mut Ui, Page) -> Ret,
) -> PagerRet<Page, Ret> {
    animated_pager_with_direction(ui, target_page, style, id, |_, _| true, add_contents)
}

/// Shows one of several possible pages with transition animation between them. The animation goes _backward_.
///
/// # Parameters
///  - `target_page`: Page to show. When changed, it will take some time for the pager to play animation
///    before actually showing this page
pub fn animated_pager_backward<Page: Sync + Send + Clone + 'static + Eq + PartialOrd, Ret>(
    ui: &mut Ui,
    target_page: Page,
    style: &TransitionStyle,
    id: egui::Id,
    add_contents: impl FnMut(&mut Ui, Page) -> Ret,
) -> PagerRet<Page, Ret> {
    animated_pager_with_direction(ui, target_page, style, id, |_, _| false, add_contents)
}

/// Shows one of several possible pages with transition animation between them.
///
/// This function requires [`PartialOrd`] of page to determine _direction_ of the animation.
/// For example, in tabview, you want switching to tab on the right of the current one to be animated by sliding content to the left.
/// In contrast switching to tab on the left of the current one should be animated by sliding content to the right.
/// If your page type doesn't implement [`PartialOrd`], use one of [`animated_pager_with_direction`], [`animated_pager_forward`] or [`animated_pager_backward`].
///
/// # Parameters
///  - `target_page`: Page to show. When changed, it will take some time for the pager to play animation
///    before actually showing this page
pub fn animated_pager<Page: Sync + Send + Clone + 'static + Eq + PartialOrd, Ret>(
    ui: &mut Ui,
    target_page: Page,
    style: &TransitionStyle,
    id: egui::Id,
    add_contents: impl FnMut(&mut Ui, Page) -> Ret,
) -> PagerRet<Page, Ret> {
    animated_pager_with_direction(
        ui,
        target_page,
        style,
        id,
        |original_page, new_page| original_page < new_page,
        add_contents,
    )
}
/// Shows one of several possible pages with transition animation between them.
///
/// # Parameters
///  - `target_page`: Page to show. When changed, it will take some time for the pager to play animation
///    before actually showing this page
///  - `invert_direction`: Function that returns `true` for forward direction of animation and `false` for backward direction of animation.
///    It takes the original page as the first argument and target page as the second argument.
///    For example, in tabview, you want switching to tab on the right of the current one to be animated by sliding content to the left.
///    In contrast switching to tab on the left of the current one should be animated by sliding content to the right.
pub fn animated_pager_with_direction<Page: Sync + Send + Clone + 'static + Eq, Ret>(
    ui: &mut Ui,
    target_page: Page,
    style: &TransitionStyle,
    id: egui::Id,
    invert_direction: impl FnOnce(&Page, &Page) -> bool,
    add_contents: impl FnOnce(&mut Ui, Page) -> Ret,
) -> PagerRet<Page, Ret> {
    let animation_length = ui.style().animation_time;

    let prev_page = {
        let target_page_cloned = target_page.clone();
        ui.ctx().memory_mut(|mem| {
            mem.data
                .get_persisted_mut_or_insert_with(id.with("pager_current_page"), move || {
                    target_page_cloned
                })
                .to_owned()
        })
    };
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
            style,
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

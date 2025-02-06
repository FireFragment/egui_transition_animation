#![doc = include_str!("../docs/main.md")]

use egui::{
    self,
    emath::{easing, TSTransform},
    Ui, Vec2,
};
use std::{
    fmt::{self},
    hash::Hash,
    time::{Duration, Instant},
};

pub mod prelude {
    //! Re-exports of the most commonly used types and functions.
    pub use super::{
        animated_pager, animated_pager_backward, animated_pager_forward,
        animated_pager_with_direction, TransitionStyle, TransitionType,
    };
}

/// Type of transition animation.
///
/// Specifies the direction of the animation.
///
/// Used in [`TransitionStyle::t_type`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum TransitionType {
    /// The animated UI will move horizontally.
    #[default]
    HorizontalMove,
    /// The animated UI will move vertically.
    VerticalMove,
}

impl TransitionType {
    /// Generates the [`TSTransform`] based on the transition type and animation amount.
    ///
    /// This function is used internally by [`page_transition`] to apply the transformation to the UI.
    ///
    /// # Parameters
    /// - `amount`: The amount of translation to apply. Positive values move in one direction, negative in the opposite.
    /// - `origin`: Currently unused. Intended for potentially adding `TransitionType::Scale` in future where origin of the scale transformation effect will be needed.
    fn generate_tstransform(&self, amount: f32, _origin: Vec2) -> TSTransform {
        match self {
            Self::HorizontalMove => TSTransform::from_translation(Vec2::new(amount, 0.)),
            Self::VerticalMove => TSTransform::from_translation(Vec2::new(0., amount)),
        }
    }
}

/// Applies a page transition animation to the given UI contents.
///
/// This is the core animation function used by [`animated_pager`] family of functions.
/// It handles the visual transformation of the UI based on the animation `time` and [`TransitionStyle`].
///
/// # Parameters
/// - `ui`: The current [`Ui`] context.
/// - `time`: The current animation time, ranging from `0.0` (start) to `1.0` (end).
/// - `style`: The [`TransitionStyle`] defining the animation parameters.
/// - `invert_direction`: If `true`, the animation goes "backwards" (ie. if transition type is [`TransitionType::HorizontalMove`], this decides whether the animation goes left or right).
/// - `add_contents`: A closure that adds the UI contents to be animated.
///                   It takes the [`Ui`] and a `bool` indicating if this is the "second stage" of the animation.
///                   The "second stage" is when the animation is past 50% and the new content starts becoming fully visible.
pub fn page_transition<T>(
    ui: &mut Ui,
    time: f32,
    style: &TransitionStyle,
    invert_direction: bool,
    add_contents: impl FnOnce(&mut Ui, bool) -> T,
) -> T {
    let anim_state = (style.easing)(time);
    let first_stage = anim_state <= 0.5;

    // Calculate the offset based on the animation state and style.
    // In the first stage (0-0.5), the old page moves out.
    // In the second stage (0.5-1), the new page moves in.
    let offset_size = if first_stage {
        -style.amount * anim_state * 2.
    } else {
        style.amount + -style.amount * (2. * anim_state - 1.)
    } * if invert_direction { 1. } else { -1. };

    ui.with_visual_transform(
        style
            .t_type
            .generate_tstransform(offset_size, Vec2::new(32., 32.)), // Origin is currently unused.
        |ui| add_contents(ui, !first_stage), // `!first_stage` indicates the "second stage"
    )
        .inner
}

/// Return type of the [`animated_pager`] family of functions.
///
/// Provides information about the current state of the pager,
/// including the currently displayed page, the UI return value from the page content function,
/// and whether an animation is currently running.
pub struct PagerRet<Page, Ret> {
    /// The page that is currently actually shown in the UI.
    ///
    /// This may be different from the "target" page when an animation is in progress.
    pub real_page: Page,
    /// The return value from the `add_contents` function for the currently displayed page.
    pub ui_ret: Ret,
    /// Indicates whether a page transition animation is currently running.
    pub animation_running: bool,
}

impl<Page: fmt::Debug, Ret> PagerRet<Page, Ret> {
    /// Shows debug information about the pager in a simple grid.
    ///
    /// Omits the `ui_ret` field from the displayed information.
    ///
    /// # Parameters
    /// - `id`: A unique [`Hash`]able ID for the grid.
    /// - `ui`: The current [`Ui`] context.
    pub fn show(&self, id: impl Hash, ui: &mut Ui) {
        egui::Grid::new(id).num_columns(2).show(ui, |ui| {
            self.show_in_grid(ui);
        });
    }

    /// Shows debug information about the pager in a grid, without using a specific [`egui::Grid`] ID.
    ///
    /// Omits the `ui_ret` field from the displayed information.
    ///
    /// This is useful when you want to embed the debug info within an existing grid.
    pub fn show_in_grid(&self, ui: &mut Ui) {
        ui.strong("Real page: ")
            .on_hover_text("The page that is currently actually shown. May be different from the \"target\" page when there's animation running");
        ui.monospace(format!("{:?}", self.real_page));
        ui.end_row();

        ui.strong("Animation running: ");
        ui.monospace(self.animation_running.to_string());
        ui.end_row();
    }
}

/// Defines the style of the transition animation used by [`animated_pager`].
///
/// Use the constructor functions like [`TransitionStyle::horizontal`] or [`TransitionStyle::vertical`]
/// to create instances with sensible defaults.
pub struct TransitionStyle {
    /// Easing function to control the animation progress over time.
    ///
    /// This should be an in-out easing function for smooth transitions.
    /// It _can_ return values outside the `0.0..=1.0` range,
    /// for example, [`easing::back_in_out`](egui::emath::easing::back_in_out) can create an overshoot effect.
    ///
    /// Should be an in+out easing for best visual results.
    pub easing: fn(f32) -> f32,
    /// Duration of the animation in seconds.
    pub duration: f32,
    /// Type of transition to apply, e.g., horizontal or vertical movement.
    pub t_type: TransitionType,
    /// The amount of movement during the animation.
    ///
    /// This value determines how far the UI elements will slide during the transition.
    pub amount: f32,
}

/// # Constructors for [`TransitionStyle`]
impl TransitionStyle {
    /// Creates a new [`TransitionStyle`] with default settings and the given [type](TransitionType).
    ///
    /// Default settings are mostly based on the current UI [style](egui::Ui::style),
    /// but some fields (e.g., [easing](TransitionStyle::easing)) are opinionated and may change slightly
    /// between versions to provide a good default animation.
    ///
    /// # Parameters
    /// - `ui`: The current [`Ui`] context, used to derive default style settings.
    /// - `t_type`: The [`TransitionType`] for the animation (horizontal or vertical).
    pub fn new_with_type(ui: &Ui, t_type: TransitionType) -> Self {
        TransitionStyle {
            t_type,
            duration: ui.style().animation_time, // Default animation time from egui style
            easing: easing::circular_in_out,    // Opinionated default easing function
            amount: 16.0,                       // Opinionated default animation amount
        }
    }

    /// Creates a new [`TransitionStyle`] for horizontal page transitions.
    ///
    /// Uses default settings based on the provided UI's [style](egui::Ui::style),
    /// with horizontal movement as the transition type.
    ///
    /// # Parameters
    /// - `ui`: The current [`Ui`] context, used to derive default style settings.
    pub fn horizontal(ui: &Ui) -> Self {
        Self::new_with_type(ui, TransitionType::HorizontalMove)
    }

    /// Creates a new [`TransitionStyle`] for vertical page transitions.
    ///
    /// Uses default settings based on the provided UI's [style](egui::Ui::style),
    /// with vertical movement as the transition type.
    ///
    /// # Parameters
    /// - `ui`: The current [`Ui`] context, used to derive default style settings.
    pub fn vertical(ui: &Ui) -> Self {
        Self::new_with_type(ui, TransitionType::VerticalMove)
    }

    /// Creates a new [`TransitionStyle`] with default settings and [`TransitionType::HorizontalMove`].
    ///
    /// It is generally recommended to use [`TransitionStyle::horizontal`] or [`TransitionStyle::vertical`]
    /// to explicitly specify the transition type, improving code clarity.
    ///
    /// # Parameters
    /// - `ui`: The current [`Ui`] context, used to derive default style settings.
    pub fn new(ui: &Ui) -> Self {
        Self::new_with_type(ui, TransitionType::default())
    }
}

/// Shows one of several possible pages with a forward transition animation.
///
/// When the `target_page` changes, a "forward" animation will be triggered to transition to the new page.
///
/// # Parameters
/// - `ui`: The current [`Ui`] context.
/// - `target_page`: The page to show. When this changes, the animation starts.
/// - `style`: The [`TransitionStyle`] to use for the animation.
/// - `id`: A unique [`egui::Id`] to persist pager state (current page, animation state).
/// - `add_contents`: A closure that renders the UI for a given `Page`.
///                   The `Page` argument may be the `target_page` or the `prev_page` during animation.
///
/// # Returns
/// A [`PagerRet`] containing information about the current pager state.
///
/// # Example
/// ```rust,no_run
/// use egui::{Context, CentralPanel};
/// use egui_animated_pager::prelude::*;
///
/// #[derive(Clone, PartialEq, Eq, Debug)]
/// enum Page {
///     First,
///     Second,
/// }
///
/// fn main() {
///     let native_options = eframe::NativeOptions::default();
///     eframe::run_native(
///         "Animated Pager Example",
///         native_options,
///         Box::new(|cc| Box::new(MyApp::new(cc))),
///     ).unwrap();
/// }
///
/// struct MyApp {
///     current_page: Page,
/// }
///
/// impl MyApp {
///     fn new(_cc: &eframe::CreationContext<'_>) -> Self {
///         Self {
///             current_page: Page::First,
///         }
///     }
/// }
///
/// impl eframe::App for MyApp {
///     fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
///         CentralPanel::default().show(ctx, |ui| {
///             let pager_id = egui::Id::new("my_pager");
///             let style = TransitionStyle::horizontal(ui);
///
///             ui.horizontal(|ui| {
///                 if ui.button("First Page").clicked() {
///                     self.current_page = Page::First;
///                 }
///                 if ui.button("Second Page").clicked() {
///                     self.current_page = Page::Second;
///                 }
///             });
///
///             let pager_ret = animated_pager_forward(
///                 ui,
///                 self.current_page.clone(),
///                 &style,
///                 pager_id,
///                 |ui, page| {
///                     match page {
///                         Page::First => {
///                             ui.label("This is the first page.");
///                         }
///                         Page::Second => {
///                             ui.label("This is the second page.");
///                         }
///                     }
///                 },
///             );
///
///             pager_ret.show(pager_id.with("debug"), ui); // Optional debug info
///         });
///     }
/// }
/// ```
pub fn animated_pager_forward<Page: Sync + Send + Clone + 'static + Eq + PartialOrd, Ret>(
    ui: &mut Ui,
    target_page: Page,
    style: &TransitionStyle,
    id: egui::Id,
    add_contents: impl FnMut(&mut Ui, Page) -> Ret,
) -> PagerRet<Page, Ret> {
    animated_pager_with_direction(ui, target_page, style, id, |_, _| true, add_contents)
}

/// Shows one of several possible pages with a backward transition animation.
///
/// When the `target_page` changes, a backward animation will be triggered to transition to the new page.
/// "Backward" animation direction is determined by the provided `invert_direction` closure in [`animated_pager_with_direction`].
/// In `animated_pager_backward`, the direction is always considered "backward".
///
/// # Parameters
/// - `ui`: The current [`Ui`] context.
/// - `target_page`: The page to show. When this changes, the animation starts.
/// - `style`: The [`TransitionStyle`] to use for the animation.
/// - `id`: A unique [`egui::Id`] to persist pager state (current page, animation state).
/// - `add_contents`: A closure that renders the UI for a given `Page`.
///                   The `Page` argument may be the `target_page` or the `prev_page` during animation.
///
/// # Returns
/// A [`PagerRet`] containing information about the current pager state.
pub fn animated_pager_backward<Page: Sync + Send + Clone + 'static + Eq + PartialOrd, Ret>(
    ui: &mut Ui,
    target_page: Page,
    style: &TransitionStyle,
    id: egui::Id,
    add_contents: impl FnMut(&mut Ui, Page) -> Ret,
) -> PagerRet<Page, Ret> {
    animated_pager_with_direction(ui, target_page, style, id, |_, _| false, add_contents)
}

/// Shows one of several possible pages with transition animation between them, automatically determining direction.
///
/// This function requires `Page` to implement [`PartialOrd`] to determine the animation direction.
/// If `target_page` is "greater than" the `original_page` (persisted from the last frame), the animation is forward.
/// Otherwise, it is backward.
///
/// If your page type doesn't implement [`PartialOrd`], use [`animated_pager_with_direction`],
/// [`animated_pager_forward`] or [`animated_pager_backward`] to explicitly control the direction.
///
/// # Parameters
/// - `ui`: The current [`Ui`] context.
/// - `target_page`: The page to show. When this changes, the animation starts.
/// - `style`: The [`TransitionStyle`] to use for the animation.
/// - `id`: A unique [`egui::Id`] to persist pager state (current page, animation state).
/// - `add_contents`: A closure that renders the UI for a given `Page`.
///                   The `Page` argument may be the `target_page` or the `prev_page` during animation.
///
/// # Returns
/// A [`PagerRet`] containing information about the current pager state.
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
        |original_page, new_page| original_page < new_page, // Determine direction based on PartialOrd
        add_contents,
    )
}

/// Shows one of several possible pages with transition animation between them, with explicit direction control.
///
/// This is the most flexible version of the animated pager, allowing you to define the animation direction
/// using the `invert_direction` closure.
///
/// # Parameters
/// - `ui`: The current [`Ui`] context.
/// - `target_page`: The page to show. When this changes, the animation starts.
/// - `style`: The [`TransitionStyle`] to use for the animation.
/// - `id`: A unique [`egui::Id`] to persist pager state (current page, animation state).
/// - `invert_direction`: A closure that determines the animation direction.
///                       It takes the `original_page` (previous page) and `new_page` (target page) as arguments.
///                       It should return `true` for "forward" animation direction and `false` for "backward" direction.
///                       For example, in a tab view, switching to a tab on the right might be considered "forward".
/// - `add_contents`: A closure that renders the UI for a given `Page`.
///                   The `Page` argument may be the `target_page` or the `prev_page` during animation.
///
/// # Returns
/// A [`PagerRet`] containing information about the current pager state.
pub fn animated_pager_with_direction<Page: Sync + Send + Clone + 'static + Eq, Ret>(
    ui: &mut Ui,
    target_page: Page,
    style: &TransitionStyle,
    id: egui::Id,
    invert_direction: impl FnOnce(&Page, &Page) -> bool,
    add_contents: impl FnOnce(&mut Ui, Page) -> Ret,
) -> PagerRet<Page, Ret> {
    let animation_length = style.duration;

    // Retrieve the previously shown page from memory, or use the target page as initial page.
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
    // Retrieve the animation end time from temporary memory, if animation is running.
    let animation_end: Option<Instant> = ui
        .ctx()
        .memory(|mem| mem.data.get_temp(id.with("pager_animation_end")));

    // If animation is running...
    if let Some(animation_end) = animation_end {
        let now = Instant::now();

        // Calculate the current animation state (0.0 to 1.0).
        // 0 means animation start, 1 means animation end.
        let current_animation_state = 1. - ((animation_end - now).as_secs_f32() / animation_length);

        // If the animation is done (or past the end due to frame timing issues), finish it.
        if current_animation_state >= 1. {
            ui.ctx().memory_mut(|mem| {
                // Update the persisted current page to the target page.
                mem.data
                    .insert_persisted(id.with("pager_current_page"), target_page.clone());
                // Remove the animation end time from temporary memory, stopping the animation.
                mem.data.remove::<Instant>(id.with("pager_animation_end"));
            });

            // Render the target page content without animation.
            let ui_ret = add_contents(ui, target_page.clone());
            return PagerRet {
                real_page: target_page,
                ui_ret,
                animation_running: false, // Animation is no longer running.
            };
        }

        ui.ctx().request_repaint(); // Request repaint to continue animation in the next frame.

        // Apply page transition animation using `page_transition` function.
        return page_transition(
            ui,
            current_animation_state,
            style,
            invert_direction(&prev_page, &target_page), // Determine animation direction.
            |ui, show_second_page| {
                // Render either the previous page or the target page depending on the animation stage.
                let show_page = if show_second_page {
                    target_page.clone() // Show target page in the second stage of animation.
                } else {
                    prev_page.clone()   // Show previous page in the first stage of animation.
                };
                let ui_ret = add_contents(ui, show_page);
                PagerRet {
                    real_page: if show_second_page {
                        target_page.clone()
                    } else {
                        prev_page.clone()
                    },
                    ui_ret,
                    animation_running: true, // Animation is still running.
                }
            },
        );
    };

    // If pages have changed and animation isn't running...
    if prev_page != target_page {
        // ...start the animation.
        ui.ctx().memory_mut(|mem| {
            // Store the animation end time in temporary memory.
            mem.data.insert_temp(
                id.with("pager_animation_end"),
                Instant::now() + Duration::from_millis((animation_length * 1000.0) as u64),
            )
        });

        // Initially show the previous page (before animation starts).
        let ui_ret = add_contents(ui, prev_page.clone());
        ui.ctx().request_repaint(); // Request repaint to start animation in the next frame.

        return PagerRet {
            real_page: prev_page,
            ui_ret,
            animation_running: true, // Animation has just started.
        };
    }

    // If pages haven't changed and no animation is running, just show the target page.
    // It doesn't matter whether we show `target_page` or `prev_page`, because they are the same.
    let ui_ret = add_contents(ui, target_page);
    PagerRet {
        real_page: prev_page,
        ui_ret,
        animation_running: false, // No animation is running.
    }
}
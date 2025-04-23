use egui::{InnerResponse, Response, WidgetText};

use super::*;

/// Begin a new group of animated selectable values.
/// Returns a [`State`] object through which you can create the individual values using the
/// [`value`](State::value) method.
///
/// # Arguments
///
/// - `ui`: The UI to draw the selectable value on.
/// - `id`: A unique ID.
/// - `config`: The configuration of the animation.
/// - `selected_value`: The value that is currently selected.
///
/// # Quickstart
///
/// ```rust,no_run
///   use egui_animated_selectable_value::prelude::*;
///   use eframe::egui;
///
///   #[derive(PartialEq, PartialOrd, Clone, Eq, Hash)]
///   enum Page {
///       Page1,
///       Page2,
///       Page3,
///       Page4,
///   }
///
///   let mut page_1 = Page::Page1;
///   let mut page_2 = Page::Page1;
///   let mut page_3 = Page::Page1;
///
///   let mut show = false;
///
///   eframe::run_simple_native(
///        "Egui page transition example",
///        Default::default(),
///        move |ctx, _frame| {
///            ctx.style_mut(|style| style.animation_time = 0.2);
///            egui::CentralPanel::default().show(ctx, |ui| {
///                ui.horizontal(|ui| {
///                    let mut tabs =
///                        begin_animated_selectable_value(ui, 1, Default::default(), &mut page_1);
///                    tabs.value(ui, Page::Page1, "Page 1");
///                    tabs.value(ui, Page::Page2, "Page 2 with a long name");
///                    tabs.value(ui, Page::Page3, "Page 3");
///                    tabs.value(ui, Page::Page4, "Page 4");
///                });
///            });
///        },
///    );
/// ```
pub fn begin<'a, T: Eq, FP: animated_frame::FramePainter>(
    ui: &mut egui::Ui,
    id: impl Hash,
    config: AnimatedFrame<FP>,
    selected_value: &'a mut T,
) -> State<'a, T, FP> {
    State {
        frame: config.begin(ui, id),
        selected_value,
    }
}

pub struct State<'a, T: Eq, FP: animated_frame::FramePainter> {
    frame: animated_frame::State<FP>,
    selected_value: &'a mut T,
}

impl<T: Eq, FP: animated_frame::FramePainter> State<'_, T, FP> {
    /// Same as [`value`](State::value), but does not modify
    /// the value specified as the `selected_value` argument for [`begin`].
    pub fn value_immutable(&mut self, ui: &mut egui::Ui, value: &T, text: &str) -> Response {
        let selected = value == self.selected_value;

        let inner_widget = |ui: &mut egui::Ui| {
            InnerResponse {
                inner: (),
                response: ui.add(
                    Button::new(text)
                        .selected(selected)
                        // Enable frame to have padding, but disable border and fill, because we draw our own frame
                        .frame(true)
                        .fill(Color32::TRANSPARENT)
                        .stroke(Stroke::NONE),
                ),
            }
        };

        if selected {
            self.frame.show(ui, inner_widget).response
        } else {
            inner_widget(ui).response
        }
    }

    /// Same as [`value`](State::value), but taking a reference to the value instead of consuming it.
    pub fn value_ref(&mut self, ui: &mut egui::Ui, value: &T, text: &str) -> Response
    where
        T: Clone,
    {
        let response = self.value_immutable(ui, value, text);

        if response.clicked() {
            *self.selected_value = value.to_owned();
        }

        response
    }

    /// Add a new selectable value to the group.
    ///
    /// # Arguments
    /// * `ui` - The UI to draw the value in.
    /// * `value` - The value corresponding to the widget.
    /// * `text` - The text to display for this value.
    pub fn value(&mut self, ui: &mut egui::Ui, value: T, text: &str) -> Response {
        let response = self.value_immutable(ui, &value, text);

        if response.clicked() {
            *self.selected_value = value;
        }

        response
    }
}

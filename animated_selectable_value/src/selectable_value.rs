use egui::{InnerResponse, Response, WidgetText};

use super::*;

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
    pub fn value_immutable(
        &mut self,
        ui: &mut egui::Ui,
        value: &T,
        text: impl Into<WidgetText>,
    ) -> Response {
        let selected = value == self.selected_value;
        let text = text.into();

        let inner_widget = |ui: &mut egui::Ui| {
            InnerResponse {
                inner: (),
                response: ui.add(
                    if let WidgetText::RichText(text) = text {
                        if selected {
                            Button::new(text.color(ui.visuals().selection.stroke.color))
                        } else {
                            Button::new(text)
                        }
                    } else {
                        Button::new(text)
                    }
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

    pub fn value_ref(
        &mut self,
        ui: &mut egui::Ui,
        value: &T,
        text: impl Into<WidgetText>,
    ) -> Response
    where
        T: Clone,
    {
        let response = self.value_immutable(ui, value, text);

        if response.clicked() {
            *self.selected_value = value.to_owned();
        }

        response
    }

    pub fn value(&mut self, ui: &mut egui::Ui, value: T, text: impl Into<WidgetText>) -> Response {
        let response = self.value_immutable(ui, &value, text);

        if response.clicked() {
            *self.selected_value = value;
        }

        response
    }
}

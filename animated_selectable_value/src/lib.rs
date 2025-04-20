use std::hash::{DefaultHasher, Hash, Hasher};

use egui::{
    emath::easing, epaint, Button, Color32, CornerRadius, Id, LayerId, Order, Pos2, Rect, RichText,
    Shape, Stroke, UiBuilder, Vec2,
};
use egui_animation::animate_eased;

pub mod prelude {
    pub use super::animated_selectable_value;
}

pub fn animated_selectable_value<T: Eq>(
    ui: &mut egui::Ui,
    group_id: impl Hash,
    current_value: &mut T,
    selected_value: T,
    text: &str,
) -> egui::Response {
    // A hash uniquely indentifiyng this widget
    /*let widget_hash = {
        let mut hasher = DefaultHasher::new();
        group_id.hash(&mut hasher);
        selected_value.hash(&mut hasher);
        hasher.finish()
    };*/

    let selected = *current_value == selected_value;

    let frame_shape_id = ui.painter().add(Shape::Noop);

    let label_response = ui.add(
        Button::new(if selected {
            RichText::new(text).color(ui.visuals().selection.stroke.color)
        } else {
            RichText::new(text)
        })
        .selected(selected)
        // Enable frame to have padding, but disable border and fill, because we draw our own frame
        .frame(true)
        .fill(Color32::TRANSPARENT)
        .stroke(Stroke::NONE),
    );
    let rect = label_response.rect;

    let visuals = ui.style().interact_selectable(&label_response, true);

    if selected {
        ui.painter().set(
            frame_shape_id,
            egui::Frame::new()
                .fill(visuals.bg_fill)
                .stroke(visuals.bg_stroke)
                .corner_radius(visuals.corner_radius)
                .paint(animate_rect_with_time(
                    ui.ctx(),
                    Id::new(group_id),
                    rect,
                    ui.style().animation_time,
                )),
        );
    }

    if label_response.clicked() {
        *current_value = selected_value;
    }

    label_response
}

pub fn animate_rect_with_time(
    ctx: &egui::Context,
    id: Id,
    target_value: Rect,
    animation_time: f32,
) -> Rect {
    let min_x = animate_eased(
        ctx,
        id.with("animate_rect_with_time, min.x"),
        target_value.min.x,
        animation_time,
        easing::circular_in_out,
    );
    let min_y = animate_eased(
        ctx,
        id.with("animate_rect_with_time, min.y"),
        target_value.min.y,
        animation_time,
        easing::circular_in_out,
    );
    let max_x = animate_eased(
        ctx,
        id.with("animate_rect_with_time, max.x"),
        target_value.max.x,
        animation_time,
        easing::circular_in_out,
    );
    let max_y = animate_eased(
        ctx,
        id.with("animate_rect_with_time, max.y"),
        target_value.max.y,
        animation_time,
        easing::circular_in_out,
    );

    Rect {
        min: Pos2 { x: min_x, y: min_y },
        max: Pos2 { x: max_x, y: max_y },
    }
}

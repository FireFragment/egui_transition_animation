use std::hash::Hash;

use egui::{emath::easing, Button, Color32, Id, Pos2, Rect, RichText, Shape, Stroke, WidgetText};
use egui_animation::animate_eased;

pub mod animated_frame;
pub mod selectable_value;
pub use animated_frame::AnimatedFrame;
pub use selectable_value::begin as begin_animated_selectable_value;

pub mod prelude {
    pub use super::begin_animated_selectable_value;
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

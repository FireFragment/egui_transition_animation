//! More low-level api for creating a frame switching position from behind another frame
//!
//! Not yet documented

use crate::*;
use egui::{layers::ShapeIdx, InnerResponse, Response, Shape, Ui};
use std::hash::Hash;

pub use frame_painter::*;
mod frame_painter {
    use egui::Rect;

    use super::*;

    pub trait FramePainter {
        fn paint_frame(&self, ui: &Ui, rect: &Rect, response: &Response) -> Shape;
    }

    impl FramePainter for egui::Frame {
        fn paint_frame(&self, _ui: &Ui, rect: &Rect, _response: &Response) -> Shape {
            self.paint(*rect)
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct DefaultFramePainter;

    impl FramePainter for DefaultFramePainter {
        fn paint_frame(&self, ui: &Ui, rect: &Rect, response: &Response) -> Shape {
            let visuals = ui.style().interact_selectable(response, true);

            egui::Frame::new()
                .fill(visuals.bg_fill)
                .stroke(visuals.bg_stroke)
                .corner_radius(visuals.corner_radius)
                .paint(*rect)
        }
    }
}

/// _Not yet documented_
#[derive(Debug, Clone)]
pub struct AnimatedFrame<FP: FramePainter> {
    frame_painter: FP,
}

impl Default for AnimatedFrame<DefaultFramePainter> {
    fn default() -> Self {
        AnimatedFrame::new_with_frame(DefaultFramePainter)
    }
}

impl<FP: FramePainter> AnimatedFrame<FP> {
    pub fn new_with_frame(frame: FP) -> Self {
        AnimatedFrame {
            frame_painter: frame,
        }
    }

    pub fn begin(self, ui: &mut Ui, id: impl Hash) -> State<FP> {
        let frame_shape_id = ui.painter().add(Shape::Noop);
        State {
            frame_painter: self.frame_painter,
            frame_shape_id,
            id: egui::Id::new(id).with("animated_frame"),
        }
    }
}

/// Typically created with [`AnimatedFrame::begin`].
/// See [`AnimatedFrame`] for more information.
pub struct State<FP: FramePainter> {
    frame_painter: FP,
    frame_shape_id: ShapeIdx,
    id: egui::Id,
}

impl<FP: FramePainter> State<FP> {
    /// Show a widget with an animated frame around it.
    pub fn show<R>(
        &self,
        ui: &mut egui::Ui,
        inner_ui: impl FnOnce(&mut egui::Ui) -> InnerResponse<R>,
    ) -> InnerResponse<R> {
        let response = inner_ui(ui);

        ui.painter().set(
            self.frame_shape_id,
            self.frame_painter.paint_frame(
                ui,
                &animate_rect_with_time(
                    ui.ctx(),
                    self.id,
                    response.response.rect,
                    ui.style().animation_time,
                ),
                &response.response,
            ),
        );

        response
    }
}

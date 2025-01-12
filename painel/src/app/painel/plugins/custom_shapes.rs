use crate::app::painel;

use painel::global::places;

use egui::{Color32, Response, Ui};
use walkers::{Plugin, Projector};

/// Sample map plugin which draws custom stuff on the map.
pub struct CustomShapes {}

impl Plugin for CustomShapes {
    fn run(self: Box<Self>, ui: &mut Ui, response: &Response, projector: &Projector) {
        // Position of the point we want to put our shapes.
        let position = places::capitol();

        // Compute pixel radius for a 100-meter circle.
        let radius = 100.0 * projector.scale_pixel_per_meter(position);

        // Project it into the position on the screen.
        let position = projector.project(position).to_pos2();

        let hovered = response
            .hover_pos()
            .map(|hover_pos| hover_pos.distance(position) < radius)
            .unwrap_or(false);

        ui.painter().circle_filled(
            position,
            radius,
            Color32::BLACK.gamma_multiply(if hovered { 0.5 } else { 0.2 }),
        );
    }
}

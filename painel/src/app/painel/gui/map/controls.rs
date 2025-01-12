use crate::app::painel;

use painel::plugins::image_data::ImagesPluginData;
use painel::types::provider::Provider;

use egui::{Align2, Ui, Window};

pub fn controls(
    ui: &Ui,
    selected_provider: &mut Provider,
    possible_providers: &mut dyn Iterator<Item = &Provider>,
    image: &mut ImagesPluginData,
) {
    Window::new("Satellite")
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .anchor(Align2::RIGHT_TOP, [-10., 10.])
        .fixed_size([150., 150.])
        .show(ui.ctx(), |ui| {
            ui.label("Omnibus");
            ui.collapsing("Map", |ui| {
                egui::ComboBox::from_label("Tile Provider")
                    .selected_text(format!("{:?}", selected_provider))
                    .show_ui(ui, |ui| {
                        for p in possible_providers {
                            ui.selectable_value(selected_provider, *p, format!("{:?}", p));
                        }
                    });
            });

            ui.collapsing("Images plugin", |ui| {
                ui.add(egui::Slider::new(&mut image.angle, 0.0..=360.0).text("Rotate"));
                ui.add(egui::Slider::new(&mut image.x_scale, 0.1..=3.0).text("Scale X"));
                ui.add(egui::Slider::new(&mut image.y_scale, 0.1..=3.0).text("Scale Y"));
            });
        });
}

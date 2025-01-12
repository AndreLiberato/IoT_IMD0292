pub mod global;
pub mod gui;
pub mod plugins;
pub mod types;

use global::places;

use std::collections::HashMap;

use walkers::{Map, MapMemory, Tiles};

pub use types::provider::Provider;

pub struct Painel {
    providers: HashMap<Provider, Box<dyn Tiles + Send>>,
    selected_provider: Provider,
    map_memory: MapMemory,
    images_plugin_data: plugins::ImagesPluginData,
    click_watcher: plugins::ClickWatcher,
}

impl super::App for Painel {
    fn new(cc: &eframe::CreationContext) -> Self {
        let egui_ctx = cc.egui_ctx.clone();
        egui_extras::install_image_loaders(&egui_ctx);

        // Data for the `images` plugin showcase.
        let images_plugin_data = plugins::ImagesPluginData::new(egui_ctx.to_owned());

        Self {
            providers: Provider::providers(egui_ctx.to_owned()),
            selected_provider: Provider::OpenStreetMap,
            map_memory: MapMemory::default(),
            images_plugin_data,
            click_watcher: Default::default(),
        }
    }
}

impl eframe::App for Painel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let rimless = egui::Frame {
            fill: ctx.style().visuals.panel_fill,
            ..Default::default()
        };

        egui::CentralPanel::default()
            .frame(rimless)
            .show(ctx, |ui| {
                // Typically this would be a GPS acquired position which is tracked by the map.
                let my_position = places::wroclaw_glowny();

                let tiles = self
                    .providers
                    .get_mut(&self.selected_provider)
                    .unwrap()
                    .as_mut();
                let attribution = tiles.attribution();

                // In egui, widgets are constructed and consumed in each frame.
                let map = Map::new(Some(tiles), &mut self.map_memory, my_position);

                // Optionally, plugins can be attached.
                let map = map
                    .with_plugin(plugins::places::places())
                    .with_plugin(plugins::image_data::images(&mut self.images_plugin_data))
                    .with_plugin(plugins::CustomShapes {})
                    .with_plugin(&mut self.click_watcher);

                // Draw the map widget.
                ui.add(map);

                // Draw utility windows.
                {
                    gui::map::zoom::zoom(ui, &mut self.map_memory);
                    gui::map::goto::go_to_my_position(ui, &mut self.map_memory);
                    self.click_watcher.show_position(ui);
                    gui::map::controls::controls(
                        ui,
                        &mut self.selected_provider,
                        &mut self.providers.keys(),
                        &mut self.images_plugin_data,
                    );
                    gui::map::acknowledge::acknowledge(ui, attribution);
                }
            });
    }
}

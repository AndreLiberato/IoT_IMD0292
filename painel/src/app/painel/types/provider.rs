use crate::app::painel;

use painel::global::http_options::http_options;
use painel::gui::tiles;

use std::collections::HashMap;

use egui::Context;
use walkers::{HttpTiles, Tiles};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Provider {
    OpenStreetMap,
    Geoportal,
    MapboxStreets,
    MapboxSatellite,
    LocalTiles,
}

impl Provider {
    pub fn providers(egui_ctx: Context) -> HashMap<Provider, Box<dyn Tiles + Send>> {
        let mut providers: HashMap<Provider, Box<dyn Tiles + Send>> = HashMap::default();

        providers.insert(
            Provider::OpenStreetMap,
            Box::new(HttpTiles::with_options(
                walkers::sources::OpenStreetMap,
                http_options(),
                egui_ctx.to_owned(),
            )),
        );

        providers.insert(
            Provider::Geoportal,
            Box::new(HttpTiles::with_options(
                walkers::sources::Geoportal,
                http_options(),
                egui_ctx.to_owned(),
            )),
        );

        providers.insert(
            Provider::LocalTiles,
            Box::new(tiles::local::LocalTiles::new(egui_ctx.to_owned())),
        );

        // Pass in a mapbox access token at compile time. May or may not be what you want to do,
        // potentially loading it from application settings instead.
        let mapbox_access_token = std::option_env!("MAPBOX_ACCESS_TOKEN");

        // We only show the mapbox map if we have an access token
        if let Some(token) = mapbox_access_token {
            providers.insert(
                Provider::MapboxStreets,
                Box::new(HttpTiles::with_options(
                    walkers::sources::Mapbox {
                        style: walkers::sources::MapboxStyle::Streets,
                        access_token: token.to_string(),
                        high_resolution: false,
                    },
                    http_options(),
                    egui_ctx.to_owned(),
                )),
            );
            providers.insert(
                Provider::MapboxSatellite,
                Box::new(HttpTiles::with_options(
                    walkers::sources::Mapbox {
                        style: walkers::sources::MapboxStyle::Satellite,
                        access_token: token.to_string(),
                        high_resolution: true,
                    },
                    http_options(),
                    egui_ctx.to_owned(),
                )),
            );
        }

        providers
    }
}

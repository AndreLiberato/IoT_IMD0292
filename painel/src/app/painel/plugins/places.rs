use crate::app::painel;

use painel::global::places;

use walkers::extras::{Place, Places, Style};
use walkers::Plugin;

pub fn places() -> impl Plugin {
    Places::new(vec![
        Place {
            position: places::wroclaw_glowny(),
            label: "Wrocław Główny\ntrain station".to_owned(),
            symbol: '🚆',
            style: Style::default(),
        },
        Place {
            position: places::dworcowa_bus_stop(),
            label: "Bus stop".to_owned(),
            symbol: '🚌',
            style: Style::default(),
        },
    ])
}

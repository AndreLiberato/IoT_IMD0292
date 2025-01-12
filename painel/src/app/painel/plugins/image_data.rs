use crate::app::painel;

use painel::global::places;

use walkers::{
    extras::{Image, Images, Texture},
    Plugin,
};

/// Helper structure for the `Images` plugin.
pub struct ImagesPluginData {
    pub texture: Texture,
    pub angle: f32,
    pub x_scale: f32,
    pub y_scale: f32,
}

impl ImagesPluginData {
    pub fn new(egui_ctx: egui::Context) -> Self {
        Self {
            texture: Texture::from_color_image(egui::ColorImage::example(), &egui_ctx),
            angle: 0.0,
            x_scale: 1.0,
            y_scale: 1.0,
        }
    }
}

/// Creates a built-in `Images` plugin with an example image.
pub fn images(images_plugin_data: &mut ImagesPluginData) -> impl Plugin {
    Images::new(vec![{
        let mut image = Image::new(images_plugin_data.texture.clone(), places::wroclavia());
        image.scale(images_plugin_data.x_scale, images_plugin_data.y_scale);
        image.angle(images_plugin_data.angle.to_radians());
        image
    }])
}

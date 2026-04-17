use macroquad::prelude::*;

pub struct Asset {
    pub texture: Option<Texture2D>
}

impl Asset {
    pub async fn new(path: &str) ->Asset{

        match load_texture(path).await {
            Ok(tex) => {
                tex.set_filter(FilterMode::Linear);
                Asset{
                    texture: Some(tex),
                }
            }
            Err(e) => {
                eprintln!("Failed to load texture: {e:?}");
                Asset { texture: (None) }
            }
        }
    }
}
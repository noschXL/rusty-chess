use macroquad::miniquad::TextureParams;
use macroquad::prelude::*;
use crate::engine::pieces::*;
use crate::engine::constants::*;
use crate::gui::assets::Asset;

pub async fn draw_piece(piece: &Piece, x: f32, y: f32, size: f32, piece_asset: &Asset) {
    if piece_asset.texture.is_none() {
        panic!("piece_asset is None!")
    }
    let piece_color = piece.get_color();
    let piece_type = piece.get_type();

    if piece_type == piece_constants::EMPTY {
        warn!("piece type is Empty")
    }

    let mut width = 0.0;
    let mut height = 0.0;

    if let Some(tex) = &piece_asset.texture {
        width = tex.width();
        height = tex.height();
    }

    if width == 0.0 {
        panic!("width of piece_assets is 0")
    }
    if height == 0.0 {
        panic!("height of piece_assets is 0")
    }

    let source_x = ((piece_type >> 1) -1) as f32 * width / 6.0;
    let source_y = piece_color as f32 * height / 2.0;

    let source = Some(Rect::new(source_x, source_y, width / 6.0, height / 2.0));

    draw_texture_ex(
        piece_asset.texture.as_ref().unwrap(),
        x,
        y,
        WHITE,
        DrawTextureParams{
            source: source,
            dest_size: Some(Vec2 {x: size,y: size }),
            ..Default::default()
            
        }
    );

}
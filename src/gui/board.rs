use macroquad::prelude::*;
use std::cmp::min;
use crate::engine::constants::piece_constants::EMPTY;
use crate::gui::{assets::Asset, constants, piece::*};
use crate::engine::board::*;


async fn render_blank_board() {
    clear_background(RED);
        
    let width = screen_width();
    let height = screen_height();
    let square_size = min(width as i32 / 8, height as i32 / 8) as f32;
    let width_offset = (width - square_size * 8.0) / 2.0;
    let height_offset = (height - square_size * 8.0) / 2.0;

    for x in 0..8 {
        for y in 0..8 {
            let mut currentcolor = constants::colors::WHITESQUARE;
            
            if (x + y) % 2 == 0 {
                currentcolor = constants::colors::BLACKSQUARE;
            }
            
            draw_rectangle(x as f32 * square_size + width_offset,y as f32 * square_size + height_offset,square_size,square_size,currentcolor)
        }
    }
}

async fn render_pieces(board: &Board, piece_asset: &Asset) {
    let width = screen_width();
    let height = screen_height();
    let square_size = min(width as i32 / 8, height as i32 / 8) as f32;
    let width_offset = (width - square_size * 8.0) / 2.0;
    let height_offset = (height - square_size * 8.0) / 2.0;
    for i in 0..64{
        let piece = board.get_piece_at(i as usize);
        if piece.get_type() == EMPTY {
            continue;
        }
        let x = (i % 8) as f32 * square_size + width_offset;
        let y = (i / 8) as f32 * square_size + height_offset;
        draw_piece(piece, x, y, square_size, piece_asset).await;
    }
}

pub async fn render_board(piece_asset: &Asset, board: &Board) {
    render_blank_board().await;
    render_pieces(board, piece_asset).await;
}

use macroquad::prelude::*;
use std::cmp::min;
use crate::engine::constants::piece_constants::EMPTY;
use crate::gui::{assets::Asset, constants, piece::*};
use crate::engine::board::*;

pub struct MouseState {
    pub selected: bool,
    pub selected_piece: i32,
    pub last_frame_down: bool,
}

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

fn relative_to_index(rel_pos: (f32, f32), square_size: f32) -> i32{
    (rel_pos.0 / square_size).floor() as i32 + (rel_pos.1 / square_size).floor() as i32 * 8
}

pub async fn handle_mouse(piece_asset: &Asset, board: &Board, state: &mut MouseState) -> Move{
    let width = screen_width();
    let height = screen_height();
    let square_size = min(width as i32 / 8, height as i32 / 8) as f32;
    let width_offset = (width - square_size * 8.0) / 2.0;
    let height_offset = (height - square_size * 8.0) / 2.0;
    
    let board_rect = Rect::new(width_offset, height_offset, width-2.0*width_offset, height-2.0*height_offset);

    let down = is_mouse_button_down(MouseButton::Left);
    let pos = mouse_position();
    let rel_pos = (pos.0-width_offset, pos.1-height_offset);
    let index = relative_to_index(rel_pos, square_size);
    if down & !state.selected {
        if board_rect.contains(Vec2 { x: pos.0, y: pos.1 }) {
            state.selected = true;
            state.selected_piece = index as i32;
        }

        Move::new_invalid()

    }else if !down & state.selected {
        let cap = !board.get_piece_at(index as usize).is_empty();
        let ret = Move::new(
            state.selected_piece,
            index,
            cap,
            0,
            false,
        );
        state.selected = false;
        state.selected_piece = -1;

        ret
    }else {
        Move::new_invalid()
    }

}

pub async fn render_board(piece_asset: &Asset, board: &Board) {
    render_blank_board().await;
    render_pieces(board, piece_asset).await;
}

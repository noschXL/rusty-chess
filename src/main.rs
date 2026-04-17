mod engine;
mod gui;

use macroquad::prelude::*;
use crate::engine::constants::fen::*;

use crate::gui::board::MouseState;

fn window_conf() -> Conf {
    Conf {
        window_title: "MyGame".to_string(),
        sample_count: 4, // MSAA 4x
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let piece_asset = gui::init().await;
    let mut board = engine::board::Board::new_fen(&START.to_string());
    //board.set_piece_data_at(0, piece_constants::WHITE | piece_constants::QUEEN);
    let mut mstate = MouseState{selected: false, selected_piece: 0, last_frame_down: false};
    
    loop {
        let mov = gui::run(&piece_asset, &board, &mut mstate).await;
        if !mov.is_invalid() {
            board.make_move(mov);
        }

        if is_key_pressed(KeyCode::Z){
            board.unmake_move();
        }


        next_frame().await;
    }
}

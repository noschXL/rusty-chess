mod engine;
mod gui;

use macroquad::prelude::*;

use crate::engine::constants::*;
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
    let mut board = engine::board::Board::new_empty();
    board.set_piece_data_at(0, piece_constants::WHITE | piece_constants::QUEEN);
    
    loop {
        gui::render(&piece_asset, &board).await;


        next_frame().await;
    }
}

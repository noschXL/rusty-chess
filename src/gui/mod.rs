use crate::{engine::board::{Board, Move}, gui::{assets::Asset, board::MouseState}};

pub mod board;
pub mod piece;
pub mod constants;
pub mod assets;


pub async fn init() -> Asset{
    let piece_asset: Asset = Asset::new(&"assets/pieces.png").await;
    piece_asset
}

pub async fn run(piece_asset: &Asset, board: &Board, state: &mut MouseState) -> Move{
    board::render_board(piece_asset, board).await;
    board::handle_mouse(piece_asset, board, state).await
}

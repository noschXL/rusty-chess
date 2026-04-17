use crate::{engine::board::Board, gui::assets::Asset};

pub mod board;
pub mod piece;
pub mod constants;
pub mod assets;


pub async fn init() -> Asset{
    let piece_asset: Asset = Asset::new(&"assets/pieces.png").await;
    piece_asset
}

pub async fn render(piece_asset: &Asset, board: &Board) {
    board::render_board(piece_asset, board).await
}

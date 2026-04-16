pub mod board;
pub mod piece;
pub mod constants;

pub async fn render_board() {
    board::render_board().await
}
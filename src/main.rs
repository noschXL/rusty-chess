mod engine;
mod gui;

#[macroquad::main("MyGame")]
async fn main() {

    let piece_asset = gui::init().await;
    let board = engine::board::Board::new_empty();
    
    loop {
        gui::render(&piece_asset, &board).await;
    }
}

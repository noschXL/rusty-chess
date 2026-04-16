mod engine;
mod gui;

#[macroquad::main("MyGame")]
async fn main() {
    
    loop {
        gui::render_board().await;
    }
}

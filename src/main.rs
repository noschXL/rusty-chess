use macroquad::prelude::*;
use std::cmp::min;

#[macroquad::main("MyGame")]
async fn main() {

    println!("works :)");
    let white_square = Color::from_rgba(0x52,0x31,0x1e,0xff);
    let black_square = Color::from_rgba(0xc5,0x95,0x62,0xff);

    loop {
        clear_background(RED);
        
        let width = screen_width();
        let height = screen_height();
        let square_size = min(width as i32 / 8, height as i32 / 8) as f32;
        let width_offset = (width - square_size * 8.0) / 2.0;
        let height_offset = (height - square_size * 8.0) / 2.0;
        
        for x in 0..8 {
            for y in 0..8 {
                let mut currentcolor = white_square;
                
                if (x + y) % 2 == 0 {
                    currentcolor = black_square;
                }
                
                draw_rectangle(x as f32 * square_size + width_offset,y as f32 * square_size + height_offset,square_size,square_size,currentcolor)
            }
        }

        next_frame().await
    }
}

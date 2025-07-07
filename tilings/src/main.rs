use macroquad::prelude::*;

const SIZE: f32 = 10.0;
const SIDES: f32 = 5.0;

#[macroquad::main("MyGame")]
async fn main() {
    let width = screen_width();
    let height = screen_height();

    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;

    let mut positions: Vec<(f32, f32)> = Vec::new();
    while y < width {
        x = 0.0;
        while x < height {
            positions.push((x, y));
            x += SIZE;
        }

        y += SIZE;
    }

    loop {
        clear_background(BLACK);

        for p in &positions {
            draw_poly(p.0, p.1, SIZE as uj/2.0 as u8, 30.0, 0.0, WHITE);
        }

        next_frame().await;
    }
}


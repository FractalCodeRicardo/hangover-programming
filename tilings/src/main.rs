use macroquad::prelude::*;

const SIZE: f32 = 10.0;
const SIDES: u8 = 8;
const ROTATION: f32 = 5.0;


#[macroquad::main("MyGame")]
async fn main() {
    let w = screen_width();
    let h = screen_height();

    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut r: f32 = 0.0;
    let mut s: u8 = 3;
    let mut tilings: Vec<(f32, f32, f32, u8)> = Vec::new();

    while y < h*1.5 {
        x = 0.0;

        while x < w {
            x += SIZE;
            tilings.push((x, y, r, s));
            r += ROTATION;
            s += 1;

            if s > SIDES {
                s = 3;
            }

        }
        y += SIZE;
    }

    loop {
        clear_background(RED);

        for t in &tilings{
            draw_poly(t.0, t.1, t.3, SIZE / 2.0, t.2, GREEN);
        }

        next_frame().await
    }
}

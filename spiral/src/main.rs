use macroquad::prelude::*;

#[macroquad::main("My Macroquad Game")]
async fn main() {
    let mut r: f32 = 0.0;
    let mut angle: f32 = 0.0;
    let alpha: f32 = 1.0;

    let mut x = 0.0;
    let mut y = 0.0;

    let mut points: Vec<(f32, f32)> = Vec::new();
    loop {
        clear_background(BLACK); // Clear the screen with a red background
        r = alpha * angle;

        x = (screen_width() / 2.0) + r * angle.sin();
        y = (screen_height() / 2.0) + r * angle.cos();

        points.push((x, y));

        angle = angle + 0.5;

        for p in &points {
            draw_rectangle(
                 p.0,
                 p.1,
                2.0,
                2.0,
                WHITE,
            ); 

            draw_circle(p.0, p.1, 10.0, WHITE);
        }
        next_frame().await; // Wait for the next frame
    }
}

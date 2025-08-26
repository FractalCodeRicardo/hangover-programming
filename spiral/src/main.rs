use macroquad::prelude::*;

#[macroquad::main("My Game")]
async fn main() {
    let mut r: f32 = 0.0;
    let mut angle: f32 = 0.0;
    let alpha: f32 = 1.0;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let every_point = 10;

    let mut points: Vec<(f32, f32)> = Vec::new();

    loop {
        r = alpha * angle;
        angle += 0.5;

        y = r * angle.sin();
        x = r * angle.cos();

        y = y + screen_height() / 2.0;
        x = x + screen_width() / 2.0;

        points.push((x, y));

        clear_background(BLACK);

        let mut i = 0;
        for p in &points {
            draw_rectangle(p.0, p.1, 2.0, 2.0, WHITE);

            if i == every_point {

                draw_poly_lines(p.0, p.1, 6, r / 3.0, angle, 1.0, GREEN);

                i = 0;
            }

            i += 1;
        }

        next_frame().await
    }
}

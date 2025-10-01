use core::num;

use macroquad::{prelude::*, rand::RandomRange};

struct Point {
    x: f32,
    y: f32,
}

fn create_point(theta: f32, n: f32, d: f32) -> Point {
    let alpha = 2.;
    let k = n / d;
    let thetak = theta * k;
    let r = alpha * thetak.cos();

    let x = r * theta.cos();
    let y = r * theta.sin();

    Point { x: x, y: y }
}

fn draw_points(points: &Vec<Point>, count: usize) {
    let center_x = screen_width() / 2.;
    let center_y = screen_height() / 2.;
    let zoom = 150.;

    for i in 0..count {
        let p = &points[i];
        draw_circle(p.x * zoom + center_x, p.y * zoom + center_y, 2., GREEN)
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut theta: f32 = 0.0;
    let n = 4.;
    let d = 9.;
    let mut points: Vec<Point> = Vec::new();

    let mut number_of_points =20;

    while theta < 5000.0 {
        let new_point = create_point(theta.to_radians(), n, d);
        points.push(new_point);

        theta = theta + 1.0;
    }

    loop {
        clear_background(BLACK);

        draw_points(&points, number_of_points);

        number_of_points = number_of_points + 100;
        number_of_points = number_of_points.min(points.len());

        next_frame().await
    }
}

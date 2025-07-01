use macroquad::prelude::*;


#[macroquad::main("ASCII RGB Viewer")]
async fn main() {
    let ascii: Vec<char> = " .:-=+*#%@".chars().collect(); // shorter set for clarity

    let image = load_image("./gracias1.png").await.unwrap_or_else(|err| {
        eprintln!("Failed to read file: {}", err);
        std::process::exit(1);
    });

    let width = image.width();
    let height = image.height();
    let bytes = image.bytes;

    let font_size = 8.0; 
    let scale =3;       
    let mut ascii_grid: Vec<(char, f32, f32)> = vec![];

    for y in (0..height).step_by(scale) {
        for x in (0..width).step_by(scale) {
            let idx = ((y * width + x) * 4) as usize;
            let r = bytes[idx] as u16;
            let g = bytes[idx + 1] as u16;
            let b = bytes[idx + 2] as u16;

            let brightness = (r + g + b) / 3;
            let ascii_index = brightness as usize * (ascii.len() - 1) / 255;
            let ch = ascii[ascii_index];

            ascii_grid.push((ch, x as f32, y as f32));
        }
    }

    // Draw loop
    loop {
        clear_background(YELLOW);

        for (ch, x, y) in &ascii_grid {
            draw_text(&ch.to_string(), *x, *y, font_size, BLACK);
        }

        next_frame().await;
    }
}

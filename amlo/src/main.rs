use macroquad::prelude::*;

#[macroquad::main("RGB Reader")]
async fn main() {
    clear_background(WHITE);
    let mut ascii: Vec<char> = "ÿþýüûúùø÷öõôóòñðïîíìëêéèçæåäãâáàßÞÝÜÛÚÙØ×ÖÕÔÓÒÑÐÏÎÍÌËÊÉÈÇÆÅÄÃÂÁÀ¿»½¼»║╣¸·¶µ´│▓▒░¯®¬«ª©¨§¦¥¤£¢¡~}|{zyxwvutsrqponmlkjihgfedcba`_^]\\[ZYXWVUTSRQPONMLKJIHGFEDCBA@?>=<;:9876543210/.-,+*)('&%$#".chars().collect();

    // Load image from file
    let image = load_image("./gracias.png").await.unwrap_or_else(|err| {
        eprintln!("Failed to read file: {}", err);
        std::process::exit(1);
    });
    // Get width and height
    let width = image.width();
    let height = image.height();

    // Access raw RGBA bytes
    let bytes = image.bytes;

    // Iterate over pixels
    for y in 0..height {
        for x in 0..width {
            let index = ((y * width + x) * 4) as usize;
            let r = bytes[index];
            let g = bytes[index + 1];
            let b = bytes[index + 2];
            let a = bytes[index + 3];

            let b = (r + g + b) / 3;
            let ib = b as usize;

            let item = if ib < ascii.len() { ascii[ib] } else { ' ' };

            draw_text(&item.to_string(), x as f32, y as f32, 1.0, BLACK);
            println!("Pixel ({}, {}): R={}, G={}, B={}, A={}", x, y, r, g, b, a);
        }
    }

    loop {
        next_frame().await;
    }
}

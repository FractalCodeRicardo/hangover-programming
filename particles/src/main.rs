use macroquad::prelude::*;

struct Particle {
    position: (f32, f32),
    direction: (f32, f32),
    speed: f32,
    color: Color
}

impl Particle {
    pub fn new() -> Self {
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        
        Particle {
            position: (cx, cy),
            direction: Particle::get_random_direction(),
            speed: rand::gen_range(2.0, 6.0),
            color: Particle::get_color()
        }
    }

    pub fn get_color() -> Color {
        let color = [MAGENTA, GREEN, YELLOW, WHITE];
        let i = rand::gen_range(0.0, 3.9) as usize;

        return color[i];
    }

    pub fn get_random_direction() -> (f32, f32) {
        let x = rand::gen_range(0.0, 1.0) * Particle::get_random_sign();
        let y = rand::gen_range(0.0, 1.0) * Particle::get_random_sign();

        // print!("direction ({}, {})", x, y);
        return (x, y);
    }

    pub fn get_random_sign() -> f32 {
        let v = rand::gen_range(0.0, 1.0);

        if v < 0.5 {
            return -1.0;
        }

        return 1.0;
    }

    pub fn print(&self) {
        draw_circle(self.position.0, self.position.1, 2.0, self.color);
    }

    pub fn mov(&mut self) {
        let x = self.position.0;
        let y = self.position.1;

        let nx = x + self.direction.0 * self.speed;
        let ny = y + self.direction.1 * self.speed;

        self.position = (nx, ny);
    }

    pub fn change_direction(&mut self) {
        self.direction = Particle::get_random_direction();
    }

    pub fn change_speed(&mut self) {
        self.speed = rand::gen_range(2.0, 6.0)
    }
}

#[macroquad::main("My Game")] // The attribute sets the window title
async fn main() {
    let mut particles: Vec<Particle> = Vec::new();

    for i in 1..=7000 {
        particles.push(Particle::new());
    }

    loop {
        clear_background(BLACK);

        for particle in &mut particles {
            particle.print();
            particle.mov();
            particle.change_direction();
            particle.change_speed();
        }

        next_frame().await
    }
}


use macroquad::{prelude::*, rand::RandomRange};
const DELAY: f64 = 1.;
const CREATE_EVERY: usize= 5;
const SPEED: f32 = 5.;

struct Point {
    x: f32,
    y: f32,
}

struct Text {
    text: String,
    pos: Point,
}

impl Text {
    fn new(x: f32, y: f32, text: String) -> Self {
        let pos = Point { x: x, y: y };

        Text {
            text: text,
            pos: pos,
        }
    }

    fn random_color() -> Color {
        let r = RandomRange::gen_range(0., 1.);
        let g = RandomRange::gen_range(0., 1.);
        let b = RandomRange::gen_range(0., 1.);
        return Color::new(r, g, b, 1.) ;
    }

    fn draw(&self) {
        let color = Text::random_color();

        // println!("drawing at {} {}", self.pos.x, self.pos.y);
        draw_text(&self.text,
            self.pos.x,
            self.pos.y,
            40.,
            color
        );
    }

    fn mov(&mut self) {
        self.pos = Point {
            x: self.pos.x,
            y: self.pos.y + SPEED
        }
    }
}

struct Game {
    texts: Vec<Text>,
    last_created: usize
}

impl Game {
    fn new() -> Game {
        Game { texts: Vec::new(), last_created: 0 }
    }

    fn add_text(&mut self) {
        let width = screen_width();

        let y = 30.;
        let x = RandomRange::gen_range(100., width);

        let string = String::from("Hell0");
        let text = Text::new(x, y, string);
        self.texts.push(text);
    }

    fn draw_texts(&self) {
        for t in &self.texts {
            t.draw();
        }
    }

    fn draw(&self) {
        self.draw_texts();
    }

    fn mov_texts(&mut self) {
        for t in &mut self.texts {
            t.mov();
        }
    }

    fn mov(&mut self) {
        self.mov_texts();
        self.last_created += 1;

        if self.last_created >= CREATE_EVERY {
            self.add_text();
            self.last_created = 0;
        }
    }

    fn kill_text(&mut self, text: &str) -> bool {
        for i in 0..self.texts.len() {
            let t = &self.texts[i];

            if t.text == text {
                self.texts.remove(i);
                return true;
            } 
        }

        return false;
    }

}

#[macroquad::main("MyGame")]
async fn main() {
    let mut time = 0.;
    let mut game = Game::new();
    let mut current_string = String::new();

    game.add_text();
    loop {

        clear_background(BLACK);

        game.draw();

        let pressed = get_char_pressed();

        if pressed.is_some() {
            current_string.push(pressed.unwrap());

            println!("String {}", current_string);
            if game.kill_text(&current_string) {
                current_string = String::new();
            }
        }

        if get_time() - time > DELAY {
            game.mov();
            time = get_time();
        }

        next_frame().await
    }
}

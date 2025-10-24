use macroquad::{prelude::*, rand::RandomRange};

const SPEED: f32 = 10.;
const DELAY: f64 = 0.7;
const ADD_TEXT_EVERY_MOV: usize = 2;

struct Point {
    x: f32,
    y: f32,
}

struct Text {
    pos: Point,
    text: String,
}

impl Text {
    fn random_color() -> Color {
        let r = RandomRange::gen_range(0., 1.);
        let g = RandomRange::gen_range(0., 1.);
        let b = RandomRange::gen_range(0., 1.);

        return Color::new(r, g, b, 1.);
    }

    fn draw(&self) {
        let color = Text::random_color();
        draw_text(&self.text, self.pos.x, self.pos.y, 40., color);
    }

    fn mov(&mut self) {
        self.pos = Point {
            x: self.pos.x,
            y: self.pos.y + SPEED,
        }
    }
}

struct Game {
    texts: Vec<Text>,
    last_added: usize,
    current_text: String,
}

impl Game {
    fn new() -> Self {
        Game {
            texts: Vec::new(),
            last_added: 0,
            current_text: String::from(""),
        }
    }

    fn add_text(&mut self) {
        let y = 100.;
        let x = RandomRange::gen_range(0. + 100., (screen_width() / 2.) - 100.);
        let pos = Point { x: x, y: y };
        let word = Game::random_word();

        let text = Text {
            pos: pos,
            text: word,
        };

        self.texts.push(text);
    }

    fn draw(&self) {
        self.draw_texts();
        self.draw_current_text();
    }

    fn draw_texts(&self) {
        for t in &self.texts {
            t.draw();
        }
    }

    fn draw_current_text(&self) {
        draw_text(
            &self.current_text,
            screen_width() - 200.,
            screen_height() - 50.,
            50.,
            WHITE,
        );
    }

    fn add_character(&mut self, character: char) {
        self.current_text.push(character);
        let killed = self.kill_text();

        if killed {
            self.reset();
        }
    }

    fn reset(&mut self) {
        self.current_text = String::new();
    }

    fn kill_text(&mut self) -> bool {
        for i in 0..self.texts.len() {
            let text = &self.texts[i].text;

            if text == &self.current_text {
                self.texts.remove(i);
                return true;
            }
        }

        return false;
    }

    fn mov_texts(&mut self) {
        for t in &mut self.texts {
            t.mov();
        }
    }

    fn mov(&mut self) {
        self.mov_texts();
        self.last_added += 1;

        if self.last_added > ADD_TEXT_EVERY_MOV {
            self.last_added = 0;
            self.add_text();
        }
    }

    fn random_word() -> String {
        let words: Vec<&str> = vec!["hello", "word", "hope", "god", "thisisfine"];
        let index = RandomRange::gen_range(0, words.len());

        return String::from(words[index]);
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut game = Game::new();
    let mut time = get_time();

    game.add_text();
    loop {
        clear_background(BLACK);

        game.draw();

        let character = get_char_pressed();
        if character.is_some() {
            game.add_character(character.unwrap());
        }

        if is_key_pressed(KeyCode::Escape) {
            game.reset();
        }

        if get_time() - time > DELAY {
            time = get_time();
            game.mov();
        }

        next_frame().await
    }
}

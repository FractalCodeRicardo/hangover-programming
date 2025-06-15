use macroquad::prelude::*;
use macroquad::{color::WHITE, shapes::draw_rectangle};
use std::vec;

const SQUARE_SIZE: f32 = 30.0;
const SPEED: f32 = 1.0;
const FRAME_DELAY: f64 = 0.3;

struct Square {
    pub x: f32,
    pub y: f32,
}

impl Square {
    pub fn new(x: f32, y: f32) -> Square {
        Square { x: x, y: y }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.x * SQUARE_SIZE,
            self.y * SQUARE_SIZE,
            SQUARE_SIZE,
            SQUARE_SIZE,
            WHITE,
        );
    }
}

struct Snake {
    pub dir_x: f32,
    pub dir_y: f32,
    pub squares: Vec<Square>,
}

impl Snake {
    pub fn new() -> Snake {
        let first = Square::new(0.0, 0.0);
        let vector = vec![first];
        Snake {
            squares: vector,
            dir_x: 0.0,
            dir_y: SPEED,
        }
    }

    pub fn get_head(&self) -> &Square {
        return &self.squares[0];
    }

    pub fn draw(&self) {
        for square in &self.squares {
            square.draw();
        }
    }

    pub fn right(&mut self) {
        self.dir_x = SPEED;
        self.dir_y = 0.0;
    }

    pub fn left(&mut self) {
        self.dir_x = SPEED * -1.0;
        self.dir_y = 0.0;
    }

    pub fn down(&mut self) {
        self.dir_x = 0.0;
        self.dir_y = SPEED;
    }

    pub fn up(&mut self) {
        self.dir_x = 0.0;
        self.dir_y = SPEED * -1.0;
    }

    pub fn mov(&mut self) {
        let mut i: usize = self.squares.len() - 1;

        while i >= 1 {
            self.squares[i].x = self.squares[i - 1].x;
            self.squares[i].y = self.squares[i - 1].y;
            i -= 1;
        }

        self.squares[0].x += self.dir_x;
        self.squares[0].y += self.dir_y;
    }

    pub fn insert(&mut self, x: f32, y: f32) {
        self.squares.insert(0, Square::new(x, y));
    }
}

struct Game {
    pub snake: Snake,
    pub food: Square,
}

impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake::new(),
            food: Square::new(10.0, 10.0),
        }
    }

    pub fn right(&mut self) {
        self.snake.right();
    }

    pub fn left(&mut self) {
        self.snake.left();
    }

    pub fn down(&mut self) {
        self.snake.down();
    }

    pub fn up(&mut self) {
        self.snake.up();
    }

    pub fn draw(&mut self) {
        self.snake.draw();
        self.food.draw();
    }

    pub fn mov(&mut self) {
        self.log();
        self.eat();
        self.snake.mov()
    }

    fn log(&self) {
        let first = &self.snake.squares[0];
        let food = &self.food;

        println!("Head ({},{})", first.x, first.y);
        println!("Food ({},{})", food.x, food.y);
    }

    pub fn eat(&mut self) {
        let head = self.snake.get_head();
        let next_head = Square {
            x: head.x + self.snake.dir_x,
            y: head.y + self.snake.dir_y,
        };

        let food = &self.food;
        let eated = food.x == next_head.x && food.y == next_head.y;

        if !eated {
            return;
        }

        self.snake.insert(food.x, food.y);

        let x: f32 = rand::gen_range(1, 20) as f32; 
        let y: f32 = rand::gen_range(1, 20) as f32;
        self.food = Square::new(x, y);
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut game = Game::new();
    let mut timer_start = get_time();
    loop {
        clear_background(BLACK);
        game.draw();

        if get_time() - timer_start >= FRAME_DELAY {
            game.mov();
            timer_start = get_time();
        }

        if is_key_released(KeyCode::Right) {
            game.right();
        }

        if is_key_released(KeyCode::Left) {
            game.left();
        }

        if is_key_released(KeyCode::Up) {
            game.up();
        }

        if is_key_released(KeyCode::Down) {
            game.down();
        }

        next_frame().await;
    }
}

use macroquad::prelude::*;

const SQUARE_SIZE: f32 = 30.0;
const SPEED: f32 = 1.0;
const DELAY: f64 = 0.3;

struct Square {
    x: f32,
    y: f32,
}

impl Square {
    pub fn new(x: f32, y: f32) -> Square {
        Square { x: x, y: y }
    }
}

struct Game {
    pub dir_x: f32,
    pub dir_y: f32,
    pub food: Square,
    pub snake: Vec<Square>,
}

impl Game {
    pub fn new() -> Game {
        let first = Square::new(0.0, 0.0);

        Game {
            snake: vec![first],
            dir_x: 1.0,
            dir_y: 0.0,
            food: Square::new(5.0, 5.0)
        }
    }

    pub fn mov(&mut self) {
        self.log();
        self.move_snake();
        self.eat();
    }

    pub fn right(&mut self) {
        self.dir_x = SPEED;
        self.dir_y = 0.0;
    }

    pub fn left(&mut self) {
        self.dir_x = SPEED * -1.0;
        self.dir_y = 0.0;
    }

    pub fn up(&mut self) {
        self.dir_x = 0.0;
        self.dir_y = SPEED*-1.0;
    }

    pub fn down(&mut self) {
        self.dir_x = 0.0;
        self.dir_y = SPEED;
    }

    pub fn draw(&mut self) {
        self.draw_snake();
        self.draw_food();
    }

    pub fn draw_food(&self) {
        self.draw_square(self.food.x, self.food.y);
    }

    pub fn move_snake(&mut self) {
        let mut i = self.snake.len() -1;

        while i >= 1 {
            self.snake[i].x = self.snake[i-1].x;
            self.snake[i].y = self.snake[i-1].y;
            i -= 1;
        }
        self.snake[0].x += self.dir_x;
        self.snake[0].y += self.dir_y;
    }

    pub fn draw_snake(&mut self) {
        for s in &self.snake {
            self.draw_square(s.x, s.y);
        }
    }

    pub fn draw_square(&self, x: f32, y: f32) {
        draw_rectangle(
            x * SQUARE_SIZE,
            y * SQUARE_SIZE,
            SQUARE_SIZE,
            SQUARE_SIZE,
            WHITE,
        );
    }

    pub fn log(&self) {
        let head = &self.snake[0];
        println!("HEAD ({}, {})", head.x, head.y);
    }

    pub fn eat(&mut self) {
        let head = &self.snake[0];
        let next_head = Square {
            x:  head.x+self.dir_x,
            y:  head.y+self.dir_y
        };

        let food = &self.food;

        let eated = next_head.x == food.x && next_head.y == food.y;

        if !eated {
            return;
        }
        
        println!("Eaten!");
        self.snake.insert(0, Square {x: food.x, y: food.y});

        let fx = rand::gen_range(1, 20) as f32;
        let fy = rand::gen_range(1, 20) as f32;

        self.food.x = fx;
        self.food.y = fy;
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut game = Game::new();
    let mut start_timer = get_time();
    loop {

        clear_background(BLACK);
        game.draw();

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

        if get_time() - start_timer > DELAY {
            start_timer = get_time();
            game.mov();
        }
        next_frame().await
    }
}

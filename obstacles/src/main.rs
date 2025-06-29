use std::fs::{self};

use macroquad::prelude::*;
const SQUARE_SIZE: f32 = 10.0;
const DELAY: f64 = 0.3;
const SPEED: f32 = 1.0;
struct Position {
    x: f32,
    y: f32,
}

impl Position {
    fn new(x: f32, y: f32) -> Position {
        Position { x: x, y: y }
    }
}

struct Game {
    obstacles: Vec<Position>,
    ball: Position,
    ball_direction: Position,
}

impl Game {
    fn new() -> Game {
        Game {
            obstacles: Game::init_obstacles(),
            ball: Position::new(5.0, 1.0),
            ball_direction: Position::new(1.0, 1.0),
        }
    }

    fn mov(&mut self) {
        self.handle_crash();
        self.ball.x += self.ball_direction.x * SPEED;
        self.ball.y += self.ball_direction.y * SPEED;
    }

    fn is_crash_up(&self) -> bool {
        for obs in &self.obstacles {
            let dx = obs.x + SQUARE_SIZE / 2.0;
            let dy = obs.y * SQUARE_SIZE;

            let bx = self.ball.x + SQUARE_SIZE / 2.0;
            let by = self.ball.y * SQUARE_SIZE;

            if dx == bx && dy == by {
                return true;
            }
        }

        return false;
    }

    fn top_center(position: &Position)-> Position {
        let x = position.x * SQUARE_SIZE;
        let y = position.y * SQUARE_SIZE + SQUARE_SIZE / 2.0;

        return Position {x: x, y: y};
    }
    fn left_center(position: &Position)-> Position {

        let x = position.x * SQUARE_SIZE;
        let y = position.y * SQUARE_SIZE + SQUARE_SIZE / 2.0;

        return Position {x: x, y: y};
    }


    fn right_center(position: &Position)-> Position {
        let x = position.x * SQUARE_SIZE + SQUARE_SIZE;
        let y = position.y * SQUARE_SIZE + SQUARE_SIZE / 2.0;

        return Position {x: x, y: y};
    }

    fn is_crash_down(&self) -> bool {
        for obs in &self.obstacles {
            let bx = obs.x + SQUARE_SIZE / 2.0;
            let by = obs.y * SQUARE_SIZE;

            let dx = self.ball.x + SQUARE_SIZE / 2.0;
            let dy = self.ball.y * SQUARE_SIZE;

            if dx == bx && dy == by {
                return true;
            }
        }
        return false;
    }

    fn is_crash_right(&self) -> bool {
        for obs in &self.obstacles {
            let lox = obs.x * SQUARE_SIZE;
            let loy = obs.y + SQUARE_SIZE / 2.0;

            let rbx = self.ball.x * SQUARE_SIZE + SQUARE_SIZE;
            let rby = self.ball.y + SQUARE_SIZE / 2.0;

            if lox == rbx && loy == rby {
                return true;
            }
        }
        return false;
    }

    fn is_crash_left(&self) -> bool {
        for obs in &self.obstacles {
            let rbx = obs.x * SQUARE_SIZE;
            let rby = obs.y + SQUARE_SIZE / 2.0;

            let lox = self.ball.x * SQUARE_SIZE + SQUARE_SIZE;
            let loy = self.ball.y + SQUARE_SIZE / 2.0;

            if lox == rbx && loy == rby {
                return true;
            }
        }
        return false;
    }

    fn handle_crash(&mut self) {
        if self.is_crash_up() {
            self.changey_direction();
            return;
        }

        if self.is_crash_down() {
            self.changey_direction();
            return;
        }

        if self.is_crash_right() {
            self.changex_direction();
            return;
        }

        if self.is_crash_left() {
            self.changex_direction();
            return;
        }
    }

    fn changey_direction(&mut self) {
        self.ball_direction.y = self.ball_direction.y * -1.0;
    }

    fn changex_direction(&mut self) {
        self.ball_direction.x = self.ball_direction.x * -1.0;
    }

    fn get_content_file() -> Vec<String> {
        let file = fs::read_to_string("./src/board.txt").unwrap_or_else(|err| {
            eprintln!("Failed to read file: {}", err);
            std::process::exit(1);
        });

        let list: Vec<_> = file.lines().map(|line| line.to_string()).collect();

        return list;
    }

    fn init_obstacles() -> Vec<Position> {
        let lines = Game::get_content_file();
        let mut positions: Vec<Position> = Vec::new();
        let mut y: usize = 0;

        for l in lines {
            let mut x: usize = 0;
            for c in l.chars() {
                if c == '.' {
                    positions.push(Position::new(x as f32, y as f32));
                }
                x += 1;
            }
            y += 1;
        }

        return positions;
    }

    fn draw(&self) {
        self.draw_positions();
        self.draw_ball();
    }

    fn draw_positions(&self) {
        for obs in &self.obstacles {
            Game::draw_square(obs.x as f32, obs.y as f32);
        }
    }

    fn draw_square(x: f32, y: f32) {
        draw_rectangle(
            x * SQUARE_SIZE,
            y * SQUARE_SIZE,
            SQUARE_SIZE,
            SQUARE_SIZE,
            WHITE,
        );
    }

    fn draw_ball(&self) {
        Game::draw_square(self.ball.x as f32, self.ball.y as f32);
    }

    fn print(&self) {
        println!("Ball ({}, {})", self.ball.x, self.ball.y);
        println!(
            " Direction ({}, {})",
            self.ball_direction.x, self.ball_direction.y
        );
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut game = Game::new();
    let time = get_time();
    loop {
        clear_background(BLACK);

        game.draw();

        if get_time() - time > DELAY {
            game.print();
            game.mov();
        }

        next_frame().await
    }
}

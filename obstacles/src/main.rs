use std::{fs::{self, File}, io::{BufReader, Lines}};

use macroquad::prelude::*;
const SQUARE_SIZE: f32 = 10.0;
const DELAY: f64 = 0.3;
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
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
            ball: Position::new(5, 5),
            ball_direction: Position::new(1, 1),
        }
    }

    fn mov(&mut self) {
        self.ball.x += self.ball_direction.x;
        self.ball.y += self.ball_direction.y;
    }

    fn get_content_file()-> Vec<String>{
       let file = fs::read_to_string("board.txt")
           .expect("Error reading file");

       let list: Vec<_> = file.lines()
           .map(|line| line.to_string())
           .collect();

        return list;
    }

    fn init_obstacles() -> Vec<Position> {
        let lines = Game::get_content_file();
        let mut positions: Vec<Position> = Vec::new();
        let mut y:usize = 0;

        for l in lines {
            let mut x: usize = 0;
            for c in l.chars() {
                if c == '.' {
                    positions.push(Position::new(x, y));
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
        draw_rectangle(x * 10.0, y * 10.0, 10.0, 10.0, WHITE);
    }

    fn draw_circle(x: f32, y: f32) {
        draw_circle(x * 10.0, y * 10.0, 5.0, WHITE);
    }

    fn draw_ball(&self) {
        Game::draw_circle(self.ball.x as f32, self.ball.y as f32);
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
            game.mov();
        }

        next_frame().await
    }
}

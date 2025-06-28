use std::fs::{self};

use macroquad::prelude::*;
const SQUARE_SIZE: f32 = 10.0;
const DELAY: f64 = 0.3;
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
            ball_direction: Position::new(1.0, -1.0),
        }
    }

    fn mov(&mut self) {
        self.handle_crash();
        self.ball.x += self.ball_direction.x;
        self.ball.y += self.ball_direction.y;
    }

    fn get_next_move(&self) -> Position {
        Position {
            x: self.ball.x + self.ball_direction.x,
            y: self.ball.y + self.ball_direction.y,
        }
    }

    fn get_crash_obstacle(&self, position: &Position) -> Option<Position> {
        for obs in &self.obstacles {
            if obs.x == position.x && position.y == obs.y {
                return Some(Position {
                    x: obs.x,
                    y: obs.y
                })
            }
        }

        None
    }

    fn handle_crash(&mut self) {
        let next = self.get_next_move();
        let opt_obs = self.get_crash_obstacle(&next);

        if opt_obs.is_none(){
            return
        }
        
        let obs = opt_obs.unwrap();
        let cx = self.ball.x;
        let cy = self.ball.y;

        if next.x == obs.x && cx != obs.x {
            self.changex_direction();
            return;
        }

        
        if next.y == obs.y && cy != obs.y {
            self.changey_direction();
            return;
        }
    }

    fn changex_direction(&mut self) {
        self.ball_direction.x = self.ball_direction.x * -1.0;
    }

    fn changey_direction(&mut self) {
        self.ball_direction.y = self.ball_direction.y * -1.0;
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

    fn draw_circle(x: f32, y: f32) {
        draw_circle(x * 10.0, y * 10.0, 5.0, WHITE);
    }

    fn draw_ball(&self) {
        Game::draw_circle(self.ball.x as f32, self.ball.y as f32);
    }

    fn print(&self) {
        print!("Ball ({}, {})", self.ball.x, self.ball.y);
        print!(" Direction ({}, {})", self.ball_direction.x, self.ball_direction.y);
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

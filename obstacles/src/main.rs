use std::fs;

use macroquad::prelude::*;

const SQUARE_SIZE: f32 = 10.0;
const DELAY: f64 = 0.1;

struct Position {
    x: f32,
    y: f32,
}

impl Position {
    fn new(x: f32, y: f32) -> Position {
        Position { x: x, y: y }
    }

    fn is_same(&self, pos: &Position) -> bool{
        return self.x == pos.x && self.y == pos.y;
    }
}

struct Game {
    ball: Position,
    ball_dir: Position,
    obstacles: Vec<Position>,
}

impl Game {
    fn new() -> Game {
        Game {
            ball: Position::new(5.0, 5.0),
            ball_dir: Position::new(0.0, 1.0),
            obstacles: Game::create_obstacles(),
        }
    }

    fn mov(&mut self) {
        self.handle_crash();
        self.ball.x += self.ball_dir.y;
        self.ball.y += self.ball_dir.x;
    }

    fn draw(&self) {
        self.draw_ball();
        self.draw_obstacles();
    }

    fn draw_ball(&self) {
        Game::draw_square(&self.ball);
    }

    fn draw_obstacles(&self) {
        for obs in &self.obstacles {
            Game::draw_square(obs);
        }
    }

    fn draw_square(position: &Position) {
        draw_rectangle(
            position.x * SQUARE_SIZE,
            position.y * SQUARE_SIZE,
            SQUARE_SIZE,
            SQUARE_SIZE,
            MAGENTA,
        );
    }

    fn create_obstacles() -> Vec<Position> {
        let file = fs::read_to_string("./src/obstacles.txt").unwrap_or_else(|err| {
            eprintln!("Failed to read file: {}", err);
            std::process::exit(1);
        });

        let list: Vec<_> = file.lines().map(|line| line.to_string()).collect();
        let mut positions: Vec<Position> = Vec::new();
        let mut y: f32 = 0.0;
        let mut x: f32 = 0.0;

        for line in list {
            x = 0.0;
            for c in line.chars() {
                if c == '.' {
                    positions.push(Position { x: x, y: y });
                }
                x += 1.0;
            }
            y += 1.0;
        }

        return positions;
    }

    fn top_center(position: &Position)-> Position {
        let x = position.x * SQUARE_SIZE + SQUARE_SIZE / 2.0;
        let y = position.y * SQUARE_SIZE;

        return Position {x: x, y: y};
    }

    fn bottom_center(position: &Position)-> Position {
        let x = position.x * SQUARE_SIZE;
        let y = position.y * SQUARE_SIZE+ SQUARE_SIZE / 2.0;

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

    fn is_crash_horizontal(&self) -> bool {
        for obs in &self.obstacles {
            
            let ro = Game::right_center(obs);
            let lo = Game::left_center(obs);

            let rb = Game::right_center(&self.ball);
            let lb = Game::left_center(&self.ball);

            if rb.is_same(&lo) || lb.is_same(&ro) {
                return true;
            }
        }
        return false;
    }


    fn is_crash_vertical(&self) -> bool {
        for obs in &self.obstacles {
            
            let to = Game::top_center(obs);
            let bo = Game::bottom_center(obs);

            let tb = Game::top_center(&self.ball);
            let bb = Game::bottom_center(&self.ball);

            if bo.is_same(&tb) || to.is_same(&bb) {
                return true;
            }
        }
        return false;
    }

    fn handle_crash(&mut self) {

        if self.is_crash_vertical() {
            self.change_vertical_direction();
            return
        }


        if self.is_crash_horizontal() {
            self.change_horizontal_direction();
            return
        }
    }
    
    fn change_vertical_direction(&mut self) {
        self.ball_dir.y = self.ball_dir.y * -1.0;
    }

    fn change_horizontal_direction(&mut self) {
        self.ball_dir.x = self.ball_dir.x * -1.0;
    }
}
#[macroquad::main("MyGame")]
async fn main() {
    let mut game = Game::new();
    let mut time = get_time();

    loop {
        clear_background(DARKBLUE);
        game.draw();

        if get_time() - time > DELAY {
            game.mov();
            time = get_time();
        }
        next_frame().await;
    }
}

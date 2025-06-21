use macroquad::prelude::*;
const SQUARE_SIZE: f32 = 10.0;
const BOARD: &str=r#"
.......................
..                  ...
.             .       .
.             .       .
.             .       .
.             .       .
.             .       .
.             .       .
.             .       .
.......................
"#;

struct Game {
    board: Vec<Vec<char>>
}

impl Game {

    fn new() -> Game {
        let mut v_board: Vec<Vec<char>> = Vec::new();
        for l in BOARD.lines() {
            let mut v: Vec<char> = Vec::new(); 

            for c in l.chars() {
                v.push(c);
            }

            v_board.push(v);
        }

        Game {
            board: v_board
        }
    }

    fn draw(&self) {

    }

    fn draw_board(&self) {
        for x in 0..self.board.len() {
            let v = &self.board[x];
            for y in 0..v.len() {
                let c = v[y];
                if c == '.' {
                    Game::draw_square(x as f32, y as f32);
                }
            } 
        }
    }

    fn draw_square(x :f32, y:f32) {
        draw_rectangle(x * 10.0, y * 10.0, 10.0, 10.0, WHITE);
    }
    
}

#[macroquad::main("MyGame")]
async fn main() {
    let game = Game::new();
    loop {
        clear_background(BLACK);

        game.draw_board();

        next_frame().await
    }
}


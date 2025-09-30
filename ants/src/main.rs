use macroquad::{prelude::*, rand::RandomRange};

const SIZE: f32 = 100.0;
const SQUARE_SIZE: f32 = 5.0;
const ANTS_NUMBER: usize = 50;

struct Position {
    x: f32,
    y: f32,
}

impl Position {
    fn new(x: f32, y: f32) -> Self {
        return Position { x: x, y: y };
    }
}

struct Direction {
    x: f32,
    y: f32,
}

impl Direction {
    fn new(x: f32, y: f32) -> Self {
        return Direction { x: x, y: y };
    }
}

struct Ant {
    pos: Position,
    dir: Direction,
}

impl Ant {
    fn new() -> Self {
        // let pos = Position::new(
        //     1.0, 
        //     1.0
        // );
        //
        // let dir = Direction::new(
        //     1.0, 
        //     0.0
        // );

        let pos = Position::new(
            RandomRange::gen_range(0.0, SIZE-1.0),
            RandomRange::gen_range(0.0, SIZE-1.0),
        );

        let dir = Direction::new(
            Ant::random_binary(), 
            Ant::random_binary()
        );


        return Ant { pos: pos, dir: dir };
    }

    fn random_binary() -> f32 {
        let number = RandomRange::gen_range(0.0, 1.0);
        if number > 0.5 { 1.0 } else { 0.0 }
    }

    fn mov(&mut self) {
        let mut nx = self.pos.x + self.dir.x;
        let mut ny = self.pos.y + self.dir.y;

        nx = nx.max(0.0);
        ny = ny.max(0.0);

        nx = nx.min(SIZE - 1  as f32);
        ny = ny.min(SIZE - 1 as f32);

        self.pos = Position::new(nx, ny)
    }

    fn move_left(&mut self) {
        let left = self.left_direction();
        self.dir = left;
        self.mov();
    }

    fn move_right(&mut self) {
        let right = self.right_direction();
        self.dir = right;
        self.mov();
    }

    fn right_direction(&self) -> Direction {
        if self.dir.y == -1.0 {
            return Direction::new(1.0, 0.0);
        }

        if self.dir.x == 1.0 {
            return Direction::new(0.0, 1.0);
        }

        if self.dir.y == 1.0 {
            return Direction::new(-1.0, 0.0);
        }

        return Direction::new(0.0, -1.0);
    }

    fn left_direction(&self) -> Direction {
        if self.dir.y == -1.0 {
            return Direction::new(-1.0, 0.0);
        }

        if self.dir.x == 1.0 {
            return Direction::new(0.0, -1.0);
        }

        if self.dir.y == 1.0 {
            return Direction::new(1.0, 0.0);
        }

        return Direction::new(0.0, 1.0);
    }

    fn draw(&self) {
        draw_circle(
            (self.pos.x * SQUARE_SIZE) + SQUARE_SIZE / 2.0,
            (self.pos.y * SQUARE_SIZE) + SQUARE_SIZE / 2.0,
            SQUARE_SIZE / 2.0,
            RED,
        )
    }
}

struct Square {
    pos: Position,
    color: i32,
}

impl Square {
    fn new(x: f32, y: f32) -> Self {
        return Square {
            pos: Position::new(x, y),
            color: 0,
        };
    }

    fn draw(&self) {
        let color = if self.color == 1 { WHITE } else { BLACK};

        draw_rectangle(
            self.pos.x * SQUARE_SIZE,
            self.pos.y * SQUARE_SIZE,
            SQUARE_SIZE,
            SQUARE_SIZE,
            color,
        );
    }

    fn change_color(&mut self) {
        let new_color = if self.color == 1 { 0 } else { 1 };

        self.color = new_color;
    }
}

struct Game {
    ants: Vec<Ant>,
    squares: Vec<Square>,
}

impl Game {
    fn new() -> Self {
        return Game {
            ants: Game::random_ants(),
            squares: Game::create_squares(),
        };
    }

    fn random_ants() -> Vec<Ant> {
        let mut ants: Vec<Ant> = Vec::new();

        for i in 0..ANTS_NUMBER {
            ants.push(Game::random_ant());
        }

        return ants;
    }

    fn random_ant() -> Ant {
        return Ant::new();
    }

    fn create_squares() -> Vec<Square> {
        let mut squares: Vec<Square> = Vec::new();
        for j in 0..SIZE as usize {
            for i in 0..SIZE as usize {
                squares.push(Square::new(i as f32, j as f32))
            }
        }
        return squares;
    }

    fn draw(&self) {
        self.draw_squares();
        self.draw_ants();
    }

    fn draw_squares(&self) {
        for square in &self.squares {
            square.draw();
        }
    }

    fn draw_ants(&self) {
        for ant in &self.ants {
            ant.draw();
        }
    }

    fn mov(&mut self) {
        self.mov_ants();
    }

    fn mov_ants(&mut self) {
        for ant in &mut self.ants {
            let i = ant.pos.x as usize;
            let j = ant.pos.y as usize;
            let index = j * SIZE as usize + i;
            let square = &mut self.squares[index];

            if square.color == 1 {
                ant.move_right();
            } else {
                ant.move_left();
            }

            square.change_color();
        }
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut game = Game::new();
    loop {
        clear_background(BLACK);

        game.draw();
        game.mov();
        next_frame().await
    }
}

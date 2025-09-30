use macroquad::{prelude::*, rand::RandomRange};

const SIZE: f32 = 100.0;
const SQUARE_SIZE: f32 = 10.0;
const ANTS_NUMBER: usize = 200;

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
        // let pos = Position::new(1.0, 1.0);
        // let dir = Direction::new(0.0, 1.0);
        
        let pos = Ant::random_position();
        let dir = Ant::random_direction();

        return Ant { pos: pos, dir: dir };
    }

    fn random_position() -> Position {
        let x = RandomRange::gen_range(0.0, SIZE -1.0);
        let y = RandomRange::gen_range(0.0, SIZE -1.0);

        Position {
            x:x,
            y:y
        }
    }

    fn random_direction() -> Direction{
        let x = if RandomRange::gen_range(0.0, 1.0) >= 0.5 {1.0} else {0.0};
        let y = if RandomRange::gen_range(0.0, 1.0) >= 0.5 {1.0} else {0.0};

        Direction {
            x:x,
            y:y
        }
    }

    fn draw(&self) {
        draw_circle(
            self.pos.x * SQUARE_SIZE + SQUARE_SIZE / 2.0,
            self.pos.y * SQUARE_SIZE + SQUARE_SIZE / 2.0,
            SQUARE_SIZE / 2.0,
            RED,
        );
    }

    fn mov(&mut self) {
        let mut nx = self.pos.x + self.dir.x;
        let mut ny = self.pos.y + self.dir.y;

        if nx > SIZE - 1.0 {
            nx = 0.0;
        }

        if nx < 0.0 {
            nx = SIZE - 1.0;
        }

        if ny > SIZE - 1.0 {
            ny = 0.0;
        }

        if ny < 0.0 {
            ny = SIZE - 1.0;
        }

        self.pos = Position::new(nx, ny);
    }

    fn turn_right(&mut self) {
        self.dir = self.right_direction();
        self.mov();
    }

    fn turn_left(&mut self) {
        self.dir = self.left_direction();
        self.mov();
    }

    fn right_direction(&mut self) -> Direction {

        if self.dir.y == -1.0 {
            return Direction::new(1.0, 0.0);
        } 

        if self.dir.x == 1.0 {
            return Direction::new(0.0, 1.0)
        } 

        if self.dir.y == 1.0 {
            return Direction::new(-1.0, 0.0);
        } 

        return Direction::new(0.0, -1.0);
    }


    fn left_direction(&mut self) -> Direction {

        if self.dir.y == -1.0 {
            return Direction::new(-1.0, 0.0);
        } 

        if self.dir.x == -1.0 {
            return Direction::new(0.0, 1.0)
        } 

        if self.dir.y == 1.0 {
            return Direction::new(1.0, 0.0);
        } 

        return Direction::new(0.0, -1.0);
    }
}

struct Square {
    pos: Position,
    color: usize
}

impl Square {
    fn new(x: f32, y: f32) -> Self {
        let pos = Position::new(x, y);
        return Square { pos: pos, color: 1 };
    }

    fn draw(&self) {
        let color = if self.color == 1 {WHITE} else {BLACK};
        draw_rectangle(
            self.pos.x * SQUARE_SIZE,
            self.pos.y * SQUARE_SIZE,
            SQUARE_SIZE,
            SQUARE_SIZE,
            color,
        );
    }

    fn change_color(&mut self) {
        let new_color = if self.color ==1 {0} else {1};
        self.color = new_color;

    }
}

struct Game {
    squares: Vec<Square>,
    ants: Vec<Ant>,
}

impl Game {
    fn new() -> Self {
        Game {
            squares: Game::create_squares(),
            ants: Game::create_ants(),
        }
    }

    fn create_squares() -> Vec<Square> {
        let mut squares: Vec<Square> = Vec::new();
        for j in 0..SIZE as usize {
            for i in 0..SIZE as usize {
                let s = Square::new(i as f32, j as f32);
                squares.push(s);
            }
        }

        return squares;
    }

    fn create_ants() -> Vec<Ant> {
        let mut ants: Vec<Ant> = Vec::new();

        for i in 0..ANTS_NUMBER {
            ants.push(Ant::new());
        }

        return ants;
    }

    fn mov_ants(&mut self) {
        for a in &mut self.ants {

            let i = a.pos.x as usize;
            let j = a.pos.y as usize;
            let size = SIZE as usize;

            let s = &mut self.squares[j*size +i];

            if s.color == 1 {
                a.turn_right();
            } else {
                a.turn_left();
            }

            s.change_color();
        }
    }

    fn mov(&mut self) {
        self.mov_ants();
    }

    fn draw(&self) {
        self.draw_squares();
        self.draw_ants();
    }

    fn draw_squares(&self) {
        for s in &self.squares {
            s.draw();
        }
    }

    fn draw_ants(&self) {
        for a in &self.ants {
            a.draw();
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

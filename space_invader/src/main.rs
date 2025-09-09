use macroquad::{prelude::*, rand::RandomRange};

const SQUARE_SIZE: f32 = 20.0;
const DELAY: f64 = 0.2;
const INVADER_EVERY:u16 = 2;

struct Position {
    x: f32,
    y: f32
}

impl Position {
    pub fn new(x: f32, y:f32) -> Self {
        return Position {
            x: x,
            y: y
        }
    }

    pub fn add(& mut self, x: f32, y:f32) {
        self.x = self.x + x;
        self.y = self.y + y;
    }
}

struct Invader {
    position: Position,
    speed: f32
}

impl  Invader {

    pub fn new() -> Self {
        Invader{
            position: Position::new(
                RandomRange::gen_range(0.0, screen_width()),
                0.0
            ),
            speed: 6.0
        }
    }

    pub fn draw(&self) {
        let x = self.position.x;
        let y = self.position.y;
        draw_rectangle(x, y, SQUARE_SIZE , SQUARE_SIZE, RED);
    }

    pub fn mov(& mut self) {
        self.position.add(0.0, self.speed);
    }
    
}

struct Bullet {
    position: Position,
    speed: f32
}

impl  Bullet {
    
    pub fn new(x: f32, y: f32) -> Self
    {
        let pos = Position::new(x, y);
        Bullet {
            position : pos,
            speed: 6.0
        }
    }

    pub fn mov(& mut self) {
        self.position.add(0.0, -1.0 * self.speed);
    }

    pub fn draw(&self) {
        let x = self.position.x;
        let y = self.position.y;
        draw_rectangle(x, y, SQUARE_SIZE , SQUARE_SIZE, WHITE);
    }

}

struct Ship{
    position: Position
}

impl Ship {
   
    pub fn new() -> Self
    {
        let x = screen_width() / 2.0; 
        let y = screen_height() - 1.0;
        let pos = Position::new(x, y);
        Ship {
            position : pos
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.position.x, 
            self.position.y, 
            SQUARE_SIZE * 3.0, 
            SQUARE_SIZE * 1.0, 
            WHITE);
    }

    pub fn right(& mut self) {
       self.position.add(SQUARE_SIZE, 0.0);
    }

    pub fn left(& mut self) {
       self.position.add(SQUARE_SIZE*-1.0, 0.0);
    }

    pub fn shot(& mut self) -> Bullet {
         return Bullet::new(self.position.x, self.position.y);
    }
}

struct Game {
    ship: Ship,
    bullets: Vec<Bullet>,
    invaders: Vec<Invader>,
    invader_count: u16
}

impl Game {

    pub fn new() -> Self {
        return Game {
            ship: Ship::new(),
            bullets: Vec::new(),
            invaders: Vec::new(),
            invader_count: 0
        }
    }

    pub fn draw(& self) {
        self.ship.draw();
        self.draw_bullets();
        self.draw_invaders();
    }

    pub fn mov(&mut self) {
        self.mov_bullets();
        self.mov_invaders();
        self.create_invader();
    }

    pub fn mov_bullets(& mut self) {
        for bullet in & mut self.bullets {
            bullet.mov();
        }
    }

    pub fn mov_invaders(& mut self) {
        for invader in & mut self.invaders {
            invader.mov();
        }
    }

    pub fn create_invader(& mut self) {
        if self.invader_count >= INVADER_EVERY {
            self.invaders.push(Invader::new());
            self.invader_count = 0;
            return;
        }

        self.invader_count += 1;
    }

    pub fn draw_bullets(& self) {
        for bullet in &self.bullets {
            bullet.draw();
        }
    }

    pub fn draw_invaders(& self) {
        for invader in &self.invaders {
            invader.draw();
        }
    }

    pub fn right(& mut self) {
        self.ship.right();
    }

    pub fn left(& mut self) {
        self.ship.left();
    }
    
    pub fn shot(& mut self) {
        let bullet = self
            .ship
            .shot();

        self.bullets.push(bullet);
    }

    
}

#[macroquad::main("Invaders")] 
async fn main() {

    let mut game = Game::new();
    let mut time = get_time();
    loop {
        clear_background(BLACK);
        game.draw();

        if is_key_down(KeyCode::Left) {
            game.left();
        }

        if is_key_down(KeyCode::Right) {
            game.right();
        }

        if is_key_pressed(KeyCode::Space) {
            game.shot();
        }

        let elapsed_time = get_time() - time;

        if elapsed_time >= DELAY {
            game.mov();
            time = get_time();
        }

        next_frame().await
    }
}


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
        self.handle_crash();
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

    pub fn handle_crash(& mut self) {

        let mut bullets_remove = vec![false; self.bullets.len()];
        let mut invaders_remove = vec![false; self.invaders.len()];

        for (b_index, bullet) in self.bullets.iter().enumerate() {
            for (i_index, invader) in self.invaders.iter().enumerate() {
                
                if Game::is_crash(invader, bullet) {
                    bullets_remove[b_index] = true;
                    invaders_remove[i_index] = true;
                }
            }
        }
       
        let mut i: usize = 0;
        self.bullets.retain(|_| {
            let keep = !bullets_remove[i];
            i += 1;
            keep
        });


        i = 0;
        self.invaders.retain(|_| {
            let keep = !invaders_remove[i];
            i += 1;
            keep
        });

    }

    pub fn is_crash(invader: &Invader, bullet: &Bullet) -> bool {
        let vertical = Game::vertical_crash(invader, bullet); 
        let horizontal = Game::horizontal_crash(invader, bullet);

        return  vertical && horizontal;
    }

    pub fn vertical_crash(invader: &Invader, bullet: &Bullet) -> bool {
        let i_left = invader.position.x;
        let i_right = i_left + SQUARE_SIZE;

        
        let b_left = bullet.position.x;
        let b_right = b_left + SQUARE_SIZE;

        if i_left == b_left {
            return true;
        }
        
        if i_left < b_left {
            return i_right >= b_left;
        }

        if i_left > b_left {
            return b_right >= i_left;
        }
        return false;
    }

    pub fn horizontal_crash(invader: &Invader, bullet: &Bullet) -> bool {
        let invader_bottom = invader.position.y + SQUARE_SIZE;
        let bullet_top = bullet.position.y;

        return invader_bottom > bullet_top;
    }
    
}

#[macroquad::main("Invaders")] 
async fn main() {

    let mut game = Game::new();
    let mut time = get_time();
    loop {
        clear_background(BLACK);
        game.draw();

        if is_key_pressed(KeyCode::Left) {
            game.left();
        }

        if is_key_pressed(KeyCode::Right) {
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


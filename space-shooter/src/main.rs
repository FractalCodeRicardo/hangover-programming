mod consts;
mod assets;
mod components;

use macroquad::{prelude::*, rand::RandomRange};

use crate::{assets::Assets, components::{Background, Bullet, Enemy, Spaceship}, consts::{ENEMY_HEIGHT, SHIP_HEIGHT, SHIP_WIDTH, SHOT_EVERY}};

struct Game {
    assets: Assets,
    background: Background,
    spaceship: Spaceship,
    enemy: Enemy,
    bullets: Vec<Bullet>,
    last_shoot: f32,
    game_over: bool,
    you_win: bool

}

impl Game {

    async fn new() -> Self {
        let assets = Assets::new().await;

        let background = Background::new(assets.image("background.png"));
        let spaceship = Spaceship::new(assets.image("spaceship.png"));

        let enemy_img = assets.image("enemy.png");
        let enemy_hurt_img = assets.image("enemy-shoot.png");
        let enemy = Enemy::new(enemy_img, enemy_hurt_img);

        
        Game {
            assets: assets,
            background: background,
            spaceship: spaceship,
            enemy: enemy,
            bullets: Vec::new(),
            last_shoot: 0.,
            game_over: false,
            you_win: false
        }
    }

    fn draw(&mut self) {
        if self.game_over || self.you_win {
            self.background.draw();
            self.handle_end_game();
            return;
        }

        self.background.draw();
        self.spaceship.draw();
        self.enemy.draw();
        self.draw_bullets();
        self.boss_shoot();
        self.handle_enemy_hit();
        self.handle_spaceship_hit();
    }

    fn draw_bullets(&mut self) {
        for b in &mut self.bullets {
            b.draw();
        }
    }

    fn handle_end_game(&self) {
        let mut text = "";

        if self.you_win {
            text = "YOU WIN"
        } 

        if self.game_over {
            text = "GAME OVER"
        } 

        draw_text(&text, screen_width() / 2. -100. , 300., 40., YELLOW);
    }

    fn left(&mut self) {
        self.spaceship.left();
    }

    fn right(&mut self) {
        self.spaceship.right();
    }

    fn up(&mut self) {
        self.spaceship.up();
    }

    fn down(&mut self) {
        self.spaceship.down();
    }

    fn spaceship_shoot(&mut self) {
        let image = self.assets.image("shoot.png");
        let pos = Vec2 {
            x: self.spaceship.pos.x + SHIP_WIDTH + 5.,
            y: self.spaceship.pos.y + SHIP_HEIGHT / 2. 
        };
        let dir= vec2(3.,0.);
        let bullet = Bullet::new(image, pos, dir);
        self.bullets.push(bullet);
        self.play_shoot();
    }

    fn boss_shoot(&mut self) {
        if self.last_shoot <= SHOT_EVERY {
            self.last_shoot += 1.;
            return;
        }

        let multishot = RandomRange::gen_range(0, 2);

        if multishot == 1 {
            self.boss_multi_shoot();
        }

        self.boss_single_shot();
    }

    fn boss_single_shot(&mut self) {

        let image = self.assets.image("shoot.png");
        let pos = Vec2 {
            x: self.enemy.pos.x - 5.,
            y: self.enemy.pos.y + ENEMY_HEIGHT / 2. 
        };

        let mut to = self.spaceship.pos - self.enemy.pos;
        to = to.normalize() * vec2(3.,1.);

        let bullet = Bullet::new(image, pos, to);
        self.bullets.push(bullet);

        self.last_shoot = 0.;

    }

    fn boss_multi_shoot(&mut self) {

        let image = self.assets.image("shoot.png");
        let pos = Vec2 {
            x: self.enemy.pos.x - 5.,
            y: self.enemy.pos.y + ENEMY_HEIGHT / 2. 
        };

        let bullet1 = Bullet::new(image.clone(), pos, vec2(-2., 0.));
        let bullet2 = Bullet::new(image.clone(), pos, vec2(-2., 1.));
        let bullet3 = Bullet::new(image.clone(), pos, vec2(-2., 2.));
        let bullet4 = Bullet::new(image.clone() , pos, vec2(-2., -1.));
        let bullet5 = Bullet::new(image.clone(), pos, vec2(-2., -2.));

        self.bullets.push(bullet1);
        self.bullets.push(bullet2);
        self.bullets.push(bullet3);
        self.bullets.push(bullet4);
        self.bullets.push(bullet5);

        self.play_shoot();
        self.play_shoot();
        self.play_shoot();

    }

    fn handle_enemy_hit(&mut self) {
        let bullets_hit = self.get_bullets_hit_enemy();
        self.remove_bullets(&bullets_hit);
        self.enemy.hit(bullets_hit.len());

        if bullets_hit.len() > 0{
            self.play_explosion();
        }

        if self.enemy.life <= 0. {
            self.you_win = true;
        }
    }


    fn handle_spaceship_hit(&mut self) {
        let bullets_hit = self.get_bullets_hit_spaceship();
        self.remove_bullets(&bullets_hit);

        if bullets_hit.len() > 0 {
            self.play_explosion();
            self.game_over = true;
        }
    }

    fn get_bullets_hit_enemy(&self) -> Vec<usize> {
        let mut indexes = Vec::new();

        for i in 0..self.bullets.len() {
            let b = &self.bullets[i];
            let p = b.get_center();

            if self.enemy.overlaps(&p) {
                indexes.push(i);
            }
        }

        return indexes;
    }


    fn get_bullets_hit_spaceship(&self) -> Vec<usize> {
        let mut indexes = Vec::new();

        for i in 0..self.bullets.len() {
            let b = &self.bullets[i];
            let p = b.get_center();

            if self.spaceship.overlaps(&p) {
                indexes.push(i);
            }
        }

        return indexes;
    }

    fn remove_bullets(&mut self, indexes: &Vec<usize>) {
        let mut i = 0;
        self.bullets.retain(|_| {
            let keep = !indexes.contains(&i);
            i+= 1;
            keep
        });
    }

    fn play_explosion(&self) {
        self.assets.play_sound("explosion.wav");
    }

    fn play_shoot(&self) {
        self.assets.play_sound("shoot.wav");
    }
}
#[macroquad::main("MyGame")]
async fn main() {
    let mut game = Game::new().await;

    loop {
        clear_background(RED);

        events(&mut game);
        game.draw();

        next_frame().await
    }
}
    
fn events(game: &mut Game) {

    if is_key_down(KeyCode::H) {
        game.left();
    }

    if is_key_down(KeyCode::L) {
        game.right();
    }

    if is_key_down(KeyCode::J) {
        game.down();
    }

    if is_key_down(KeyCode::K) {
        game.up();
    }

    if is_key_pressed(KeyCode::Space) {
        game.spaceship_shoot();
    }

}

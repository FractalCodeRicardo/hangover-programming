mod consts;
mod models;

use macroquad::color::BLACK;
use macroquad::input::is_key_down;
use macroquad::input::is_key_pressed;
use macroquad::input::KeyCode;
use macroquad::window::clear_background;
use macroquad::window::next_frame;
use models::Position;
use models::Road;
use models::Player;

use crate::consts::ENEMY_EVERY;
use crate::models::Enemy;

struct Game {
    road: Road,
    player: Player,
    enemies: Vec<Enemy>,
    last_enemy: usize
}

impl Game {

    async fn new() -> Self {
        let mut game = Game {
            road: Road::new().await,
            player: Player::new().await,
            enemies: Vec::new(),
            last_enemy: 0
        };

        game.add_enemy().await;

        return game;
    }

    async fn add_enemy(&mut self)  {
        let enemy = Enemy::new().await;
        self.enemies.push(enemy);
    }

    async fn add_enemy_condition(&mut self) {
        if self.last_enemy > ENEMY_EVERY {
            self.add_enemy().await;
            self.last_enemy = 0;
        }

        self.last_enemy += 1;
    }

    async fn draw(&mut self) {
        self.road.draw();
        self.player.draw();
        self.add_enemy_condition().await;
        self.draw_enemies();
    }

    fn draw_enemies(&mut self) {
        for e in &mut self.enemies {
            e.draw();
        }
    }

    fn left(&mut self) {
        self.player.left();
    }

    fn right(&mut self) {
        self.player.right();
    }
    
}

#[macroquad::main("RoadFighter")]
async fn main() {
    let mut game = Game::new().await;

    loop {
        clear_background(BLACK);

        events(&mut game);
        game.draw().await;

        next_frame().await;
    }
}

fn events(game: &mut Game) {

    if is_key_down(KeyCode::L) {
        game.right();
    }


    if is_key_down(KeyCode::H) {
        game.left();
    }
}


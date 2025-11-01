mod consts;
mod images;
mod models;
mod audio;

use std::rc;
use std::rc::Rc;

use images::Images;
use audio::Audio;
use macroquad::color::BLACK;
use macroquad::color::RED;
use macroquad::input::KeyCode;
use macroquad::input::is_key_down;
use macroquad::input::is_key_pressed;
use macroquad::text::draw_text;
use macroquad::texture::Image;
use macroquad::window::clear_background;
use macroquad::window::next_frame;
use macroquad::window::screen_height;
use macroquad::window::screen_width;
use models::Player;
use models::Position;
use models::Road;

use crate::consts::ENEMY_EVERY;
use crate::consts::PLAYER_SIZE;
use crate::consts::ROAD_BORDER;
use crate::models::Enemy;

struct Game {
    road: Road,
    player: Player,
    enemies: Vec<Enemy>,
    last_enemy: usize,
    images: Images,
    audio: Audio,
    game_over: bool
}

impl Game {
    async fn new() -> Self {
        let images = Images::new().await;
        let road_image = images.get_image_road();
        let player_image = images.get_image_player();

        let audio = Audio::new().await;
        audio.play_engine();

        let mut game = Game {
            road: Road::new(road_image),
            player: Player::new(player_image),
            enemies: Vec::new(),
            last_enemy: 0,
            images: images,
            audio: audio,
            game_over: false
        };

        game.add_enemy().await;
        return game;
    }

    async fn add_enemy(&mut self) {
        let image = self.images.get_enemy_image();
        let enemy = Enemy::new(image);
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
        if self.game_over {
            Game::show_game_over();
            return;
        }

        self.handle_road_crash();
        self.handle_enemy_crash();

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

    fn is_road_crash(&self) -> bool {
        println!("{} {} {}", self.player.pos.x, ROAD_BORDER, screen_width());
        if self.player.pos.x <= ROAD_BORDER {
            return true;
        }

        if self.player.pos.x + PLAYER_SIZE >= screen_width() - ROAD_BORDER {
            return true;
        }

        return false;
    }

    fn is_enemy_crash(&self) -> bool {
        let p1 = self.player.top_left();
        let p2 = self.player.top_right();

        for e in &self.enemies {

            if e.overlaps(&p1) {
                return true
            }
            
            if e.overlaps(&p2) {
                return true
            }

        }

        return false;
    }

    fn handle_enemy_crash(&mut self)  {
        if self.is_enemy_crash() {
            self.audio.play_explosion();
           self.game_over = true;
        }
    }

    fn handle_road_crash(&mut self) {
        if self.is_road_crash() {
            self.audio.play_explosion();
            self.game_over = true;
        }
    }

    fn show_game_over() {
        draw_text(
            "GAME OVER",
            screen_width() / 2. - 50.,
            screen_height() / 2.,
            40.,
            RED,
        );
    }
}

#[macroquad::main("RoadFighter")]
async fn main() {
    let mut game = init_game().await;

    loop {
        clear_background(BLACK);

        events(&mut game);
        game.draw().await;

        next_frame().await;
    }
}

async fn init_game() -> Game {
    let game = Game::new().await;
    return game;
}

fn events(game: &mut Game) {
    if is_key_down(KeyCode::L) {
        game.right();
    }

    if is_key_down(KeyCode::H) {
        game.left();
    }
}

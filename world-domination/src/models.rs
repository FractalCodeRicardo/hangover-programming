const PLAYER_IMAGE: &str = "player.png";
const BULLET_IMAGE: &str = "bullet.png";
const BG_IMAGE: &str = "background.png";
const ENEMIES_IMAGES: &str = "enemy1.png,enemy2.png";
const PLAYER_SIZE: f32 = 50.;

use std::{collections::HashMap, rc::Rc};

use macroquad::{color::WHITE, math::Vec2, prelude::coroutines::tweens::linear, rand::RandomRange, texture::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D}, window::{screen_height, screen_width}};


pub struct ImageHandler {
    images: HashMap<String, Rc<Texture2D>>,
}

impl ImageHandler {
    pub fn new() -> Self {
        ImageHandler {
            images: HashMap::new(),
        }
    }

    fn get_enemies_names() -> Vec<String> {
        let enemies = ENEMIES_IMAGES
            .split(",")
            .map(|i| i.to_string())
            .collect();

        return enemies;
    }

    fn get_all_names() -> Vec<String> {
        let mut names = ImageHandler::get_enemies_names();
        names.push(PLAYER_IMAGE.to_string());
        names.push(BULLET_IMAGE.to_string());
        names.push(BG_IMAGE.to_string());
        return names;
    }

    pub async fn load_images(&mut self) {
        let names = ImageHandler::get_all_names();
        for img in &names {
            let path = "./assets/".to_string() + img;
            let picture = load_texture(&path).await.unwrap();
            let rc_picture = Rc::new(picture);

            self.images.insert(img.to_string(), rc_picture);
        }
    }

    pub fn get_image_player(&self) -> Rc<Texture2D> {
        return self.images[PLAYER_IMAGE].clone();
    }

    pub fn get_image_bullet(&self) -> Rc<Texture2D> {
        return self.images[BULLET_IMAGE].clone();
    }

    pub fn get_image_enemy(&self) -> Rc<Texture2D> {
        let enemy_names = ImageHandler::get_enemies_names();
        let image_index = RandomRange::gen_range(0, enemy_names.len());
        let image_name = enemy_names[image_index].to_string();
        return self.images[&image_name].clone();
    }

    pub fn set_background(&self) {
        let picture= &self.images[BG_IMAGE];
        draw_texture_ex(
            picture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );
    }
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct GameObject {
    pub id: String,
    pub picture: Rc<Texture2D>,
    pub pos: Position,
}

impl GameObject {
    pub fn draw(&self) {
        draw_texture_ex(
            &self.picture,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..Default::default()
            },
        );
    }

    pub fn mov_down(&mut self, step: f32) {
        self.pos = Position {
            x: self.pos.x,
            y: self.pos.y + step,
        }
    }

    pub fn mov_up(&mut self, step: f32) {
        self.pos = Position {
            x: self.pos.x,
            y: self.pos.y - step,
        }
    }

    pub fn mov_left(&mut self, step: f32) {
        self.pos = Position {
            x: self.pos.x - step ,
            y: self.pos.y,
        }
    }

    pub fn mov_right(&mut self, step: f32) {
        self.pos = Position {
            x: self.pos.x + step ,
            y: self.pos.y,
        }
    }

    pub fn print(&self, label: &str) {
        println!("{} {} {}", label, self.pos.x, self.pos.y);
    }
}

use crate::consts::*;

use macroquad::{
    color::WHITE, math::Vec2, rand::RandomRange, texture::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D}, window::{screen_height, screen_width}
};

pub struct Position {
    x: f32,
    y: f32,
}

pub struct Road {
    image: Texture2D,
    y1: f32,
    y2: f32,
}

impl Road {
    pub async fn new() -> Self {
        let road = load_texture("./assets/road.png").await.unwrap();

        Road {
            image: road,
            y2: 0.,
            y1: screen_height() * -1.,
        }
    }

    pub fn draw(&mut self) {
        let size = Vec2::new(ROAD_WITHD, screen_height());
        let center_x = screen_width() / 2.;
        draw_texture_ex(
            &self.image,
            center_x - ROAD_WITHD / 2.,
            self.y1,
            WHITE,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );

        draw_texture_ex(
            &self.image,
            center_x - ROAD_WITHD / 2.,
            self.y2,
            WHITE,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );

        self.y2 += ROAD_SPEED;
        self.y1 = (screen_height() - self.y2) * -1.;
    }
}

pub struct Player {
    image: Texture2D,
    pos: Position,
}

impl Player {
    pub async fn new() -> Player {
        let image = load_texture("./assets/red-car.png").await.unwrap();

        let pos = Position {
            x: screen_width() / 2.,
            y: screen_height() - 100.,
        };

        Player {
            pos: pos,
            image: image,
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.image,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..Default::default()
            },
        );
    }

    pub fn left(&mut self) {
       self.pos = Position {
           x: self.pos.x - PLAYER_SIDE_SPEED,
           y: self.pos.y
       } 
    }

    pub fn right(&mut self) {
       self.pos = Position {
           x: self.pos.x + PLAYER_SIDE_SPEED,
           y: self.pos.y
       } 
    }
}

pub struct Enemy {
    image: Texture2D,
    pos: Position
}

impl Enemy {

    pub async fn new() -> Self {
        let centerx = screen_width() / 2.;
        let image = Enemy::get_random_image().await;
        let limit_r = centerx + ROAD_WITHD / 2.;
        let limit_l = centerx - ROAD_WITHD / 2.;
        let pos = Position {
            x: RandomRange::gen_range(limit_l, limit_r),
            y: 100. 
        };

         Enemy {
            image: image,
            pos: pos
        }
    }

    pub async fn get_random_image() -> Texture2D {
        let images = vec!["enemy1.png", "enemy2.png"];
        let index = RandomRange::gen_range(0, images.len());
        let image_name = images[index];
        let path = "./assets/".to_string() + &image_name.to_string();

        let texture = load_texture(&path)
            .await
            .unwrap();

        return texture;
    }

    pub fn draw(&mut self) {
        draw_texture_ex(
            &self.image,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                ..Default::default()
            },
        );

        self.pos = Position {
            x: self.pos.x,
            y: self.pos.y + ENEMY_SPEED
        }
    }


}


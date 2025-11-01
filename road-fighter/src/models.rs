use std::rc::Rc;

use crate::{consts::*, images::Images};

use macroquad::{
    color::WHITE, math::Vec2, rand::RandomRange, texture::{draw_texture_ex, load_texture, DrawTextureParams, Texture2D}, window::{screen_height, screen_width}
};

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Road {
    image: Rc<Texture2D>,
    y1: f32,
    y2: f32,
}

impl Road {
    pub fn new(image: Rc<Texture2D>) -> Self {
        Road {
            image: image,
            y2: 0.,
            y1: screen_height() * -1.,
        }
    }

    pub fn draw(&mut self) {
        let size = Vec2::new(ROAD_WITDH, screen_height());
        let center_x = screen_width() / 2.;
        draw_texture_ex(
            &self.image,
            center_x - ROAD_WITDH / 2.,
            self.y1,
            WHITE,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );

        draw_texture_ex(
            &self.image,
            center_x - ROAD_WITDH / 2.,
            self.y2,
            WHITE,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );

        if self.y2 >= screen_height() {
            self.y2 = 0.;
        } else {
            self.y2 += ROAD_SPEED;
        }

        self.y1 = (screen_height() - self.y2) * -1.;
    }
}

pub struct Player {
    image: Rc<Texture2D>,
    pub pos: Position,
}

impl Player {
    pub fn new(image: Rc<Texture2D>) -> Player {

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

    pub fn top_left(&self) -> Position {
        Position {
            x: self.pos.x,
            y: self.pos.y
        }
    }

    pub fn top_right(&self) -> Position {
        Position {
            x: self.pos.x + PLAYER_SIZE,
            y: self.pos.y
        }
    }
}

pub struct Enemy {
    image: Rc<Texture2D>,
    pos: Position
}

impl Enemy {

    pub fn new(image: Rc<Texture2D>) -> Self {
        let limit_r = screen_width() - ROAD_BORDER - PLAYER_SIZE - 100.;
        let limit_l = ROAD_BORDER + 100.;
        let pos = Position {
            x: RandomRange::gen_range(limit_l, limit_r),
            y: 0. 
        };

         Enemy {
            image: image,
            pos: pos
        }
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

    pub fn overlaps(&self, pos: &Position) -> bool {
        let horizontal_overlap = self.pos.x <= pos.x && pos.x <= self.pos.x + PLAYER_SIZE;
        let vertical_overlap = self.pos.y <= pos.y && pos.y <= self.pos.y + PLAYER_SIZE;

        return horizontal_overlap && vertical_overlap;
    }
}


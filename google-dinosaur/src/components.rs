use std::rc::Rc;

use macroquad::{
    color::WHITE,
    math::Vec2,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex},
    window::{screen_height, screen_width},
};

use crate::consts::{
    CACTUS_HEIGHT, CACTUS_WIDTH, DESERT_SPEED, DINOSAUR_HEIGHT, DINOSAUR_WIDTH, GRAVITY,
    INIT_JUMP_FORCE,
};

pub struct Position {
    x: f32,
    y: f32,
}

pub struct Desert {
    image: Rc<Texture2D>,
    x1: f32,
    x2: f32,
}

impl Desert {
    pub fn new(image: Rc<Texture2D>) -> Self {
        Desert {
            image: image,
            x1: -screen_width(),
            x2: 0.,
        }
    }

    pub fn draw(&mut self) {
        draw_texture_ex(
            &self.image,
            self.x1,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        draw_texture_ex(
            &self.image,
            self.x2,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        if self.x2 >= screen_width() {
            self.x2 = 0.;
        } else {
            self.x2 += DESERT_SPEED;
        }

        self.x1 = (screen_width() - self.x2) * -1.;
    }
}

pub struct Dinosaur {
    images: Vec<Rc<Texture2D>>,
    pos: Position,
    index_image: usize,
    image_count: usize,
    jump_force: f32,
}

impl Dinosaur {
    pub fn new(images: Vec<Rc<Texture2D>>) -> Self {
        Dinosaur {
            images: images,
            pos: Position {
                x: 100.,
                y: screen_height() / 2. - DINOSAUR_HEIGHT,
            },
            jump_force: 0.,
            index_image: 0,
            image_count: 0,
        }
    }

    pub fn draw(&mut self) {
        let image = &self.images[self.index_image];

        self.image_count += 1;

        if self.image_count > 10 {
            self.index_image = if self.index_image == 1 { 0 } else { 1 };
            self.image_count = 0;
        }

        draw_texture_ex(
            image,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(DINOSAUR_WIDTH, DINOSAUR_HEIGHT)),
                ..Default::default()
            },
        );

        self.mov_jump();
    }

    fn mov_jump(&mut self) {
        let y_ground = screen_height() / 2. - DINOSAUR_HEIGHT;

        //is in ground
        if self.jump_force < 0. && self.pos.y >= y_ground {
            self.pos.y = y_ground;
            self.jump_force = 0.;
            return;
        }

        self.pos.y -= self.jump_force;
        self.jump_force -= GRAVITY;
    }

    pub fn jump(&mut self) {
        self.jump_force = INIT_JUMP_FORCE;
    }

    pub fn get_vertices(&self) -> Vec<Position> {
        let mut vertices: Vec<Position> = Vec::new();
        let top_left = Position {
            x: self.pos.x - DINOSAUR_WIDTH,
            y: self.pos.y,
        };
        let top_right = Position {
            x: self.pos.x + DINOSAUR_WIDTH,
            y: self.pos.y,
        };
        let botom_left = Position {
            x: self.pos.x,
            y: self.pos.y + DINOSAUR_HEIGHT,
        };
        let botom_right = Position {
            x: self.pos.x + DINOSAUR_WIDTH,
            y: self.pos.y + DINOSAUR_HEIGHT,
        };

        vertices.push(top_right);
        vertices.push(top_left);
        vertices.push(botom_left);
        vertices.push(botom_right);

        return vertices;
    }
}

pub struct Cactus {
    image: Rc<Texture2D>,
    pos: Position,
}

impl Cactus {
    pub fn new(image: Rc<Texture2D>) -> Self {
        let pos = Position {
            x: screen_width() - CACTUS_WIDTH,
            y: screen_height() / 2. - CACTUS_HEIGHT,
        };
        Cactus {
            image: image,
            pos: pos,
        }
    }

    pub fn draw(&mut self) {
        draw_texture_ex(
            &self.image,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(CACTUS_WIDTH, CACTUS_HEIGHT)),
                ..Default::default()
            },
        );

        self.pos.x -= DESERT_SPEED;
    }

    pub fn overlaps(&self, pos: &Position) -> bool {
        let v_overlap = self.pos.x <= pos.x && pos.x <= self.pos.x + CACTUS_WIDTH;
        let h_overlap = self.pos.y <= pos.y && pos.y <= self.pos.y + CACTUS_HEIGHT;

        return v_overlap && h_overlap;
    }
}

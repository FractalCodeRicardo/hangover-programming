use std::rc::Rc;

use macroquad::{color::{RED, WHITE, YELLOW}, math::Vec2, shapes::draw_rectangle, text::draw_text, texture::{draw_texture, draw_texture_ex, DrawTextureParams, Texture2D}, window::{screen_height, screen_width}};

use crate::{assets::Assets, consts::{BG_SPEED, BULLET_SIZE, ENEMY_HEIGHT, ENEMY_SPEED, ENEMY_WIDTH, HURT_DURATION, SHIP_HEIGHT, SHIP_SPEED, SHIP_WIDTH}};

pub struct Background { 
    pub image: Rc<Texture2D>,
    x1: f32,
    x2: f32
}

impl Background  {

    pub fn new(image: Rc<Texture2D>) -> Self {
        Background {
            image: image,
            x1: 0.,
            x2: screen_width()
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
            }
        );

        draw_texture_ex(
            &self.image,
            self.x2,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(screen_width(), screen_height())),
                ..Default::default()
            }
        );

        self.mov();
    }

    fn mov(&mut self) {
        if self.x1 <= screen_width()*-1. {
            self.x1 = 0.;
        } else {
            self.x1 -= BG_SPEED;
        }

        self.x2 = screen_width() + self.x1;
    }

}


pub struct Spaceship {
    image: Rc<Texture2D>,
    pub pos: Vec2
}

impl Spaceship {
    
    pub fn new(image: Rc<Texture2D>) -> Self {
        Spaceship {
            image: image,
            pos: Vec2 {
                x: 50.,
                y: screen_height() / 2.
            }
        }
    }

    pub fn draw(&mut self) {
        draw_texture_ex(
            &self.image,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(SHIP_WIDTH, SHIP_HEIGHT)),
                ..Default::default()
            }
        );
    }

    pub fn left(&mut self) {
        self.pos = Vec2{
            x: self.pos.x - SHIP_SPEED,
            y: self.pos.y
        }

    }


    pub fn right(&mut self) {
        self.pos = Vec2{
            x: self.pos.x + SHIP_SPEED,
            y: self.pos.y
        }
    }


    pub fn up(&mut self) {
        self.pos = Vec2{
            x: self.pos.x ,
            y: self.pos.y - SHIP_SPEED,
        }
    }

    pub fn down(&mut self) {
        self.pos = Vec2{
            x: self.pos.x ,
            y: self.pos.y + SHIP_SPEED,
        }
    }

    pub fn overlaps(&self, pos: &Vec2) -> bool {
        let v_overlap = self.pos.x <= pos.x && pos.x <= self.pos.x + SHIP_WIDTH;
        let h_overlap = self.pos.y <= pos.y && pos.y <= self.pos.y + SHIP_HEIGHT;

        return v_overlap && h_overlap;
    }
}


pub struct Enemy {
    image: Rc<Texture2D>,
    image_hurt: Rc<Texture2D>,
    pub pos: Vec2,
    dir_y: f32,
    pub life: f32,
    is_hurt: bool,
    last_hurst: usize
}

impl Enemy {

    pub fn new(image: Rc<Texture2D>, image_hurt: Rc<Texture2D>) -> Self {
        Enemy {
            image: image,
            image_hurt: image_hurt,
            pos: Vec2 {
                x: screen_width() - ENEMY_WIDTH - 50., 
                y:screen_height() / 2.
            },
            dir_y: 1.,
            life: 100.,
            is_hurt: false,
            last_hurst: 0

        }
    }

    pub fn draw(&mut self) {
        let image: Rc<Texture2D> = if self.is_hurt {self.image_hurt.clone()} else {self.image.clone()};
        draw_texture_ex(
            &image,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT)),
                ..Default::default()
            }
        );

        self.draw_life();
        self.mov();
        self.change_hurt_state();
    }

    fn change_hurt_state(&mut self) {
        if self.last_hurst > HURT_DURATION {
            self.is_hurt = false;
            self.last_hurst = 0;
            return;
        }

        self.last_hurst += 1;

    }

    fn draw_life(&self) {
        let size = (screen_width() - 20.) / 100.;
        let width_life = size * self.life;
        let width_death = screen_width() - 20. - width_life;

        draw_rectangle(10., 10., width_life, 15., YELLOW);
        draw_rectangle(width_life + 1., 10., width_death, 15., RED);
        let text = format!("{} / {}", self.life, 100.);
        draw_text(&text, 10., 45., 30., YELLOW);

    }

    pub fn mov(&mut self) {
        let nexty = self.pos.y;

        if nexty<= 10. {
            self.dir_y = 1.;
        }

        if nexty >= screen_height() - ENEMY_HEIGHT -10. {
            self.dir_y = -1.;
        }

        self.pos = Vec2::new(self.pos.x, self.pos.y + (ENEMY_SPEED * self.dir_y));
    }

    pub fn overlaps(&self, pos: &Vec2) -> bool {
        let v_overlap = self.pos.x <= pos.x && pos.x <= self.pos.x + ENEMY_WIDTH;
        let h_overlap = self.pos.y <= pos.y && pos.y <= self.pos.y + ENEMY_HEIGHT;

        return v_overlap && h_overlap;
    }

    pub fn hit(&mut self, times: usize) {
        self.life -= times as f32;

        if times > 0 {
            self.is_hurt = true;
        }
    }
}

pub struct Bullet {
    image: Rc<Texture2D>,
    pos: Vec2,
    dir: Vec2
}

impl Bullet {

    pub fn new(image: Rc<Texture2D>, pos: Vec2, dir: Vec2) -> Self {
        Bullet {
            image,
            pos,
            dir
        }
    }


    pub fn draw(&mut self) {
        draw_texture_ex(
            &self.image,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(BULLET_SIZE, BULLET_SIZE)),
                ..Default::default()
            }
        );

        self.mov();
    }

    pub fn mov(&mut self) {
        self.pos = self.pos + self.dir;
    }

    pub fn get_center(&self) -> Vec2 {
        Vec2 {
            x: self.pos.x + BULLET_SIZE /2.,
            y: self.pos.y + BULLET_SIZE /2.,
        }
    }
}

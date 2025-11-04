use std::rc::Rc;

use macroquad::{
    audio::load_sound,
    color::{GREEN, WHITE},
    math::{Vec2, vec2},
    rand::RandomRange,
    shapes::{draw_circle_lines, draw_poly},
    text::draw_text_ex,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture},
    window::{screen_height, screen_width},
};

use crate::consts::{CANION_HEIGHT, CANION_WIDTH, GRAVITY, HEAD_SIZE, RESISTENCE, TRASH_HEIGHT, TRASH_SPEED, TRASH_WIDTH};

pub struct Landfill {
    image: Texture2D,
}

impl Landfill {
    pub async fn new() -> Self {
        let image = load_texture("./assets/background.png").await.unwrap();

        Landfill { image: image }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.image,
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

pub struct HeadImages {
    images: Vec<Rc<Texture2D>>,
}

impl HeadImages {
    pub async fn new() -> Self {
        let mut images: Vec<Rc<Texture2D>> = Vec::new();

        for i in 1..5 {
            let path = "./assets/head".to_string() + &i.to_string() + ".png";
            let texture = load_texture(&path).await.unwrap();

            images.push(Rc::new(texture));
        }

        HeadImages { images }
    }

    pub fn get(&self) -> Rc<Texture2D> {
        let index = RandomRange::gen_range(0, self.images.len());
        let texture = &self.images[index];
        return texture.clone();
    }
}

pub struct Head {
    image: Rc<Texture2D>,
    force: Vec2,
    pos: Vec2,
}

impl Head {
    pub fn new(image: Rc<Texture2D>) -> Self {
        let force = Vec2::new(10., 30.);
        let pos = Vec2::new(0., screen_height());

        Head {
            image: image,
            force: force,
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
                dest_size: Some(Vec2::new(HEAD_SIZE, HEAD_SIZE)),
                ..Default::default()
            },
        );

        self.mov();
    }

    fn mov(&mut self) {
        self.pos = Vec2 {
            x: self.pos.x + self.force.x,
            y: self.pos.y - self.force.y,
        };

        self.force = Vec2 {
            x: self.force.x * RESISTENCE,
            y: (self.force.y - GRAVITY) * RESISTENCE,
        };
    }

    pub fn shot(&mut self, pos: &Vec2, force: &Vec2) {
        self.pos = Vec2::new(pos.x - HEAD_SIZE / 2., pos.y - HEAD_SIZE / 2.);
        self.force = Vec2::new(force.x, force.y);
    }

    pub fn bottom_center(&self)-> Vec2 {
        Vec2 {
            x: self.pos.x + HEAD_SIZE / 2.,
            y: self.pos.y + HEAD_SIZE
        }
    }
}

pub struct Canion {
    image: Texture2D,
    angle: f32,
    power: f32,
}

impl Canion {
    pub async fn new() -> Self {
        let image = load_texture("./assets/canion.png").await.unwrap();

        Canion {
            image: image,
            angle: 45.,
            power: 30.,
        }
    }

    pub fn draw(&self) {
        let base_radians = (90. as f32).to_radians();
        draw_texture_ex(
            &self.image,
            -CANION_WIDTH / 2.,
            screen_height() - CANION_HEIGHT / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(CANION_WIDTH, CANION_HEIGHT)),
                rotation: base_radians - self.angle.to_radians(),
                ..Default::default()
            },
        );
    }

    pub fn left(&mut self) {
        self.angle += 1.;
    }

    pub fn right(&mut self) {
        self.angle -= 1.;
    }

    pub fn shot_position(&self) -> Vec2 {
        let original_x = CANION_HEIGHT / 2.;
        let original_y = 0.;
        let angle = self.angle;

        let cos = angle.to_radians().cos();

        let sin = angle.to_radians().sin();

        let new_x = original_x * cos - original_y * sin;

        let new_y = original_x * sin + original_y * cos;

        println!("POS {} {}", new_x, new_y);

        return Vec2::new(new_x, screen_height() - new_y);
    }

    pub fn force(&self) -> Vec2 {
        let magnitude = 20.;
        let angle = self.angle;
        let radians = angle.to_radians();
        let x = magnitude * radians.cos();
        let y = magnitude * radians.sin();

        println!("Force {} {}", x, y);

        return Vec2::new(x, y);
    }
}


pub struct TrashCan {
    image: Texture2D,
    direction_x: f32,
    pos: Vec2
}

impl TrashCan {

    pub async fn new() -> Self {
        let image = load_texture("./assets/trash.png").await.unwrap();

        TrashCan {
            image: image,
            direction_x: 1.,
            pos: Vec2::new(200., screen_height() - TRASH_HEIGHT - 10.)
        }
    }


    pub fn draw(&mut self) {
        draw_texture_ex(
            &self.image,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(TRASH_WIDTH, TRASH_HEIGHT)),
                ..Default::default()
            },
        );

        let nextx = self.pos.x + self.direction_x * TRASH_SPEED;


        if nextx < 200. {
            self.direction_x = 1.;
            return;
        }

        if nextx + TRASH_WIDTH > screen_width() {
            self.direction_x = -1.;
            return;
        }

        self.pos = Vec2::new(nextx, self.pos.y);
    }

    pub fn overlaps(&self, pos: &Vec2) -> bool {
        let v_overlap = self.pos.y <= pos.y && pos.y<= self.pos.y + TRASH_HEIGHT;
        let h_overlap = self.pos.x <= pos.x && pos.x<= self.pos.x + TRASH_WIDTH;

        return v_overlap && h_overlap;
    }
}

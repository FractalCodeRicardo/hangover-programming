use std::{collections::HashMap, rc::Rc};

use macroquad::{prelude::*, rand::RandomRange};

const PlAYER_SIZE: f32 = 50.;
const PLAYER_IMAGE: &str = "player.png";
const DELAY: f64 = 8.;
const ENEMY_EVERY: usize = 3;

struct Position {
    x: f32,
    y: f32,
}

struct GameObject {
    picture: Rc<Texture2D>,
    pos: Position,
}

impl GameObject {
    fn mov_down(&mut self) {
        self.pos = Position {
            x: self.pos.x,
            y: self.pos.y,
        }
    }
}

struct ImageHandler {
    images: HashMap<String, Rc<Texture2D>>,
    names: Vec<String>,
}

impl ImageHandler {
    fn new() -> Self {
        ImageHandler {
            images: HashMap::new(),
            names: ["enemy1.png", "enemy2.png", "player.png"]
                .iter()
                .map(|i| i.to_string()) // convert &str -> String
                .collect(),
        }
    }

    async fn load_images(&mut self) {
        for img in &self.names {
            let path = "./assets/".to_string() + img;
            let picture = load_texture(&path).await.unwrap();
            let rc_picture = Rc::new(picture);

            self.images.insert(img.to_string(), rc_picture);
        }
    }

    fn get_image_player(&self) -> Rc<Texture2D> {
        return self.images[PLAYER_IMAGE].clone();
    }

    fn get_image_enemy(&self) -> Rc<Texture2D> {
        let image_index = RandomRange::gen_range(0, self.names.len() - 1);
        let image_name = self.names[image_index].to_string();
        return self.images[&image_name].clone();
    }
}

impl GameObject {
    fn draw(&self) {
        draw_texture_ex(
            &self.picture,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(PlAYER_SIZE, PlAYER_SIZE)),
                ..Default::default()
            },
        );
    }
}

struct Domination {
    enemies: Vec<GameObject>,
    player: GameObject,
    image_handler: Rc<ImageHandler>,
    last_enemy: usize,
}

impl Domination {
    fn new(image_handler: Rc<ImageHandler>) -> Self {
        Domination {
            enemies: Vec::new(),
            player: Domination::create_player(&image_handler),
            image_handler: image_handler,
            last_enemy: 0,
        }
    }

    fn create_player(image_handler: &ImageHandler) -> GameObject {
        let x = screen_width() / 2.;
        let y = screen_height() - 100.;
        let image = image_handler.get_image_player();

        return GameObject {
            picture: image,
            pos: Position { x: x, y: y },
        };
    }

    fn mov(&mut self) {
        self.mov_enemies();
        self.add_enemy();
        self.last_enemy += 1;
    }

    fn mov_enemies(&mut self) {
        for e in &mut self.enemies {
            e.mov_down();
        }
    }

    fn add_enemy(&mut self) {
        if self.last_enemy < ENEMY_EVERY {
            return;
        }

        let enemy = self.create_enemy();
        self.enemies.push(enemy);
    }

    fn create_enemy(&self) -> GameObject {
        let x = RandomRange::gen_range(0.,screen_width() );
        let y = 100.;
        let image = self.image_handler.get_image_enemy();

        return GameObject {
            picture: image,
            pos: Position { x: x, y: y },
        };
    }

    fn draw_player(&self) {
        self.player.draw();
    }

    fn draw_enemies(&self) {
        for e in &self.enemies {
            e.draw();
        }
    }

    fn draw(&self) {
        self.draw_player();
        self.draw_enemies();
    }
}

#[macroquad::main("World domination")]
async fn main() {
    let mut time = get_time();

    let mut images = ImageHandler::new();
    images.load_images().await;

    let mut game = Domination::new(Rc::new(images));

    loop {
        clear_background(RED);

        game.draw();

        if get_time() - time > DELAY {
            time = get_time();
            game.mov();
        }

        next_frame().await
    }
}

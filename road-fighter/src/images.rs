use std::{cell::OnceCell, collections::HashMap, fs, rc::Rc};

use macroquad::{rand::RandomRange, texture::{load_texture, Texture2D}};

pub struct Images {
    textures: HashMap<String, Rc<Texture2D>>,
}

impl Images {

    pub async fn new() -> Self {
        let files = fs::read_dir("./assets/images").unwrap();
        let mut textures: HashMap<String, Rc<Texture2D>> = HashMap::new();

        for f in files {
            let file_name = f
                .unwrap()
                .file_name()
                .to_string_lossy()
                .to_string();

            let path = format!("{}{}", "./assets/images/", file_name);

            let texture = load_texture(&path)
                .await
                .unwrap();

            textures.insert(file_name, Rc::new(texture));
        }

        Images {
            textures: textures
        }
    }

    pub fn get(&self, name: &str) -> &Texture2D {
        return &self.textures[name];
    }

    pub fn get_image_player(& self) -> Rc<Texture2D> {
        return self.textures["red-car.png"].clone();
    }
    
    pub fn get_image_road(& self) -> Rc<Texture2D> {
        return self.textures["road.png"].clone();
    }

    pub fn get_enemy_image(&self ) -> Rc<Texture2D> {
        let images = vec!["enemy1.png", "enemy2.png", "enemy3.png"];
        let index = RandomRange::gen_range(0, images.len());
        let image_name = images[index];

        return self.textures[image_name].clone();
    }
}


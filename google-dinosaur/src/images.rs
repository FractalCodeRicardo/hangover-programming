use std::{collections::HashMap, rc::Rc, sync::RwLock};

use macroquad::texture::{Texture2D, load_texture};
pub struct Images {
    background: Rc<Texture2D>,
    dinosaur1: Rc<Texture2D>,
    dinosaur2: Rc<Texture2D>,
    cactus: Rc<Texture2D>
}

impl Images {
    pub async fn new() -> Self {

        let background = load_texture("./assets/images/background.png")
            .await
            .unwrap();

        let dinosaur1 = load_texture("./assets/images/dinosaur1.png")
            .await
            .unwrap();

        let dinosaur2= load_texture("./assets/images/dinosaur2.png")
            .await
            .unwrap();

        let cactus = load_texture("./assets/images/cactus.png")
            .await
            .unwrap();

        Images {
            background: Rc::new(background),
            dinosaur1: Rc::new(dinosaur1),
            dinosaur2: Rc::new(dinosaur2),
            cactus: Rc::new(cactus)
        }
    }

    pub fn get_background(&self) -> Rc<Texture2D> {
        return self.background.clone();
    }

    pub fn get_dinosaurs(&self) -> Vec<Rc<Texture2D>> {
        let mut images: Vec<Rc<Texture2D>> = Vec::new();
        images.push(self.dinosaur1.clone());
        images.push(self.dinosaur2.clone());
        return images;
    }

    pub fn get_cactus(&self) -> Rc<Texture2D> {
        return self.cactus.clone();
    }

}




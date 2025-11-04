mod audio;
mod components;
mod consts;

use macroquad::prelude::*;

use crate::{audio::Audio, components::{Canion, Head, HeadImages, Landfill, TrashCan}};

struct Game {
    landfill: Landfill,
    heads: Vec<Head>,
    canion: Canion,
    head_images: HeadImages,
    trash: TrashCan,
    audio: Audio,
    score: usize
}

impl Game {
    async fn new() -> Self {
        let head_images = HeadImages::new().await;
        let landfill = Landfill::new().await;
        let canion = Canion::new().await;
        let trash = TrashCan::new().await;
        let audio = Audio::new().await;

        Game {
            heads: Vec::new(),
            head_images: head_images,
            canion: canion,
            landfill: landfill,
            trash: trash,
            audio: audio,
            score: 0
        }
    }

    fn draw(&mut self) {
        self.handle_crash();
        self.landfill.draw();
        self.draw_heads();
        self.canion.draw();
        self.trash.draw();
        self.draw_score();
    }

    fn draw_heads(&mut self) {
        for h in &mut self.heads {
            h.draw();
        }
    }

    fn left(&mut self) {
        self.canion.left();
    }

    fn right(&mut self) {
        self.canion.right();
    }

    fn shot(&mut self) {
        let pos = self.canion.shot_position();
        let force = self.canion.force();
        println!("{} {}", pos.x, pos.y);
        
        let mut head = Head::new(self.head_images.get());
        head.shot(&pos, &force);

        self.audio.shoot();

        self.heads.push(head);
    }

    fn get_indexes_crash(&self) -> Vec<usize> {
        let mut indexes: Vec<usize> = Vec::new();

        for i in 0..self.heads.len() {
            let head = &self.heads[i];
            let point = head.bottom_center();

            if self.trash.overlaps(&point) {
                indexes.push(i);
            }
        }

        return indexes;
    }

    fn handle_crash(&mut self) {
        let to_remove = self.get_indexes_crash();

        if to_remove.len() > 0 {
            self.audio.explosion();
            self.score += to_remove.len();
        }

        self.remove_heads(&to_remove);
    }

    fn remove_heads(&mut self, indexes: &Vec<usize>) {
        let mut i = 0;
        self.heads
            .retain(|_| {
                let keep = !indexes.contains(&i);
                i += 1;
                keep
            });

    }

    fn draw_score(&self) {
        let score = format!("SCORE: {}", self.score);
        draw_text(&score, screen_width() / 2.-100.,  50., 80., YELLOW);
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut game = Game::new().await;

    loop {
        clear_background(RED);

        events(&mut game);
        game.draw();

        next_frame().await
    }
}

fn events(game: &mut Game) {
    if is_key_down(KeyCode::H) {
        game.left();
    }

    if is_key_down(KeyCode::L) {
        game.right();
    }

    if is_key_pressed(KeyCode::Space) {
        game.shot();
    }
}

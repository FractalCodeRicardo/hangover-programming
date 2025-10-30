use macroquad::audio::*;
use macroquad::prelude::*;

pub struct AudioHandler {
    shoot: Sound,
    explosion: Sound
}

impl AudioHandler {

    pub async fn new() -> Self {
        let shoot = load_sound("./assets/shoot.wav").await;
        let explosion = load_sound("./assets/explosion.wav").await;

        return AudioHandler {
            shoot: shoot.unwrap(),
            explosion: explosion.unwrap()
        }
    }

    pub fn play_shoot(&self) {
        AudioHandler::play(&self.shoot);
    }

    pub fn play_explosion(&self) {
        AudioHandler::play(&self.explosion);
    }

    pub fn play(sound: &Sound) {
        play_sound(
            sound,
            PlaySoundParams {
                looped: false,
                volume: 1.0,
            },
        );
    }
}

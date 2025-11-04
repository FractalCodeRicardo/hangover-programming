use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};

pub struct Audio {
    explosion: Sound,
    shoot: Sound
}

impl Audio {

    pub async fn new() -> Self {
        let shoot = load_sound("./assets/shoot.wav")
            .await
            .unwrap();

        let explosion = load_sound("./assets/explosion.wav")
            .await
            .unwrap();

        Audio {
            shoot: shoot,
            explosion: explosion
        }
    }

    pub fn shoot(&self) {
        play_sound(&self.shoot, PlaySoundParams {
            looped: false,
            volume: 0.7
        });
    }

    pub fn explosion(&self) {
        play_sound(&self.explosion, PlaySoundParams {
            looped: false,
            volume: 0.7
        });
    }
}

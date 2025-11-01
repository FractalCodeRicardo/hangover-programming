use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};

pub struct  Audio {
    engine: Sound,
    explosion: Sound
}

impl Audio {

    pub async fn new() -> Self {
        let engine = load_sound("./assets/audio/engine.ogg").await.unwrap();
        let explosion = load_sound("./assets/audio/explosion.wav").await.unwrap();

        Audio {
            engine: engine,
            explosion: explosion
        }
    }

    pub fn play_engine(&self) {
        play_sound(&self.engine, PlaySoundParams {
            looped: true,
            volume: 0.5
        })
    }

    pub fn play_explosion(&self) {
        play_sound(&self.explosion, PlaySoundParams {
            looped: false,
            volume: 0.7
        })
    }
}

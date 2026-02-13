use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams, Sound};

const INTRO_MUSIC: &[u8] = include_bytes!("../assets/audio/intro.wav");

pub struct AudioManager {
    intro_music: Option<Sound>,
    playing: bool,
}

impl AudioManager {
    pub async fn new() -> Self {
        let intro_music = match load_sound_from_bytes(INTRO_MUSIC).await {
            Ok(sound) => Some(sound),
            Err(e) => {
                eprintln!("Failed to load intro music: {}", e);
                None
            }
        };

        Self {
            intro_music,
            playing: false,
        }
    }

    /// Start playing the intro music. Call after first user interaction
    /// to satisfy browser autoplay policy.
    pub fn play_intro(&mut self) {
        if self.playing {
            return;
        }
        if let Some(ref sound) = self.intro_music {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: true,
                    volume: 0.5,
                },
            );
            self.playing = true;
        }
    }
}

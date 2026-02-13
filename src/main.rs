mod audio;
mod song;
mod splash;

use audio::AudioManager;
use macroquad::prelude::*;
use splash::SplashScreen;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dungeon Creator".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut splash = SplashScreen::new();
    let mut audio_mgr = AudioManager::new().await;

    // Attempt autoplay immediately
    audio_mgr.play();

    loop {
        splash.update();

        // Retry audio on first user interaction (if autoplay was blocked)
        if splash.user_interacted() && !audio_mgr.is_playing() {
            audio_mgr.play();
        }

        splash.draw();
        next_frame().await;
    }
}

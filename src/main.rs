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
    let mut audio_mgr = AudioManager::new();

    loop {
        splash.update();

        if splash.user_interacted() {
            if !audio_mgr.started() {
                // First tap: load and play audio
                audio_mgr.load_and_play().await;
            } else {
                // Subsequent taps: trigger AudioContext resume
                audio_mgr.on_tap_after_start();
            }
        }

        splash.draw();
        splash.draw_status(audio_mgr.status());
        next_frame().await;
    }
}

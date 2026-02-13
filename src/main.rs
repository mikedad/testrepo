#[cfg(target_arch = "wasm32")]
mod audio;
mod song;
mod splash;

#[cfg(target_arch = "wasm32")]
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
    let mut audio_started = false;

    loop {
        splash.update();

        // Start audio after first user interaction (browser autoplay policy)
        if splash.user_interacted() && !audio_started {
            audio_mgr.start();
            audio_started = true;
        }

        if audio_started {
            audio_mgr.update();
        }

        splash.draw();
        next_frame().await;
    }
}

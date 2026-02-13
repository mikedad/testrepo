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
    let _audio = AudioManager::start();

    loop {
        splash.update();
        splash.draw();
        next_frame().await;
    }
}

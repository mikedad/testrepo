mod audio;
mod dungeon;
mod renderer;
mod song;
mod splash;
mod types;

use dungeon::DungeonGenerator;
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

enum GameState {
    Splash {
        generator: DungeonGenerator,
        show_play_button: bool,
    },
    Playing {
        map: dungeon::TileMap,
    },
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut splash = SplashScreen::new();

    // Seed RNG with current time
    macroquad::rand::srand(macroquad::miniquad::date::now() as u64);

    let mut state = GameState::Splash {
        generator: DungeonGenerator::new(),
        show_play_button: false,
    };

    loop {
        match &mut state {
            GameState::Splash { generator, show_play_button } => {
                splash.update();

                // Step dungeon generation
                if !*show_play_button {
                    if !generator.step_generation() {
                        *show_play_button = true;
                    }
                }

                // Draw splash
                splash.draw();

                if *show_play_button {
                    splash.draw_play_button();
                    if splash.check_play_button_tap() {
                        // Transition to playing
                        let gen = std::mem::replace(generator, DungeonGenerator::new());
                        state = GameState::Playing { map: gen.map };
                        next_frame().await;
                        continue;
                    }
                } else {
                    splash.draw_progress_bar(generator.progress());
                }

                draw_build_timestamp();
            }
            GameState::Playing { map } => {
                renderer::draw_dungeon(map);
                draw_build_timestamp();
            }
        }

        next_frame().await;
    }
}

fn draw_build_timestamp() {
    let ts = env!("BUILD_TIMESTAMP");
    let sw = screen_width();
    let sh = screen_height();
    let font_size = (sw / 320.0 * 5.0).max(12.0);
    let color = Color::new(1.0, 1.0, 1.0, 0.5);
    let dims = measure_text(ts, None, font_size as u16, 1.0);
    draw_text(ts, (sw - dims.width) / 2.0, sh - 4.0, font_size, color);
}

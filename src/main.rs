use macroquad::prelude::*;
use macroquad_toolkit::colors::dark;
use serde::Deserialize;

mod data;
mod engine;
mod game;
mod state;
mod ui;

use game::Game;

const WINDOW_TITLE: &str = "Monsterhall";

#[derive(Deserialize)]
struct StartupConfig {
    title: String,
    display: StartupDisplayConfig,
}

#[derive(Deserialize)]
struct StartupDisplayConfig {
    start_fullscreen: bool,
    default_resolution_id: String,
    available_resolutions: Vec<StartupResolutionOption>,
}

#[derive(Deserialize)]
struct StartupResolutionOption {
    id: String,
    width: u32,
    height: u32,
}

fn window_conf() -> Conf {
    let startup_config =
        serde_json::from_str::<StartupConfig>(include_str!("../assets/data/config.json")).ok();
    let title = startup_config
        .as_ref()
        .map(|config| config.title.clone())
        .unwrap_or_else(|| WINDOW_TITLE.to_owned());

    let (width, height) = startup_config
        .as_ref()
        .and_then(|config| {
            config
                .display
                .available_resolutions
                .iter()
                .find(|resolution| resolution.id == config.display.default_resolution_id)
                .map(|resolution| (resolution.width as i32, resolution.height as i32))
        })
        .unwrap_or((1920, 1080));

    let fullscreen = startup_config
        .as_ref()
        .map(|config| config.display.start_fullscreen)
        .unwrap_or(true);

    Conf {
        window_title: title,
        window_width: width,
        window_height: height,
        fullscreen,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    loop {
        clear_background(dark::BACKGROUND);
        game.update();
        game.draw();
        next_frame().await;
    }
}

#![allow(clippy::too_many_arguments)]

use macroquad::prelude::*;
use macroquad_toolkit::capture;
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

    // While capturing, force a windowed, fixed-size framebuffer so screenshots
    // are deterministic (MONSTERHALL_WINDOW_WIDTH/HEIGHT override the config).
    let capturing = capture::capture_requested("MONSTERHALL");
    Conf {
        window_title: title,
        window_width: capture::env_i32("MONSTERHALL_WINDOW_WIDTH", width),
        window_height: capture::env_i32("MONSTERHALL_WINDOW_HEIGHT", height),
        fullscreen: fullscreen && !capturing,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    macroquad_toolkit::ui::ensure_default_ui_font().expect("toolkit UI font should load");
    macroquad_toolkit::ui::set_min_ui_font_size(18.0);

    let mut game = Game::new().await;

    // Screenshot harness: when MONSTERHALL_CAPTURE_PATH is set, render
    // deterministic frames, write a PNG, and exit. The boot flow lands on the
    // main menu, so that is what gets photographed; the scene env var is
    // currently unused.
    if let Some(config) = capture::CaptureConfig::from_env("MONSTERHALL") {
        capture::run_capture(&config, |_dt| {
            clear_background(dark::BACKGROUND);
            game.update();
            game.draw();
        })
        .await;
        return;
    }

    loop {
        clear_background(dark::BACKGROUND);
        game.update();
        game.draw();
        next_frame().await;
    }
}

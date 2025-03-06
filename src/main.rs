mod state;
mod menu;
mod settings;
mod gameplay;

use macroquad::prelude::*;
use state::GameState;
use menu::Menu;
use settings::update_settings;
use gameplay::GamePlay;

const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Laser Blast".to_owned(),
        fullscreen: false,
        window_width: 1280,
        window_height: (1280.0 / ASPECT_RATIO) as i32,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandWithX11Fallback,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    set_pc_assets_folder("assets");

    let mut state = GameState::MainMenu;
    let mut menu = Menu::new().await;
    let mut gameplay = GamePlay::new().await;

    loop {
        clear_background(BLACK);

        let delta_time = get_frame_time();

        match state {
            GameState::MainMenu => {
                menu.update(delta_time);
                menu.draw(&mut state);
            },
            GameState::Settings => update_settings(&mut state),
            GameState::Gameplay => {
                gameplay.update(delta_time);
                gameplay.draw();
            },
        }

        next_frame().await;
    }
}
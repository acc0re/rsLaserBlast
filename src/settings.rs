use macroquad::prelude::*;
use crate::state::GameState;

pub fn update_settings(state: &mut GameState) {
    draw_text("Settings", 20.0, 40.0, 40.0, WHITE);
    draw_text("Back", 20.0, 100.0, 30.0, WHITE);

    if is_key_pressed(KeyCode::B) {
        *state = GameState::MainMenu;
    }
}

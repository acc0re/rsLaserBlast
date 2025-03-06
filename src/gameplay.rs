use std::f32::consts;
use macroquad::prelude::*;
use crate::state::GameState;
use tori::  animated_sprite::AnimatedSprite;
use crate::assets;

pub struct GamePlay {
    player_sprite: AnimatedSprite,
}

impl GamePlay {
    pub async fn new() -> Self {
        Self {
            player_sprite: AnimatedSprite::new(
                &*assets::get_gfx_path("spaceship.png"),
                vec2(10.0, 10.0),
                5f32,
                consts::PI, //Looking down
                vec2(8f32, 8f32),
                3,
                3f32,
            ).await,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let speed = 500.0; // Speed in units per second

        if is_key_down(KeyCode::Right) {
            self.player_sprite.move_by(vec2(speed * delta_time, 0.0));
        }
        if is_key_down(KeyCode::Left) {
            self.player_sprite.move_by(vec2(-speed * delta_time, 0.0));
        }

        self.player_sprite.update(delta_time);
    }

    pub fn draw(&self) {
        self.player_sprite.draw();
    }
}

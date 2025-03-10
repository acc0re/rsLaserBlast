use std::f32::consts;
use macroquad::prelude::*;
use tori::  animated_sprite::AnimatedSprite;

pub struct GamePlay {
    player_sprite: AnimatedSprite,
}

impl GamePlay {
    pub async fn new() -> Self {
        Self {
            player_sprite: AnimatedSprite::new(
                "gfx/spaceship.png",
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
        let mut direction = vec2(0.0, 0.0);

        if is_key_down(KeyCode::Right) {
            direction.x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::Up) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            direction.y += 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        self.player_sprite.move_by(direction * speed * delta_time);
        self.player_sprite.update(delta_time);
    }

    pub fn draw(&self) {
        self.player_sprite.draw();
    }
}

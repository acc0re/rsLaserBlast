use macroquad::prelude::*;
use std::f32::consts;
use tori::animated_sprite::AnimatedSprite;
use crate::state::GameState;
use macroquad_text::Fonts;

const ARCADECLASSIC: &[u8] = include_bytes!("../ttf/ARCADECLASSIC.TTF");

pub struct Menu<'a> {
    selected_index: usize,
    spaceship_sprite: AnimatedSprite,
    stars: Vec<Star>,
    fonts: Fonts<'a>
}

pub struct Star {
    position: Vec2,
    speed: f32,
    pub direction: Vec2,
}

impl<'a> Menu<'a> {
    pub async fn new() -> Self {
        let mut stars = Vec::new();
        let mut fonts = Fonts::default();

        fonts.load_font_from_bytes("ARCADECLASSIC", ARCADECLASSIC).unwrap();

        // Erstelle eine zufällige Anzahl an Sternen
        for _ in 0..100 {
            stars.push(Star {
                position: vec2(rand::gen_range(0.0, screen_width()), rand::gen_range(0.0, screen_height())),
                speed: rand::gen_range(30.0, 100.0),
                direction: vec2(0f32, -1f32), // To the top
            });
        }
        set_pc_assets_folder("assets");
        let tex = load_texture("gfx/spaceship.png").await.unwrap();

        Self {
            selected_index: 0,
            spaceship_sprite: AnimatedSprite::new(
                "gfx/spaceship.png",
                vec2(screen_width() / 2f32, 50.0),
                5f32,
                consts::PI, // Looking down
                vec2(8f32, 8f32),
                3,
                3f32,
            ).await,
            stars,
            fonts,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.spaceship_sprite.update(delta_time);

        // Update the stars with more complex motion
        for star in &mut self.stars {
            star.position.x += star.direction.x * star.speed * delta_time;
            star.position.y += star.direction.y * star.speed * delta_time;

            if star.position.x < 0.0 {
                star.position.x = screen_width();
            } else if star.position.x > screen_width() {
                star.position.x = 0.0;
            }

            if star.position.y < 0.0 {
                star.position.y = screen_height();
            } else if star.position.y > screen_height() {
                star.position.y = 0.0;
            }
        }
    }

    pub fn draw(&mut self, state: &mut GameState) {
        // Zeichne die Sterne im Hintergrund
        for star in &self.stars {
            draw_circle(star.position.x, star.position.y, 1f32, WHITE);
        }

        self.spaceship_sprite.draw();

        let screen_width = screen_width();
        let screen_height = screen_height();

        let text_size = 48;
        let spacing = 60.0;

        let center_x = screen_width / 2.0;
        let start_y = screen_height / 2.0 - spacing;

        let menu_items = ["Start", "Settings", "Exit"];

        self.fonts.draw_text("Laser Blast", center_x - 200.0, start_y - 100.0, 72, WHITE);

        if is_key_pressed(KeyCode::Down) {
            self.selected_index = (self.selected_index + 1) % menu_items.len();
        } else if is_key_pressed(KeyCode::Up) {
            if self.selected_index == 0 {
                self.selected_index = menu_items.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }

        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            match self.selected_index {
                0 => *state = GameState::Gameplay,
                1 => *state = GameState::Settings,
                2 => std::process::exit(0),
                _ => {}
            }
        }

        for (i, &item) in menu_items.iter().enumerate() {
            let y = start_y + (i as f32 * spacing);
            let text_x = center_x - 100.0;

            if i == self.selected_index {
                self.fonts.draw_text(item, text_x, y, text_size, BLUE);
                self.spaceship_sprite.set_position(vec2(text_x - 60f32, y + 10f32));
            } else {
                self.fonts.draw_text(item, text_x, y, text_size, WHITE);
            }
        }
    }
}

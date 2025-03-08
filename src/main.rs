use macroquad::camera::Camera2D;
use macroquad::color::BLACK;
use macroquad::input::{is_key_pressed, is_key_released, KeyCode};
use macroquad::math::Vec2;
use macroquad::rand::RandGenerator;
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::entity::Entity;
use crate::planet::Planet;
use crate::player::Player;

mod entity;
mod planet;
mod player;

#[macroquad::main("MyGame")]
async fn main() {
    // random number generator
    let rng = RandGenerator::new();
    // this is the seed of the number generator (the current time in milliseconds)
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("yooo, the time is all fucked up 'n shit")
        .as_millis();
    rng.srand(current_time as u64);

    let mut player: Player = Player::new(50f32);
    let mut camera: Camera2D = Camera2D::default();
    let mut planets: Vec<Planet> = vec![Planet::new(
        Vec2 {
            x: 10f32,
            y: 100f32,
        },
        10f32,
    )];

    for i in 0..10 {
        planets.push(Planet::new(
            Vec2 {
                x: rng.gen_range(0f32, screen_width()),
                y: (i as f32) * -rng.gen_range(10f32, 1000f32),
            },
            rng.gen_range(10f32, 100f32),
        ));
    }

    camera.offset = Vec2 {
        x: screen_width() * 0.5f32,
        y: screen_height() * 0.5f32,
    };

    loop {
        let delta_time: f32 = get_frame_time();

        if is_key_pressed(KeyCode::Space) {
            let mut nearest: (f32, Option<&Planet>) = (f32::INFINITY, None);
            for p in &planets {
                let dist = (p.position.x - player.position.x) * (p.position.x - player.position.x)
                    + (p.position.y - player.position.y) * (p.position.y - player.position.y);
                if dist < nearest.0 {
                    nearest.0 = dist;
                    nearest.1 = Some(p);
                }
            }
            player.linked_planet = nearest.1;
        }
        if is_key_released(KeyCode::Space) {
            player.linked_planet = None;
        }
        player.update(delta_time);
        player.update_camera(&mut camera);
        clear_background(BLACK);
        player.draw(&camera);
        for planet in &planets {
            planet.draw(&camera);
        }

        next_frame().await
    }
}

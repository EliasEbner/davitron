use macroquad::camera::Camera2D;
use macroquad::color::BLACK;
use macroquad::input::{is_key_pressed, is_key_released, KeyCode};
use macroquad::math::Vec2;
use macroquad::rand::RandGenerator;
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};

use crate::entity::Entity;
use crate::planet::Planet;
use crate::player::Player;
use crate::random_generator::get_rand_generator;

mod entity;
mod planet;
mod player;
mod random_generator;

#[macroquad::main("MyGame")]
async fn main() {
    let rand_num_generator: RandGenerator = get_rand_generator();

    let mut player: Player = Player::new(50f32);
    let mut camera: Camera2D = Camera2D::default();
    let mut planets: Vec<Planet> = Vec::new();

    for i in 0..10 {
        planets.push(Planet::new(
            Vec2 {
                x: rand_num_generator.gen_range(0f32, screen_width()),
                y: (i as f32) * -rand_num_generator.gen_range(10f32, 1000f32),
            },
            rand_num_generator.gen_range(10f32, 100f32),
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
            match nearest.1 {
                Some(planet) => player.linked_planet_position = Some(planet.position),
                None => player.linked_planet_position = None,
            }
        }

        if is_key_released(KeyCode::Space) {
            player.linked_planet_position = None;
        }

        player.update(delta_time);
        player.update_camera(&mut camera);

        for p in &mut planets {
            p.update(delta_time);
        }

        clear_background(BLACK);
        player.draw(&camera);
        for planet in &planets {
            planet.draw(&camera);
        }

        next_frame().await
    }
}

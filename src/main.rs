use macroquad::camera::Camera2D;
use macroquad::color::BLACK;
use macroquad::input::{is_key_pressed, is_key_released, KeyCode};
use macroquad::math::Vec2;
use macroquad::rand::RandGenerator;
use macroquad::time::get_frame_time;
use macroquad::window::{
    clear_background, next_frame, request_new_screen_size, screen_height, screen_width,
};

use crate::planet::Planet;
use crate::player::Player;
use crate::random_generator::get_rand_generator;

mod planet;
mod player;
mod random_generator;

#[macroquad::main("MyGame")]
async fn main() {
    request_new_screen_size(1000f32, 800f32);
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
            Vec2 {
                x: rand_num_generator.gen_range(-1000f32, 1000f32),
                y: rand_num_generator.gen_range(-1000f32, 1000f32),
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
            let mut nearest: (f32, Option<usize>) = (f32::INFINITY, None);
            for (index, planet) in planets.iter().enumerate() {
                let dist = (planet.position.x - player.position.x)
                    * (planet.position.x - player.position.x)
                    + (planet.position.y - player.position.y)
                        * (planet.position.y - player.position.y);

                if dist < nearest.0 {
                    nearest.0 = dist;
                    nearest.1 = Some(index);
                }
            }
            match nearest.1 {
                Some(index) => player.linked_planet_index = Some(index),
                None => player.linked_planet_index = None,
            }
        }

        if is_key_released(KeyCode::Space) {
            player.let_go_of_planet(&planets, delta_time);
        }

        player.update_camera(&mut camera);
        player.update(&planets, delta_time);

        for planet in &mut planets {
            planet.update(delta_time);
        }

        clear_background(BLACK);
        player.draw(&planets, &camera);
        for planet in &planets {
            planet.draw(&camera);
        }

        next_frame().await
    }
}

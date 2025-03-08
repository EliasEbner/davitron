use macroquad::camera::Camera2D;
use macroquad::color::BLACK;
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

        player.update(delta_time);
        player.update_camera(&mut camera);
        clear_background(BLACK);
        player.draw(&camera);
        for planet in planets.iter_mut() {
            planet.draw(&camera);
        }
        next_frame().await
    }
}

use macroquad::camera::Camera2D;
use macroquad::color::BLACK;
use macroquad::input::{is_key_pressed, is_key_released, KeyCode};
use macroquad::math::Vec2;
use macroquad::rand::RandGenerator;
use macroquad::text::draw_text;
use macroquad::time::get_frame_time;
use macroquad::window::{
    clear_background, next_frame, screen_height, screen_width, set_fullscreen,
};

use danger_zone::DangerZone;
use planet::Planet;
use player::Player;
use random_generator::get_rand_generator;

mod danger_zone;
mod particle;
mod particle_controller;
mod planet;
mod player;
mod random_generator;

#[macroquad::main("MyGame")]
async fn main() {
    // request_new_screen_size(1000f32, 800f32);
    set_fullscreen(true);
    let rand_num_generator: RandGenerator = get_rand_generator();

    let mut player: Player = Player::new(50f32);
    let mut camera: Camera2D = Camera2D::default();
    let mut planets: Vec<Planet> = Vec::new();
    let mut bottom_danger_zone: DangerZone = DangerZone::new(
        Vec2 {
            x: 0f32,
            y: 1000f32,
        },
        Vec2 {
            x: 3000f32,
            y: 1200f32,
        },
        Vec2 { x: 0f32, y: -20f32 },
        0.003,
    );
    let mut left_danger_zone: DangerZone = DangerZone::new(
        Vec2 {
            x: -1600f32,
            y: 0f32,
        },
        Vec2 {
            x: 1200f32,
            y: 2000f32,
        },
        Vec2 { x: 0f32, y: 0f32 },
        0.01,
    );
    let mut right_danger_zone: DangerZone = DangerZone::new(
        Vec2 {
            x: 1600f32,
            y: 0f32,
        },
        Vec2 {
            x: 1200f32,
            y: 2000f32,
        },
        Vec2 { x: 0f32, y: 0f32 },
        0.01,
    );

    for i in 0..10 {
        planets.push(Planet::new(
            Vec2 {
                x: rand_num_generator.gen_range(0f32, screen_width()),
                y: (i as f32) * -rand_num_generator.gen_range(10f32, 1000f32),
            },
            Vec2 {
                x: rand_num_generator.gen_range(-30f32, 30f32), // not uniform!
                y: rand_num_generator.gen_range(-30f32, 30f32),
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

        if !player.is_dead && is_key_pressed(KeyCode::Space) {
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

        if player.is_dead || is_key_released(KeyCode::Space) {
            player.let_go_of_planet(&planets);
        }

        player.update_camera(&mut camera);
        player.update(&planets, delta_time);

        bottom_danger_zone.update_as_bottom_zone(delta_time, player.position.x);
        bottom_danger_zone.check_and_handle_player_collision(&mut player);
        if !player.is_dead {
            left_danger_zone.update_as_side_zone(delta_time, player.position.y);
            right_danger_zone.update_as_side_zone(delta_time, player.position.y);
            left_danger_zone.check_and_handle_player_collision(&mut player);
            right_danger_zone.check_and_handle_player_collision(&mut player);
            if player.is_dead {
                bottom_danger_zone.position.y = player.position.y + 600f32;
            }
        }

        for planet in &mut planets {
            planet.update(delta_time);
        }

        for planet in &mut planets {
            if (player.position - planet.position).length() < player.radius + planet.radius {
                player.handle_collistion(planet);
            }
        }

        for i in 0..(planets.len() - 1) {
            let (left, right) = planets.split_at_mut(i + 1);
            for j in 0..right.len() {
                if (left[i].position - right[j].position).length()
                    < left[i].radius + right[j].radius
                {
                    left[i].handle_collistion(&mut right[j]);
                }
            }
        }

        clear_background(BLACK);
        
        player.draw(&planets, &camera);
        bottom_danger_zone.draw(&camera);
        if !player.is_dead {
            left_danger_zone.draw(&camera);
            right_danger_zone.draw(&camera);
        }
        for planet in &planets {
            planet.draw(&camera);
        }

        if player.is_dead {
            draw_text(
                "YOU DIED LOSER",
                screen_width() / 2f32 - 306.25 / 2f32,
                screen_height() / 2f32 - 25f32 / 2f32,
                50f32,
                BLACK,
            );
        }

        next_frame().await
    }
}

use std::f32::consts::{PI, SQRT_2};

use macroquad::{
    camera::Camera2D,
    color::{Color, GREEN, RED},
    input::is_key_down,
    math::Vec2,
    shapes::{draw_circle, draw_line},
    window::{screen_height, screen_width},
};

use crate::{danger_zone::DangerZone, planet::Planet};

const PLAYER_COLOR: Color = RED;

pub struct Player {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub linked_planet_index: Option<usize>,
    pub is_dead: bool,
}

impl Player {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            position: Vec2::default(),
            velocity: Vec2::default(),
            linked_planet_index: None,
            is_dead: false,
        }
    }

    pub fn update(
        self: &mut Self,
        planets: &Vec<Planet>,
        danger_zone: &DangerZone,
        delta_time: f32,
    ) {
        if self.check_danger_zone_collision(danger_zone) {
            self.die();
        } else {
            let mut abs_velocity =
                f32::sqrt(self.velocity.y * self.velocity.y + self.velocity.x * self.velocity.x);
            if abs_velocity < 0.0001 {
                self.velocity.y = -0.01;
                abs_velocity = 0.0001;
            }

            let velocity_factor = 1f32
                + (-1.2f32
                    + f32::from(is_key_down(macroquad::input::KeyCode::Space)) * 2000f32
                        / abs_velocity)
                    * delta_time;

            self.velocity.x *= velocity_factor;
            self.velocity.y *= velocity_factor;

            //     // for debugging
            // if is_key_down(macroquad::input::KeyCode::D) {
            //     self.velocity.x = 50f32;
            // }
            // if is_key_down(macroquad::input::KeyCode::A) {
            //     self.velocity.x = -50f32;
            // }
            // if is_key_down(macroquad::input::KeyCode::W) {
            //     self.velocity.y = 50f32;
            // }
            // if is_key_down(macroquad::input::KeyCode::S) {
            //     self.velocity.y = -50f32;
            // }

            match self.linked_planet_index {
                Some(linked_planet_index) => {
                    let linked_planet: &Planet = &planets[linked_planet_index];

                    let to_planet = linked_planet.position - self.position;

                    let mut angle_diff = (to_planet.y.atan2(to_planet.x)
                        - self.velocity.y.atan2(self.velocity.x))
                        % PI;
                    if angle_diff > 0.0 {
                        angle_diff -= 0.5 * PI;
                    } else {
                        angle_diff += 0.5 * PI;
                    };

                    let max_rotation = 6.0 * delta_time; // rotation per second
                    let rotation_angle = angle_diff.clamp(-max_rotation, max_rotation);

                    // rotate velocity:
                    let sin_angle = rotation_angle.sin();
                    let cos_angle = rotation_angle.cos();
                    let old_vel_x = self.velocity.x;
                    self.velocity.x = old_vel_x * cos_angle - self.velocity.y * sin_angle;
                    self.velocity.y = old_vel_x * sin_angle + self.velocity.y * cos_angle;

                    // correcting so that you don't drift outward (no clue why it doesn't work properly)
                    self.velocity += to_planet.normalize()
                        * (1f32
                            - f32::cos(f32::asin(
                                (self.velocity.length() * delta_time / to_planet.length())
                                    .min(1f32),
                            )))
                        / delta_time;

                    // 'not so clean linking solution'™
                    self.position += linked_planet.velocity * delta_time;
                }
                None => {}
            }

            // position
            self.position.x += self.velocity.x * delta_time;
            self.position.y += self.velocity.y * delta_time;
        }
    }

    fn die(self: &mut Self) {
        self.is_dead = true;
    }

    fn check_danger_zone_collision(self: &Self, danger_zone: &DangerZone) -> bool {
        if self.position.y + self.radius > danger_zone.position_y {
            true
        } else {
            false
        }
    }

    pub fn draw(self: &Self, planets: &Vec<Planet>, camera: &Camera2D) {
        match self.linked_planet_index {
            Some(linked_planet_index) => {
                let linked_planet_position = planets[linked_planet_index].position;
                draw_line(
                    self.position.x - camera.target.x + camera.offset.x,
                    self.position.y - camera.target.y + camera.offset.y,
                    linked_planet_position.x - camera.target.x + camera.offset.x,
                    linked_planet_position.y - camera.target.y + camera.offset.y,
                    10f32,
                    GREEN,
                );
            }
            None => {}
        }
        draw_circle(
            self.position.x - camera.target.x + camera.offset.x,
            self.position.y - camera.target.y + camera.offset.y,
            self.radius,
            PLAYER_COLOR,
        );
    }

    pub fn let_go_of_planet(self: &mut Self, planets: &Vec<Planet>) {
        match self.linked_planet_index {
            Some(linked_planet_index) => {
                let linked_planet = &planets[linked_planet_index];
                self.velocity += linked_planet.velocity;

                self.linked_planet_index = None;
            }
            None => {}
        }
    }

    pub fn update_camera(self: &Self, camera: &mut Camera2D) {
        camera.offset = Vec2 {
            x: screen_width() * 0.5f32 - self.velocity.x * 0.02f32,
            y: screen_height() * 0.5f32 - self.velocity.y * 0.02f32,
        };

        camera.target = Vec2 {
            x: self.position.x,
            y: self.position.y,
        };
    }
}

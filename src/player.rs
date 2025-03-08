use std::f32::consts::PI;

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
            let mut abs_speed =
                f32::sqrt(self.velocity.y * self.velocity.y + self.velocity.x * self.velocity.x);
            if abs_speed < 0.0001 {
                self.velocity.y = -0.01;
                abs_speed = 0.0001;
            }

            let speed_factor = 1f32
                + (-0.5
                    + f32::from(is_key_down(macroquad::input::KeyCode::Space)) * 200f32
                        / abs_speed)
                    * delta_time;

        self.velocity.x *= speed_factor;
        self.velocity.y *= speed_factor;

        
        if (is_key_down(macroquad::input::KeyCode::D)) { // for debugging
            self.velocity.x = 50f32;
        }
        if (is_key_down(macroquad::input::KeyCode::A)) {
            self.velocity.x = -50f32;
        }
        if (is_key_down(macroquad::input::KeyCode::W)) {
            self.velocity.y = 50f32;
        }
        if (is_key_down(macroquad::input::KeyCode::S)) {
            self.velocity.y = -50f32;
        }

        match self.linked_planet_index {
            Some(linked_planet_index) => {
                let linked_planet: &Planet = &planets[linked_planet_index];
                
                abs_vel = f32::sqrt(
                    self.velocity.y * self.velocity.y + self.velocity.x * self.velocity.x,
                );
                let delta_x = self.position.x - linked_planet.position.x;
                let delta_y = self.position.y - linked_planet.position.y;
                let abs_dist = f32::sqrt(delta_x * delta_x + delta_y * delta_y);
                let angle: f32 = f32::atan(delta_y / delta_x);
                let mut angle_vel: f32 =
                    f32::rem_euclid(angle + f32::atan(self.velocity.y / self.velocity.x), PI);
                if angle_vel > PI*0.5 {
                    angle_vel = PI - angle_vel;
                }
                let angle_for_orbit: f32 = abs_vel * delta_time / abs_dist;
                println!("{angle_vel}\n{angle_for_orbit}\n");
                // let mut old_vel_x: f32 = self.velocity.x;
                // self.velocity.x = old_vel_x * cos_angle - self.velocity.y * sin_angle;
                // self.velocity.y = old_vel_x * sin_angle + self.velocity.y * cos_angle;
            
                self.position += linked_planet.speed * delta_time;
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
                self.velocity += linked_planet.speed;

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

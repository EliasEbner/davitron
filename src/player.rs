use crate::entity::Entity;
use crate::planet::Planet;
use macroquad::{
    camera::Camera2D,
    color::{Color, GREEN, RED},
    input::is_key_down,
    math::Vec2,
    shapes::{draw_circle, draw_line},
    window::{screen_height, screen_width},
};

const PLAYER_COLOR: Color = RED;

pub struct Player<'a> {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub linked_planet: Option<&'a Planet>,
}

impl<'a> Entity for Player<'a> {
    fn update(self: &mut Self, delta_time: f32) {
        let mut abs_vel =
            f32::sqrt(self.velocity.y * self.velocity.y + self.velocity.x * self.velocity.x);
        if abs_vel < 0.0001 {
            self.velocity.y = -0.01;
            abs_vel = 0.0001;
        }

        let speed_factor = 1f32
            + (-0.5 + f32::from(is_key_down(macroquad::input::KeyCode::Space)) * 200f32 / abs_vel)
                * delta_time;
        self.velocity.y *= speed_factor;
        self.velocity.x *= speed_factor;

        if let Some(linked_planet) = self.linked_planet {
            let delta_x = self.position.x - linked_planet.position.x;
            let delta_y = self.position.y - linked_planet.position.y;
            let abs_dist = f32::sqrt(delta_x * delta_x + delta_y * delta_y);
            let angle: f32 = -f32::atan(delta_y / delta_x);
            let sin_angle: f32 = f32::sin(angle);
            let cos_angle: f32 = f32::cos(angle);

            let mut old_vel_x: f32 = self.velocity.x;
            self.velocity.x = old_vel_x * cos_angle - self.velocity.y * sin_angle;
            self.velocity.y = old_vel_x * sin_angle + self.velocity.y * cos_angle;

            let offset =
                abs_dist * (1f32 - f32::cos(f32::asin(self.velocity.y * delta_time / abs_dist)));
            // println!("{angle}\noff: {offset}\nabs: {abs_dist}\natan: {}", f32::atan(self.velocity.y/abs_dist));

            // self.velocity.y += f32::signum(self.velocity.y)
            //     * 0.1f32
            //     * f32::abs(self.velocity.x + offset * f32::signum(delta_x)); // sqrt(x^2 + y^2) != x + y lol
            self.velocity.x = self.velocity.x * 0.3f32 - 0.7f32 * offset * f32::signum(delta_x);

            old_vel_x = self.velocity.x;
            self.velocity.x = old_vel_x * cos_angle - self.velocity.y * (-sin_angle);
            self.velocity.y = old_vel_x * (-sin_angle) + self.velocity.y * cos_angle;
        }

        // position
        self.position.x += self.velocity.x * delta_time;
        self.position.y += self.velocity.y * delta_time;
    }

    fn draw(self: &Self, camera: &Camera2D) {
        if let Some(linked_planet) = self.linked_planet {
            draw_line(
                self.position.x - camera.target.x + camera.offset.x,
                self.position.y - camera.target.y + camera.offset.y,
                linked_planet.position.x - camera.target.x + camera.offset.x,
                linked_planet.position.y - camera.target.y + camera.offset.y,
                10f32,
                GREEN,
            );
        }
        draw_circle(
            self.position.x - camera.target.x + camera.offset.x,
            self.position.y - camera.target.y + camera.offset.y,
            self.radius,
            PLAYER_COLOR,
        );
    }
}

impl<'a> Player<'a> {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            position: Vec2::default(),
            velocity: Vec2::default(),
            linked_planet: None,
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

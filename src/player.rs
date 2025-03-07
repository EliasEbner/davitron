use crate::entity::Entity;
use macroquad::{
    camera::Camera2D,
    color::{Color, RED},
    input::is_key_down,
    math::Vec2,
    shapes::draw_circle,
    window::{screen_height, screen_width},
};

const PLAYER_COLOR: Color = RED;

pub struct Player {
    pub position: Vec2,
    pub speed: Vec2,
    pub radius: f32,
}

impl Entity for Player {
    fn update(self: &mut Self) {
        let mut abs_speed = f32::sqrt(self.speed.y * self.speed.y + self.speed.x * self.speed.x);
        if abs_speed < 0.01 {
            self.speed.y = -0.1;
            abs_speed = 0.01;
        }
        let speed_factor =
            0.95 + f32::from(is_key_down(macroquad::input::KeyCode::Space)) * 2f32 / abs_speed;
        self.speed.y *= speed_factor;
        self.speed.x *= speed_factor;

        // position
        self.position.x += self.speed.x;
        self.position.y += self.speed.y;
    }

    fn draw(self: &mut Self, camera: &Camera2D) {
        draw_circle(
            self.position.x - camera.target.x + camera.offset.x,
            self.position.y - camera.target.y + camera.offset.y,
            self.radius,
            PLAYER_COLOR,
        );
    }
}

impl Player {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            position: Vec2::default(),
            speed: Vec2::default(),
        }
    }

    pub fn update_camera(self: &Self, camera: &mut Camera2D) {
        camera.offset = Vec2 {
            x: screen_width() * 0.5f32 - self.speed.x * 2f32,
            y: screen_height() * 0.5f32 - self.speed.y * 2f32,
        };

        camera.target = Vec2 {
            x: self.position.x,
            y: self.position.y,
        };
    }
}

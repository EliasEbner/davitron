use macroquad::{
    camera::Camera2D,
    color::{Color, RED},
    math::Vec2,
    shapes::draw_circle,
    window::{screen_height, screen_width},
};

use crate::entity::Entity;

const PLAYER_COLOR: Color = RED;

pub struct Planet {
    pub position: Vec2,
    pub speed: Vec2,
    pub radius: f32,
}

impl Entity for Planet {
    fn update(self: &mut Self) {
        // TODO: I don't know what to do here
    }

    fn draw(self: &mut Self, camera: &Camera2D) {
        draw_circle(self.position.x, self.position.y, self.radius, PLAYER_COLOR);
    }
}

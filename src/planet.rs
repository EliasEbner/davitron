use macroquad::{
    camera::Camera2D,
    color::{Color, BLUE},
    math::Vec2,
    shapes::draw_circle,
};

use crate::entity::Entity;

const PLANET_COLOR: Color = BLUE;

pub struct Planet {
    pub position: Vec2,
    pub speed: Vec2,
    pub radius: f32,
}
impl Planet {
    pub fn new(position: Vec2, radius: f32) -> Self {
        Self {
            radius,
            position,
            speed: Vec2::default(),
        }
    }
}
impl Entity for Planet {
    fn update(self: &mut Self, delta_time: f32) {
        // TODO: I don't know what to do here
    }

    fn draw(self: &Self, camera: &Camera2D) {
        draw_circle(
            self.position.x - camera.target.x + camera.offset.x,
            self.position.y - camera.target.y + camera.offset.y,
            self.radius,
            PLANET_COLOR,
        );
    }
}

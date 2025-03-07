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

    // only draw a planet if it is shown in the screen
    fn draw(self: &mut Self, camera: &Camera2D) {
        if self.position.x < camera.target.x + camera.offset.x + screen_width()
            && self.position.x > camera.target.x + camera.offset.x - screen_width()
            && self.position.y < camera.target.y + camera.offset.y + screen_height()
            && self.position.y > camera.target.y + camera.offset.y - screen_height()
        {
            draw_circle(
                self.position.x - camera.target.x + camera.offset.x,
                self.position.y - camera.target.y + camera.offset.y,
                self.radius,
                PLAYER_COLOR,
            );
        }
    }
}

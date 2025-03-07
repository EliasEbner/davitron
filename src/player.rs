use crate::entity::Entity;
use macroquad::{
    color::{Color, RED},
    input::is_key_down,
    math::Vec2,
    shapes::draw_circle,
};

const PLAYER_COLOR: Color = RED;

pub struct Player {
    pub position: Vec2,
    pub speed: Vec2,
    pub radius: f32,
}

impl Entity for Player {
    fn update(self: &mut Self) {
        self.speed.y = self.speed.y
            * (0.95
                + f32::from(is_key_down(macroquad::input::KeyCode::Space)) * 10f32
                    / f32::sqrt(self.speed.y * self.speed.y + self.speed.x * self.speed.x));

        // position
        self.position.x += self.speed.x;
        self.position.y += self.speed.y;
    }

    fn draw(self: &mut Self) {
        draw_circle(self.position.x, self.position.y, self.radius, PLAYER_COLOR);
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
}

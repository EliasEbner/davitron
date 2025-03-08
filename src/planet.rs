use macroquad::{
    camera::Camera2D,
    color::{Color, BLUE},
    math::Vec2,
    rand::RandGenerator,
    shapes::draw_circle,
};

use crate::{entity::Entity, random_generator::get_rand_generator};

const PLANET_COLOR: Color = BLUE;

pub struct Planet {
    pub position: Vec2,
    pub speed: Vec2,
    pub radius: f32,
}

impl Planet {
    pub fn new(position: Vec2, radius: f32) -> Self {
        let random_num_generator: RandGenerator = get_rand_generator();
        Self {
            radius,
            position,
            speed: Vec2::from((
                random_num_generator.gen_range(-1000f32, 1000f32),
                random_num_generator.gen_range(-1000f32, 1000f32),
            )),
        }
    }
}
impl Entity for Planet {
    fn update(self: &mut Self, delta_time: f32) {
        self.position.x += self.speed.x * delta_time;
        self.position.y += self.speed.y * delta_time;
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

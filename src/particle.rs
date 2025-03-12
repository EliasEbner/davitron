use macroquad::{camera::Camera2D, color::Color, math::Vec2, shapes::draw_circle};

pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub color: Color,
    pub time_left: f32,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2, radius: f32, color: Color, time_left: f32) -> Self {
        Self {
            position: position,
            velocity: velocity,
            radius: radius,
            color: color,
            time_left: time_left,
        }
    }
    pub fn draw(self: &Self, camera: &Camera2D) {
        draw_circle(
            self.position.x - camera.target.x + camera.offset.x,
            self.position.y - camera.target.y + camera.offset.y,
            self.radius,
            self.color,
        );
    }

    pub fn update(self: &mut Self, delta_time: f32) {
        self.time_left -= delta_time;
        self.position.x += self.velocity.x * delta_time;
        self.position.y += self.velocity.y * delta_time;
        if self.time_left < self.color.a {
            self.color.a = self.time_left;
        }
    }
}

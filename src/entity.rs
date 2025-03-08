use macroquad::camera::Camera2D;

pub trait Entity {
    fn update(self: &mut Self, delta_time: f32);
    fn draw(self: &mut Self, camera: &Camera2D);
}

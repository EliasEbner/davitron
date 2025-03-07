use macroquad::camera::Camera2D;

pub trait Entity {
    fn update(self: &mut Self);
    fn draw(self: &mut Self, camera: &Camera2D);
}

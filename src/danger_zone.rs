use macroquad::{
    camera::Camera2D,
    color::RED,
    shapes::draw_rectangle,
    window::{screen_height, screen_width},
};

pub struct DangerZone {
    pub position_y: f32,
    pub speed_y: f32,
}

const INITIAL_VERTICAL_SPEED: f32 = -20f32;
impl DangerZone {
    pub fn new() -> Self {
        return DangerZone {
            position_y: screen_height(),
            speed_y: INITIAL_VERTICAL_SPEED,
        };
    }

    pub fn update(self: &mut Self, delta_time: f32) {
        self.speed_y *= 1f32 + 0.2 * delta_time;
        self.position_y += self.speed_y * delta_time;
    }

    pub fn draw(self: &Self, camera: &Camera2D) {
        draw_rectangle(
            0f32,
            {
                let final_position_y: f32 = self.position_y - camera.target.y + camera.offset.y;
                if final_position_y < 0f32 {
                    0f32
                } else {
                    final_position_y
                }
            },
            screen_width(),
            screen_height(),
            RED,
        );
    }
}

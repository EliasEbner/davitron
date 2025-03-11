use crate::particle_controller::ParticleController;
use macroquad::math::Vec2;
use macroquad::{
    camera::Camera2D,
    color::Color,
    window::{screen_height, screen_width},
};

pub struct DangerZone {
    pub position_y: f32,
    pub velocity_y: f32,
    pub particle_controller: ParticleController,
}

const ZONE_COLOR: Color = Color {
    r: 0.8,
    g: 0.2,
    b: 0.2,
    a: 0.4,
};

const INITIAL_VERTICAL_SPEED: f32 = -20f32;
impl DangerZone {
    pub fn new() -> Self {
        return DangerZone {
            position_y: screen_height(),
            velocity_y: INITIAL_VERTICAL_SPEED,
            particle_controller: ParticleController::new(
                0.003,
                screen_width() * 0.1,
                screen_width() * 0.1,
                ZONE_COLOR,
                3.0,
            ),
        };
    }

    pub fn update(self: &mut Self, delta_time: f32, position_x: f32) {
        self.velocity_y *= 1f32 + 0.2 * delta_time;
        self.position_y += self.velocity_y * delta_time;
        self.particle_controller.update_with_range(
            delta_time,
            Vec2 {
                x: position_x - screen_width() * 0.5,
                y: self.position_y,
            },
            Vec2 {
                x: position_x + screen_width() * 0.5,
                y: self.position_y + screen_height() * 0.5,
            },
        );
    }

    pub fn draw(self: &Self, camera: &Camera2D) {
        self.particle_controller.draw(camera);
        // draw_rectangle(
        //     0f32,
        //     {
        //         let final_position_y: f32 = self.position_y - camera.target.y + camera.offset.y;
        //         if final_position_y < 0f32 {
        //             0f32
        //         } else {
        //             final_position_y
        //         }
        //     },
        //     screen_width(),
        //     screen_height(),
        //     RED,
        // );
    }
}

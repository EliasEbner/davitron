use crate::particle_controller::ParticleController;
use crate::player::Player;
use macroquad::math::Vec2;
use macroquad::{camera::Camera2D, color::Color};

pub struct DangerZone {
    pub position: Vec2,
    pub size: Vec2,
    pub velocity: Vec2,
    pub particle_controller: ParticleController,
}

const ZONE_COLOR: Color = Color {
    r: 0.8,
    g: 0.1,
    b: 0.1,
    a: 0.7,
};

impl DangerZone {
    pub fn new(position: Vec2, size: Vec2, velocity: Vec2, time_per_particle: f32) -> Self {
        return DangerZone {
            position: position,
            size: size,
            velocity: velocity,
            particle_controller: ParticleController::new(
                time_per_particle,
                size.max_element() * 0.03,
                size.max_element() * 0.03,
                ZONE_COLOR,
                3.0,
            ),
        };
    }

    pub fn update(self: &mut Self, delta_time: f32) {
        self.position.y += self.velocity.y * delta_time;

        self.particle_controller.update_with_range(
            delta_time,
            Vec2 {
                x: self.position.x - self.size.x * 0.5,
                y: self.position.y - self.size.y * 0.5,
            },
            Vec2 {
                x: self.position.x + self.size.x * 0.5,
                y: self.position.y + self.size.y * 0.5,
            },
        );

        self.particle_controller
            .shift_color(-0.4 * delta_time, 0f32, 0f32, 0f32);
    }

    pub fn update_as_bottom_zone(self: &mut Self, delta_time: f32, position_x: f32) {
        self.position.x = position_x;
        self.velocity.y *= 1f32 + 0.2 * delta_time;
        // if self.size.y > self.size.max_element() * 0.03 {
        //     self.position.y -= self.size.y + 90000f32 / self.velocity.y;
        //     self.size.y = 90000f32 / self.velocity.y.abs();
        // }
        self.update(delta_time);
    }
    pub fn update_as_side_zone(self: &mut Self, delta_time: f32, position_y: f32) {
        self.position.y = position_y;
        self.update(delta_time);
    }

    pub fn draw(self: &Self, camera: &Camera2D) {
        self.particle_controller.draw(camera);
    }

    pub fn check_and_handle_player_collision(self: &mut Self, player: &mut Player) {
        if self.position.x + self.size.x * 0.5 > player.position.x - player.radius
            && self.position.x - self.size.x * 0.5 < player.position.x + player.radius
            && self.position.y + self.size.y * 0.5 > player.position.y - player.radius
            && self.position.y - self.size.y * 0.5 < player.position.y + player.radius
        {
            player.is_dead = true;
            self.particle_controller.lifespan = 20f32;
            self.velocity.y = -600f32;
        }
    }
}

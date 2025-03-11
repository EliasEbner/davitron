use crate::particle_controller::ParticleController;
use macroquad::{camera::Camera2D, color::Color, math::Vec2};

const PLANET_COLOR: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 1.0,
    a: 0.3,
};

pub struct Planet {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub particle_controller: ParticleController,
}

impl Planet {
    pub fn new(position: Vec2, velocity: Vec2, radius: f32) -> Self {
        Self {
            radius,
            position,
            velocity,
            particle_controller: ParticleController::new(
                0.003,
                radius * 1.2,
                radius * 0.4,
                PLANET_COLOR,
                0.5,
            ),
        }
    }

    pub fn update(self: &mut Self, delta_time: f32) {
        self.particle_controller.update(delta_time, self.position);
        let change = self.velocity * delta_time;
        self.position += change;
        self.particle_controller.inherit_movement(change);
    }

    pub fn draw(self: &Self, camera: &Camera2D) {
        self.particle_controller.draw(camera);
    }

    pub fn handle_collistion(self: &mut Self, other: &mut Planet) {
        // position
        let angle: f32 =
            -(self.position.y - other.position.y).atan2(self.position.x - other.position.x);
        let sin_angle = angle.sin();
        let cos_angle = angle.cos();

        let mut old_pos_x = self.position.x;
        self.position.x = old_pos_x * cos_angle - self.position.y * sin_angle;
        self.position.y = old_pos_x * sin_angle + self.position.y * cos_angle;

        old_pos_x = other.position.x;
        other.position.x = old_pos_x * cos_angle - other.position.y * sin_angle;
        other.position.y = old_pos_x * sin_angle + other.position.y * cos_angle;

        let dist_per_mass: f32 =
            (-(self.position.x - other.position.x).abs() + self.radius + other.radius) / 2f32; // / (mass + e.mass);
        self.position.x += dist_per_mass; // * e.mass;
        other.position.x -= dist_per_mass; // * mass;

        old_pos_x = self.position.x;
        self.position.x = old_pos_x * cos_angle + self.position.y * sin_angle;
        self.position.y = old_pos_x * (-sin_angle) + self.position.y * cos_angle;

        old_pos_x = other.position.x;
        other.position.x = old_pos_x * cos_angle + other.position.y * sin_angle;
        other.position.y = old_pos_x * (-sin_angle) + other.position.y * cos_angle;

        // velocity
        let mut old_vel_x = self.velocity.x;
        self.velocity.x = old_vel_x * cos_angle - self.velocity.y * sin_angle;
        self.velocity.y = old_vel_x * sin_angle + self.velocity.y * cos_angle;

        old_vel_x = other.velocity.x;
        other.velocity.x = old_vel_x * cos_angle - other.velocity.y * sin_angle;
        other.velocity.y = old_vel_x * sin_angle + other.velocity.y * cos_angle;

        // let totalMass: f32 = mass + e.mass;
        // let factor: f32 = (mass - e.mass) / totalMass;
        // let prevVel: f32 = self.velocity.x;
        // vel.x = factor * vel.x + 2 * e.vel.x * e.mass / totalMass;
        // e.vel.x = 2 * prevVel * mass / totalMass - factor * e.vel.x;
        let prev_vel: f32 = self.velocity.x;
        self.velocity.x = other.velocity.x;
        other.velocity.x = prev_vel;

        old_vel_x = self.velocity.x;
        self.velocity.x = old_vel_x * cos_angle + self.velocity.y * sin_angle;
        self.velocity.y = old_vel_x * (-sin_angle) + self.velocity.y * cos_angle;

        old_vel_x = other.velocity.x;
        other.velocity.x = old_vel_x * cos_angle + other.velocity.y * sin_angle;
        other.velocity.y = old_vel_x * (-sin_angle) + other.velocity.y * cos_angle;
    }
}

use std::f32::consts::PI;

use crate::particle::Particle;
use crate::random_generator::get_rand_generator;
use macroquad::{camera::Camera2D, color::Color, math::Vec2, rand::RandGenerator};

pub struct ParticleController {
    pub particles: Vec<Particle>,
    pub spawn_timer: f32,
    pub time_per_particle: f32,
    pub initial_velocity: f32,
    pub initial_radius: f32,
    pub initial_color: Color,
    pub lifespan: f32,
    pub random_generator: RandGenerator,
}

impl ParticleController {
    pub fn new(
        time_per_particle: f32,
        initial_velocity: f32,
        initial_radius: f32,
        initial_color: Color,
        lifespan: f32,
    ) -> Self {
        Self {
            particles: Vec::new(),
            spawn_timer: time_per_particle,
            time_per_particle: time_per_particle,
            initial_velocity: initial_velocity,
            initial_radius: initial_radius,
            initial_color: initial_color,
            lifespan: lifespan,
            random_generator: get_rand_generator(),
        }
    }
    pub fn draw(self: &Self, camera: &Camera2D) {
        for i in 0..self.particles.len() {
            self.particles[i].draw(camera);
        }
    }
    pub fn inherit_movement(self: &mut Self, change: Vec2) {
        for i in 0..self.particles.len() {
            self.particles[i].position.x += change.x;
            self.particles[i].position.y += change.y;
        }
    }
    pub fn update(self: &mut Self, delta_time: f32, position: Vec2) {
        for i in 0..self.particles.len() {
            self.particles[i].update(delta_time);
        }
        self.particles.retain(|particle| particle.time_left > 0.0);
        self.spawn_timer -= delta_time;

        while self.spawn_timer <= 0.0 {
            self.spawn_timer += self.time_per_particle;
            self.spawn(position);
        }
    }
    pub fn update_with_range(
        self: &mut Self,
        delta_time: f32,
        from_position: Vec2,
        to_position: Vec2,
    ) {
        for i in 0..self.particles.len() {
            self.particles[i].update(delta_time);
        }
        self.particles.retain(|particle| particle.time_left > 0.0);
        self.spawn_timer -= delta_time;

        while self.spawn_timer <= 0.0 {
            self.spawn_timer += self.time_per_particle;
            self.spawn_in_range(from_position, to_position);
        }
    }
    pub fn spawn(self: &mut Self, position: Vec2) {
        let rand_angle = self.random_generator.gen_range(0.0, 2.0 * PI);
        self.particles.push(Particle::new(
            position,
            Vec2 {
                x: rand_angle.cos() * self.initial_velocity,
                y: rand_angle.sin() * self.initial_velocity,
            },
            self.initial_radius,
            self.initial_color,
            self.lifespan,
        ));
    }
    pub fn spawn_in_range(self: &mut Self, from_position: Vec2, to_position: Vec2) {
        let rand_angle = self.random_generator.gen_range(0.0, 2.0 * PI);
        self.particles.push(Particle::new(
            Vec2 {
                x: self
                    .random_generator
                    .gen_range(from_position.x, to_position.x),
                y: self
                    .random_generator
                    .gen_range(from_position.y, to_position.y),
            },
            Vec2 {
                x: rand_angle.cos() * self.initial_velocity,
                y: rand_angle.sin() * self.initial_velocity,
            },
            self.initial_radius,
            self.initial_color,
            self.lifespan,
        ));
    }
}

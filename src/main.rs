use macroquad::camera::Camera2D;
use macroquad::color::BLACK;
use macroquad::math::Vec2;
use macroquad::window::{
    clear_background, next_frame, screen_height, screen_width,
};

use crate::entity::Entity;
use crate::player::Player;

mod entity;
mod player;

#[macroquad::main("MyGame")]
async fn main() {
    let mut player: Player = Player::new(100f32);
    let mut camera: Camera2D = Camera2D::default();
    camera.offset = Vec2 {
        x: screen_width() * 0.5f32,
        y: screen_height() * 0.5f32,
    };

    loop {
        camera.offset = Vec2 {
            x: screen_width() * 0.5f32 + player.speed.x * 2f32,
            y: screen_height() * 0.5f32 + player.speed.y * 2f32,
        };

        camera.target = Vec2 {
            x: player.position.x,
            y: player.position.y,
        };
        player.update();
        clear_background(BLACK);
        player.draw(&camera);

        next_frame().await
    }
}

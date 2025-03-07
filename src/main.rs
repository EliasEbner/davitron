use macroquad::color::BLACK;
use macroquad::window::{
    clear_background, next_frame,
};

use crate::entity::Entity;
use crate::player::Player;

mod camera;
mod entity;
mod player;

#[macroquad::main("MyGame")]
async fn main() {
    let mut player: Player = Player::new(100f32);

    loop {
        player.update();
        clear_background(BLACK);
        player.draw();

        next_frame().await
    }
}

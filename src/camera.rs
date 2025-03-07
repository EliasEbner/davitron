use macroquad::{
    math::Vec2, miniquad::window::screen_size,
};

use crate::player::Player;

#[derive(Default)]
struct Camera {
    position: Vec2,
}

const PLAYER_SPEED_OFFSET: f32 = 2f32;
impl Camera {
    fn update(self: &mut Self, player: Player) {
        self.position.x = player.position.x
            + player.speed.x
                * PLAYER_SPEED_OFFSET
            + screen_size().0 * 0.5f32;

        self.position.y = player.position.y
            + player.speed.y
                * PLAYER_SPEED_OFFSET
            + screen_size().1 * 0.5f32;
    }
}

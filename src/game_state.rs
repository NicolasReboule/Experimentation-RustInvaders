use ggez::Context;
use crate::player::Player;
use crate::sprite::Sprite;

pub struct GameState {
    pub player : Player,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameState {
        // Load/create resources such as images here.
        GameState {
            player: Player::new("./assets/spaceship.png".to_string(), (1.0, 1.0), 50.0, 50.0, (0.0, 0.0))
        }
    }
}
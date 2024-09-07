use super::game_controller::GamePad;

#[derive(Debug)]
pub enum Message {
    GamePad(GamePad)
}
use super::game_controller::GamePad;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    GamePad(GamePad),
    GetSerial
}
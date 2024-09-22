use super::game_controller::interface::Controller;

#[derive(Debug, Clone)]
pub enum Message {
    MainLoop(Controller),
    GetSerial,
    FileSelect1(String),
    FileSelect2(String)
}
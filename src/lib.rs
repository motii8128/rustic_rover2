mod game_controller;
mod thread;
mod message;

use iced;
use message::Message;

pub struct RusticRover2
{
    gamepad_manager: game_controller::GamePadManager
}

impl iced::Application for RusticRover2 {
    type Executor = iced::executor::Default;
    type Message = message::Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self{
                gamepad_manager : game_controller::GamePadManager::new()
            },
            iced::Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("RusticRover2")
    }

    fn theme(&self) -> Self::Theme {
        iced::theme::Theme::KanagawaWave
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::unfold(
            "gamepad_subscription", 
            self.gamepad_manager.node.get_subscriber(), 
            move |mut subscriber| async move{
                let get = subscriber.as_mut().unwrap().recv().await.unwrap();

                (Message::GamePad(get), subscriber)
            })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::GamePad(get)=>{

                iced::Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        iced::widget::text("TEE").into()
    }
}
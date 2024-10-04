mod game_controller;
mod thread;
mod message;
mod serial;
mod view_utils;
mod packet_creator;

use iced;
use message::Message;
use packet_creator::PacketManager;
use serial::SerialManager;

pub struct RusticRover2
{
    gamepad_manager : game_controller::ControllerManager,
    packet_manager : packet_creator::PacketManager,
    serial_manager : SerialManager
}

impl iced::Application for RusticRover2 {
    type Executor = iced::executor::Default;
    type Message = message::Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        
        let mut gm = game_controller::ControllerManager::new();
        gm.scan_device();

        gm.spawn_driver(game_controller::interface::ControllerConnectionType::SERIAL);
        gm.spawn_driver2(game_controller::interface::ControllerConnectionType::SERIAL);

        let mut pm = PacketManager::new();
        pm.search_data_files();
        

        (
            Self{
                gamepad_manager : gm,
                packet_manager : pm,
                serial_manager : SerialManager::new()
            },
            iced::Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("RusticRover2")
    }

    fn theme(&self) -> Self::Theme {
        iced::theme::Theme::TokyoNight
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::unfold(
            "gamepad_subscription", 
            self.gamepad_manager.controller_1.get_subscriber(), 
            move |mut subscriber| async move{
                let get = subscriber.as_mut().unwrap().recv().await.unwrap();

                (Message::MainLoop(get), subscriber)
            })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::MainLoop(check)=>{
                self.gamepad_manager.get_value_1 = check;
                self.gamepad_manager.get_value_2 = self.gamepad_manager.controller_2.subscribe();

                self.packet_manager.creator_1.create(self.gamepad_manager.get_value_1);
                self.packet_manager.creator_2.create(self.gamepad_manager.get_value_2);

                self.serial_manager.manage1(self.packet_manager.creator_1.new_packet);
                self.serial_manager.manage2(self.packet_manager.creator_2.new_packet);
            }
            Message::GetSerial=>{
                self.serial_manager.scan_available();
            }
            Message::FileSelect1(name)=>{
                self.packet_manager.yaml_list1.selected = Some(name);
                self.packet_manager.creator_1.load_from_yaml(self.packet_manager.get_selected_file1());
            }
            Message::FileSelect2(name)=>{
                self.packet_manager.yaml_list2.selected = Some(name);
                self.packet_manager.creator_2.load_from_yaml(self.packet_manager.get_selected_file2());
            }
        }

        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {

        let title_image = view_utils::path_to_image("./rr2.png", 300);
        let row = self.packet_manager.packet_view();
        let scan_button = iced::widget::button("Scan available serialport").width(400).height(50).on_press(Message::GetSerial);

        let serial1 = if self.serial_manager.node1_state
        {
            let text = iced::widget::text(self.serial_manager.node1_type.clone()).size(30);
            let image = view_utils::path_to_image("./assets/micro_controller.png", 250);

            iced::widget::column![image, text]
        }
        else
        {
            let text = iced::widget::text("Waiting Connection ...").size(30);

            iced::widget::column![text]
        };

        let serial2 = if self.serial_manager.node2_state
        {
            let text = iced::widget::text(self.serial_manager.node2_type.clone()).size(30);
            let image = view_utils::path_to_image("./assets/micro_controller.png", 250);

            iced::widget::column![image, text]
        }
        else
        {
            let text = iced::widget::text("Waiting Connection ...").size(30);

            iced::widget::column![text]
        };

        let serial_row = iced::widget::row![serial1, serial2].spacing(60);

        iced::widget::column![title_image, row, scan_button, serial_row].spacing(30).align_items(iced::Alignment::Center).into()
    }
}
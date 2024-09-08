mod game_controller;
mod thread;
mod message;
mod serial;

use iced;
use message::Message;
use serial::SerialManager;

pub struct RusticRover2
{
    gamepad_manager: game_controller::GamePadManager,
    gamepad_battery : u8,
    x : i32,
    y : i32,
    rotation : i32,
    m1 : i32,
    m2 : i32,
    m3 : i32,
    serial_manager : serial::SerialManager,
    wheel_ready : bool,
    machine_ready : bool
}

impl iced::Application for RusticRover2 {
    type Executor = iced::executor::Default;
    type Message = message::Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mut gamepad_manager = game_controller::GamePadManager::new();
        gamepad_manager.spawn_gamepad_driver();

        (
            Self{
                gamepad_manager : gamepad_manager,
                gamepad_battery : 0,
                x : 0,
                y : 0,
                rotation : 0,
                m1 : 0,
                m2 : 0,
                m3 : 0,
                serial_manager : SerialManager::new(),
                wheel_ready : false,
                machine_ready : false
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
            self.gamepad_manager.node.get_subscriber(), 
            move |mut subscriber| async move{
                let get = subscriber.as_mut().unwrap().recv().await.unwrap();

                (Message::GamePad(get), subscriber)
            })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::GamePad(get)=>{
                if self.serial_manager.check_ready_to_serial()
                {
                    if !self.serial_manager.wheel_is_spawned
                    {
                        self.serial_manager.spawn_wheel();
                        self.wheel_ready = true;
                    }
                    else {
                        self.serial_manager.spawn_machine();
                        self.machine_ready = true;
                    }
                }

                self.x = (get.joy_stick.left_x * 100.0) as i32;
                self.y = (get.joy_stick.left_y * 100.0) as i32;
                self.rotation = (get.joy_stick.right_x * 100.0) as i32;
                if get.buttons.l1
                {
                    self.m1 = 100;
                }
                else if get.buttons.l2 == 1.0
                {
                    self.m1 = -100;
                }
                else {
                    self.m1 = 0;
                }

                if get.buttons.r1
                {
                    self.m2 = 100;
                }
                else if get.buttons.r2 == 1.0
                {
                    self.m2 = -100;
                }
                else {
                    self.m2 = 0;
                }

                self.m3 = (get.dpad.y * 100.0) as i32;
                self.gamepad_battery = get.battery;

                let mut new_packet = serial::Packet::new();
                new_packet.x = self.x;
                new_packet.y = self.y;
                new_packet.rotation = self.rotation;
                new_packet.m1 = self.m1;
                new_packet.m2 = self.m2;
                new_packet.m3 = self.m3;

                self.serial_manager.exec_publisher(new_packet);
                iced::Command::none()
            }
            Message::GetSerial=>{
                self.serial_manager.scan_available();

                iced::Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let battery_title = iced::widget::text("Controller Battery").size(50);
        let batt = iced::widget::ProgressBar::new(0.0..=100.0, self.gamepad_battery as f32).height(50).width(700);
        let battery = iced::widget::text(format!("{}%", self.gamepad_battery)).size(50);
        let battery_row = iced::widget::row![battery, batt].align_items(iced::Alignment::Center);
        let battery_info = iced::widget::column![battery_title, battery_row].align_items(iced::Alignment::Center);

        let command_string = format!("x:{}\ny:{}\nrotation:{}\nm1:{}\nm2:{}\nm3:{}",
            self.x,
            self.y,
            self.rotation,
            self.m1,
            self.m2,
            self.m3);

        let command_text = iced::widget::text(command_string).size(50);

        let scan_button: iced::widget::Button<'_, Message, _, _> = iced::widget::button(iced::widget::text("Scan Serial").size(50)).on_press(Message::GetSerial).height(250).width(250);

        if self.wheel_ready
        {
            let controller_clm = iced::widget::column![battery_info, command_text, scan_button];
            let wi = path_to_image("./assets/wheel_ok.png", 500);
            
            iced::widget::row![controller_clm, wi].spacing(30).align_items(iced::Alignment::Start).into()
        }
        else if self.wheel_ready && self.machine_ready
        {
            let controller_clm = iced::widget::column![battery_info, command_text, scan_button];
            let wi = path_to_image("./assets/wheel_ok.png", 500);
            let mi = path_to_image("./assets/machine_ok.png", 500);
            let image_clm = iced::widget::column![wi, mi].spacing(10);
            
            iced::widget::row![controller_clm, image_clm].spacing(30).align_items(iced::Alignment::Start).into()
        }
        else {
            iced::widget::column![battery_info, command_text, scan_button].into()
        }        
    }
}

pub fn path_to_image(path:&str, size:u16)->iced::widget::Image<iced::widget::image::Handle>
{
    iced::widget::image::Image::new(iced::widget::image::Handle::from_path(path)).width(size).height(size)
}
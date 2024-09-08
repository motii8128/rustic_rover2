use gilrs::{Axis, Button, Event, Gilrs, Gamepad as GP};

use super::thread::AsyncNode;

pub struct GamePadManager
{
    pub node : AsyncNode<GamePad>,
}

impl GamePadManager {
    pub fn new()->GamePadManager
    {
        GamePadManager { node: AsyncNode::<GamePad>::new() }
    }
    pub fn spawn_gamepad_driver(&mut self)
    {
        let mut gilrs = Gilrs::new().unwrap();
        let mut active_gamepad = None;
        let publisher = self.node.get_publisher();

        std::thread::spawn(move ||{
            loop {
                while let Some(Event{id , ..}) = gilrs.next_event()
                {
                    active_gamepad = Some(id)
                }

                if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)){
                    let get_value = GamePad::from_gilrs(gamepad);

                    let _ = publisher.send(get_value);
                }
            }
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GamePad
{
    pub joy_stick:JoyStick,
    pub buttons:Buttons,
    pub dpad:DPad,
    pub battery:u8
}

impl GamePad {
    pub fn new()->GamePad
    {
        GamePad { joy_stick: JoyStick::new(), buttons: Buttons::new(), dpad: DPad::new() , battery:0}
    }
    pub fn from_gilrs(input:GP)->GamePad
    {
        let mut output = GamePad::new();

        if input.is_pressed(Button::East)
        {
            output.buttons.east = true
        }
        if input.is_pressed(Button::South)
        {
            output.buttons.south = true
        }
        if input.is_pressed(Button::North)
        {
            output.buttons.north = true
        }
        if input.is_pressed(Button::West)
        {
            output.buttons.west = true
        }
        if input.is_pressed(Button::RightTrigger)
        {
            output.buttons.r1 = true
        }
        if input.is_pressed(Button::LeftTrigger)
        {
            output.buttons.l1 = true
        }
        if input.is_pressed(Button::RightThumb)
        {
            output.buttons.right_thumb = true
        }
        if input.is_pressed(Button::LeftThumb)
        {
            output.buttons.left_thumb = true
        }
        if input.is_pressed(Button::DPadUp)
        {
            output.dpad.y = 1.0
        }
        if input.is_pressed(Button::DPadDown)
        {
            output.dpad.y = -1.0
        }
        if input.is_pressed(Button::DPadRight)
        {
            output.dpad.x = 1.0
        }
        if input.is_pressed(Button::DPadLeft)
        {
            output.dpad.x = -1.0
        }
        if input.is_pressed(Button::LeftTrigger2)
        {
            output.buttons.l2 = 1.0
        }
        if input.is_pressed(Button::RightTrigger2)
        {
            output.buttons.r2 = 1.0
        }
        match input.axis_data(Axis::LeftStickX) {
            Some(data)=>{
                output.joy_stick.left_x = data.value();
                if output.joy_stick.left_x.abs() < 0.05
                {
                    output.joy_stick.left_x = 0.0;
                }

                if output.joy_stick.left_x > 0.991
                {
                    output.joy_stick.left_x = 1.0;
                }
                else if output.joy_stick.left_x < -0.990
                {
                    output.joy_stick.left_x = -1.0
                }
            }
            None=>{

            }
        }
        match input.axis_data(Axis::LeftStickY) {
            Some(data)=>{
                output.joy_stick.left_y = data.value();
                if output.joy_stick.left_y.abs() < 0.05
                {
                    output.joy_stick.left_y = 0.0;
                }
                if output.joy_stick.left_y > 0.991
                {
                    output.joy_stick.left_y = 1.0;
                }
                else if output.joy_stick.left_y < -0.990
                {
                    output.joy_stick.left_y = -1.0
                }
            }
            None=>{

            }
        }
        match input.axis_data(Axis::RightStickX) {
            Some(data)=>{
                output.joy_stick.right_x = data.value();
                if output.joy_stick.right_x.abs() < 0.05
                {
                    output.joy_stick.right_x = 0.0;
                }
                if output.joy_stick.right_x > 0.991
                {
                    output.joy_stick.right_x = 1.0;
                }
                else if output.joy_stick.right_x < -0.990
                {
                    output.joy_stick.right_x = -1.0
                }
            }
            None=>{

            }
        }
        match input.axis_data(Axis::RightStickY) {
            Some(data)=>{
                output.joy_stick.right_y = data.value();
                if output.joy_stick.right_y.abs() < 0.05
                {
                    output.joy_stick.right_y = 0.0;
                }
                if output.joy_stick.right_y > 0.991
                {
                    output.joy_stick.right_y = 1.0;
                }
                else if output.joy_stick.right_y < -0.990
                {
                    output.joy_stick.right_y = -1.0
                }
            }
            None=>{

            }
        }

        if let gilrs::PowerInfo::Discharging(lvl) = input.power_info()
        {
            output.battery = lvl   
        }


        output
    }
}

#[derive(Debug,Clone, Copy)]
pub struct JoyStick
{
    pub left_x:f32,
    pub left_y:f32,
    pub right_x:f32,
    pub right_y:f32
}

impl JoyStick {
    pub fn new()->JoyStick
    {
        JoyStick { left_x: 0.0, left_y: 0.0, right_x: 0.0, right_y: 0.0 }
    }
}

#[derive(Debug,Clone,Copy)]
pub struct Buttons
{
    pub east:bool,
    pub north:bool,
    pub south:bool,
    pub west:bool,
    pub r1:bool,
    pub l1:bool,
    pub r2:f32,
    pub l2:f32,
    pub left_thumb:bool,
    pub right_thumb:bool
}

impl Buttons {
    pub fn new()->Buttons
    {
        Buttons { east: false, north: false, south: false, west: false, r1: false, l1: false, r2: 0.0, l2: 0.0, left_thumb: false, right_thumb: false }
    }
}

#[derive(Debug,Clone, Copy)]
pub struct DPad
{
    pub x:f32,
    pub y:f32
}

impl DPad {
    pub fn new()->DPad
    {
        DPad { x: 0.0, y: 0.0 }
    }
}
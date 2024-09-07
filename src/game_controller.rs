use gilrs::{Gilrs, Axis, Button};

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
}

#[derive(Debug)]
pub struct GamePad
{
    pub joy_stick:JoyStick,
    pub buttons:Buttons,
    pub dpad:DPad
}

impl GamePad {
    pub fn new()->GamePad
    {
        GamePad { joy_stick: JoyStick::new(), buttons: Buttons::new(), dpad: DPad::new() }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
use std::io::Read;
use yaml_rust::YamlLoader;

use super::game_controller::interface::Controller;
use super::view_utils::ComboBox;
use super::message::Message;

pub struct PacketManager
{
    pub creator_1 : PacketCreator,
    pub creator_2 : PacketCreator,
    pub yaml_list1 : ComboBox<String>,
    pub yaml_list2 : ComboBox<String>,
    pub packet1 : Packet,
    pub packet2 : Packet
}
impl PacketManager {
    pub fn new()->PacketManager
    {
        let cb = ComboBox::new(Vec::<String>::new());
        let cb2 = ComboBox::new(Vec::<String>::new());
        PacketManager { creator_1: PacketCreator::new(), creator_2: PacketCreator::new() , yaml_list1 : cb, yaml_list2 : cb2, packet1 : Packet::new(), packet2 : Packet::new()}
    }
    pub fn search_data_files(&mut self)
    {
        let mut get_strings = Vec::<String>::new();
        match std::fs::read_dir("./config")
        {
            Ok(entries)=>{
                for i in entries
                {
                    let file = i.unwrap().file_name();
                    get_strings.push(file.into_string().unwrap());
                }

                self.yaml_list1 = ComboBox::new(get_strings.clone());
                self.yaml_list2 = ComboBox::new(get_strings.clone());
            }
            Err(_e)=>{
            }
        }
    }
    pub fn get_selected_file1(&self)->String
    {
        match self.yaml_list1.selected.clone() {
            Some(select)=>{
                select.clone()
            }
            None=>{
                String::from("default.yaml")
            }
        }
    }
    pub fn get_selected_file2(&self)->String
    {
        match self.yaml_list2.selected.clone() {
            Some(select)=>{
                select.clone()
            }
            None=>{
                String::from("default.yaml")
            }
        }
    }
    pub fn packet_view(&self)->iced::widget::Row<Message>
    {
        let file_select1 = iced::widget::combo_box(
            &self.yaml_list1.all, 
            "Select Config file", 
            self.yaml_list1.selected.as_ref(), 
            Message::FileSelect1);
        let p1_info_str = format!(
            "ID: {}\n1.{}&{} : {}\n2.{}&{} : {}\n3.{}&{} : {}\n4.{}&{} : {}\n5.{}&{} : {}", 
            self.packet1.id, 
            self.creator_1.value_1_assign.plus_assign, self.creator_1.value_1_assign.minus_assign, self.packet1.x,
            self.creator_1.value_2_assign.plus_assign, self.creator_1.value_2_assign.minus_assign, self.packet1.y,
            self.creator_1.value_3_assign.plus_assign, self.creator_1.value_3_assign.minus_assign, self.packet1.rotation,
            self.creator_1.value_4_assign.plus_assign, self.creator_1.value_4_assign.minus_assign, self.packet1.m1,
            self.creator_1.value_5_assign.plus_assign, self.creator_1.value_5_assign.minus_assign, self.packet1.m2);

        let p1_info_t = iced::widget::text(p1_info_str).size(30);

        let p1_info = iced::widget::column![file_select1, p1_info_t];

        let file_select2 = iced::widget::combo_box(
            &self.yaml_list2.all, 
            "Select Config file", 
            self.yaml_list2.selected.as_ref(), 
            Message::FileSelect2);
        let p2_info_str = format!(
            "ID: {}\n1.{}&{} : {}\n2.{}&{} : {}\n3.{}&{} : {}\n4.{}&{} : {}\n5.{}&{} : {}", 
            self.packet2.id, 
            self.creator_2.value_1_assign.plus_assign, self.creator_2.value_1_assign.minus_assign,self.packet2.x,
            self.creator_2.value_2_assign.plus_assign, self.creator_2.value_2_assign.minus_assign,self.packet2.y,
            self.creator_2.value_3_assign.plus_assign, self.creator_2.value_3_assign.minus_assign,self.packet2.rotation,
            self.creator_2.value_4_assign.plus_assign, self.creator_2.value_4_assign.minus_assign,self.packet2.m1,
            self.creator_2.value_5_assign.plus_assign, self.creator_2.value_5_assign.minus_assign,self.packet2.m2);

        let p2_info_t = iced::widget::text(p2_info_str).size(30);

        let p2_info = iced::widget::column![file_select2, p2_info_t];

        iced::widget::row![p1_info, p2_info]
    }
}

pub struct PacketCreator
{
    id_1 : usize,
    id_2 : usize,
    value_1_assign : ControllerAssigns,
    value_2_assign : ControllerAssigns,
    value_3_assign : ControllerAssigns,
    value_4_assign : ControllerAssigns,
    value_5_assign : ControllerAssigns,
    pub value_1_rate:u16,
    pub value_2_rate:u16,
    pub value_3_rate:u16,
    pub value_4_rate:u16,
    pub value_5_rate:u16,
    pub value_1_smooth : bool,
    pub value_2_smooth : bool,
    pub value_3_smooth : bool,
    pub value_4_smooth : bool,
    pub value_5_smooth : bool,
}
impl PacketCreator {
    pub fn new()->PacketCreator
    {
        PacketCreator {
            id_1 : 0,
            id_2 : 0,
            value_1_assign: ControllerAssigns::new(AssignController::JoyLeftY, AssignController::JoyLeftY), 
            value_2_assign: ControllerAssigns::new(AssignController::JoyRightY, AssignController::JoyRightY), 
            value_3_assign: ControllerAssigns::new(AssignController::BtnR1, AssignController::BtnR2), 
            value_4_assign: ControllerAssigns::new(AssignController::DPadUp, AssignController::DPadDown), 
            value_5_assign: ControllerAssigns::new(AssignController::BtnL1, AssignController::BtnL2),
            value_1_rate : 100,
            value_2_rate : 100,
            value_3_rate : 100,
            value_4_rate : 100,
            value_5_rate : 100,
            value_1_smooth : true,
            value_2_smooth : true,
            value_3_smooth : false,
            value_4_smooth : false,
            value_5_smooth : false
        }
    }
    pub fn load_from_yaml(&mut self, f_name_:String)
    {
        let file_name = format!("./config/{}", f_name_);
        match std::fs::File::open(file_name)
        {
            Ok(mut f)=>{
                let mut content = String::new();

                let _ = f.read_to_string(&mut content);

                match YamlLoader::load_from_str(&content)
                {
                    Ok(docs)=>{
                        let doc = &docs[0];

                        self.value_1_assign.plus_assign = AssignController::from_str(doc["/**"]["x"]["plus"].as_str().unwrap());
                        self.value_1_assign.minus_assign = AssignController::from_str(doc["/**"]["x"]["minus"].as_str().unwrap());
                        self.value_1_rate = doc["/**"]["x"]["rate"].as_i64().unwrap() as u16;
                        self.value_1_smooth = doc["/**"]["x"]["smooth"].as_bool().unwrap();
                        
                        self.value_2_assign.plus_assign = AssignController::from_str(doc["/**"]["y"]["plus"].as_str().unwrap());
                        self.value_2_assign.minus_assign = AssignController::from_str(doc["/**"]["y"]["minus"].as_str().unwrap());
                        self.value_2_rate = doc["/**"]["y"]["rate"].as_i64().unwrap() as u16;
                        self.value_2_smooth = doc["/**"]["y"]["smooth"].as_bool().unwrap();

                        self.value_3_assign.plus_assign = AssignController::from_str(doc["/**"]["rotation"]["plus"].as_str().unwrap());
                        self.value_3_assign.minus_assign = AssignController::from_str(doc["/**"]["rotation"]["minus"].as_str().unwrap());
                        self.value_3_rate = doc["/**"]["rotation"]["rate"].as_i64().unwrap() as u16;
                        self.value_3_smooth = doc["/**"]["rotation"]["smooth"].as_bool().unwrap();
                        
                        self.value_4_assign.plus_assign = AssignController::from_str(doc["/**"]["m1"]["plus"].as_str().unwrap());
                        self.value_4_assign.minus_assign = AssignController::from_str(doc["/**"]["m1"]["minus"].as_str().unwrap());
                        self.value_4_rate = doc["/**"]["m1"]["rate"].as_i64().unwrap() as u16;
                        self.value_4_smooth = doc["/**"]["m1"]["smooth"].as_bool().unwrap();

                        self.value_5_assign.plus_assign = AssignController::from_str(doc["/**"]["m2"]["plus"].as_str().unwrap());
                        self.value_5_assign.minus_assign = AssignController::from_str(doc["/**"]["m2"]["minus"].as_str().unwrap());
                        self.value_5_rate = doc["/**"]["m2"]["rate"].as_i64().unwrap() as u16;
                        self.value_5_smooth = doc["/**"]["m2"]["smooth"].as_bool().unwrap();

                        self.id_1 = doc["/**"]["id"].as_i64().unwrap() as usize;
                        self.id_2 = doc["/**"]["second"].as_i64().unwrap() as usize;
                    }
                    Err(_e)=>{

                    }
                }
            }
            Err(_e)=>{
                println!("Failed to open config file")
            }
        }
    }
    pub fn create(&self, input : Controller)->Packet
    {
        let mut new_packet = Packet::new();

        new_packet.x = (assign_to_controller(self.value_1_assign, input) * self.value_1_rate as f32) as i32;
        new_packet.y = (assign_to_controller(self.value_2_assign, input) * self.value_2_rate as f32) as i32;
        new_packet.rotation = (assign_to_controller(self.value_3_assign, input) * self.value_3_rate as f32) as i32;
        new_packet.m1 = (assign_to_controller(self.value_4_assign, input) * self.value_4_rate as f32) as i32;
        new_packet.m2 = (assign_to_controller(self.value_5_assign, input) * self.value_5_rate as f32) as i32;

        new_packet.value_1_smooth = self.value_1_smooth;
        new_packet.value_2_smooth = self.value_2_smooth;
        new_packet.value_3_smooth = self.value_3_smooth;
        new_packet.value_4_smooth = self.value_4_smooth;
        new_packet.value_5_smooth = self.value_5_smooth;

        if !input.btns.circle
        {
            new_packet.id = self.id_1
        }
        else {
            new_packet.id = self.id_2
        }


        new_packet
    }
}

fn assign_to_controller(cb:ControllerAssigns, input:Controller)->f32
{
    match cb.plus_assign {
        AssignController::JoyLeftX=>input.sticks.left_x,
        AssignController::JoyLeftY=>input.sticks.left_y,
        AssignController::JoyRightX=>input.sticks.right_x,
        AssignController::JoyRightY=>input.sticks.right_y,
        _=>{
            if assign_btns(cb.plus_assign, input)
            {
                1.0
            }
            else if assign_btns(cb.minus_assign, input)
            {
                -1.0
            }
            else {
                0.0
            }
        }
    }
}

fn assign_btns(assign:AssignController, input:Controller)->bool
{
    match assign {
        AssignController::BtnCircle=>input.btns.circle,
        AssignController::BtnCross=>input.btns.cross,
        AssignController::BtnCube=>input.btns.cube,
        AssignController::BtnTriangle=>input.btns.triangle,
        AssignController::BtnL1=>input.btns.l1,
        AssignController::BtnL2=>input.btns.l2,
        AssignController::BtnR1=>input.btns.r1,
        AssignController::BtnR2=>input.btns.r2,
        AssignController::DPadUp=>input.dpad.up_key,
        AssignController::DPadDown=>input.dpad.down_key,
        AssignController::DPadLeft=>input.dpad.left_key,
        AssignController::DPadRight=>input.dpad.right_key,
        _=> false
    }
}

#[derive(Clone, Copy)]
pub struct ControllerAssigns
{
    plus_assign : AssignController,
    minus_assign : AssignController
}
impl ControllerAssigns {
    pub fn new(plus : AssignController, minus : AssignController)->ControllerAssigns
    {
        ControllerAssigns{plus_assign : plus, minus_assign : minus}
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum AssignController {
    JoyLeftX,
    JoyLeftY,
    JoyRightX,
    JoyRightY,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
    BtnCircle,
    BtnCross,
    BtnTriangle,
    BtnCube,
    BtnL1,
    BtnR1,
    BtnL2,
    BtnR2,
}
impl std::fmt::Display for AssignController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AssignController::JoyLeftX=>"Left_Stick_X",
                AssignController::JoyLeftY=>"Left_Stick_Y",
                AssignController::JoyRightX=>"Right_Stick_X",
                AssignController::JoyRightY=>"Right_Stick_Y",
                AssignController::DPadUp=>"Up_Key",
                AssignController::DPadDown=>"Down_Key",
                AssignController::DPadLeft=>"Left_Key",
                AssignController::DPadRight=>"Right_Key",
                AssignController::BtnCircle=>"Circle_Button",
                AssignController::BtnCross=>"Cross_Button",
                AssignController::BtnCube=>"Cube_Button",
                AssignController::BtnTriangle=>"Triangle_Button",
                AssignController::BtnL1=>"L1_Button",
                AssignController::BtnL2=>"L2_Button",
                AssignController::BtnR1=>"R1_Button",
                AssignController::BtnR2=>"R2_Button",
            }
        )
    }
}
impl AssignController {
    // pub const ALL:[AssignController;16]=[
    //     AssignController::JoyLeftX,
    //     AssignController::JoyLeftY,
    //     AssignController::JoyRightX,
    //     AssignController::JoyRightY,
    //     AssignController::BtnCircle,
    //     AssignController::BtnCross,
    //     AssignController::BtnCube,
    //     AssignController::BtnTriangle,
    //     AssignController::BtnL1,
    //     AssignController::BtnL2,
    //     AssignController::BtnR1,
    //     AssignController::BtnR2,
    //     AssignController::DPadUp,
    //     AssignController::DPadDown,
    //     AssignController::DPadRight,
    //     AssignController::DPadLeft
    // ];
    pub fn from_str(str:&str)->AssignController
    {
        match str {
            "Left_Stick_X"=>AssignController::JoyLeftX,
            "Left_Stick_Y"=>AssignController::JoyLeftY,
            "Right_Stick_X"=>AssignController::JoyRightX,
            "Right_Stick_Y"=>AssignController::JoyRightY,
            "Up_Key"=>AssignController::DPadUp,
            "Down_Key"=>AssignController::DPadDown,
            "Left_Key"=>AssignController::DPadLeft,
            "Right_Key"=>AssignController::DPadRight,
            "Circle_Button"=>AssignController::BtnCircle,
            "Cross_Button"=>AssignController::BtnCross,
            "Cube_Button"=>AssignController::BtnCube,
            "Triangle_Button"=>AssignController::BtnTriangle,
            "L1_Button"=>AssignController::BtnL1,
            "L2_Button"=>AssignController::BtnL2,
            "R1_Button"=>AssignController::BtnR1,
            "R2_Button"=>AssignController::BtnR2,
            _=>AssignController::BtnCube
        }
    }
}

#[derive(Debug,Clone, Copy)]
pub struct Packet
{
    pub id : usize,
    pub x : i32,
    pub y : i32,
    pub rotation : i32,
    pub m1 : i32,
    pub m2 : i32,
    pub value_1_smooth : bool,
    pub value_2_smooth : bool,
    pub value_3_smooth : bool,
    pub value_4_smooth : bool,
    pub value_5_smooth : bool,
}

impl Packet {
    pub fn new()->Packet
    {
        Packet {id : 0, x: 0, y: 0, rotation: 0, m1: 0, m2: 0, value_1_smooth : true, value_2_smooth : true, value_3_smooth : true, value_4_smooth : true, value_5_smooth : true}
    }
}
pub mod dualsense;
pub mod dualshock4;
pub mod interface;

extern crate hidapi;
use hidapi::{HidApi, DeviceInfo};
use interface::{Controller, ControllerConnectionType, ControllerName};

use super::thread;

pub struct ControllerManager
{
    pub controller_names:Vec<ControllerName>,
    pub controller_1 : thread::AsyncNode<Controller>,
    pub controller_2 : thread::Node<Controller>,
    pub controller_num:usize,
    pub device_list:Vec<DeviceInfo>,
    api:HidApi,
    pub get_value_1:Controller,
    pub get_value_2:Controller
}

impl ControllerManager {
    pub fn new()->ControllerManager
    {
        ControllerManager {
            controller_names: Vec::<ControllerName>::new(),
            controller_1 : thread::AsyncNode::new(),
            controller_2 : thread::Node::new(),
            controller_num:0, 
            device_list: Vec::<DeviceInfo>::new(), 
            api: HidApi::new().unwrap() ,
            get_value_1 : Controller::new(),
            get_value_2 : Controller::new()
        }
    }

    pub fn scan_device(&mut self)
    {
        self.api = HidApi::new().unwrap();
        let mut dev_vec = Vec::<DeviceInfo>::new();
        for i in self.api.device_list()
        {
            if i.vendor_id() == 1356 && i.product_id() == 2508
            {
                let s = i.clone();
                dev_vec.push(s);
                self.controller_names.push(ControllerName::DualShock4);
            }
            else if i.vendor_id() == 1356 && i.product_id() == 3302
            {
                let s = i.clone();
                dev_vec.push(s);
                self.controller_names.push(ControllerName::DualSense);
            }
        }

        for i in &dev_vec
        {
            println!("{:?}", i)
        }

        self.controller_num = dev_vec.clone().len();
        self.device_list = dev_vec;
        println!("{}", self.controller_num)
    }

    pub fn spawn_driver(&mut self, mode_:ControllerConnectionType)
    {
        let publisher_ = self.controller_1.get_publisher();
        match self.device_list.first()
        {
            Some(dr)=>{
                match dr.open_device(&self.api) {
                    Ok(device_)=>{
                        match dr.product_id()
                        {
                            2508=>{
                                let mut controller = dualshock4::DualShock4Driver{device:device_, mode:mode_, buf:[0_u8;256], result:Controller::new()};

                                std::thread::spawn(move ||{
                                    loop {
                                        let get = controller.task();

                                        let _ = publisher_.send(get);
                                    }
                                });
                            }
                            3302=>{
                                let mut controller = dualsense::DualSenseDriver{device:device_, mode:mode_};

                                std::thread::spawn(move ||{
                                    loop {
                                        let get = controller.task();

                                        let _ = publisher_.send(get);
                                    }
                                });
                            }
                            _=>{

                            }
                        }
                    }
                    Err(_e)=>{

                    }
                }
            }
            None=>{
            }


        }
        self.device_list.remove(0);
    }
    pub fn spawn_driver2(&mut self, mode_:ControllerConnectionType)
    {
        let publisher_ = self.controller_2.get_publisher();
        let api = HidApi::new().unwrap();
        match self.device_list.first()
        {
            Some(dr)=>{
                match dr.open_device(&api) {
                    Ok(device_)=>{
                        match dr.product_id()
                        {
                            2508=>{
                                let mut controller = dualshock4::DualShock4Driver{device:device_, mode:mode_, buf:[0_u8;256], result:Controller::new()};

                                std::thread::spawn(move ||{
                                    loop {
                                        let get = controller.task();

                                        let _ = publisher_.send(get);
                                    }
                                });
                            }
                            3302=>{
                                let mut controller = dualsense::DualSenseDriver{device:device_, mode:mode_};

                                std::thread::spawn(move ||{
                                    loop {
                                        let get = controller.task();

                                        let _ = publisher_.send(get);
                                    }
                                });
                            }
                            _=>{

                            }
                        }
                    }
                    Err(_e)=>{

                    }
                }
            }
            None=>{
            }


        }
        self.device_list.remove(0);
    }
}
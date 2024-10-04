use serialport::{self, SerialPortInfo};

use super::thread::Node;
use super::packet_creator::Packet;

pub struct SerialManager
{
    available_ports : Vec<String>,
    available_port_info : Vec<SerialPortInfo>,
    pub node1 : Node<Packet>,
    pub node_checker1 : Node<bool>,
    pub node1_state : bool,
    pub node1_port : String,
    pub node1_type : String,
    pub node_checker2 : Node<bool>,
    pub node2 : Node<Packet>,
    pub node2_state : bool,
    pub node2_port : String,
    pub node2_type : String,
}

impl SerialManager {
    pub fn new()->SerialManager
    {
        SerialManager {
            available_ports: Vec::<String>::new() , 
            available_port_info : Vec::new(), 
            node1 : Node::new(), 
            node2 : Node::new(),
            node1_state : false,
            node2_state : false,
            node_checker1 : Node::new(),
            node_checker2 : Node::new(),
            node1_port : String::new(),
            node2_port : String::new(),
            node1_type : String::new(),
            node2_type : String::new()
        }
    }

    pub fn scan_available(&mut self)
    {
        match serialport::available_ports() {
            Ok(result)=>{
                for port_info in result
                {
                    if !self.available_ports.contains(&port_info.port_name) && self.node1_port != port_info.port_name.clone() && self.node2_port != port_info.port_name.clone()
                    {
                        if !port_info.port_name.contains("/dev/ttyS")
                        {
                            println!("{}", port_info.port_name.clone());
                            self.available_ports.push(port_info.clone().port_name);
                            self.available_port_info.push(port_info);
                        }
                    }
                }
            }
            Err(_e)=>{

            }
        }
    }
    pub fn manage1(&mut self, packet : Packet)
    {
        if !self.node1_state
        {
            match self.available_ports.first() {
                Some(port_name)=>{
                    let new_node = Node::<Packet>::new();
                    self.node_checker1 = Node::<bool>::new();

                    let mut port = serialport::new(port_name.as_str(), 115200)
                        .timeout(std::time::Duration::from_millis(100))
                        .open().unwrap();

                    let mut send_ = Packet::new();
                    let mut prev_ = Packet::new();

                    self.node1.publisher = new_node.get_publisher();
                    let new_checker = self.node_checker1.get_publisher();

                    std::thread::spawn(move ||{
                        let mut state = true;

                        while state {
                            let _ = new_checker.send(true);
                            let recv_packet = new_node.subscribe();

                            let mut vec = Packet::new();
                            vec.id = recv_packet.id;
                            vec.x = recv_packet.x - prev_.x;
                            vec.y = recv_packet.y - prev_.y;
                            vec.rotation = recv_packet.rotation - prev_.rotation;
                            vec.m1 = recv_packet.m1 - prev_.m1;
                            vec.m2 = recv_packet.m2 - prev_.m2;

                            if recv_packet.value_1_smooth
                            {
                                if vec.x > 0
                                {
                                    send_.x += 1
                                }
                                else if vec.x < 0
                                {
                                    send_.x -= 1
                                }
                            }
                            else {
                                send_.x = recv_packet.x;
                            }
                            if recv_packet.value_2_smooth
                            {
                                if vec.y > 0
                                {
                                    send_.y += 1
                                }
                                else if vec.y < 0
                                {
                                    send_.y -= 1
                                }
                            }
                            else {
                                send_.y = recv_packet.y;
                            }
                            if recv_packet.value_3_smooth
                            {
                                if vec.rotation > 0
                                {
                                    send_.rotation += 1
                                }
                                else if vec.rotation < 0
                                {
                                    send_.rotation -= 1
                                }
                            }
                            else {
                                send_.rotation = recv_packet.rotation;
                            }
                            if recv_packet.value_4_smooth
                            {
                                if vec.m1 > 0
                                {
                                    send_.m1 += 1
                                }
                                else if vec.m1 < 0
                                {
                                    send_.m1 -= 1
                                }
                            }
                            else {
                                send_.m1 = recv_packet.m1;
                            }
                            if recv_packet.value_5_smooth
                            {
                                if vec.m2 > 0
                                {
                                    send_.m2 += 1
                                }
                                else if vec.m2 < 0
                                {
                                    send_.m2 -= 1
                                }
                            }
                            else {
                                send_.m2 = recv_packet.m2;
                            }

                            let send_str = format!("{},{},{},{},{}e",
                                recv_packet.id,
                                send_.x / 10 + 10,
                                send_.y / 10 + 10,
                                send_.rotation / 10 + 10,
                                send_.m1 / 10 + 10);
                            
                            match port.write(send_str.as_bytes()) {
                                Ok(_size)=>{
                                    println!("Write:{}", send_str.clone());
                                    let _ = port.clear(serialport::ClearBuffer::Input);
                                }
                                Err(_e)=>{
                                    state = false;
                                }
                            }

                            prev_ = send_
                        }

                        let _ = new_checker.send(false);
                        println!("Closed Serial");
                    });

                    self.node1_state = true;
                    self.node1_port = port_name.clone();
                    match &self.available_port_info[0].port_type
                    {
                        serialport::SerialPortType::UsbPort(usb)=>{
                            println!("{}",usb.manufacturer.clone().unwrap());
                            if usb.manufacturer == Some(String::from("1a86"))
                            {
                                self.node1_type = String::from("ESP-WROOM-32");
                            }
                            else {
                                self.node1_type = String::from("Arduino")
                            }
                        }
                        _=>{

                        }
                    }
                    println!("{}", self.node1_type.clone());
                    self.available_ports.remove(0);
                    self.available_port_info.remove(0);
                }
                None=>{

                }
            }
        }
        else {
            let check = self.node_checker1.subscribe();

            if check
            {
                let _ = self.node1.publisher.send(packet);
            }
            else {
                self.node1_port = String::new();
                self.node1_state = false;
            }
        }
    }
    pub fn manage2(&mut self, packet : Packet)
    {
        if !self.node2_state
        {
            match self.available_ports.first() {
                Some(port_name)=>{
                    let new_node = Node::<Packet>::new();
                    self.node_checker2 = Node::<bool>::new();

                    let mut port = serialport::new(port_name.as_str(), 115200)
                        .timeout(std::time::Duration::from_millis(100))
                        .open().unwrap();

                    let mut send_ = Packet::new();
                    let mut prev_ = Packet::new();

                    self.node2.publisher = new_node.get_publisher();
                    let new_checker = self.node_checker2.get_publisher();

                    std::thread::spawn(move ||{
                        let mut state = true;

                        while state {
                            let _ = new_checker.send(true);
                            let recv_packet = new_node.subscribe();

                            let mut vec = Packet::new();
                            vec.id = recv_packet.id;
                            vec.x = recv_packet.x - prev_.x;
                            vec.y = recv_packet.y - prev_.y;
                            vec.rotation = recv_packet.rotation - prev_.rotation;
                            vec.m1 = recv_packet.m1 - prev_.m1;
                            vec.m2 = recv_packet.m2 - prev_.m2;

                            if recv_packet.value_1_smooth
                            {
                                if vec.x > 0
                                {
                                    send_.x += 1
                                }
                                else if vec.x < 0
                                {
                                    send_.x -= 1
                                }
                            }
                            else {
                                send_.x = recv_packet.x;
                            }
                            if recv_packet.value_2_smooth
                            {
                                if vec.y > 0
                                {
                                    send_.y += 1
                                }
                                else if vec.y < 0
                                {
                                    send_.y -= 1
                                }
                            }
                            else {
                                send_.y = recv_packet.y;
                            }
                            if recv_packet.value_3_smooth
                            {
                                if vec.rotation > 0
                                {
                                    send_.rotation += 1
                                }
                                else if vec.rotation < 0
                                {
                                    send_.rotation -= 1
                                }
                            }
                            else {
                                send_.rotation = recv_packet.rotation;
                            }
                            if recv_packet.value_4_smooth
                            {
                                if vec.m1 > 0
                                {
                                    send_.m1 += 1
                                }
                                else if vec.m1 < 0
                                {
                                    send_.m1 -= 1
                                }
                            }
                            else {
                                send_.m1 = recv_packet.m1;
                            }
                            if recv_packet.value_5_smooth
                            {
                                if vec.m2 > 0
                                {
                                    send_.m2 += 1
                                }
                                else if vec.m2 < 0
                                {
                                    send_.m2 -= 1
                                }
                            }
                            else {
                                send_.m2 = recv_packet.m2;
                            }

                            let send_str = format!("{},{},{},{},{}e",
                                recv_packet.id,
                                send_.x / 10 + 10,
                                send_.y / 10 + 10,
                                send_.rotation / 10 + 10,
                                send_.m1 / 10 + 10);
                            
                            match port.write(send_str.as_bytes()) {
                                Ok(_size)=>{
                                    println!("Write:{}", send_str.clone());
                                    let _ = port.clear(serialport::ClearBuffer::Input);
                                }
                                Err(_e)=>{
                                    state = false;
                                }
                            }

                            prev_ = send_
                        }

                        let _ = new_checker.send(false);
                        println!("Closed Serial");
                    });

                    self.node2_state = true;
                    self.node2_port = port_name.clone();
                    match &self.available_port_info[0].port_type
                    {
                        serialport::SerialPortType::UsbPort(usb)=>{
                            if usb.manufacturer == None
                            {
                                self.node2_type = String::from("ESP-WROOM-32");
                            }
                            else {
                                println!("{}",usb.manufacturer.clone().unwrap());
                                self.node2_type = String::from("Arduino")
                            }
                        }
                        _=>{

                        }
                    }
                    self.available_ports.remove(0);
                    self.available_port_info.remove(0);
                }
                None=>{

                }
            }
        }
        else {
            let check = self.node_checker2.subscribe();

            if check
            {
                let _ = self.node2.publisher.send(packet);
            }
            else {
                self.node2_port = String::new();
                self.node2_state = false;
            }
        }
    }
}
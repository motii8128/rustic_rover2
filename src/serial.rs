use serialport;

use super::thread::Node;

pub struct SerialManager
{
    serial_num : usize,
    available_ports : Vec<String>,
    opend_ports : Vec<String>,
    wheel_node : Node<Packet>,
    machine_node : Node<Packet>,
    pub wheel_is_spawned : bool
}

impl SerialManager {
    pub fn new()->SerialManager
    {
        SerialManager {serial_num : 0,  available_ports: Vec::<String>::new() , opend_ports : Vec::<String>::new(), wheel_node : Node::new(), machine_node : Node::new(), wheel_is_spawned : false}
    }

    pub fn scan_available(&mut self)
    {
        match serialport::available_ports() {
            Ok(result)=>{
                for port_info in result
                {
                    if !self.opend_ports.contains(&port_info.port_name) && !self.available_ports.contains(&port_info.port_name)
                    {
                        if !port_info.port_name.contains("/dev/ttyS")
                        {
                            self.available_ports.push(port_info.port_name);
                        }
                    }
                }
            }
            Err(_e)=>{

            }
        }
    }
    pub fn check_ready_to_serial(&self)->bool
    {
        self.serial_num != 0
    }

    pub fn spawn_wheel(&mut self)
    {
        match self.available_ports.first() {
            Some(path)=>{
                let selected = path.clone();
                let new_node = Node::<Packet>::new();
                self.wheel_node.publisher = new_node.publisher;

                std::thread::spawn(move ||{
                    let mut port = serialport::new(selected, 115200).timeout(std::time::Duration::from_millis(100)).open().unwrap();

                    let mut send = Packet::new();
                    let mut history = Packet::new();

                    loop {
                        let target = new_node.subscriber.recv().unwrap();
                        
                        let mut vec = Packet::new();
                        vec.x = target.x+100 - history.x;
                        vec.y = target.y+100 - history.y;
                        vec.rotation = target.rotation+100 - history.rotation;

                        if vec.x > 0
                        {
                            send.x += 1;
                        }
                        else if vec.x < 0
                        {
                            send.x -= 1;
                        }
                        if vec.y > 0
                        {
                            send.y += 1;
                        }
                        else if vec.y < 0
                        {
                            send.y -= 1;
                        }
                        if vec.rotation > 0
                        {
                            send.rotation += 1;
                        }
                        else if vec.rotation < 0
                        {
                            send.rotation -= 1;
                        }

                        let send_text = format!("w{},{},{}e", send.x, send.y, send.rotation);

                        match port.write(send_text.as_bytes()) {
                            Ok(_size)=>{
                                let _ = port.clear(serialport::ClearBuffer::Input);
                            }
                            Err(_e)=>{
                                let _ = port.clear(serialport::ClearBuffer::Output);
                            }
                        }
                        history = send;
                    }
                });
            }
            None=>{

            }
        }
        self.wheel_is_spawned = true;
        self.opend_ports.push(self.available_ports[0].clone());
        self.available_ports.remove(0);
    }
    pub fn spawn_machine(&mut self)
    {
        match self.available_ports.first() {
            Some(path)=>{
                let selected = path.clone();
                let new_node = Node::<Packet>::new();
                self.machine_node.publisher = new_node.publisher;

                std::thread::spawn(move ||{
                    let mut port = serialport::new(selected, 115200).timeout(std::time::Duration::from_millis(100)).open().unwrap();

                    loop {
                        let target = new_node.subscriber.recv().unwrap();

                        let send_text = format!("m{},{},{}e", target.m1, target.m2, target.m3);

                        match port.write(send_text.as_bytes()) {
                            Ok(_size)=>{
                                let _ = port.clear(serialport::ClearBuffer::Input);
                            }
                            Err(_e)=>{
                                let _ = port.clear(serialport::ClearBuffer::Output);
                            }
                        }
                    }
                });
            }
            None=>{

            }
        }
        self.opend_ports.push(self.available_ports[0].clone());
        self.available_ports.remove(0);
    }
    pub fn exec_publisher(&self, target:Packet)
    {
        let _ = self.wheel_node.publisher.send(target);
        let _ = self.machine_node.publisher.send(target);
    }
}

#[derive(Debug,Clone, Copy)]
pub struct Packet
{
    pub x : i32,
    pub y : i32,
    pub rotation : i32,
    pub m1 : i32,
    pub m2 : i32,
    pub m3 : i32
}

impl Packet {
    pub fn new()->Packet
    {
        Packet { x: 100, y: 100, rotation: 100, m1: 100, m2: 100, m3: 100 }
    }
}
use serialport;

use super::thread::Node;

pub struct SerialManager
{
    available_ports : Vec<String>,
    opend_ports : Vec<String>,
    pub wheel_node : Node<Packet>,
    pub machine_node : Node<Packet>,
    pub wheel_is_spawned : bool
}

impl SerialManager {
    pub fn new()->SerialManager
    {
        SerialManager {available_ports: Vec::<String>::new() , opend_ports : Vec::<String>::new(), wheel_node : Node::new(), machine_node : Node::new(), wheel_is_spawned : false}
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
                            println!("{}", port_info.port_name.clone());
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
        self.available_ports.len() != 0
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
                    let mut vec = Packet::new();

                    loop {
                        let target = match new_node.subscriber.recv() {
                            Ok(p)=>{
                                println!("{:?}", p);
                                p
                            }
                            Err(_e)=>{
                                Packet::new()
                            }
                        };
                        
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

                        let send_text = format!("{},{},{}e", 
                            (send.x - 100) as f32 / 100.0,
                            (send.y - 100) as f32 / 100.0,
                            (send.rotation - 100) as f32 / 100.0);

                        match port.write(send_text.as_bytes()) {
                            Ok(_size)=>{
                                println!("Write:{}", send_text);
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
                self.machine_node.publisher = new_node.publisher.clone();

                std::thread::spawn(move ||{
                    let mut port = serialport::new(selected, 115200).timeout(std::time::Duration::from_millis(100)).open().unwrap();

                    loop {
                        let target = new_node.subscriber.recv().unwrap();

                        let send_text = format!("{},{},{}e", target.m1+100, target.m2+100, target.m3+100);

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
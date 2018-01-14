use std::net::UdpSocket;
//use std::{thread, str, time};
use std::collections::HashMap;
use std::time::Duration;
use std::str;

use serde_json::{to_string as json, from_str as from_json};

enum States {
	Init,
	Discovering,
	SelectingChannel,
	CreatingChannel,
	Chatting,
}

enum Roles {
	Slave,
	Knight,
	King,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub header: String,
	pub body: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
	pub header: String,
	pub body: HashMap<String, String>,
}

#[derive(Debug)]
struct Channel {
	pub knight: String,
	pub king: String,
}

pub trait ServerProtocol {
	fn handle_request(&self, request: Request) -> Response;
}


pub struct Server {
	state: States,
	selected_channel: String,
	role: Roles,
	socket: UdpSocket,
	queue: Vec<Request>,
	channels: HashMap<String,Channel>,
}

impl Server {
	pub fn new() -> Server {
		let sock = UdpSocket::bind("0.0.0.0:12345").unwrap();
		sock.set_broadcast(true).expect("Couldn't set broadcast on socket.");

		Server {
			state: States::Init,
			selected_channel: String::new(),
			role: Roles::Slave,
			socket: sock,
			queue: Vec::new(),
			channels: HashMap::new(),
		}
	}

	fn add_channel(&mut self, name: String, king: String) {
		&self.channels.insert(name, Channel {
			king: king,
			knight: String::new(),
		});
	}

	fn discover(&self) {
	}

	pub fn shake(&mut self) {
		let mut buff = [0; 255];
		let body = HashMap::new();
		let mut discover_msg = Response {
			header: "DISCOVER".to_owned(),
			body: body,
		};

		self.socket.send_to(json(&discover_msg).unwrap().as_bytes(), "192.168.1.255:12345");
		self.socket.set_read_timeout(Some(Duration::new(10, 0)));
		let mut channels = vec![0];
		self.socket.set_broadcast(true).expect("Couldn't set broadcast on socket.");
		loop {
			match self.socket.recv_from(&mut buff) {
				Ok(t) => {
					let (bytes_n, src_addr) = t;
					let request: Request = from_json(str::from_utf8(&buff[0..bytes_n]).unwrap()).unwrap();
					match &request.header[..] {
						"HEREIAM" => {
							&self.add_channel(request.body.get("name").unwrap().to_string(), src_addr.to_string());
						},
						_ => {
							println!("Useless message");
						}
					}
					//println!("Data recieved ({}) from {}: {:?}", bytes_n, src_addr, request.header);
				},
				Err(e) => {
					break;
				}
			}
		}
		println!("{:?}", &self.channels);
	}

	pub fn listen<S>(&self, p: S) {
	}
}

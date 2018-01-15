use std::net::UdpSocket;
//use std::{thread, str, time};
use std::collections::HashMap;
use std::time::Duration;
use std::{str, io};

use serde_json::{to_string as json, from_str as from_json};

enum States {
	Init,
	Discovering,
	SelectingChannel,
	CreatingChannel,
	Connected,
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

	fn create_channel (&mut self, mut name: String) -> Result<&'static str, &'static str> {
		match name.len() {
			0 => {
				println!("Channel must be named somehow.");
				return Err("Zero length name.");
			},
			_ => {
				self.selected_channel = name;
				self.role = Roles::King;
				self.state = States::Connected;
				return Ok("Channel created");
			},
		}
	}

	fn select_channel (&mut self) -> Result<&'static str, &'static str> {
		println!("Choose channel or type new name if you wish to create a new one.");
		println!("Avaliable channels:\n-----");
		for chann in self.channels.keys() {
			println!("{}", chann);
		}
		println!("-----");
		let mut chann_name = String::new();
		match io::stdin().read_line(&mut chann_name) {
			Ok(len) => {
				chann_name.pop(); // remove newline
				match self.channels.get(&chann_name) {
					Some(x) => {
						self.selected_channel = String::from(chann_name);
						self.role = Roles::Slave;
						self.state = States::Connected;
						return Ok("Selected");
					},
					None => { },
				}
				&self.create_channel(chann_name);
				Ok("Whatever.")
			},
			Err(_) => { return Err("Whoops"); },
		}

	}

	pub fn shake(&mut self) {
		let mut buff = [0; 8192];
		let body = HashMap::new();
		let mut discover_msg = Response {
			header: "DISCOVER".to_owned(),
			body: body,
		};

		self.socket.send_to(json(&discover_msg).unwrap().as_bytes(), "192.168.1.255:12345");
		self.socket.set_read_timeout(Some(Duration::new(5, 0)));
		let mut channels = vec![0];
		self.socket.set_broadcast(true).expect("Couldn't set broadcast on socket.");

		// Listen for DISCOVERY Answers
		loop {
			match self.socket.recv_from(&mut buff) {
				Ok(t) => {
					let (bytes_n, src_addr) = t;
					let request: Request = from_json(str::from_utf8(&buff[0..bytes_n]).unwrap()).unwrap();
					match &request.header[..] {
						"HEREIAM" => {
							match request.body.get("name") {
								Some(name) => self.add_channel(name.to_string(), src_addr.to_string()),
								None => {}
							}
						}
						_ => {}, 
					}
					//println!("Data recieved ({}) from {}: {:?}", bytes_n, src_addr, request.header);
				},
				Err(e) => {
					break;
				}
			}
		}

		match self.channels.len() {
			0 => {
				println!("None channel discovered.");
				println!("Enter channel name:");
				let mut name = String::new();
				io::stdin().read_line(&mut name).unwrap();
				name.pop(); //remove newline
				self.create_channel(name);
			},
			_ => { self.select_channel(); },
		}
		loop {
			/**
			 * Fucking deadlock coz it sends when recving
			 * needed to rewrite to Request protocol and use threads
			 */
			match self.socket.recv_from(&mut buff) {
				Ok(t) => {
					let (bytes_n, src_addr) = t;
					let request: Request = from_json(str::from_utf8(&buff[0..bytes_n]).unwrap()).unwrap();
					match &request.header[..] {
						"DISCOVER" => {
							/*
							match self.role {
								Roles::King => {
									*/
									let body = HashMap::new();
									let mut hereiam_msg = Response {
										header: "HEREIAM".to_owned(),
										body: body,
									};
									self.socket.send_to(json(&hereiam_msg).unwrap().as_bytes(), &src_addr);
									/*
								},
								_ => {},
							}
							*/
						}
						_ => {}, 
					}
					//println!("Data recieved ({}) from {}: {:?}", bytes_n, src_addr, request.header);
				},
				Err(e) => {
					println!("{:?}", e);
					break;
				}
			}
		}

	}

	pub fn listen<S>(&self, p: S) {
	}
}

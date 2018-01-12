use std::net::UdpSocket;
//use std::{thread, str, time};
use std::collections::HashMap;

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

pub struct Request {
	pub header: String,
	pub body: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Result {
	pub header: String,
	pub body: HashMap<String, String>,
}

pub trait ServerProtocol {
	fn handle_request(&self, request: Request) -> Result;
}


pub struct Server {
	state: States,
	selected_channel: String,
	role: Roles,
	socket: UdpSocket,
	queue: Vec<Request>,
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
		}
	}

	pub fn shake(&self) {

	}

	pub fn listen<S>(&self, p: S) {
		/*
		let mut buff = [0; 255];
		loop {
			println!("Opening recv");
			let (bytes_n, src_addr) = socket.recv_from(&mut buff).expect("Didn't rcv data");
			println!("Data recieved ({}) from {}: {:?}", bytes_n, src_addr, str::from_utf8(&buff[0..bytes_n]));
		};
		*/
	}
}

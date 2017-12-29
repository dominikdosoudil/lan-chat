extern crate pnet_datalink;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io;
use std::time::Duration;

use pnet_datalink::{channel, Channel, NetworkInterface, Config, interfaces, ChannelType};

fn run_server() {
	fn handle_client(s: TcpStream) {
		println!("Handling stream");	
	}

	let listener = TcpListener::bind("0.0.0.0:8000").unwrap();

	for stream in listener.incoming() {
		match stream {
			Ok(x) => handle_client(x),
			_ => println!("whoops"),
		}
	}
}

fn main() {
	fn send(text: String) {
		println!("Sending: {}", text);
	}

//	thread::spawn(|| {
//		
//		loop {
//			let mut input = String::new();
//			match io::stdin().read_line(&mut input) {
//				Ok(len) => send(input),
//				Err(error) => println!("Fuking error {:?}", error),
//			}
//		}
//		println!("loop killed");
//	});
	
	let ifcs = interfaces();

	for ifc in &ifcs { println!("{:?}", ifc.name); }
	
	println!("Select interface:");
	let mut ifcname = String::new();
	match io::stdin().read_line(&mut ifcname) {
		Ok(len) => {
			ifcname.pop(); // remove newline char
			match ifcs.iter().find(|&x| x.name == ifcname) {
				Some(ifc) => {
					let config = Config {
						write_buffer_size: 32,
						read_buffer_size: 32,
						read_timeout: Some(Duration::new(5, 0)),
						write_timeout: Some(Duration::new(5, 0)),
						channel_type: ChannelType::Layer2,
						bpf_fd_attempts: 3,
					};
					let chann = channel(&ifc, config);
				},
				None => println!("Interface not found."),
			}
		},
		Err(error) => println!("{:?}", error),
	};
}

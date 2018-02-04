#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::net::UdpSocket;

use std::thread;
use std::time;

use std::str;

mod server;

use server::types::IServer; // needed so Server methods are useable, dunno why or what


fn main() {
	thread::spawn(move || {
		let mut i = 0;
		loop {
			let sock = UdpSocket::bind("192.168.1.0:12346").unwrap();
			sock.set_broadcast(true).expect("Couldn't set broadcast on socket.");
			sock.set_nonblocking(true).expect("Couldn't make socket nonblocking.");
			i += 1;
			match i {
				5 => {
					println!("{:?}", sock.send_to("{\"header\":{\"kind\":\"Discover\",\"sender\":{\"ipv4\":\"192.168.1.102:12346\",\"name\":\"user6\",\"uid\":6}},\"body\":{}}".as_bytes(), "192.168.1.255:12345"));
				},
				10 => break,
				_ => {
					sock.send_to("saying hello".as_bytes(), "192.168.1.255:12345");
				}
			}
			thread::sleep(time::Duration::from_millis(300));
		}
	});

	//	let sock = UdpSocket::bind("0.0.0.0:12347").unwrap();
	//	sock.set_broadcast(true).expect("Couldn't set broadcast on socket.");
	//	let mut buff = [0;255];
	//	loop {
	//		match sock.recv_from(&mut buff) {
	//			Ok((n, sender)) => {
	//				println!("[debug] {:?}", str::from_utf8(&buff[0..n]));
	//			},
	//			Err(e) => println!("{:?}", e),
	//		}
	//	}
	match server::Server::new() {
		Ok(srv) => {
			match srv.start() {
				Ok((tx, socket)) => {
					loop {
						match socket.recv() {
							Ok(data) => {}, //println!("{:?}", data),
							Err(e) => { println!("{:?}", e); break; },
						}
					}
				},
				Err(e) => println!("{:?}", e),
			}
		},
		Err(msg) => println!("{}", msg),
	}
}

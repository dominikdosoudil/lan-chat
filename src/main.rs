use std::net::UdpSocket;
//use std::thread;
//use std::io;
use std::env;
use std::str;

fn main() {

	
	let args: Vec<String> = env::args().collect();
	
	match &args[1][..] {
		"srv" => {
			let socket = UdpSocket::bind("0.0.0.0:8001").unwrap();
			socket.set_broadcast(true).expect("Whoops");
			
			let mut buff = [0; 255];
			loop {
				println!("Opening recv");
				let (bytes_n, src_addr) = socket.recv_from(&mut buff).expect("Didn't rcv data");
				println!("Data recieved ({}) from {}: {:?}", bytes_n, src_addr, str::from_utf8(&buff[0..bytes_n]));
			};
		},
		"client" => {	
			let socket = UdpSocket::bind("0.0.0.0:8002").unwrap();
			socket.set_broadcast(true).expect("Whoops");

			println!("Sending DISC");
			socket.send_to("DISCOVER CHAT NODES".as_bytes(), "192.168.1.255:8001").unwrap();
		},
		_ => println!("srv|client"),
	}

	
//	thread::spawn(|| {
//		
//		loop {
//			let mut input = String::new();
//			match io::stdin().read_line(&mut input) {
//				Ok(_) => send(input),
//				Err(error) => println!("Fuking error {:?}", error),
//			}
//		}
//		println!("loop killed");
//	});
}

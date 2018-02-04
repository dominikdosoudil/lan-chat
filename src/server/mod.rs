use std::collections::HashMap;
use std::io::ErrorKind;
use std::net::{UdpSocket, SocketAddr};

use std::str;

use std::{thread, time};
use std::sync::mpsc;
use self::types::{IServer, Request, Node, RequestKind, ServerState, ServerError, Message};

use serde_json;
use serde_json::{to_string as json, from_str as from_json};

pub mod types;

pub struct Server {
	socket: UdpSocket,
	state: ServerState,
	//request_sender: mpsc::SyncSender<Request>,
}

impl IServer for Server {
	fn new() -> Result<Box<Server>, &'static str> {
		match UdpSocket::bind("0.0.0.0:12345") {
			Err(e) => return match e.kind() {
					ErrorKind::AddrInUse => Err("Port already in use"),
					_ => { println!("{:?}", e); Err("Unknown Error") },
				},
			Ok(socket) => {
				println!("{:?}", socket);
				// it's expected that this will always work
				socket.set_broadcast(true).expect("Couldn't set broadcast on socket.");
				socket.set_nonblocking(true).expect("Couldn't make socket nonblocking.");

				Ok(Box::new(Server {
					socket,
					state: ServerState::Initialized,
				}))
			}
		}
	}

	fn handle_request(
		&self,
		msg_tx: &mpsc::SyncSender<Message>,
		req_tx: &mpsc::SyncSender<Request>,
		request: Request
	) -> Result<(), ()> {
		match request.header.kind() {
			&RequestKind::Message => {
				msg_tx.try_send(Message::new(String::from("Test Message"), request.header.sender));
			},
			&RequestKind::Discover => {
				req_tx.send(Request::new(
					RequestKind::HereIAm,
					request.header.sender,
					HashMap::new(),
				)).unwrap();
			}
			_ => println!("Unknow Request kind"),
		
		}
		Ok(())
	}

	fn start(self) -> Result<(mpsc::SyncSender<String>, mpsc::Receiver<Message>), ServerError>{
		let (srv_tx, client_rx) = mpsc::sync_channel::<Message>(200);
		let (client_tx, srv_rx) = mpsc::sync_channel::<String>(200);

		let disc_req = Request::new(
			RequestKind::Discover,
			Node::new("192.168.1.102:12346".parse().unwrap(), "user6", 6),
			HashMap::new(),
		);
		
		let (request_sender, request_queue) = mpsc::sync_channel::<Request>(200);
		

		thread::Builder::new().name("network_communication".to_string()).spawn(move || {
			loop {
				thread::sleep(time::Duration::from_millis(500));
				match self.try_get_request() {
					Some(request) => {
						println!("{:?}", request);
						self.handle_request(&srv_tx, &request_sender, request);
					},
					None => {}, //println!("No request"),
				}
				match request_queue.try_recv() {
					Ok(request) => {
						self.send_request(request);
						//println!("will send request: {:?}", request.header.kind());
					},
					Err(e) => {}, //println!("Q empty."),
				}
			}

		});
		Ok((client_tx, client_rx))
	}

	fn try_get_request(&self) -> Option<Request> {
		let mut buff = [0; 8192];
		match self.socket.recv_from(&mut buff) {
			Ok((n, senderIp)) if senderIp == self.socket.local_addr().unwrap() => {
				println!("same");
				None
			},
			Ok((n, SocketAddr::V4(senderIp))) => {
				println!("{:?} {:?}", senderIp, self.socket.local_addr().unwrap());
				match from_json::<Request>(str::from_utf8(&buff[0..n]).unwrap()) {
					Ok(request) => Some(request),
					Err(_) => { /* println!("didn't parse {}",str::from_utf8(&buff[0..n]).unwrap()); */ None },
				}
			},
			Ok((_, SocketAddr::V6(_))) => {
				println!("Ipv6 not supported");
				None
			}
			Err(ref e) if e.kind() == ErrorKind::WouldBlock => { println!("would block"); None },
			Err(e) => panic!("Unhandled io err: {}", e),
		}
	}

	fn send_request(&self, request: Request) -> Result<(), &'static str> {
		println!("sending request | bc: {}", self.socket.broadcast().unwrap());	
		self.socket.send_to(json(&request).unwrap().as_bytes(), "192.168.1.255:12345");

		Ok(())
	}
}

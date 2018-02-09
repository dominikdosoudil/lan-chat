use std::collections::HashMap;
use std::sync::mpsc::{Receiver, SyncSender};
use std::net::SocketAddrV4;
use std::str;
use serde_json::{to_string as json, from_str as from_json};

pub enum ServerState {
	Initialized,
	Discovering,
	WaitingForChannelSelect,
	Handshaking,
	Connected,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
	pub ipv4: SocketAddrV4,
	pub name: String,
	pub uid: u32, // we could use u16 but we know what happened with IP...
}

impl Node {
	// TODO maybe change to String (we will see what's better for udp communication and to_bytes etc.)
	pub fn new(ipv4: SocketAddrV4, name: &'static str, uid: u32) -> Self {
		Node { 
			ipv4,
			name: String::from(name),
			uid,
		} 
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum RequestKind {
	Discover,
	HereIAm,
	Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestHeader {
	kind: RequestKind,
	pub sender: Node,
}

impl RequestHeader {
	pub fn kind(&self) -> &RequestKind {
		&self.kind
	}
	pub fn of_kind(&self, k: RequestKind) {
		k == self.kind;
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub header: RequestHeader,
	pub body: HashMap<String, String>,
}

impl Request {
	pub fn new(kind: RequestKind, sender: Node, body: HashMap<String, String>) -> Self {
		Request {
			header: RequestHeader { kind, sender },
			body,
		}
	}

	pub fn from_utf8(buff: &[u8]) -> Result<Self, ServerError> {
		match str::from_utf8(&buff)	{
			Ok(jsonRequest) => {
				match from_json::<Self>(jsonRequest) {
					Ok(request) => return Ok(request),
					Err(_) => return Err(ServerError::RequestParseFail),
				}
			},
			Err(_) => return Err(ServerError::RequestParseFail),
		}
	}

	pub fn as_bytes(&self) -> &[u8] {
		//json(&self).unwrap().as_bytes()
		match json(&self) {
			Ok(jsonRequest) => { &jsonRequest.as_bytes() },
			Err(e) => panic!("Cannot jsonify Request ({})", e),
		}
	}
}

#[derive(Debug)]
struct Channel {
	pub uid: u32,
	pub name: String,
	pub knight: Node,
	pub king: Node,
}

#[derive(Debug)]
pub enum ServerError {
	AlreadyStarted,
	RequestParseFail,
}

#[derive(Debug)]
pub struct Message {
	sender: Node,
	text: String,
}

impl Message {
	pub fn new(text: String, sender: Node) -> Self {
		Message {
			sender,
			text,
		}
	}
}

pub trait IServer {
	
	fn new() -> Result<Box<Self>, &'static str>;

	/**
	 * Recv json data from UDP socket and make Request from it
	 * nonblocking
	 */
	fn try_get_request(&self) -> Option<Request>;

	/**
	 * Make json from Request and send it to UDP socket
	 * nonblocking
	 */
	fn send_request(&self, request: Request) -> Result<(), &'static str>;

	fn start(self) -> Result<(SyncSender<String>, Receiver<Message>), ServerError>;

	fn handle_request(
		&self,
		msg_tx: &SyncSender<Message>,
		req_tx: &SyncSender<Request>,
		request: Request
	) -> Result<(), ()>;

//	/**
//	 * Discover channels and then give map of them
//	 * State: -> DISCOVERING -> WAITING_FOR_CHANNEL_SELECT
//	 */
//	fn find_channels(&self) -> HashMap<u32,Channel>;
//
//	/**
//	 * Allow user to select a channel by passing the one's uid.
//	 * Should verify, that user can connect to the channel, if yes, do it.
//	 * State: -> HANDSHAKING -> CONNECTED
//	 */
//	fn select_channel(&self, channel_uid: u32) -> Result<bool, &'static str>;
//
//	/**
//	 * Returns message from someone else
//	 */
//	fn try_get_message(&self) -> Option<Message>;
//
//	/**
//	 * Sends message to channel
//	 */
//	fn send_message(&self, message: String) -> Result<bool, &'static str>;
}


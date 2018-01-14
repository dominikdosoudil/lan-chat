#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod server;

use std::collections::HashMap;
use server::ServerProtocol;
use serde_json::to_string;
struct Protocol {}

impl ServerProtocol for Protocol {
	fn handle_request(&self, request: server::Request) -> server::Response {
		let mut body = HashMap::new();
		body.insert("content".to_string(), "Hello fucking world".to_string());
		server::Response {
			header: String::from("MESSAGE"),
			body: body,
		}
	}
}

fn main() {
	let mut srv = server::Server::new();
	srv.shake();

	/*
	let mut map: HashMap<String, String> = HashMap::new();

	map.insert("a".to_string(), "b".to_string());

	println!("{:?}", to_string(&map));
	*/
}

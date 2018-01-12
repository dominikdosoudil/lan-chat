use std::collections::HashMap;


mod server;

use server::ServerProtocol;

struct Protocol {}

impl ServerProtocol for Protocol {
	fn handle_request(&self, request: server::Request) -> server::Result {
		let mut body = HashMap::new();
		body.insert("content".to_string(), "Hello fucking world".to_string());
		server::Result {
			header: String::from("MESSAGE"),
			body: body,
		}
	}
}

fn main() {
	let srv = server::Server::new();
	srv.shake();
	srv.listen(p);

}

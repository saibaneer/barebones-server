use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;
// use std::io::{Read, Write};
pub struct Server<'a> {
    socket_address: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server {
            socket_address: socket_addr,
        }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_address).unwrap();
        println!("Running on {}", self.socket_address);
        // Listen to incoming connections in a loop
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buffer = [0; 90];

            stream
                .read(&mut read_buffer)
                .expect("Failed to read from stream");
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }
}

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    //write to stream
    stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap(); //read from it
    println!(
        "Got response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}

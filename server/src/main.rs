use std::{
    fs,
    io::{BufReader, Write},
    net::TcpListener,
    thread,
};

use serde::Deserialize;

fn main() {
    let file = fs::File::open("input.json").unwrap();
    let reader = BufReader::new(file);
    let input: Input = serde_json::from_reader(reader).unwrap();

    let handles = input
        .servers
        .into_iter()
        .map(|server| thread::spawn(move || serve(&server.socket_address, &server.response)))
        .collect::<Vec<thread::JoinHandle<_>>>();

    for handle in handles {
        handle.join().unwrap();
    }
}

fn serve(socket_addr: &str, response: &Response) {
    let listener = TcpListener::bind(socket_addr).unwrap();
    println!("Listening on {}", socket_addr);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let buf = match response {
            Response::String(s) => s.as_bytes(),
            Response::Bytes(b) => b.as_slice(),
        };
        stream.write_all(buf).unwrap();
    }
}

#[derive(Debug, Deserialize)]
pub struct Input {
    pub servers: Vec<Server>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub socket_address: String,
    pub response: Response,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response {
    String(String),
    Bytes(Vec<u8>),
}

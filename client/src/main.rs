use std::{io::Read, net::TcpStream};

use indexmap::IndexMap;
use serde::Serialize;
use serde_json::to_string_pretty;

fn main() {
    let Some(socket_addr) = std::env::args().nth(1) else {
        panic!("Missing socket address argument");
    };

    let mut stream = TcpStream::connect(socket_addr).unwrap();
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).unwrap();

    let response: IndexMap<String, ResponseItemValue> = buf
        .into_iter()
        .enumerate()
        .map(|(i, byte)| {
            let response_item_value = if byte.is_ascii_digit() {
                ResponseItemValue::Digit(byte - b'0')
            } else {
                ResponseItemValue::Char(byte as char)
            };
            (i.to_string(), response_item_value)
        })
        .collect();

    let output_string = to_string_pretty(&response).unwrap();
    println!("{}", output_string);
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ResponseItemValue {
    Digit(u8),
    Char(char),
}

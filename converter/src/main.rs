use std::{fs, io::BufReader};

use indexmap::IndexMap;
use serde_json::to_string_pretty;

fn main() {
    let file = fs::File::open("input.json").unwrap();
    let reader = BufReader::new(file);
    let input: input::Input = serde_json::from_reader(reader).unwrap();

    let output = output::Output::from(input);

    let output_string = to_string_pretty(&output).unwrap();
    println!("{}", output_string);

    fs::write("output.json", output_string).unwrap();
}

mod input {
    use serde::Deserialize;

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
}

mod output {
    use indexmap::IndexMap;

    use serde::Serialize;

    #[derive(Debug, Serialize)]
    pub struct Output {
        pub servers: Vec<Server>,
    }

    #[derive(Debug, Serialize)]
    pub struct Server {
        pub socket_address: String,
        pub response: IndexMap<String, ResponseItemValue>,
    }

    #[derive(Debug, Serialize)]
    #[serde(untagged)]
    pub enum ResponseItemValue {
        Digit(u8),
        Char(char),
    }
}

impl From<input::Input> for output::Output {
    fn from(input: input::Input) -> Self {
        output::Output {
            servers: input
                .servers
                .into_iter()
                .map(output::Server::from)
                .collect(),
        }
    }
}

impl From<input::Server> for output::Server {
    fn from(server: input::Server) -> Self {
        let response: IndexMap<String, output::ResponseItemValue> = match server.response {
            input::Response::String(string) => string
                .chars()
                .enumerate()
                .map(|(i, char)| {
                    let response_item_value = if char.is_ascii_digit() {
                        output::ResponseItemValue::Digit((char as u8) - b'0')
                    } else {
                        output::ResponseItemValue::Char(char)
                    };
                    (i.to_string(), response_item_value)
                })
                .collect(),
            input::Response::Bytes(bytes) => bytes
                .iter()
                .enumerate()
                .map(|(i, byte)| {
                    let response_item_value = if byte.is_ascii_digit() {
                        output::ResponseItemValue::Digit(byte - b'0')
                    } else {
                        output::ResponseItemValue::Char(*byte as char)
                    };
                    (i.to_string(), response_item_value)
                })
                .collect(),
        };

        output::Server {
            socket_address: server.socket_address,
            response,
        }
    }
}

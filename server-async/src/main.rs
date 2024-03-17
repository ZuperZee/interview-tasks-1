use serde::Deserialize;
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    task,
};

#[tokio::main]
async fn main() {
    let mut file = fs::File::open("input.json").await.unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await.unwrap();
    let input: Input = serde_json::from_slice(&buf).unwrap();

    let mut server_tasks = task::JoinSet::new();
    for server in input.servers {
        server_tasks.spawn(async move { serve(&server.socket_address, &server.response).await });
    }

    while server_tasks.join_next().await.is_some() {}
}

async fn serve(socket_addr: &str, response: &Response) {
    let listener = TcpListener::bind(socket_addr).await.unwrap();
    println!("Listening on {}", socket_addr);

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();

        let buf = match response {
            Response::String(s) => s.as_bytes(),
            Response::Bytes(b) => b.as_slice(),
        };
        stream.write_all(buf).await.unwrap();
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

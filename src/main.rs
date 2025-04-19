use chrono::NaiveDate;
use serde::Serialize;
use std::{
    io::Read,
    net::{TcpListener, TcpStream}, fmt::Display,
};

#[derive(Serialize, Debug)]
struct ResponseStatus {
    protocol: String,
    status_code: u16,
    status_message: String,
}

#[derive(Serialize, Debug)]
struct ResponseHeaders {
    server_name: String,
    date: NaiveDate,
    content_length: Option<u64>,
    content_type: Option<String>,
}

#[derive(Serialize, Debug)]
struct ResponseBody {
    body: String,
}

#[derive(Serialize, Debug)]
struct Response {
    status: ResponseStatus,
    headers: ResponseHeaders,
    body: Option<ResponseBody>,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "
        
        ")
    }
}

fn handle_request(stream: TcpStream) {}

fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_request(stream);
            }
            Err(_) => {}
        }
    }
}

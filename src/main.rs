use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Serialize, Debug)]
struct ResponseStatus {
    http_version: String,
    status_code: u16,
    status_message: String,
}

#[derive(Serialize, Debug)]
struct ResponseHeaders {
    server_name: String,
    date: DateTime<Utc>,
    content_length: u64,
    content_type: String,
}

#[derive(Serialize, Debug)]
struct ResponseBody {
    body: String,
}

#[derive(Serialize, Debug)]
struct Response {
    status: ResponseStatus,
    headers: ResponseHeaders,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<ResponseBody>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
enum HTTPMethod {
    POST,
    GET,
    DELETE,
    HEAD,
    PUT,
    PATCH,
    OPTIONS,
}

#[derive(Deserialize, Debug)]
struct RequestLine {
    method: HTTPMethod,
    request_uri: String,
    http_version: String,
}

#[derive(Deserialize, Debug)]
struct Request {
    request_line: RequestLine,
    headers: HashMap<String, String>,
    body: Option<String>,
}

fn read_request(mut stream: TcpStream) -> String {
    let mut buffer = [0; 2048];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(0) => todo!(),
        Ok(n) => n,
        Err(_) => {
            eprintln!("Failed to read the stream bytes");
            todo!()
        }
    };

    let request_str = match std::str::from_utf8(&buffer[..bytes_read]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error occurred while transforming to string {}", e);
            todo!()
        }
    };

    println!("{}", request_str);

    "test".to_string()
}

fn handle_request(mut stream: TcpStream) {
    let request_json = read_request(stream);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Start of server");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("received request");
                handle_request(stream)
            }
            Err(e) => eprintln!("connection failed: {}", e),
        }
    }
}

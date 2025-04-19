use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;

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
    GET,
    POST,
    DELETE,
    HEAD,
    PUT,
    PATCH,
    OPTIONS,
}

impl FromStr for HTTPMethod {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_uppercase().as_str() {
            "GET" => Ok(HTTPMethod::GET),
            "POST" => Ok(HTTPMethod::POST),
            "DELETE" => Ok(HTTPMethod::DELETE),
            "HEAD" => Ok(HTTPMethod::HEAD),
            "PUT" => Ok(HTTPMethod::PUT),
            "PATCH" => Ok(HTTPMethod::PATCH),
            "OPTIONS" => Ok(HTTPMethod::OPTIONS),
            _ => Err(()),
        }
    }
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

fn parse_request_line(raw: &str) -> Result<RequestLine, &'static str> {
    let parts: Vec<&str> = raw.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid request line");
    }
    let method = parts[0]
        .parse::<HTTPMethod>()
        .map_err(|_| "Unknown HTTP method")?;
    Ok(RequestLine {
        method,
        request_uri: parts[1].to_string(),
        http_version: parts[2].to_string(),
    })
}

fn read_request(stream: &mut TcpStream) -> std::io::Result<Request> {
    let mut buffer = [0u8; 4096];
    let n = stream.read(&mut buffer)?;
    let text = std::str::from_utf8(&buffer[..n])
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

    let mut lines = text.split("\r\n");
    let raw_line = lines.next().unwrap_or("");
    let request_line = parse_request_line(raw_line)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

    let mut headers = HashMap::new();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            headers.insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    let body = lines.collect::<Vec<&str>>().join("\r\n");
    let body = if body.is_empty() { None } else { Some(body) };

    Ok(Request {
        request_line,
        headers,
        body,
    })
}

fn handle_request(mut stream: TcpStream) -> std::io::Result<()> {
    let req = read_request(&mut stream)?;
    println!("Parsed request: {:?}", req);
    // TODO: Construct and write a Response to the stream here.
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on 127.0.0.1:8080");

    for conn in listener.incoming() {
        match conn {
            Ok(stream) => {
                println!("Received connection");
                if let Err(e) = handle_request(stream) {
                    eprintln!("Error handling request: {}", e);
                }
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}

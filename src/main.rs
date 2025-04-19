use std::{
    collections::HashMap,
    fmt::Write as FmtWrite,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Alias for I/O results
type IoResult<T> = std::io::Result<T>;

/// Server configuration constants
const SERVER_NAME: &str = "RustServer/0.1";
const BIND_ADDRESS: &str = "127.0.0.1:8080";

/// Represents the HTTP response status line.
#[derive(Serialize, Debug)]
struct ResponseStatus {
    version: String,
    code: u16,
    message: String,
}

/// Represents the HTTP response headers.
#[derive(Serialize, Debug)]
struct ResponseHeaders {
    server: String,
    date: DateTime<Utc>,
    content_length: usize,
    content_type: String,
}

/// Represents the HTTP response body.
#[derive(Serialize, Debug)]
struct ResponseBody {
    content: String,
}

/// Aggregates status, headers, and body into a full HTTP response.
#[derive(Serialize, Debug)]
struct Response {
    status: ResponseStatus,
    headers: ResponseHeaders,
    body: ResponseBody,
}

impl Response {
    /// Converts the Response struct into a raw HTTP response string.
    fn to_string(&self) -> String {
        let mut buf = String::new();

        // Status line
        write!(
            &mut buf,
            "{} {} {}\r\n",
            self.status.version, self.status.code, self.status.message
        )
        .unwrap();

        // Headers
        write!(&mut buf, "Server: {}\r\n", self.headers.server).unwrap();
        write!(&mut buf, "Date: {}\r\n", self.headers.date.to_rfc2822()).unwrap();
        write!(
            &mut buf,
            "Content-Length: {}\r\n",
            self.headers.content_length
        )
        .unwrap();
        write!(
            &mut buf,
            "Content-Type: {}\r\n\r\n",
            self.headers.content_type
        )
        .unwrap();

        // Body
        write!(&mut buf, "{}", self.body.content).unwrap();
        buf
    }
}

/// Supported HTTP methods.
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
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

/// Parsed request line components.
#[derive(Deserialize, Debug)]
struct RequestLine {
    method: HTTPMethod,
    uri: String,
    version: String,
}

/// Complete HTTP request.
#[derive(Deserialize, Debug)]
struct Request {
    line: RequestLine,
    headers: HashMap<String, String>,
    body: Option<String>,
}

/// Parses the raw request line into a RequestLine struct.
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
        uri: parts[1].to_string(),
        version: parts[2].to_string(),
    })
}

/// Reads from TcpStream and parses an HTTP request.
fn read_request(stream: &mut TcpStream) -> IoResult<Request> {
    let mut buffer = [0u8; 4096];
    let n = stream.read(&mut buffer)?;

    let text = std::str::from_utf8(&buffer[..n])
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;

    let mut lines = text.split("\r\n");
    let raw_line = lines.next().unwrap_or("");
    let line = parse_request_line(raw_line)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

    let mut headers = HashMap::new();
    for header in lines.by_ref() {
        if header.is_empty() {
            break;
        }
        if let Some((k, v)) = header.split_once(':') {
            headers.insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    let body_text = lines.collect::<Vec<&str>>().join("\r\n");
    let body = if body_text.is_empty() {
        None
    } else {
        Some(body_text)
    };

    Ok(Request {
        line,
        headers,
        body,
    })
}

/// Handles incoming HTTP requests and sends responses.
fn handle_request(mut stream: TcpStream) -> IoResult<()> {
    let req = read_request(&mut stream)?;
    println!("Received request: {:?}", req);

    // Construct response body
    let payload = json!({ "message": "hello from rust server" }).to_string();

    let response = Response {
        status: ResponseStatus {
            version: "HTTP/1.1".into(),
            code: 200,
            message: "OK".into(),
        },
        headers: ResponseHeaders {
            server: SERVER_NAME.to_string(),
            date: Utc::now(),
            content_length: payload.len(),
            content_type: "application/json".into(),
        },
        body: ResponseBody { content: payload },
    };

    let response_str = response.to_string();
    stream.write_all(response_str.as_bytes())?;
    stream.flush()?;
    Ok(())
}

/// Entry point: binds to address and listens for connections.
fn main() -> IoResult<()> {
    let listener = TcpListener::bind(BIND_ADDRESS)?;
    println!("Server listening on {}", BIND_ADDRESS);

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                if let Err(err) = handle_request(stream) {
                    eprintln!("Error handling request: {}", err);
                }
            }
            Err(err) => eprintln!("Connection failed: {}", err),
        }
    }
    Ok(())
}

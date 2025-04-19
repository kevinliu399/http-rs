use chrono::NaiveDate;
use std::net::TcpStream;

struct ResponseStatus {
    protocol: String,
    status_code: u16,
    status_message: String,
}

struct ResponseHeaders {
    server_name: String,
    date: NaiveDate,
    content_length: u64,
    content_type: String,
}

struct ResponseBody {
    body: String,
}

struct Response {
    status: ResponseStatus,
    headers: ResponseHeaders,
    body: ResponseBody,
}

fn main() {
    let mut server = TcpStream::connect("127.0.0.1:8080");
}

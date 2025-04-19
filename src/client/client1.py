import requests

URL = "127.0.0.1:8080"

data = {
    "request_line": {
        "method": "POST",
        "request_uri": "/example",
        "http_version": "HTTP/1.1"
    },
    "headers": {
        "Host": "localhost",
        "User-Agent": "Python"
    },
    "body": "Test body from Python"
}

response = requests.post(f"http://{URL}/", json=data)
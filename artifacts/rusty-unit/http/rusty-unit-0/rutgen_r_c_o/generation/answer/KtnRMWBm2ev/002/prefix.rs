// Answer 0

#[test]
fn test_protocol_http_len() {
    let protocol = Protocol::Http;
    let len = protocol.len();
}

#[test]
fn test_protocol_https_len() {
    let protocol = Protocol::Https;
    let len = protocol.len();
}


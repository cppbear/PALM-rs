// Answer 0

#[derive(Debug)]
enum Protocol {
    Http,
    Https,
}

pub(super) fn len(protocol: &Protocol) -> usize {
    match *protocol {
        Protocol::Http => 4,
        Protocol::Https => 5,
    }
}

#[test]
fn test_len_http() {
    let protocol = Protocol::Http;
    assert_eq!(len(&protocol), 4);
}

#[test]
fn test_len_https() {
    let protocol = Protocol::Https;
    assert_eq!(len(&protocol), 5);
}


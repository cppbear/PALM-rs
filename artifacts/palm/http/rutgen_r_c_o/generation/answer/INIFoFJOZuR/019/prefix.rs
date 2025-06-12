// Answer 0

#[test]
fn test_from_bytes_connect() {
    let input = b"CONNECT";
    let _result = Method::from_bytes(input);
}

#[test]
fn test_from_bytes_custom_extension() {
    let input = b"EXAMPLE";
    let _result = Method::from_bytes(input);
}

#[test]
fn test_from_bytes_another_custom_extension() {
    let input = b"TESTING";
    let _result = Method::from_bytes(input);
} 

#[test]
fn test_from_bytes_yet_another_custom_extension() {
    let input = b"REQUEST";
    let _result = Method::from_bytes(input);
} 

#[test]
fn test_from_bytes_longer_custom_extension() {
    let input = b"GETMETHOD";
    let _result = Method::from_bytes(input);
} 

#[test]
fn test_from_bytes_edge_case() {
    let input = b"CONNECTA"; // 8 bytes, should use extension allocation
    let _result = Method::from_bytes(input);
}


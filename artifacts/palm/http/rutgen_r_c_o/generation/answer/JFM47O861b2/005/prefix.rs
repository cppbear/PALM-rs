// Answer 0

#[test]
fn test_from_shared_max_len() {
    let input = Bytes::from_static(&vec![b'A'; MAX_LEN]);
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_single_slash() {
    let input = Bytes::from_static(b"/");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_single_star() {
    let input = Bytes::from_static(b"*");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_invalid_authority() {
    let input = Bytes::from_static(b"invalid_authority");
    match Uri::from_shared(input) {
        Err(_) => (),
        _ => panic!("Expected an error for invalid authority"),
    }
}

#[test]
fn test_from_shared_invalid_byte() {
    let input = Bytes::from_static(&[b'\x80']); // invalid byte
    match Uri::from_shared(input) {
        Err(_) => (),
        _ => panic!("Expected an error for invalid byte"),
    }
}


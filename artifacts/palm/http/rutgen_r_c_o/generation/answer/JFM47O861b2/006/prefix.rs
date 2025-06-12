// Answer 0

#[test]
fn test_from_shared_empty_bytes() {
    let input = Bytes::from_static(b"");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_single_slash() {
    let input = Bytes::from_static(b"/");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_single_asterisk() {
    let input = Bytes::from_static(b"*");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_authority_character() {
    let input = Bytes::from_static(b"valid_authority");
    let _ = Uri::from_shared(input);
}

#[test]
fn test_from_shared_max_len_valid() {
    let input: Bytes = Bytes::from_static(&[b'a'; MAX_LEN]);
    let _ = Uri::from_shared(input);
}


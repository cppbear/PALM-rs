// Answer 0

#[test]
fn test_from_bytes_empty() {
    let src: &[u8] = &[];
    from_bytes(src);
}

#[test]
fn test_from_bytes_length_one() {
    let src: &[u8] = &[b'1'];
    from_bytes(src);
}

#[test]
fn test_from_bytes_length_two() {
    let src: &[u8] = &[b'1', b'0'];
    from_bytes(src);
}

#[test]
fn test_from_bytes_length_four() {
    let src: &[u8] = &[b'1', b'0', b'0', b'0'];
    from_bytes(src);
}

#[test]
fn test_from_bytes_length_five() {
    let src: &[u8] = &[b'1', b'0', b'0', b'0', b'0'];
    from_bytes(src);
}

#[test]
fn test_from_bytes_length_six() {
    let src: &[u8] = &[b'1', b'0', b'0', b'0', b'0', b'0'];
    from_bytes(src);
}

#[test]
fn test_from_bytes_length_seven() {
    let src: &[u8] = &[b'1', b'0', b'0', b'0', b'0', b'0', b'0'];
    from_bytes(src);
}

#[test]
fn test_from_bytes_length_eight() {
    let src: &[u8] = &[b'1', b'0', b'0', b'0', b'0', b'0', b'0', b'0'];
    from_bytes(src);
}


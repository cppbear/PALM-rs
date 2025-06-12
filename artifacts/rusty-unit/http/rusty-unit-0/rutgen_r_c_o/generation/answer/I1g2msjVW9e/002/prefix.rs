// Answer 0

#[test]
fn test_create_authority_invalid_uri_char_1() {
    let input: &[u8] = &[0x31, 0x32, 0x33, 0x00]; // valid characters + invalid null byte
    let result = create_authority(input, |b| Bytes::from(b.to_vec()));
}

#[test]
fn test_create_authority_invalid_uri_char_2() {
    let input: &[u8] = &[0x74, 0x65, 0x73, 0x74, 0x20]; // valid characters + invalid space character
    let result = create_authority(input, |b| Bytes::from(b.to_vec()));
}

#[test]
fn test_create_authority_invalid_uri_char_3() {
    let input: &[u8] = &[0x61, 0x75, 0x66, 0x2f, 0x30, 0x31, 0x32, 0x20]; // valid characters + invalid space at end
    let result = create_authority(input, |b| Bytes::from(b.to_vec()));
}

#[test]
fn test_create_authority_invalid_uri_char_4() {
    let input: &[u8] = &[0x77, 0x77, 0x77, 0x2e, 0x61, 0x63, 0x63, 0x2f, 0x20]; // valid chars + invalid space at end
    let result = create_authority(input, |b| Bytes::from(b.to_vec()));
}

#[test]
fn test_create_authority_invalid_uri_char_5() {
    let input: &[u8] = &[0x70, 0x61, 0x74, 0x68, 0x3a, 0x00, 0x31]; // valid chars + null byte in the middle
    let result = create_authority(input, |b| Bytes::from(b.to_vec()));
}


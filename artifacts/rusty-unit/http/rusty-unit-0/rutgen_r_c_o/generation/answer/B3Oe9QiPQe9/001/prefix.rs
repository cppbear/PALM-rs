// Answer 0

#[test]
fn test_try_from_valid_codes() {
    let input1: &[u8] = b"200"; // Valid input for status code 200 OK
    let input2: &[u8] = b"404"; // Valid input for status code 404 Not Found
    let input3: &[u8] = b"100"; // Valid input for status code 100 Continue
    let input4: &[u8] = b"206"; // Valid input for status code 206 Partial Content
    let input5: &[u8] = b"503"; // Valid input for status code 503 Service Unavailable

    try_from(input1);
    try_from(input2);
    try_from(input3);
    try_from(input4);
    try_from(input5);
}

#[test]
#[should_panic]
fn test_try_from_too_short() {
    let input: &[u8] = b"20"; // Too short, should panic
    try_from(input);
}

#[test]
#[should_panic]
fn test_try_from_too_long() {
    let input: &[u8] = b"2000"; // Too long, should panic
    try_from(input);
}

#[test]
#[should_panic]
fn test_try_from_invalid_chars() {
    let input1: &[u8] = b"2a0"; // Contains non-digit characters, should panic
    let input2: &[u8] = b"b07"; // Contains non-digit characters, should panic
    try_from(input1);
    try_from(input2);
}

#[test]
#[should_panic]
fn test_try_from_out_of_range() {
    let input: &[u8] = b"999"; // Out of valid range, should panic
    try_from(input);
}

#[test]
fn test_try_from_zero_prefix() {
    let input1: &[u8] = b"045"; // Valid input for status code 45
    let input2: &[u8] = b"001"; // Valid input for status code 1
    try_from(input1);
    try_from(input2);
}


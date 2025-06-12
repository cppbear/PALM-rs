// Answer 0

#[test]
fn test_encode_empty_input() {
    let input = b"";
    let result = base64::encode(input);
    assert_eq!(result, "");
}

#[test]
fn test_encode_simple_string() {
    let input = b"Hello";
    let result = base64::encode(input);
    assert_eq!(result, "SGVsbG8=");
}

#[test]
fn test_encode_special_characters() {
    let input = b"Hello, World!";
    let result = base64::encode(input);
    assert_eq!(result, "SGVsbG8sIFdvcmxkIQ==");
}

#[test]
fn test_encode_binary_data() {
    let input = &[0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
    let result = base64::encode(input);
    assert_eq!(result, "AAECAwQFBg==");
}

#[test]
fn test_encode_large_input() {
    let input = vec![b'A'; 1_000_000];  // 1 million 'A's
    let result = base64::encode(&input);
    assert_eq!(result.len(), 1_334_000); // The encoded result's length should be a certain length
}

#[test]
fn test_encode_boundary_input() {
    let input = b"Man";  // "Man" should be encoded to "TWFu"
    let result = base64::encode(input);
    assert_eq!(result, "TWFu");
}

#[test]
#[should_panic]
fn test_encode_panic_on_nan() {
    let input = f32::NAN.to_le_bytes();  // Testing with NaN bytes
    base64::encode(input);
}


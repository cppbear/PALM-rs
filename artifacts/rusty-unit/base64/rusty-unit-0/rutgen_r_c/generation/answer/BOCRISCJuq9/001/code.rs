// Answer 0

#[test]
fn test_write_encoded_bytes_valid_utf8() {
    let mut output = String::new();
    let mut sink = StringSink { string: &mut output };
    
    let input = b"Hello, World!";
    let result = sink.write_encoded_bytes(input);
    assert_eq!(result, Ok(()));
    assert_eq!(output, "Hello, World!");
}

#[test]
#[should_panic]
fn test_write_encoded_bytes_invalid_utf8() {
    let mut output = String::new();
    let mut sink = StringSink { string: &mut output };

    let input = [0, 159, 146, 150]; // Invalid UTF-8 sequence
    sink.write_encoded_bytes(&input).unwrap();
} 

#[test]
fn test_write_encoded_bytes_empty_input() {
    let mut output = String::new();
    let mut sink = StringSink { string: &mut output };

    let input: &[u8] = &[];
    let result = sink.write_encoded_bytes(input);
    assert_eq!(result, Ok(()));
    assert_eq!(output, "");
} 

#[test]
fn test_write_encoded_bytes_special_characters() {
    let mut output = String::new();
    let mut sink = StringSink { string: &mut output };

    let input = "Hello, 😊!".as_bytes();
    let result = sink.write_encoded_bytes(input);
    assert_eq!(result, Ok(()));
    assert_eq!(output, "Hello, 😊!");
}


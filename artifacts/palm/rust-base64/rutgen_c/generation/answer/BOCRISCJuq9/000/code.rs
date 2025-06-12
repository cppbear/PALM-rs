// Answer 0

#[test]
fn test_write_encoded_bytes_success() {
    let mut output = String::new();
    {
        let mut sink = StringSink { string: &mut output };
        let input = b"Hello, World!";
        let result = sink.write_encoded_bytes(input);
        assert!(result.is_ok());
        assert_eq!(output, "Hello, World!");
    }
}

#[test]
fn test_write_encoded_bytes_invalid_utf8() {
    let mut output = String::new();
    {
        let mut sink = StringSink { string: &mut output };
        let input = &[0, 159, 146, 150]; // Invalid UTF-8 bytes
        let result = sink.write_encoded_bytes(input);
        assert!(result.is_err());
    }
}

#[test]
fn test_write_encoded_bytes_empty_input() {
    let mut output = String::new();
    {
        let mut sink = StringSink { string: &mut output };
        let input: &[u8] = b"";
        let result = sink.write_encoded_bytes(input);
        assert!(result.is_ok());
        assert_eq!(output, "");
    }
}


// Answer 0

#[test]
fn test_write_encoded_bytes_valid_utf8() {
    use core::fmt::Write;
    
    let mut buffer = String::new();
    {
        let mut formatter_sink = FormatterSink { f: &mut buffer };
        let result = formatter_sink.write_encoded_bytes(b"ValidUTF8");
        assert!(result.is_ok());
        assert_eq!(buffer, "ValidUTF8");
    }
}

#[test]
#[should_panic(expected = "base64 data was not utf8")]
fn test_write_encoded_bytes_invalid_utf8() {
    use core::fmt::Write;
    
    let mut buffer = String::new();
    {
        let mut formatter_sink = FormatterSink { f: &mut buffer };
        // Invalid UTF-8 sequence
        let _ = formatter_sink.write_encoded_bytes(b"\xFF\xFE\xFD");
    }
}

#[test]
fn test_write_encoded_bytes_empty() {
    use core::fmt::Write;
    
    let mut buffer = String::new();
    {
        let mut formatter_sink = FormatterSink { f: &mut buffer };
        let result = formatter_sink.write_encoded_bytes(b"");
        assert!(result.is_ok());
        assert_eq!(buffer, "");
    }
}

#[test]
fn test_write_encoded_bytes_special_characters() {
    use core::fmt::Write;

    let mut buffer = String::new();
    {
        let mut formatter_sink = FormatterSink { f: &mut buffer };
        let result = formatter_sink.write_encoded_bytes(b"!@#$%^&*()_+=-");
        assert!(result.is_ok());
        assert_eq!(buffer, "!@#$%^&*()_+=-");
    }
}


// Answer 0

#[test]
fn test_write_encoded_bytes_success() {
    struct TestWriter {
        output: String,
    }

    impl std::fmt::Write for TestWriter {
        type Error = std::fmt::Error;

        fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let encoded = b"U28gbG9uZyBhcyBXaGF0IEkgY2FuCg=="; // base64 encoded string for "So long as What I can\n"
    
    let result = writer.write_str(std::str::from_utf8(encoded).expect("base64 data was not utf8"));
    assert!(result.is_ok());
    assert_eq!(writer.output, "So long as What I can\n");
}

#[test]
#[should_panic(expected = "base64 data was not utf8")]
fn test_write_encoded_bytes_invalid_utf8() {
    struct TestWriter {
        output: String,
    }

    impl std::fmt::Write for TestWriter {
        type Error = std::fmt::Error;

        fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let invalid_encoded = b"\xFF"; // invalid UTF-8 bytes

    let _ = writer.write_str(std::str::from_utf8(invalid_encoded).expect("base64 data was not utf8"));
}


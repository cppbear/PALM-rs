// Answer 0

#[test]
fn test_empty_bytes() {
    struct TestFormatter {
        output: String,
    }

    impl std::fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = TestFormatter { output: String::new() };
    let bytes: Vec<u8> = vec![];
    let result = fmt(&bytes, &mut formatter);
    
    assert!(result.is_ok());
    assert_eq!(formatter.output, "b\"\"");
}

#[test]
fn test_bytes_with_newline() {
    struct TestFormatter {
        output: String,
    }

    impl std::fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = TestFormatter { output: String::new() };
    let bytes: Vec<u8> = vec![b'\n'];
    let result = fmt(&bytes, &mut formatter);
    
    assert!(result.is_ok());
    assert_eq!(formatter.output, "b\"\\n\"");
}

#[test]
fn test_single_byte() {
    struct TestFormatter {
        output: String,
    }

    impl std::fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = TestFormatter { output: String::new() };
    let bytes: Vec<u8> = vec![b'A'];
    let result = fmt(&bytes, &mut formatter);
    
    assert!(result.is_ok());
    assert_eq!(formatter.output, "b\"A\"");
}

#[test]
fn test_bytes_with_escapes() {
    struct TestFormatter {
        output: String,
    }

    impl std::fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = TestFormatter { output: String::new() };
    let bytes: Vec<u8> = vec![b'\\', b'"', b'\0', b'\t'];
    let result = fmt(&bytes, &mut formatter);
    
    assert!(result.is_ok());
    assert_eq!(formatter.output, "b\"\\\\\\\"\\0\\t\"");
}

#[test]
fn test_bytes_with_non_printable() {
    struct TestFormatter {
        output: String,
    }

    impl std::fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = TestFormatter { output: String::new() };
    let bytes: Vec<u8> = vec![b'\x01', b'\x7F'];
    let result = fmt(&bytes, &mut formatter);
    
    assert!(result.is_ok());
    assert_eq!(formatter.output, "b\"\\x01\\x7f\"");
}

#[test]
#[should_panic]
fn test_formatter_error() {
    struct TestFormatter;

    impl std::fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> std::fmt::Result {
            Err(std::fmt::Error)
        }
    }

    let mut formatter = TestFormatter;
    let bytes: Vec<u8> = vec![b'A'];
    let _ = fmt(&bytes, &mut formatter);
}


// Answer 0

#[test]
fn test_format_escaped_str_contents_no_escape_characters() {
    struct MockWriter {}
    struct MockFormatter {
        call_count: usize,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl MockFormatter {
        fn new() -> Self {
            Self { call_count: 0 }
        }

        fn write_string_fragment(&mut self, _writer: &mut dyn io::Write, _value: &str) -> io::Result<()> {
            self.call_count += 1;
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _char_escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter {};
    let mut formatter = MockFormatter::new();
    let result = format_escaped_str_contents(&mut writer, &mut formatter, "Hello World");

    assert!(result.is_ok());
}

#[test]
fn test_format_escaped_str_contents_with_escape_characters() {
    struct MockWriter {}
    struct MockFormatter {
        write_string_call_count: usize,
        write_char_escape_call_count: usize,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl MockFormatter {
        fn new() -> Self {
            Self {
                write_string_call_count: 0,
                write_char_escape_call_count: 0,
            }
        }

        fn write_string_fragment(&mut self, _writer: &mut dyn io::Write, _value: &str) -> io::Result<()> {
            self.write_string_call_count += 1;
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _char_escape: CharEscape) -> io::Result<()> {
            self.write_char_escape_call_count += 1;
            Err(Error::from(ErrorCode::InvalidData))
        }
    }

    let mut writer = MockWriter {};
    let mut formatter = MockFormatter::new();
    let result = format_escaped_str_contents(&mut writer, &mut formatter, "Hello, \"world\"!");

    assert!(result.is_err());
    assert_eq!(formatter.write_string_call_count, 1);
    assert_eq!(formatter.write_char_escape_call_count, 3); // for ", ", \
}

#[test]
#[should_panic]
fn test_format_escaped_str_contents_panic_on_index_out_of_bounds() {
    struct MockWriter {}
    struct MockFormatter {
        write_string_call_count: usize,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl MockFormatter {
        fn new() -> Self {
            Self {
                write_string_call_count: 0,
            }
        }

        fn write_string_fragment(&mut self, _writer: &mut dyn io::Write, _value: &str) -> io::Result<()> {
            self.write_string_call_count += 1;
            if _value.is_empty() {
                panic!("Panic: Empty string provided where value[start..i] is expected.");
            }
            Ok(())
        }

        fn write_char_escape(&mut self, _writer: &mut dyn io::Write, _char_escape: CharEscape) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter {};
    let mut formatter = MockFormatter::new();
    let _ = format_escaped_str_contents(&mut writer, &mut formatter, "Hello, \nworld!");
}


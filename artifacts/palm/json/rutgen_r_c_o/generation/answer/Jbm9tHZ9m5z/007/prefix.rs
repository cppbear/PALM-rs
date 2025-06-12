// Answer 0

#[test]
fn test_format_escaped_str_contents_escape_zero() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        should_fail: bool,
    }

    impl MockFormatter {
        fn new(should_fail: bool) -> Self {
            MockFormatter { should_fail }
        }
    }

    impl Formatter for MockFormatter {
        fn write_string_fragment<W: ?Sized + io::Write>(
            &mut self, 
            _writer: &mut W, 
            _value: &str
        ) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape<W: ?Sized + io::Write>(
            &mut self, 
            _writer: &mut W, 
            _char_escape: CharEscape
        ) -> io::Result<()> {
            if self.should_fail {
                return Err(Error::custom("failed to write char escape"));
            }
            Ok(())
        }
    }

    let input = "\u{0000}\u{0001}\u{0002}\u{0003}"; // All ASCII characters with escape == 0
    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter::new(true);

    let _result = format_escaped_str_contents(&mut writer, &mut formatter, input);
}

#[test]
fn test_format_escaped_str_contents_start_equals_i() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        should_fail: bool,
    }

    impl MockFormatter {
        fn new(should_fail: bool) -> Self {
            MockFormatter { should_fail }
        }
    }

    impl Formatter for MockFormatter {
        fn write_string_fragment<W: ?Sized + io::Write>(
            &mut self, 
            _writer: &mut W, 
            _value: &str
        ) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape<W: ?Sized + io::Write>(
            &mut self, 
            _writer: &mut W, 
            _char_escape: CharEscape
        ) -> io::Result<()> {
            if self.should_fail {
                return Err(Error::custom("failed to write char escape"));
            }
            Ok(())
        }
    }

    let input = "\"\""; // Both quotes cause a start == i scenario
    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter::new(true);

    let _result = format_escaped_str_contents(&mut writer, &mut formatter, input);
}

#[test]
fn test_format_escaped_str_contents_escape_condition() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        should_fail: bool,
    }

    impl MockFormatter {
        fn new(should_fail: bool) -> Self {
            MockFormatter { should_fail }
        }
    }

    impl Formatter for MockFormatter {
        fn write_string_fragment<W: ?Sized + io::Write>(
            &mut self, 
            _writer: &mut W, 
            _value: &str
        ) -> io::Result<()> {
            Ok(())
        }

        fn write_char_escape<W: ?Sized + io::Write>(
            &mut self, 
            _writer: &mut W, 
            _char_escape: CharEscape
        ) -> io::Result<()> {
            if self.should_fail {
                return Err(Error::custom("failed to write char escape"));
            }
            Ok(())
        }
    }

    let input = "\x00"; // Input that leads to an escape condition
    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = MockFormatter::new(true);

    let _result = format_escaped_str_contents(&mut writer, &mut formatter, input);
}


// Answer 0

#[test]
fn test_collect_str_success() {
    use std::fmt::{self, Write as FmtWrite};
    use std::io::{self, Cursor};

    struct MockFormatter {
        output: String,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut impl io::Write) -> Result<(), io::Error> {
            self.output.push('"');
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut impl io::Write) -> Result<(), io::Error> {
            self.output.push('"');
            Ok(())
        }
    }

    let mut writer = Cursor::new(Vec::new());
    let mut formatter = MockFormatter {
        output: String::new(),
    };

    let result = collect_str(&mut writer, &formatter, "test").map_err(|e| e.to_string());

    assert!(result.is_ok());
    assert_eq!(formatter.output, "\"test\"");
}

#[test]
#[should_panic]
fn test_collect_str_formatting_error() {
    use std::fmt::{self, Write as FmtWrite};
    use std::io::{self, Cursor};

    struct ErrorFormatter;

    impl ErrorFormatter {
        fn begin_string(&mut self, _writer: &mut impl io::Write) -> Result<(), io::Error> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut impl io::Write) -> Result<(), io::Error> {
            Ok(())
        }
    }

    struct FaultyWriter;

    impl io::Write for FaultyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize, io::Error> {
            // Simulate a writing error
            Err(io::Error::new(io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> Result<(), io::Error> {
            Ok(())
        }
    }

    let mut writer = FaultyWriter;
    let formatter = ErrorFormatter;

    let _ = collect_str(&mut writer, &formatter, "test");
}


// Answer 0

#[test]
fn test_end_array_with_breaking_write_all() {
    use std::io::{self, Write};
    use std::cell::RefCell;

    struct MockWriter {
        output: Vec<u8>,
        should_panic: bool,
    }

    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_panic {
                Err(io::Error::new(io::ErrorKind::Other, "mock error"))
            } else {
                self.output.extend_from_slice(buf);
                Ok(buf.len())
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter {
        output: Vec::new(),
        should_panic: true,
    };

    let mut pretty_formatter = PrettyFormatter {
        current_indent: 1,
        has_value: true,
        indent: b"  ",
    };

    let result = pretty_formatter.end_array(&mut writer);
    
    assert!(result.is_err());
}


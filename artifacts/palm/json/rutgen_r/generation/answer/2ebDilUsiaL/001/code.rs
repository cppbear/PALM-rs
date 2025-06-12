// Answer 0

fn test_indent_with_zero_iterations() {
    use std::io;
    use std::io::Cursor;

    let mut buffer = Cursor::new(Vec::new());
    let result = indent(&mut buffer, 0, b"test");
    assert!(result.is_ok());
}

fn test_indent_with_error_on_write() {
    use std::io;
    use std::io::Cursor;
    use std::cell::RefCell;

    struct FailingWrite {
        count: usize,
    }

    impl io::Write for FailingWrite {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            if self.count < 1 {
                self.count += 1;
                Err(io::Error::new(io::ErrorKind::Other, "write error"))
            } else {
                Ok(0)
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut failing_writer = FailingWrite { count: 0 };
    let result = indent(&mut failing_writer, 1, b"test");
    assert!(result.is_err());
}


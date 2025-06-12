// Answer 0

fn test_indent_with_zero_iterations() -> std::io::Result<()> {
    use std::io::Cursor;
    
    let mut buffer = Cursor::new(Vec::new());
    let n = 0;
    let s = b"test";

    // This should not panic and successfully return Ok
    let result = indent(&mut buffer, n, s);
    
    assert!(result.is_ok());
    Ok(())
}

fn test_indent_with_write_failure() -> std::io::Result<()> {
    use std::io::{Cursor, Write, Error, ErrorKind};

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(Error::new(ErrorKind::Other, "forced error"))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let mut failing_writer = FailingWriter;
    let n = 5;
    let s = b"test";

    // This should panic and return Err due to the failing writer
    let result = indent(&mut failing_writer, n, s);
    
    assert!(result.is_err());
    Ok(())
}

fn main() {
    test_indent_with_zero_iterations().unwrap();
    test_indent_with_write_failure().unwrap();
}


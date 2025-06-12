// Answer 0

#[test]
fn test_indent_success() {
    use std::io::Cursor;

    let mut writer = Cursor::new(Vec::new());
    let n = 3;
    let s = b"test\n";

    let result = indent(&mut writer, n, s);

    assert!(result.is_ok());
    assert_eq!(writer.get_ref(), b"test\ntest\ntest\n");
}

#[test]
fn test_indent_zero_iterations() {
    use std::io::Cursor;

    let mut writer = Cursor::new(Vec::new());
    let n = 0;
    let s = b"test\n";

    let result = indent(&mut writer, n, s);

    assert!(result.is_ok());
    assert_eq!(writer.get_ref(), b"");
}

#[test]
fn test_indent_one_iteration() {
    use std::io::Cursor;

    let mut writer = Cursor::new(Vec::new());
    let n = 1;
    let s = b"line\n";

    let result = indent(&mut writer, n, s);

    assert!(result.is_ok());
    assert_eq!(writer.get_ref(), b"line\n");
}

#[should_panic]
fn test_indent_panic_on_write_error() {
    use std::io::{Cursor, Write};
    struct ErrorWriter;

    impl Write for ErrorWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "error"))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let mut writer = ErrorWriter;
    let n = 1;
    let s = b"panic\n";

    let _ = indent(&mut writer, n, s);
}


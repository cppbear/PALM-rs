// Answer 0

#[test]
fn test_flush_success() {
    use std::io;

    struct Writer;

    impl Writer {
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = Writer;
    let result = writer.flush();
    assert!(result.is_ok());
}

#[test]
fn test_flush_does_not_panic() {
    use std::io;

    struct Writer;

    impl Writer {
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = Writer;
    let result = std::panic::catch_unwind(|| {
        writer.flush()
    });
    assert!(result.is_ok());
}


// Answer 0

#[test]
fn test_write_fmt_valid_input() {
    use std::fmt::{self, Write};
    use std::fmt::Formatter;

    struct BytesMut {
        buf: String,
    }

    impl Write for BytesMut {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buf.push_str(s);
            Ok(())
        }
    }

    let mut bytes_mut = BytesMut { buf: String::new() };
    let result = bytes_mut.write_fmt(format_args!("Hello, {}!", "world"));
    assert!(result.is_ok());
    assert_eq!(bytes_mut.buf, "Hello, world!");
}

#[test]
fn test_write_fmt_empty_string() {
    use std::fmt::{self, Write};

    struct BytesMut {
        buf: String,
    }

    impl Write for BytesMut {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buf.push_str(s);
            Ok(())
        }
    }

    let mut bytes_mut = BytesMut { buf: String::new() };
    let result = bytes_mut.write_fmt(format_args!(""));
    assert!(result.is_ok());
    assert_eq!(bytes_mut.buf, "");
}

#[test]
fn test_write_fmt_multiple_formats() {
    use std::fmt::{self, Write};

    struct BytesMut {
        buf: String,
    }

    impl Write for BytesMut {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buf.push_str(s);
            Ok(())
        }
    }

    let mut bytes_mut = BytesMut { buf: String::new() };
    let result = bytes_mut.write_fmt(format_args!("Number: {}, String: {}", 42, "test"));
    assert!(result.is_ok());
    assert_eq!(bytes_mut.buf, "Number: 42, String: test");
}

#[should_panic(expected = "write_str failed")]
#[test]
fn test_write_fmt_panics_on_failure() {
    struct FailingBytesMut;

    impl Write for FailingBytesMut {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error) // Simulate a write failure
        }
    }

    let mut failing_bytes_mut = FailingBytesMut;
    let _ = failing_bytes_mut.write_fmt(format_args!("This will panic"));
}


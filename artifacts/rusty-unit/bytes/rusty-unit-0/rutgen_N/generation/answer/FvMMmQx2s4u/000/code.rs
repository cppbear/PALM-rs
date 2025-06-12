// Answer 0

#[test]
fn test_write_fmt() {
    use std::fmt::{self, Write};

    struct TestBytesMut {
        buffer: String,
    }

    impl Write for TestBytesMut {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut test_bytes = TestBytesMut {
        buffer: String::new(),
    };

    let result = test_bytes.write_fmt(format_args!("Hello, {}!", "world"));
    assert!(result.is_ok());
    assert_eq!(test_bytes.buffer, "Hello, world!");
}

#[test]
#[should_panic]
fn test_write_fmt_panics_on_error() {
    use std::fmt::{self, Write};

    struct BadBytesMut;

    impl Write for BadBytesMut {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error) // Simulating a write error
        }
    }

    let mut bad_bytes = BadBytesMut;

    let _ = bad_bytes.write_fmt(format_args!("This will fail"));
}


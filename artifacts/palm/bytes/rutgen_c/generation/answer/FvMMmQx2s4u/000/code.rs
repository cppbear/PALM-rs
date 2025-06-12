// Answer 0

#[test]
fn test_write_fmt() {
    struct TestBytesMut {
        ptr: NonNull<u8>,
        len: usize,
        cap: usize,
        data: *mut Shared,
    }

    impl fmt::Write for TestBytesMut {
        #[inline]
        fn write_str(&mut self, s: &str) -> fmt::Result {
            // Mock implementation: In a real scenario, this would write `s` to the internal buffer.
            self.len += s.len();
            Ok(())
        }
        
        #[inline]
        fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
            fmt::write(self, args)
        }
    }

    let mut buffer = TestBytesMut {
        ptr: NonNull::dangling(),
        len: 0,
        cap: 100,
        data: std::ptr::null_mut(),
    };

    let result = buffer.write_fmt(format_args!("Hello, {}!", "world"));
    assert!(result.is_ok());
    assert_eq!(buffer.len, 13); // "Hello, world!" is 13 characters long.
}

#[test]
fn test_write_fmt_empty_string() {
    struct TestBytesMut {
        ptr: NonNull<u8>,
        len: usize,
        cap: usize,
        data: *mut Shared,
    }

    impl fmt::Write for TestBytesMut {
        #[inline]
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.len += s.len();
            Ok(())
        }
        
        #[inline]
        fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
            fmt::write(self, args)
        }
    }

    let mut buffer = TestBytesMut {
        ptr: NonNull::dangling(),
        len: 0,
        cap: 100,
        data: std::ptr::null_mut(),
    };

    let result = buffer.write_fmt(format_args!(""));
    assert!(result.is_ok());
    assert_eq!(buffer.len, 0); // No content added
}


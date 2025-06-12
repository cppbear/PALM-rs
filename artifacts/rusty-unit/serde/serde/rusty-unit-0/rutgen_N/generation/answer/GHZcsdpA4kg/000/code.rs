// Answer 0

#[test]
fn test_collect_str_with_string() {
    use std::fmt::{self, Write};
    use std::fmt::Display;

    struct TestWriter {
        buffer: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { buffer: String::new() }
        }
        
        fn collect_str<T>(&mut self, value: &T) -> fmt::Result
        where
            T: ?Sized + Display,
        {
            Display::fmt(value, self)
        }
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let result = writer.collect_str(&"Hello, world!");

    assert!(result.is_ok());
    assert_eq!(writer.buffer, "Hello, world!");
}

#[test]
fn test_collect_str_with_integer() {
    use std::fmt::{self, Write};
    use std::fmt::Display;

    struct TestWriter {
        buffer: String,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter { buffer: String::new() }
        }
        
        fn collect_str<T>(&mut self, value: &T) -> fmt::Result
        where
            T: ?Sized + Display,
        {
            Display::fmt(value, self)
        }
    }

    impl Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter::new();
    let result = writer.collect_str(&42);

    assert!(result.is_ok());
    assert_eq!(writer.buffer, "42");
}


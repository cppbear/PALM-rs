// Answer 0

#[test]
fn test_fmt_group_post() {
    use std::fmt::{self, Write};
    
    struct MockWriter {
        buffer: String,
    }
    
    impl MockWriter {
        fn new() -> Self {
            MockWriter { buffer: String::new() }
        }
        
        fn get_output(self) -> String {
            self.buffer
        }
    }
    
    impl Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.buffer.push_str(s);
            Ok(())
        }
    }
    
    struct MockFormatter {
        wtr: MockWriter,
    }
    
    impl MockFormatter {
        fn new() -> Self {
            MockFormatter { wtr: MockWriter::new() }
        }
        
        fn fmt_group_post(&mut self, _ast: &()) -> fmt::Result {
            self.wtr.write_str(")")
        }
    }
    
    let mut formatter = MockFormatter::new();
    formatter.fmt_group_post(&()).unwrap();
    assert_eq!(formatter.wtr.get_output(), ")");
}


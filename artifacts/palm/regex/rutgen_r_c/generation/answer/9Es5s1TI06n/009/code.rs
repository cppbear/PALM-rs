// Answer 0

#[test]
fn test_error_kind_display_group_unclosed() {
    use std::fmt::Write;
    
    struct TestWriter {
        output: String,
    }
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let error_kind = ErrorKind::GroupUnclosed;
    let mut writer = TestWriter { output: String::new() };
    
    match error_kind.fmt(&mut writer) {
        Ok(()) => assert_eq!(writer.output, "unclosed group"),
        Err(_) => panic!("Failed to write to formatter"),
    }
}

#[test]
fn test_error_kind_display_group_unopened() {
    use std::fmt::Write;
    
    struct TestWriter {
        output: String,
    }
    
    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }
    
    let error_kind = ErrorKind::GroupUnopened;
    let mut writer = TestWriter { output: String::new() };
    
    match error_kind.fmt(&mut writer) {
        Ok(()) => assert_eq!(writer.output, "unopened group"),
        Err(_) => panic!("Failed to write to formatter"),
    }
}


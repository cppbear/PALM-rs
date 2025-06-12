// Answer 0

#[test]
fn test_fmt_class_bracketed_post() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }
    }

    impl std::fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> std::fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct TestStruct<'a> {
        wtr: &'a mut MockWriter,
    }

    impl<'a> TestStruct<'a> {
        fn fmt_class_bracketed_post(&mut self, _ast: &ast::ClassBracketed) -> std::fmt::Result {
            self.wtr.write_str("]")
        }
    }

    let mut writer = MockWriter::new();
    let mut test_struct = TestStruct { wtr: &mut writer };
    let ast = ast::ClassBracketed; // Assuming this is a valid instantiation
    
    let result = test_struct.fmt_class_bracketed_post(&ast);
    assert!(result.is_ok());
    assert_eq!(writer.output, "]");
}


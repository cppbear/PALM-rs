// Answer 0

#[test]
fn test_fmt_group_pre_non_capturing() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { output: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockFormatter<'a> {
        wtr: &'a mut MockWriter,
    }

    impl<'a> MockFormatter<'a> {
        fn fmt_flags(&mut self, _flags: &Vec<ast::Flag>) -> fmt::Result {
            self.wtr.write_str("flags")
        }
    }

    let mut writer = MockWriter::new();
    let mut formatter = MockFormatter { wtr: &mut writer };
    
    let ast = ast::Group {
        kind: ast::GroupKind::NonCapturing(vec![]),
    };

    let result = formatter.fmt_group_pre(&ast);

    assert!(result.is_ok());
    assert_eq!(writer.output, "(?flags:");
}


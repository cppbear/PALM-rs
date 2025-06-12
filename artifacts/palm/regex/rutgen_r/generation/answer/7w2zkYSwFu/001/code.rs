// Answer 0

#[test]
fn test_fmt_class_set_binary_op_kind_symmetric_difference() {
    struct Writer {
        output: String,
    }

    impl Writer {
        fn new() -> Self {
            Writer {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer::new();
    let ast = ast::ClassSetBinaryOpKind::SymmetricDifference;
    let result = writer.fmt_class_set_binary_op_kind(&ast);

    assert!(result.is_ok());
    assert_eq!(writer.output, "~~");
}


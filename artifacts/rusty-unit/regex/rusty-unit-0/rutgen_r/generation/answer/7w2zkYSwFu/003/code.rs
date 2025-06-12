// Answer 0

#[test]
fn test_fmt_class_set_binary_op_kind_intersection() {
    struct MockWriter {
        output: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    struct MockFormatter {
        wtr: MockWriter,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                wtr: MockWriter::new(),
            }
        }

        fn fmt_class_set_binary_op_kind(
            &mut self,
            ast: &ast::ClassSetBinaryOpKind,
        ) -> fmt::Result {
            use ast::ClassSetBinaryOpKind::*;
            match *ast {
                Intersection => self.wtr.write_str("&&"),
                Difference => self.wtr.write_str("--"),
                SymmetricDifference => self.wtr.write_str("~~"),
            }
        }
    }

    mod ast {
        #[derive(Clone, Copy)]
        pub enum ClassSetBinaryOpKind {
            Intersection,
            Difference,
            SymmetricDifference,
        }
    }

    let mut formatter = MockFormatter::new();
    let ast = ast::ClassSetBinaryOpKind::Intersection;

    let result = formatter.fmt_class_set_binary_op_kind(&ast);
    assert!(result.is_ok());
    assert_eq!(formatter.wtr.output, "&&");
}


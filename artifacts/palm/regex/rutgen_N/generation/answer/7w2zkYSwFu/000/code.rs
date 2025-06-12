// Answer 0

#[test]
fn test_fmt_class_set_binary_op_kind_intersection() {
    struct MockWriter {
        result: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { result: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.result.push_str(s);
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

    let mut formatter = MockFormatter::new();
    formatter.fmt_class_set_binary_op_kind(&ast::ClassSetBinaryOpKind::Intersection).unwrap();
    assert_eq!(formatter.wtr.result, "&&");
}

#[test]
fn test_fmt_class_set_binary_op_kind_difference() {
    struct MockWriter {
        result: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { result: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.result.push_str(s);
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

    let mut formatter = MockFormatter::new();
    formatter.fmt_class_set_binary_op_kind(&ast::ClassSetBinaryOpKind::Difference).unwrap();
    assert_eq!(formatter.wtr.result, "--");
}

#[test]
fn test_fmt_class_set_binary_op_kind_symmetric_difference() {
    struct MockWriter {
        result: String,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { result: String::new() }
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.result.push_str(s);
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

    let mut formatter = MockFormatter::new();
    formatter.fmt_class_set_binary_op_kind(&ast::ClassSetBinaryOpKind::SymmetricDifference).unwrap();
    assert_eq!(formatter.wtr.result, "~~");
}

